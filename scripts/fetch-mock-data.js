#!/usr/bin/env node
/**
 * fetch-mock-data.js
 *
 * Fetches real match data from the OpenDota API using a Steam ID,
 * then generates a complete mock-data.json for screenshot automation.
 *
 * Usage:
 *   SCREENSHOT_STEAM_ID=76561198XXXXXXXX node scripts/fetch-mock-data.js
 *
 * Or put SCREENSHOT_STEAM_ID=... in a .env.screenshots file.
 *
 * Output: scripts/mock-data.json (gitignored)
 */

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// ---------------------------------------------------------------------------
// 1. Read Steam ID from env or .env.screenshots
// ---------------------------------------------------------------------------

function loadEnvFile(filePath) {
  if (!fs.existsSync(filePath)) return {};
  const lines = fs.readFileSync(filePath, "utf8").split("\n");
  const env = {};
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    const eq = trimmed.indexOf("=");
    if (eq === -1) continue;
    const key = trimmed.slice(0, eq).trim();
    const val = trimmed.slice(eq + 1).trim().replace(/^["']|["']$/g, "");
    env[key] = val;
  }
  return env;
}

const envFile = loadEnvFile(path.join(__dirname, "..", ".env.screenshots"));
const STEAM_ID = process.env.SCREENSHOT_STEAM_ID || envFile.SCREENSHOT_STEAM_ID;

if (!STEAM_ID) {
  console.error("Error: SCREENSHOT_STEAM_ID not set.");
  console.error("  Set it in your environment or in a .env.screenshots file:");
  console.error("  SCREENSHOT_STEAM_ID=76561198XXXXXXXX");
  process.exit(1);
}

// Steam64 → OpenDota account_id
const STEAM_BASE = 76561197960265728n;
const steam64 = BigInt(STEAM_ID);
if (steam64 <= STEAM_BASE) {
  console.error("Error: SCREENSHOT_STEAM_ID does not look like a valid Steam64 ID.");
  process.exit(1);
}
const accountId = Number(steam64 - STEAM_BASE);

// ---------------------------------------------------------------------------
// 2. Fetch match data from OpenDota
// ---------------------------------------------------------------------------

async function fetchJson(url) {
  console.log(`Fetching: ${url}`);
  const res = await fetch(url, {
    headers: { "Accept": "application/json", "User-Agent": "DotaKeeper-Screenshots/1.0" },
  });
  if (!res.ok) throw new Error(`HTTP ${res.status} from ${url}`);
  return res.json();
}

function toBackendMatch(m, index) {
  // lane_role: 1=carry, 2=mid, 3=offlane, 4=softsupp, 5=hardsupp
  // We'll alternate wins/losses slightly below the actual data
  const parseState = index < 20 ? "Parsed" : "Unparsed";
  const goalsApplicable = parseState === "Parsed" ? (Math.random() < 0.7 ? 2 : 1) : 0;
  const goalsAchieved = goalsApplicable > 0
    ? (Math.random() < 0.72 ? Math.floor(Math.random() * goalsApplicable) + 1 : 0)
    : 0;

  return {
    match_id: m.match_id,
    start_time: m.start_time,
    hero_id: m.hero_id,
    player_slot: m.player_slot,
    radiant_win: m.radiant_win,
    kills: m.kills ?? 0,
    deaths: m.deaths ?? 0,
    assists: m.assists ?? 0,
    gold_per_min: m.gold_per_min ?? 0,
    xp_per_min: m.xp_per_min ?? 0,
    duration: m.duration ?? 1800,
    game_mode: m.game_mode ?? 22,
    role: m.lane_role ?? null,
    parse_state: parseState,
    goals_achieved: goalsAchieved,
    goals_applicable: goalsApplicable,
  };
}

// ---------------------------------------------------------------------------
// 3. Generate mock goals, calendar, analysis from match data
// ---------------------------------------------------------------------------

function buildGoals(matches) {
  // Find the top 3 heroes by play count
  const heroCounts = {};
  for (const m of matches) {
    heroCounts[m.hero_id] = (heroCounts[m.hero_id] ?? 0) + 1;
  }
  const topHeroes = Object.entries(heroCounts)
    .sort((a, b) => b[1] - a[1])
    .slice(0, 3)
    .map(([id]) => parseInt(id));

  const goals = [];
  const metrics = ["LastHits", "LastHits", "Kills"];
  const targets = [52, 48, 8];
  const times = [10, 10, null];
  const modes = ["Ranked", "All", "Ranked"];

  for (let i = 0; i < topHeroes.length; i++) {
    const g = {
      id: i + 1,
      hero_id: topHeroes[i],
      metric: metrics[i],
      target_value: targets[i],
      target_time_minutes: times[i],
      item_id: null,
      game_mode: modes[i],
    };
    goals.push(g);
  }

  // Add an any-hero goal
  goals.push({
    id: goals.length + 1,
    hero_id: null,
    metric: "LastHits",
    target_value: 50,
    target_time_minutes: 10,
    item_id: null,
    game_mode: "All",
  });

  return goals;
}

function buildGoalCalendar(goals, matches) {
  const today = new Date();
  today.setHours(0, 0, 0, 0);

  return goals.map((goal) => {
    const daily_progress = [];
    for (let d = 6; d >= 0; d--) {
      const date = new Date(today);
      date.setDate(date.getDate() - d);
      const dateStr = date.toISOString().slice(0, 10);

      // Find matches for this day that apply to this goal
      const dayStart = date.getTime() / 1000;
      const dayEnd = dayStart + 86400;
      const dayMatches = matches.filter(
        (m) =>
          m.start_time >= dayStart &&
          m.start_time < dayEnd &&
          (goal.hero_id === null || m.hero_id === goal.hero_id) &&
          m.parse_state === "Parsed"
      );

      if (dayMatches.length === 0) {
        daily_progress.push({ date: dateStr, achieved: 0, total: 0 });
      } else {
        // Simulate ~72% hit rate for a realistic look
        const achieved = dayMatches.filter(() => Math.random() < 0.72).length;
        daily_progress.push({ date: dateStr, achieved, total: dayMatches.length });
      }
    }
    return { goal, daily_progress };
  });
}

function buildAnalysis(matches) {
  // Compute per-hero last-hits approximation (GPM correlates loosely)
  const heroData = {};
  for (const m of matches.filter((m) => m.parse_state === "Parsed")) {
    if (!heroData[m.hero_id]) heroData[m.hero_id] = { total: 0, count: 0 };
    // GPM / 7 is a rough CS at 10 min approximation
    const approxCS = Math.round(m.gold_per_min / 7);
    heroData[m.hero_id].total += approxCS;
    heroData[m.hero_id].count += 1;
  }

  const per_hero_stats = Object.entries(heroData)
    .filter(([, data]) => data.count > 0 && data.total > 0)
    .map(([hero_id, data]) => ({
      hero_id: parseInt(hero_id),
      count: data.count,
      average: Math.round((data.total / data.count) * 10) / 10, // avoids NaN→null in JSON
    }));

  per_hero_stats.sort((a, b) => b.average - a.average);

  const allCS = per_hero_stats.flatMap((h) =>
    Array(h.count).fill(h.average)
  );
  const currentAvg = allCS.length > 0
    ? parseFloat((allCS.reduce((a, b) => a + b, 0) / allCS.length).toFixed(1))
    : 0;

  return {
    current_period: {
      count: matches.filter((m) => m.parse_state === "Parsed").length,
      average: currentAvg,
      values: per_hero_stats.map((h) => h.average),
    },
    previous_period: {
      count: 28,
      average: parseFloat((currentAvg * 0.92).toFixed(1)),
      values: [],
    },
    per_hero_stats,
  };
}

function buildHeroSuggestion(analysis) {
  if (!analysis.per_hero_stats.length) return null;
  const top = analysis.per_hero_stats[0];
  return {
    hero_id: top.hero_id,
    current_average: top.average,
    suggested_last_hits: Math.round(top.average * 1.1),
    games_analyzed: top.count,
  };
}

function buildDailyChallenge(matches) {
  const descriptions = [
    "Get 3 kills in ranked matches",
    "Win 2 games",
    "Farm 80+ CS in 10 minutes",
    "Achieve 4+ assists in a match",
  ];
  const desc = descriptions[Math.floor(Math.random() * descriptions.length)];
  return {
    completed: false,
    current_value: 1,
    target: 3,
    challenge: { challenge_description: desc, hero_id: null },
  };
}

function buildWeeklyChallenge(matches) {
  return {
    completed: false,
    current_value: 4,
    target: 7,
    days_remaining: 3,
    challenge: {
      challenge_description: "Play 7 ranked matches",
      hero_id: null,
    },
  };
}

// Hardcoded items list (common Dota 2 items relevant to goals)
const MOCK_ITEMS = [
  { id: 1, name: "blink", display_name: "Blink Dagger" },
  { id: 50, name: "black_king_bar", display_name: "Black King Bar" },
  { id: 36, name: "power_treads", display_name: "Power Treads" },
  { id: 152, name: "phase_boots", display_name: "Phase Boots" },
  { id: 168, name: "hand_of_midas", display_name: "Hand of Midas" },
  { id: 63, name: "battlefury", display_name: "Battlefury" },
  { id: 214, name: "radiance", display_name: "Radiance" },
  { id: 208, name: "manta", display_name: "Manta Style" },
  { id: 90, name: "butterfly", display_name: "Butterfly" },
  { id: 116, name: "heart", display_name: "Heart of Tarrasque" },
];

// ---------------------------------------------------------------------------
// 4. Main
// ---------------------------------------------------------------------------

async function main() {
  const OPENDOTA = `https://api.opendota.com/api`;

  console.log(`Steam64 ID : ${STEAM_ID}`);
  console.log(`Account ID : ${accountId}`);
  console.log("");

  let rawMatches;
  try {
    rawMatches = await fetchJson(
      `${OPENDOTA}/players/${accountId}/matches?limit=30&significant=0`
    );
  } catch (e) {
    console.error(`Failed to fetch matches: ${e.message}`);
    process.exit(1);
  }

  if (!Array.isArray(rawMatches) || rawMatches.length === 0) {
    console.error(
      "No matches returned. Check that the Steam ID is correct and the profile is public."
    );
    process.exit(1);
  }

  console.log(`Fetched ${rawMatches.length} matches.`);

  const matches = rawMatches.map((m, i) => toBackendMatch(m, i));
  const goals = buildGoals(matches);
  const goal_calendar = buildGoalCalendar(goals, matches);
  const analysis = buildAnalysis(matches);
  const hero_suggestion = buildHeroSuggestion(analysis);
  const daily_challenge = buildDailyChallenge(matches);
  const daily_streak = 7;
  const weekly_challenge = buildWeeklyChallenge(matches);

  // Favorite heroes = top 3 by play count
  const heroCounts = {};
  for (const m of matches) heroCounts[m.hero_id] = (heroCounts[m.hero_id] ?? 0) + 1;
  const favorite_heroes = Object.entries(heroCounts)
    .sort((a, b) => b[1] - a[1])
    .slice(0, 3)
    .map(([id]) => parseInt(id));

  const mockData = {
    steam_id: STEAM_ID,
    matches,
    goals,
    goal_calendar,
    hero_suggestion,
    analysis,
    daily_challenge,
    daily_streak,
    weekly_challenge,
    items: MOCK_ITEMS,
    favorite_heroes,
    medal_stats: { current_rank_tier: 52 }, // Legend 2 — adjust as desired
  };

  const outPath = path.join(__dirname, "mock-data.json");
  fs.writeFileSync(outPath, JSON.stringify(mockData, null, 2));
  console.log(`\nSaved mock data to: ${outPath}`);
  console.log(`  ${matches.length} matches, ${goals.length} goals`);
  console.log(`  Top heroes: ${favorite_heroes.join(", ")}`);
  console.log("\nNext step: npm run screenshots");
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
