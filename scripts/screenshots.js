#!/usr/bin/env node
/**
 * screenshots.js
 *
 * Captures Play Store screenshots for all required device form factors.
 * Runs against the SvelteKit dev server with mocked Tauri IPC.
 *
 * Prerequisites:
 *   1. Generate mock data:  SCREENSHOT_STEAM_ID=xxx npm run screenshots:fetch
 *   2. Run this script:     npm run screenshots
 *      (The script starts the dev server automatically.)
 *
 * Output: meta/screenshots/<device>/<route>.png
 */

import { chromium } from "playwright";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { spawn, execSync } from "child_process";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.join(__dirname, "..");

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

const DEV_SERVER_PORT = 5299; // dedicated port — avoids clashing with a running dev server
let DEV_SERVER_URL = `http://localhost:${DEV_SERVER_PORT}`;
const DEV_SERVER_READY_TIMEOUT = 30000; // ms

// viewport = CSS pixels (what media queries see); deviceScaleFactor scales up to physical pixels.
// Physical output must be PNG/JPEG, 16:9 or 9:16, each side 1080–7680 px.
//   phone:      432×768  @ 2.5x → 1080×1920  (9:16) ✓
//   tablet7:    600×960  @ 2x   → 1200×1920  (5:8 — accepted by Play Store for tablets)
//   tablet10:   800×1280 @ 2x   → 1600×2560  (5:8 — accepted by Play Store for tablets)
//   chromebook: 1280×720 @ 1.5x → 1920×1080  (16:9) ✓
const DEVICE_PROFILES = [
  { name: "phone",      width:  432, height:  768, deviceScaleFactor: 2.5 },
  { name: "tablet7",   width:  600, height:  960, deviceScaleFactor: 2   },
  { name: "tablet10",  width:  800, height: 1280, deviceScaleFactor: 2   },
  { name: "chromebook", width: 1280, height:  720, deviceScaleFactor: 1.5 },
];

const ROUTES = [
  { name: "dashboard",  path: "/",         waitFor: ".goals-grid, .empty-goals, .stats-row" },
  { name: "matches",    path: "/matches",  waitFor: ".matches-table, .empty-state" },
  { name: "goals",      path: "/goals",    waitFor: ".goals-content" },
  { name: "analysis",   path: "/analysis", waitFor: ".analysis-grid, .no-data" },
];

// ---------------------------------------------------------------------------
// Load mock data
// ---------------------------------------------------------------------------

const mockDataPath = path.join(__dirname, "mock-data.json");
if (!fs.existsSync(mockDataPath)) {
  console.error("Error: scripts/mock-data.json not found.");
  console.error("  Generate it first with:");
  console.error("  SCREENSHOT_STEAM_ID=76561198XXXXXXXX npm run screenshots:fetch");
  process.exit(1);
}

const MOCK = JSON.parse(fs.readFileSync(mockDataPath, "utf8"));
console.log(`Loaded mock data: ${MOCK.matches.length} matches, ${MOCK.goals.length} goals`);

// The first parsed match and first goal for detail pages
const firstParsedMatch = MOCK.matches.find((m) => m.parse_state === "Parsed");
const firstGoal = MOCK.goals[0];

if (firstParsedMatch) {
  ROUTES.push({ name: "match-detail", path: `/matches/${firstParsedMatch.match_id}`, waitFor: ".match-detail, .match-header" });
}
if (firstGoal) {
  ROUTES.push({ name: "goal-detail", path: `/goals/${firstGoal.id}`, waitFor: ".goal-detail, .goal-header" });
}

// Analysis detail: use the most-played hero from mock data
const topHeroId = (() => {
  const counts = {};
  for (const m of MOCK.matches) counts[m.hero_id] = (counts[m.hero_id] ?? 0) + 1;
  const top = Object.entries(counts).sort((a, b) => b[1] - a[1])[0];
  return top ? parseInt(top[0]) : 1; // fallback to Anti-Mage
})();
ROUTES.push({ name: "analysis-detail", path: `/analysis/${topHeroId}`, waitFor: ".analysis-grid, .no-data" });

// ---------------------------------------------------------------------------
// Tauri IPC mock — injected into every page before app code runs
// ---------------------------------------------------------------------------

function buildTauriMockScript(mockDataJson) {
  // This runs in the browser context. We can't use module imports here,
  // so we inline everything we need.
  return `
(function() {
  const MOCK = ${mockDataJson};

  // Command handler map
  const handlers = {
    "get_settings": () => ({
      steam_id: MOCK.steam_id,
      privacy_mode: false,
      analytics_consent: "Declined", // suppress analytics in screenshots
      onboarding_completed: true,
      locale: null,
    }),
    "get_medal_stats": () => MOCK.medal_stats,
    "get_matches": () => MOCK.matches.map(m => ({
      ...m,
      last_hits: m.last_hits ?? Math.round((m.gold_per_min ?? 300) / 7),
      denies: m.denies ?? Math.round((m.gold_per_min ?? 300) / 70),
      hero_damage: m.hero_damage ?? Math.round(8000 + Math.random() * 12000),
      tower_damage: m.tower_damage ?? Math.round(500 + Math.random() * 3000),
      hero_healing: m.hero_healing ?? Math.round(Math.random() * 1000),
    })),
    "refresh_matches": () => ({ matches: MOCK.matches, new_count: 0 }),
    "get_goals": () => MOCK.goals.map(g =>
      g.metric === "LastHits" ? { ...g, target_value: 50 } : g
    ),
    "get_goal": ({ goalId }) => {
      const g = MOCK.goals.find(g => g.id === goalId) ?? MOCK.goals[0];
      return g.metric === "LastHits" ? { ...g, target_value: 50 } : g;
    },
    "get_goal_histogram_data": ({ goalId }) => {
      // Generate values in the 10-60 range centred around ~49
      const seededRand = (seed) => {
        let s = seed;
        return () => { s = (s * 1664525 + 1013904223) & 0xffffffff; return (s >>> 0) / 0xffffffff; };
      };
      const rand = seededRand(goalId ?? 1);
      return MOCK.matches.filter(m => m.parse_state === "Parsed").map(m => {
        const isRadiant = m.player_slot < 128;
        const won = (isRadiant && m.radiant_win) || (!isRadiant && !m.radiant_win);
        const value = Math.max(10, Math.min(60, Math.round(49 + (rand() - 0.5) * 40)));
        return { hero_id: m.hero_id, value, won, start_time: m.start_time };
      });
    },
    "get_goals_calendar": () => MOCK.goal_calendar,
    "get_hero_goal_suggestion": () => MOCK.hero_suggestion,
    "refresh_hero_goal_suggestion": () => MOCK.hero_suggestion,
    "get_last_hits_analysis_data": (args) => {
      // NaN/null/0 serialises badly or breaks toFixed() calls in the page.
      // safePos returns the value only if it's a positive finite number, else the fallback.
      const safePos = (v, fallback) =>
        (v !== null && v !== undefined && isFinite(v) && v > 0) ? v : fallback;

      // Realistic per-hero LH @ 10 min data (hero IDs mapped to played heroes below)
      const HERO_DATA = [
        { hero_id: 1,   average: 55.8, count: 18, trend_percentage:  6.2 },
        { hero_id: 8,   average: 52.3, count: 12, trend_percentage: -1.4 },
        { hero_id: 11,  average: 50.1, count:  8, trend_percentage:  3.1 },
        { hero_id: 44,  average: 48.6, count: 15, trend_percentage:  8.5 },
        { hero_id: 94,  average: 47.2, count:  6, trend_percentage: -3.8 },
        { hero_id: 35,  average: 45.8, count:  9, trend_percentage:  1.2 },
        { hero_id: 74,  average: 43.4, count: 11, trend_percentage: -5.1 },
        { hero_id: 109, average: 41.1, count:  7, trend_percentage:  2.7 },
      ];

      const a = MOCK.analysis || {};
      const mockIds = (a.per_hero_stats || []).map(h => h.hero_id).filter(Boolean);
      const perHeroStats = HERO_DATA.map((h, i) => ({
        ...h,
        hero_id: mockIds[i] ?? h.hero_id,
      }));

      // When filtering by hero (detail page), generate per-game data_points
      const heroId = args && args.heroId ? args.heroId : null;
      const heroEntry = heroId
        ? perHeroStats.find(h => h.hero_id === heroId) ?? perHeroStats[0]
        : null;

      const curAvg = heroEntry ? heroEntry.average : safePos((a.current_period || {}).average, 49.4);
      const prevAvg = heroEntry
        ? Math.round(curAvg * (1 - heroEntry.trend_percentage / 100) * 10) / 10
        : safePos((a.previous_period || {}).average, curAvg * 0.92);
      const count = heroEntry ? heroEntry.count : safePos((a.current_period || {}).count, 25);

      // Generate realistic game-by-game data points (used by detail page charts)
      const now = Math.floor(Date.now() / 1000);
      const seededRand = (seed) => {
        let s = seed;
        return () => { s = (s * 1664525 + 1013904223) & 0xffffffff; return (s >>> 0) / 0xffffffff; };
      };
      const rand = seededRand(heroId ?? 42);
      const dataPoints = Array.from({ length: count }, (_, i) => {
        // Slight upward trend + noise
        const trend = heroEntry ? (heroEntry.trend_percentage / 100) * (i / count) * curAvg : 0;
        const noise = (rand() - 0.5) * 30;
        const lh = Math.max(10, Math.min(60, Math.round(curAvg - trend * 0.5 + trend * (i / count) + noise)));
        return {
          last_hits: lh,
          start_time: now - (count - i) * 2 * 86400 + Math.floor(rand() * 86400),
        };
      });

      const lhValues = dataPoints.map(p => p.last_hits);
      const actualAvg = Math.round((lhValues.reduce((s, v) => s + v, 0) / lhValues.length) * 10) / 10;
      const actualMin = Math.min(...lhValues);
      const actualMax = Math.max(...lhValues);

      return {
        current_period: {
          count,
          average: actualAvg,
          max: actualMax,
          min: actualMin,
          values: lhValues,
          data_points: dataPoints,
        },
        previous_period: prevAvg > 0 ? {
          count: Math.round(count * 0.9),
          average: prevAvg,
          values: [],
        } : null,
        per_hero_stats: perHeroStats,
      };
    },
    "get_daily_challenge_progress_cmd": () => MOCK.daily_challenge,
    "get_daily_streak_cmd": () => MOCK.daily_streak,
    "get_weekly_challenge_progress_cmd": () => MOCK.weekly_challenge,
    "get_all_items": () => MOCK.items,
    "get_favorite_heroes": () => MOCK.favorite_heroes,
    "toggle_favorite_hero": ({ heroId }) => {
      const idx = MOCK.favorite_heroes.indexOf(heroId);
      if (idx === -1) MOCK.favorite_heroes.push(heroId);
      else MOCK.favorite_heroes.splice(idx, 1);
      return MOCK.favorite_heroes.includes(heroId);
    },
    "evaluate_goals_for_match": ({ matchId }) => MOCK.goals.slice(0, 2).map((g, i) => ({
      goal: g,
      achieved: i === 0,
      actual_value: i === 0 ? g.target_value + 5 : g.target_value - 8,
    })),
    "is_beta_build": () => false,
    "get_match_cs": () => {
      // Cumulative LH progression — ~4.9 LH/min so minute-10 total lands near 49
      return Array.from({ length: 10 }, (_, i) => ({
        minute: i + 1,
        last_hits: Math.round((i + 1) * 4.9 + (Math.random() * 4 - 2)),
        denies: Math.round((i + 1) * 0.5 + (Math.random() * 1 - 0.5)),
      }));
    },
    "get_tilt_assessment": () => null,
    "get_checkin_history": () => [],
    "get_pending_checkin": () => null,
    "get_os_locale": () => "en",
    "track_event": () => null,
    "identify_analytics_user": () => null,
    // Plugin commands
    "plugin:updater|check": () => null,
    "plugin:process|relaunch": () => null,
    "plugin:opener|open": () => null,
    "plugin:opener|open_url": () => null,
  };

  function mockInvoke(cmd, args) {
    const handler = handlers[cmd];
    if (handler) {
      try {
        const result = handler(args || {});
        return Promise.resolve(result);
      } catch (e) {
        return Promise.reject(String(e));
      }
    }
    // Unknown command — resolve with null so the app doesn't crash
    console.warn("[TauriMock] Unhandled command:", cmd, args);
    return Promise.resolve(null);
  }

  // Mock __TAURI_INTERNALS__ (used by @tauri-apps/api/core invoke)
  let _callbackId = 1;
  window.__TAURI_INTERNALS__ = {
    invoke: mockInvoke,
    transformCallback: (callback, once) => {
      const id = _callbackId++;
      window["_tauriCb_" + id] = once
        ? (...args) => { delete window["_tauriCb_" + id]; callback(...args); }
        : callback;
      return id;
    },
    listen: (event, handler, options) => {
      return Promise.resolve(() => {});
    },
    convertFileSrc: (path) => path,
    metadata: {
      currentWindow: { label: "main" },
      currentWebview: { label: "main" },
    },
  };

  // Some Tauri plugin packages check window.__TAURI__ as well
  window.__TAURI__ = {
    core: { invoke: mockInvoke },
    event: {
      listen: (event, handler) => Promise.resolve(() => {}),
      once: (event, handler) => Promise.resolve(() => {}),
      emit: () => Promise.resolve(),
    },
    opener: { openUrl: () => Promise.resolve() },
    process: { relaunch: () => Promise.resolve(), exit: () => Promise.resolve() },
    updater: { check: () => Promise.resolve(null) },
  };

  // Suppress Sentry in screenshots
  window.__SENTRY_DISABLED__ = true;
})();
`;
}

// ---------------------------------------------------------------------------
// Dev server management
// ---------------------------------------------------------------------------

function killStaleViteProcesses() {
  // Free the dedicated port if something is already listening on it
  try {
    const out = execSync(`netstat -ano | findstr :${DEV_SERVER_PORT}`, { shell: true }).toString();
    for (const line of out.split("\n")) {
      const parts = line.trim().split(/\s+/);
      const pid = parts[parts.length - 1];
      if (pid && /^\d+$/.test(pid) && pid !== "0") {
        try { execSync(`taskkill /F /PID ${pid}`, { shell: true }); } catch (_) {}
      }
    }
    console.log(`Freed port ${DEV_SERVER_PORT}.`);
  } catch (_) { /* port not in use — fine */ }
}

async function pollUntilReady(url, timeoutMs) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const res = await fetch(url, { signal: AbortSignal.timeout(1000) });
      if (res.status < 500) return; // server is up
    } catch (_) {
      // not yet — keep polling
    }
    await new Promise((r) => setTimeout(r, 500));
  }
  throw new Error(`Dev server not reachable at ${url} after ${timeoutMs}ms`);
}

async function startDevServer() {
  console.log("Starting dev server...");
  const proc = spawn("npm", ["run", "dev", "--", `--port=${DEV_SERVER_PORT}`], {
    cwd: ROOT,
    shell: true,
    stdio: ["ignore", "pipe", "pipe"],
  });

  proc.stdout.on("data", (chunk) => process.stdout.write(chunk));
  proc.stderr.on("data", (chunk) => process.stderr.write(chunk));
  proc.on("error", (e) => { throw e; });

  // Port is fixed (DEV_SERVER_PORT), so just poll until it accepts connections.
  try {
    await pollUntilReady(DEV_SERVER_URL, DEV_SERVER_READY_TIMEOUT);
  } catch (e) {
    proc.kill();
    throw e;
  }

  console.log("Dev server ready.");
  return proc;
}

// ---------------------------------------------------------------------------
// Screenshot capture
// ---------------------------------------------------------------------------

async function waitForAppReady(page, routeWaitFor) {
  // 1. Wait for the layout to render as logged-in (.content-area only exists then)
  try {
    await page.waitForSelector(".content-area", { timeout: 10000 });
  } catch (_) {
    await page.waitForTimeout(3000);
    return;
  }

  // 2. Wait for any .loading-state inside content-area to disappear.
  //    This is the most reliable signal that all async invoke() calls have resolved.
  try {
    await page.waitForFunction(
      () => !document.querySelector(".content-area .loading-state"),
      { timeout: 15000 }
    );
  } catch (_) {
    // Timed out — loading state stuck, proceed anyway and screenshot what we have
  }

  // 3. Optionally confirm the expected content element is present
  if (routeWaitFor) {
    try {
      await page.waitForSelector(routeWaitFor, { timeout: 5000 });
    } catch (_) {
      // May be showing an empty/error state — still take the screenshot
    }
  }

  // 4. Settle for CSS transitions and chart rendering
  await page.waitForTimeout(1000);
}

async function captureScreenshots(mockDataJson) {
  const browser = await chromium.launch({ headless: true });

  try {
    for (const device of DEVICE_PROFILES) {
      const outDir = path.join(ROOT, "meta", "screenshots", device.name);
      fs.mkdirSync(outDir, { recursive: true });

      console.log(`\n[${device.name}] ${device.width}×${device.height}`);

      const context = await browser.newContext({
        viewport: { width: device.width, height: device.height },
        deviceScaleFactor: device.deviceScaleFactor,
        // Disable service workers that might cache old responses
        serviceWorkers: "block",
      });

      // Inject Tauri mock before any page script runs
      await context.addInitScript({ content: buildTauriMockScript(mockDataJson) });

      for (const route of ROUTES) {
        const url = `${DEV_SERVER_URL}${route.path}`;
        const outFile = path.join(outDir, `${route.name}.png`);

        process.stdout.write(`  → ${route.name} ... `);
        const page = await context.newPage();

        // Pipe browser console + errors so we can see mock/invoke failures
        page.on("console", (msg) => {
          if (msg.type() === "error" || msg.text().includes("[TauriMock]")) {
            console.log(`    [browser:${msg.type()}] ${msg.text()}`);
          }
        });
        page.on("pageerror", (err) => console.log(`    [pageerror] ${err.message}`));

        try {
          await page.goto(url, { waitUntil: "load", timeout: 15000 });
          await waitForAppReady(page, route.waitFor);
          await page.screenshot({ path: outFile, fullPage: false });
          console.log("saved");
        } catch (e) {
          console.log(`FAILED: ${e.message}`);
        } finally {
          await page.close();
        }
      }

      await context.close();
    }
  } finally {
    await browser.close();
  }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  const mockDataJson = fs.readFileSync(mockDataPath, "utf8");

  const noServer = process.argv.includes("--no-server");
  let serverProc = null;

  if (!noServer) {
    console.log("Clearing any stale Vite processes...");
    killStaleViteProcesses();
  }

  try {
    if (!noServer) {
      // Check if server is already running on the dedicated port
      try {
        const res = await fetch(DEV_SERVER_URL, { signal: AbortSignal.timeout(2000) });
        if (res.ok || res.status < 500) {
          console.log("Dev server already running.");
        }
      } catch (_) {
        // Server not running — start it
        serverProc = await startDevServer();
        _serverProc = serverProc;
      }
    }

    await captureScreenshots(mockDataJson);
  } finally {
    killServerProc(serverProc);
  }

  const outRoot = path.join(ROOT, "meta", "screenshots");
  console.log(`\nDone! Screenshots saved to: ${outRoot}`);
  console.log("Devices captured:", DEVICE_PROFILES.map((d) => d.name).join(", "));
}

function killServerProc(proc) {
  if (!proc) return;
  try {
    // On Windows, kill() only kills cmd.exe — use taskkill /T to kill the whole tree
    execSync(`taskkill /F /T /PID ${proc.pid}`, { shell: true });
    console.log("\nDev server stopped.");
  } catch (_) {
    // Fallback: also free the port directly
    try {
      const out = execSync(`netstat -ano | findstr :${DEV_SERVER_PORT}`, { shell: true }).toString();
      for (const line of out.split("\n")) {
        const parts = line.trim().split(/\s+/);
        const pid = parts[parts.length - 1];
        if (pid && /^\d+$/.test(pid) && pid !== "0") {
          try { execSync(`taskkill /F /PID ${pid}`, { shell: true }); } catch (_) {}
        }
      }
      console.log("\nDev server stopped (by port).");
    } catch (_) {}
  }
}

let _serverProc = null;

// Ensure cleanup on Ctrl+C or unexpected exit
for (const sig of ["SIGINT", "SIGTERM", "exit"]) {
  process.on(sig, () => { killServerProc(_serverProc); });
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
