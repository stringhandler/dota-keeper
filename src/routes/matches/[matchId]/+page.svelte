<script>
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { getHeroName, CORE_HERO_IDS } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";
  import Chart from "$lib/Chart.svelte";
  import BenchmarkTable from "$lib/BenchmarkTable.svelte";
  import { _ } from "svelte-i18n";
  import { pqs, initQueue, enqueueParse } from "$lib/parse-queue.svelte.js";

  let matchId = $derived(parseInt($page.params.matchId ?? '0'));
  let backHref = $derived(
    $page.url.searchParams.get('from') === 'goal' && $page.url.searchParams.get('goalId')
      ? `/goals/${$page.url.searchParams.get('goalId')}`
      : '/matches'
  );
  let backLabel = $derived(
    $page.url.searchParams.get('from') === 'goal' ? '← Goal Details' : `← ${$_('nav.matches')}`
  );
  /** @type {any} */
  let match = $state(null);
  let csData = $state(/** @type {any[]} */ ([]));
  let goalDetails = $state(/** @type {any[]} */ ([]));
  let items = $state(/** @type {any[]} */ ([]));
  let isLoading = $state(true);
  let error = $state("");

  let steamId = $state("");

  // Reactive: is this match currently being parsed?
  let isParsing = $derived(pqs.active.has(matchId));
  let parseError = $derived(pqs.errors.get(matchId) ?? "");

  // Per-goal comparison data: { last, lastMatchId, best, bestMatchId, bestForHero, bestForHeroMatchId, lowerIsBetter }
  let goalComparisons = $state(/** @type {Map<number, {last: number|null, lastMatchId: number|null, best: number|null, bestMatchId: number|null, bestForHero: number|null, bestForHeroMatchId: number|null, lowerIsBetter: boolean}>} */ (new Map()));

  // CS data for compare match
  let compareCsData = $state(/** @type {any[]} */ ([]));
  // Networth / XP data
  let nwData = $state(/** @type {any[]} */ ([]));
  let xpData = $state(/** @type {any[]} */ ([]));
  let compareNwData = $state(/** @type {any[]} */ ([]));
  let compareXpData = $state(/** @type {any[]} */ ([]));
  // Item purchase timings
  let itemTimings = $state(/** @type {any[]} */ ([]));
  let compareItemTimings = $state(/** @type {any[]} */ ([]));
  // Hero CS stats (best + average per minute)
  let heroCsStats = $state(/** @type {any[]} */ ([]));
  // Benchmark data for this match
  let matchBenchmark = $state(/** @type {any} */ (null));
  // Personal LH@10 history across all parsed games (full, unfiltered)
  let personalLhHistory = $state(/** @type {any[]} */ ([]));

  // Filters for the personal LH distribution ('all'|'ranked'|'turbo' and 'all'|'this'|'cores')
  let lhFilterMode = $state(/** @type {string} */ ('all'));
  let lhFilterHero = $state(/** @type {string} */ ('all'));

  // Derived: LH@10 for this match
  let thisLhAt10 = $derived(csData.find((/** @type {any} */ d) => d.minute === 10)?.last_hits ?? null);

  // Derived: filtered personal history based on mode + hero filters, always excluding current match
  let filteredLhHistory = $derived.by(() => {
    return personalLhHistory.filter((/** @type {any} */ d) => {
      if (d.match_id === matchId) return false;
      if (lhFilterMode === 'ranked' && d.game_mode !== 22) return false;
      if (lhFilterMode === 'turbo' && d.game_mode !== 23) return false;
      if (lhFilterHero === 'this' && d.hero_id !== match?.hero_id) return false;
      if (lhFilterHero === 'cores' && !CORE_HERO_IDS.has(d.hero_id)) return false;
      return true;
    });
  });

  // Derived: percentile of this game vs filtered history
  let personalLhPercentile = $derived.by(() => {
    if (thisLhAt10 === null || filteredLhHistory.length === 0) return null;
    const below = filteredLhHistory.filter((/** @type {any} */ d) => d.last_hits < thisLhAt10).length;
    return Math.round((below / filteredLhHistory.length) * 100);
  });

  $effect(() => {
    if (compareMatch) {
      invoke("get_match_cs", { matchId: compareMatch.match_id })
        .then((cs) => { compareCsData = /** @type {any[]} */ (cs); })
        .catch(() => { compareCsData = []; });
      invoke("get_match_networth", { matchId: compareMatch.match_id })
        .then((nw) => { compareNwData = /** @type {any[]} */ (nw); })
        .catch(() => { compareNwData = []; });
      invoke("get_match_xp", { matchId: compareMatch.match_id })
        .then((xp) => { compareXpData = /** @type {any[]} */ (xp); })
        .catch(() => { compareXpData = []; });
      invoke("get_match_item_timings", { matchId: compareMatch.match_id })
        .then((t) => { compareItemTimings = /** @type {any[]} */ (t); })
        .catch(() => { compareItemTimings = []; });
    } else {
      compareCsData = [];
      compareNwData = [];
      compareXpData = [];
      compareItemTimings = [];
    }
  });

  // Compare-to feature
  let showComparePopup = $state(false);
  let compareMatchIdInput = $state("");
  let compareMatch = $state(/** @type {any} */ (null));
  let compareLabel = $state("");
  let compareError = $state("");
  let allMatches = $state(/** @type {any[]} */ ([]));

  let compareLastGame = $derived(
    [...allMatches].sort((a, b) => b.start_time - a.start_time).find(m => m.match_id !== matchId) ?? null
  );
  let compareLastSameHero = $derived(
    [...allMatches].sort((a, b) => b.start_time - a.start_time).find(m => m.match_id !== matchId && m.hero_id === match?.hero_id) ?? null
  );

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      const [fetchedMatches, cs, nw, xp, timings, goals, allItems, settings] = await Promise.all([
        invoke("get_matches"),
        invoke("get_match_cs", { matchId }),
        invoke("get_match_networth", { matchId }),
        invoke("get_match_xp", { matchId }),
        invoke("get_match_item_timings", { matchId }),
        invoke("evaluate_goals_for_match", { matchId }),
        invoke("get_all_items"),
        invoke("get_settings"),
      ]);

      allMatches = fetchedMatches;
      match = fetchedMatches.find((/** @type {any} */ m) => m.match_id === matchId) ?? null;
      csData = cs;
      nwData = nw;
      xpData = xp;
      itemTimings = timings;
      goalDetails = goals;
      items = allItems;
      steamId = settings.steam_id || "";

      if (!match) {
        error = "Match not found.";
      }

      // Load hero CS stats (best + avg per minute for this hero in same game mode)
      if (match) {
        invoke("get_hero_cs_stats", { heroId: match.hero_id, gameMode: match.game_mode, excludeMatchId: matchId })
          .then((stats) => { heroCsStats = /** @type {any[]} */ (stats); })
          .catch(() => { heroCsStats = []; });
      }

      // Load benchmark for this match's LH@10
      await loadMatchBenchmark();

      await loadComparisons();

      initQueue(steamId, async () => {
        // Reload match data after parse completes to pick up updated stats
        const refreshed = await invoke("get_matches");
        allMatches = refreshed;
        match = refreshed.find((/** @type {any} */ m) => m.match_id === matchId) ?? match;
        const [refreshedCs, refreshedNw, refreshedXp, refreshedTimings, refreshedGoals] = await Promise.all([
          invoke("get_match_cs", { matchId }),
          invoke("get_match_networth", { matchId }),
          invoke("get_match_xp", { matchId }),
          invoke("get_match_item_timings", { matchId }),
          invoke("evaluate_goals_for_match", { matchId }),
        ]);
        csData = refreshedCs;
        nwData = refreshedNw;
        xpData = refreshedXp;
        itemTimings = refreshedTimings;
        goalDetails = refreshedGoals;
        invoke("get_hero_cs_stats", { heroId: match.hero_id, gameMode: match.game_mode, excludeMatchId: matchId })
          .then((stats) => { heroCsStats = /** @type {any[]} */ (stats); })
          .catch(() => { heroCsStats = []; });
        await loadComparisons();
      });
    } catch (e) {
      error = `Failed to load match: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  async function loadPersonalLhHistory() {
    try {
      const history = await invoke("get_user_lh_history", { minute: 10 });
      personalLhHistory = /** @type {any[]} */ (history);
    } catch (e) {
      personalLhHistory = [];
    }
  }

  async function loadMatchBenchmark() {
    if (!match || csData.length === 0) { matchBenchmark = null; return; }
    try {
      const hasBench = await invoke("has_benchmarks");
      if (!hasBench) { matchBenchmark = null; return; }
      // Get LH at minute 10
      const cs10 = csData.find((/** @type {any} */ d) => d.minute === 10);
      if (!cs10) { matchBenchmark = null; return; }
      const mode = match.game_mode === 23 ? "turbo" : "ranked";
      matchBenchmark = await invoke("get_hero_benchmark", {
        heroId: match.hero_id,
        mode,
        statName: "last_hits_10min",
        userValue: cs10.last_hits,
        userHeroId: match.hero_id,
        userGameMode: match.game_mode,
      });
    } catch (e) {
      console.error("Failed to load match benchmark:", e);
      matchBenchmark = null;
    }
    // Load personal LH history alongside benchmark (both need csData)
    await loadPersonalLhHistory();
  }

  async function loadComparisons() {
    if (!match || goalDetails.length === 0) return;
    const histograms = await Promise.all(
      goalDetails.map((/** @type {any} */ ev) =>
        invoke("get_goal_histogram_data", { goalId: ev.goal.id })
      )
    );
    const map = new Map();
    for (let i = 0; i < goalDetails.length; i++) {
      const ev = goalDetails[i];
      const data = /** @type {any[]} */ (histograms[i]);
      if (data.length === 0) { map.set(ev.goal.id, { last: null, lastMatchId: null, best: null, bestMatchId: null, bestForHero: null, bestForHeroMatchId: null, lowerIsBetter: false }); continue; }
      const lowerIsBetter = ev.goal.metric === 'ItemTiming' || ev.goal.metric === 'Deaths';
      const bestEntry = data.reduce((/** @type {any} */ acc, /** @type {any} */ d) =>
        lowerIsBetter ? (d.value < acc.value ? d : acc) : (d.value > acc.value ? d : acc)
      );
      const heroData = data.filter((/** @type {any} */ d) => d.hero_id === match.hero_id);
      const bestForHeroEntry = heroData.length > 0
        ? heroData.reduce((/** @type {any} */ acc, /** @type {any} */ d) =>
            lowerIsBetter ? (d.value < acc.value ? d : acc) : (d.value > acc.value ? d : acc))
        : null;
      const prior = data
        .filter((/** @type {any} */ d) => d.start_time < match.start_time)
        .sort((/** @type {any} */ a, /** @type {any} */ b) => b.start_time - a.start_time);
      const lastEntry = prior.length > 0 ? prior[0] : null;
      map.set(ev.goal.id, {
        last: lastEntry ? lastEntry.value : null,
        lastMatchId: lastEntry ? lastEntry.match_id : null,
        best: bestEntry.value,
        bestMatchId: bestEntry.match_id,
        bestForHero: bestForHeroEntry ? bestForHeroEntry.value : null,
        bestForHeroMatchId: bestForHeroEntry ? bestForHeroEntry.match_id : null,
        lowerIsBetter,
      });
    }
    goalComparisons = map;
  }

  /** @param {any} m */
  function formatCompare(value, metric) {
    if (metric === 'ItemTiming') return formatSeconds(value);
    const unit = getMetricUnit(metric);
    return unit ? `${value} ${unit}` : String(value);
  }

  /** @param {number} current @param {number} reference @param {boolean} lowerIsBetter */
  function diffLabel(current, reference, lowerIsBetter) {
    const delta = current - reference;
    if (delta === 0) return { text: '—', cls: 'neutral' };
    const better = lowerIsBetter ? delta < 0 : delta > 0;
    const sign = delta > 0 ? '+' : '';
    return { text: `${sign}${delta}`, cls: better ? 'better' : 'worse' };
  }

  // ── Compare-to feature ────────────────────────────────────────────────────

  function openComparePopup() {
    showComparePopup = true;
    compareError = "";
    compareMatchIdInput = "";
  }

  function closeComparePopup() {
    showComparePopup = false;
  }

  /**
   * @param {any} target
   * @param {string} label
   */
  function selectCompare(target, label) {
    compareMatch = target;
    compareLabel = label;
    showComparePopup = false;
    compareError = "";
  }

  async function loadCompareById() {
    const id = parseInt(compareMatchIdInput);
    if (isNaN(id) || id <= 0) { compareError = "Enter a valid match ID."; return; }
    const found = allMatches.find((/** @type {any} */ m) => m.match_id === id);
    if (!found) { compareError = "Match not found in your history."; return; }
    selectCompare(found, `Match #${id}`);
  }

  /** @param {number} mid @param {string} label */
  function selectCompareById(mid, label) {
    const found = allMatches.find((/** @type {any} */ m) => m.match_id === mid);
    if (found) selectCompare(found, label);
  }

  function clearCompare() {
    compareMatch = null;
    compareLabel = "";
    compareError = "";
  }

  /** @param {number} seconds */
  function formatDuration(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  /** @param {number} timestamp */
  function formatDate(timestamp) {
    return new Date(timestamp * 1000).toLocaleString();
  }

  /** @param {any} m */
  function isWin(m) {
    const isRadiant = m.player_slot < 128;
    return (isRadiant && m.radiant_win) || (!isRadiant && !m.radiant_win);
  }

  /** @param {number} gameMode */
  function getGameModeName(gameMode) {
    switch (gameMode) {
      case 0: return "Unknown";
      case 1: return "All Pick";
      case 2: return "Captain's Mode";
      case 3: return "Random Draft";
      case 4: return "Single Draft";
      case 5: return "All Random";
      case 14: return "Captain's Draft";
      case 15: return "Balanced Draft";
      case 16: return "Ability Draft";
      case 18: return "AR/DM";
      case 19: return "1v1 Mid";
      case 20: return "Ranked";
      case 21: return "Turbo";
      case 22: return "Ranked All Pick";
      case 23: return "Turbo";
      default: return `Mode ${gameMode}`;
    }
  }

  /** @param {string} metric */
  function getMetricLabel(metric) {
    switch (metric) {
      case "Networth": return "Net Worth";
      case "Kills": return "Kills";
      case "LastHits": return "Last Hits";
      case "Level": return "Level";
      case "ItemTiming": return "Item Timing";
      default: return metric;
    }
  }

  /** @param {string} metric */
  function getMetricUnit(metric) {
    switch (metric) {
      case "Networth": return "gold";
      case "Kills": return "kills";
      case "LastHits": return "CS";
      default: return "";
    }
  }

  /** @param {number} totalSeconds */
  function formatSeconds(totalSeconds) {
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes}:${seconds.toString().padStart(2, "0")}`;
  }

  /** @param {number} itemId */
  function getItemName(itemId) {
    const item = items.find(i => i.id === itemId);
    return item ? item.display_name : `Item ${itemId}`;
  }

  /** @param {number} mid */
  async function openInOpenDota(mid) {
    try {
      await openUrl(`https://www.opendota.com/matches/${mid}`);
    } catch (e) {
      console.error("Failed to open OpenDota link:", e);
    }
  }

  // Chart config derived from csData
  const goalLineColors = [
    "rgba(248, 113, 113, 0.8)", // red
    "rgba(100, 180, 255, 0.8)", // blue
    "rgba(180, 100, 255, 0.8)", // purple
    "rgba(74, 222, 128, 0.8)", // green
  ];

  let chartConfig = $derived(() => {
    if (csData.length === 0) return null;

    const labels = csData.map((d) => `${d.minute}m`);
    const lhValues = csData.map((d) => d.last_hits);
    const dnValues = csData.map((d) => d.denies);

    // Build a goal target line for each applicable LastHits goal
    const lhGoals = goalDetails.filter((g) => g.goal.metric === "LastHits");
    const goalDatasets = lhGoals.map((evaluation, i) => {
      const { target_value, target_time_minutes } = evaluation.goal;
      const color = goalLineColors[i % goalLineColors.length];
      const data = csData.map((d) => {
        if (d.minute > target_time_minutes) return null;
        return Math.round((target_value * d.minute) / target_time_minutes);
      });
      const heroLabel = evaluation.goal.hero_id !== null
        ? getHeroName(evaluation.goal.hero_id) + " "
        : "";
      return {
        label: `Goal: ${heroLabel}${target_value} CS @ ${target_time_minutes}m`,
        data,
        borderColor: color,
        backgroundColor: "transparent",
        borderWidth: 2,
        borderDash: [6, 4],
        pointRadius: 0,
        pointHoverRadius: 4,
        fill: false,
        tension: 0,
        spanGaps: false,
      };
    });

    // Compare match datasets — align by minute, null where compare match ended earlier
    const cmpByMinute = new Map(compareCsData.map((d) => [d.minute, d]));
    const compareDatasets = compareCsData.length > 0 ? [
      {
        label: `${compareLabel} — Last Hits`,
        data: csData.map((d) => cmpByMinute.get(d.minute)?.last_hits ?? null),
        borderColor: "rgba(96, 165, 250, 0.9)",
        backgroundColor: "rgba(96, 165, 250, 0.08)",
        borderWidth: 2,
        borderDash: [5, 3],
        pointRadius: 2,
        pointHoverRadius: 5,
        fill: false,
        tension: 0.3,
        spanGaps: false,
      },
      {
        label: `${compareLabel} — Denies`,
        data: csData.map((d) => cmpByMinute.get(d.minute)?.denies ?? null),
        borderColor: "rgba(148, 163, 184, 0.6)",
        backgroundColor: "transparent",
        borderWidth: 1.5,
        borderDash: [5, 3],
        pointRadius: 1,
        pointHoverRadius: 4,
        fill: false,
        tension: 0.3,
        spanGaps: false,
      },
    ] : [];

    // Hero best + average CS datasets — only shown when no compare match is active
    const heroStatsByMin = new Map(heroCsStats.map((d) => [d.minute, d]));
    const heroName = match ? getHeroName(match.hero_id) : "Hero";
    const heroCsDatasets = heroCsStats.length > 0 && !compareMatch ? [
      {
        label: `Best ${heroName} — Last Hits`,
        data: csData.map((d) => heroStatsByMin.get(d.minute)?.best_last_hits ?? null),
        borderColor: "rgba(74, 222, 128, 0.8)",
        backgroundColor: "transparent",
        borderWidth: 1.5,
        borderDash: [8, 4],
        pointRadius: 0,
        pointHoverRadius: 4,
        fill: false,
        tension: 0.3,
        spanGaps: false,
      },
      {
        label: `Avg ${heroName} — Last Hits`,
        data: csData.map((d) => {
          const s = heroStatsByMin.get(d.minute);
          return s ? Math.round(s.avg_last_hits) : null;
        }),
        borderColor: "rgba(74, 222, 128, 0.4)",
        backgroundColor: "transparent",
        borderWidth: 1.5,
        borderDash: [3, 3],
        pointRadius: 0,
        pointHoverRadius: 4,
        fill: false,
        tension: 0.3,
        spanGaps: false,
      },
    ] : [];

    return {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "This match — Last Hits",
            data: lhValues,
            borderColor: "#f0b429",
            backgroundColor: "rgba(240, 180, 41, 0.15)",
            borderWidth: 2,
            pointRadius: 2,
            pointHoverRadius: 5,
            fill: true,
            tension: 0.3,
          },
          {
            label: "This match — Denies",
            data: dnValues,
            borderColor: "rgba(163, 163, 163, 0.8)",
            backgroundColor: "rgba(163, 163, 163, 0.1)",
            borderWidth: 2,
            pointRadius: 2,
            pointHoverRadius: 5,
            fill: false,
            tension: 0.3,
          },
          ...heroCsDatasets,
          ...compareDatasets,
          ...goalDatasets,
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        interaction: {
          mode: "index",
          intersect: false,
        },
        plugins: {
          legend: {
            labels: { color: "#c0c0c0" },
          },
          tooltip: {
            backgroundColor: "rgba(18, 20, 28, 0.95)",
            borderColor: "rgba(240, 180, 41, 0.3)",
            borderWidth: 1,
            titleColor: "#f0b429",
            bodyColor: "#e0e0e0",
          },
        },
        scales: {
          x: {
            ticks: { color: "#9ca3af", maxTicksLimit: 15 },
            grid: { color: "rgba(255, 200, 80, 0.08)" },
            title: { display: true, text: "Game Time", color: "#9ca3af" },
          },
          y: {
            ticks: { color: "#9ca3af" },
            grid: { color: "rgba(255, 200, 80, 0.08)" },
            title: { display: true, text: "Count", color: "#9ca3af" },
            beginAtZero: true,
          },
        },
      },
    };
  });

  // Distribution chart: user's personal LH@10 with filters applied, with this game highlighted
  let personalLhDistConfig = $derived.by(() => {
    const allValues = filteredLhHistory.map((/** @type {any} */ d) => d.last_hits);
    if (allValues.length === 0) return null;

    // Build histogram bins (width = 5)
    const binSize = 5;
    const minVal = Math.floor(Math.min(...allValues) / binSize) * binSize;
    const maxVal = Math.ceil(Math.max(...allValues) / binSize) * binSize;
    const bins = [];
    for (let b = minVal; b <= maxVal; b += binSize) bins.push(b);

    const counts = bins.map((b) => allValues.filter((v) => v >= b && v < b + binSize).length);
    const thisVal = thisLhAt10;
    const barColors = bins.map((b) =>
      thisVal !== null && thisVal >= b && thisVal < b + binSize
        ? "rgba(240, 180, 41, 0.9)"
        : "rgba(96, 165, 250, 0.5)"
    );
    const borderColors = bins.map((b) =>
      thisVal !== null && thisVal >= b && thisVal < b + binSize
        ? "rgba(240, 180, 41, 1)"
        : "rgba(96, 165, 250, 0.8)"
    );

    return {
      type: "bar",
      data: {
        labels: bins.map((b) => `${b}–${b + binSize - 1}`),
        datasets: [{
          label: "Games",
          data: counts,
          backgroundColor: barColors,
          borderColor: borderColors,
          borderWidth: 1.5,
        }],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: { display: false },
          tooltip: {
            backgroundColor: "rgba(18, 20, 28, 0.95)",
            borderColor: "rgba(240, 180, 41, 0.3)",
            borderWidth: 1,
            titleColor: "#f0b429",
            bodyColor: "#e0e0e0",
            callbacks: {
              title: (/** @type {any[]} */ items) => `LH@10: ${items[0]?.label}`,
              label: (/** @type {any} */ item) => `${item.raw} game${item.raw === 1 ? "" : "s"}`,
            },
          },
        },
        scales: {
          x: {
            ticks: { color: "#9ca3af", font: { size: 11 } },
            grid: { color: "rgba(255, 200, 80, 0.06)" },
            title: { display: true, text: "Last Hits at 10 min", color: "#9ca3af", font: { size: 11 } },
          },
          y: {
            ticks: { color: "#9ca3af", stepSize: 1 },
            grid: { color: "rgba(255, 200, 80, 0.06)" },
            title: { display: true, text: "Games", color: "#9ca3af", font: { size: 11 } },
            beginAtZero: true,
          },
        },
      },
    };
  });

  // Dota 2 cumulative XP thresholds per level (index = level, value = total XP needed)
  // Source: https://dota2.fandom.com/wiki/Experience
  const XP_PER_LEVEL = [
    0,       // level 0 placeholder
    0,       // level 1 starts at 0 XP
    230,     // level 2
    630,     // level 3
    1130,    // level 4
    1730,    // level 5
    2430,    // level 6
    3230,    // level 7
    4130,    // level 8
    5130,    // level 9
    6330,    // level 10
    7630,    // level 11
    9030,    // level 12
    10530,   // level 13
    12130,   // level 14
    13830,   // level 15
    15630,   // level 16
    17630,   // level 17
    19830,   // level 18
    22230,   // level 19
    24630,   // level 20
    27430,   // level 21
    30630,   // level 22
    34030,   // level 23
    37830,   // level 24
    42030,   // level 25
    46830,   // level 26
    52230,   // level 27
    58230,   // level 28
    64830,   // level 29
    72030,   // level 30
  ];

  /** @param {number} xp */
  function xpToLevel(xp) {
    for (let lvl = XP_PER_LEVEL.length - 1; lvl >= 1; lvl--) {
      if (xp >= XP_PER_LEVEL[lvl]) return lvl;
    }
    return 1;
  }

  /** @param {string} yTitle */
  function makeChartOptions(yTitle) {
    return {
      responsive: true,
      maintainAspectRatio: false,
      interaction: { mode: "index", intersect: false },
      plugins: {
        legend: { labels: { color: "#c0c0c0" } },
        tooltip: {
          backgroundColor: "rgba(18, 20, 28, 0.95)",
          borderColor: "rgba(240, 180, 41, 0.3)",
          borderWidth: 1,
          titleColor: "#f0b429",
          bodyColor: "#e0e0e0",
        },
      },
      scales: {
        x: {
          ticks: { color: "#9ca3af", maxTicksLimit: 15 },
          grid: { color: "rgba(255, 200, 80, 0.08)" },
          title: { display: true, text: "Game Time", color: "#9ca3af" },
        },
        y: {
          ticks: { color: "#9ca3af" },
          grid: { color: "rgba(255, 200, 80, 0.08)" },
          title: { display: true, text: yTitle, color: "#9ca3af" },
          beginAtZero: true,
        },
      },
    };
  }

  let nwChartConfig = $derived(() => {
    if (nwData.length === 0) return null;
    const labels = nwData.map((d) => `${d.minute}m`);
    const cmpByMinute = new Map(compareNwData.map((d) => [d.minute, d]));
    const compareDatasets = compareNwData.length > 0 ? [{
      label: `${compareLabel} — Networth`,
      data: nwData.map((d) => cmpByMinute.get(d.minute)?.networth ?? null),
      borderColor: "rgba(96, 165, 250, 0.9)",
      backgroundColor: "rgba(96, 165, 250, 0.08)",
      borderWidth: 2,
      borderDash: [5, 3],
      pointRadius: 2,
      pointHoverRadius: 5,
      fill: false,
      tension: 0.3,
      spanGaps: false,
    }] : [];
    return {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "This match — Networth",
            data: nwData.map((d) => d.networth),
            borderColor: "#f0b429",
            backgroundColor: "rgba(240, 180, 41, 0.15)",
            borderWidth: 2,
            pointRadius: 2,
            pointHoverRadius: 5,
            fill: true,
            tension: 0.3,
          },
          ...compareDatasets,
        ],
      },
      options: makeChartOptions("Gold"),
    };
  });

  let xpChartConfig = $derived(() => {
    if (xpData.length === 0) return null;
    const labels = xpData.map((d) => `${d.minute}m`);
    const cmpByMinute = new Map(compareXpData.map((d) => [d.minute, d]));
    const compareDatasets = compareXpData.length > 0 ? [{
      label: `${compareLabel} — XP`,
      data: xpData.map((d) => cmpByMinute.get(d.minute)?.xp ?? null),
      borderColor: "rgba(167, 139, 250, 0.9)",
      backgroundColor: "rgba(167, 139, 250, 0.08)",
      borderWidth: 2,
      borderDash: [5, 3],
      pointRadius: 2,
      pointHoverRadius: 5,
      fill: false,
      tension: 0.3,
      spanGaps: false,
    }] : [];
    return {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "This match — XP",
            data: xpData.map((d) => d.xp),
            borderColor: "#a78bfa",
            backgroundColor: "rgba(167, 139, 250, 0.15)",
            borderWidth: 2,
            pointRadius: 2,
            pointHoverRadius: 5,
            fill: true,
            tension: 0.3,
          },
          ...compareDatasets,
        ],
      },
      options: makeChartOptions("Experience"),
    };
  });

  let levelChartConfig = $derived(() => {
    if (xpData.length === 0) return null;
    const labels = xpData.map((d) => `${d.minute}m`);
    const cmpByMinute = new Map(compareXpData.map((d) => [d.minute, d]));
    const compareDatasets = compareXpData.length > 0 ? [{
      label: `${compareLabel} — Level`,
      data: xpData.map((d) => {
        const cmp = cmpByMinute.get(d.minute);
        return cmp != null ? xpToLevel(cmp.xp) : null;
      }),
      borderColor: "rgba(52, 211, 153, 0.9)",
      backgroundColor: "rgba(52, 211, 153, 0.08)",
      borderWidth: 2,
      borderDash: [5, 3],
      pointRadius: 2,
      pointHoverRadius: 5,
      fill: false,
      tension: 0,
      spanGaps: false,
    }] : [];
    return {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "This match — Level",
            data: xpData.map((d) => xpToLevel(d.xp)),
            borderColor: "#34d399",
            backgroundColor: "rgba(52, 211, 153, 0.15)",
            borderWidth: 2,
            pointRadius: 2,
            pointHoverRadius: 5,
            fill: true,
            tension: 0,
          },
          ...compareDatasets,
        ],
      },
      options: makeChartOptions("Level"),
    };
  });
</script>

<div class="match-detail-content">
  <div class="page-header">
    <a href={backHref} class="back-btn">{backLabel}</a>
    <div class="header-title">
      <h1>Match Details</h1>
      {#if match}
        <p class="subtitle">#{match.match_id}</p>
      {/if}
    </div>
    {#if match}
      <div class="external-links">
        <button
          class="reparse-btn"
          class:parsing={isParsing}
          disabled={isParsing || !steamId}
          onclick={() => enqueueParse(matchId)}
        >
          {#if isParsing}⟳ Parsing…{:else}↺ Re-parse{/if}
        </button>
        <button class="compare-btn" onclick={openComparePopup}>
          ⇄ Compare to
        </button>
        <button class="opendota-btn" onclick={() => openInOpenDota(match.match_id)}>
          View on OpenDota
        </button>
        <button class="stratz-btn" onclick={() => openUrl(`https://stratz.com/matches/${match.match_id}`)}>
          View on Stratz
        </button>
      </div>
      {#if parseError}
        <p class="parse-error">{parseError}</p>
      {/if}
    {/if}
  </div>

  {#if error}
    <p class="error">{error}</p>
  {:else if isLoading}
    <div class="loading"><p>{$_('matches.loading')}</p></div>
  {:else if match}

    <!-- Comparison banner -->
    {#if compareMatch}
      <div class="cmp-banner">
        <div class="cmp-banner-labels">
          <span class="cmp-banner-this">This game</span>
          <span class="cmp-banner-vs">vs</span>
          <span class="cmp-banner-other">{compareLabel}</span>
        </div>
        <button class="cmp-banner-clear" onclick={clearCompare}>✕ Clear comparison</button>
      </div>
    {/if}

    <!-- Overview -->
    {#if compareMatch}
      {@const b = compareMatch}
      <div class="overview-compare">
        <div class="overview-compare-col">
          <div class="oc-label this">This game</div>
          <div class="overview-card hero-card">
            <HeroIcon heroId={match.hero_id} size="large" />
            <div class="hero-info">
              <div class="hero-name">{getHeroName(match.hero_id)}</div>
              <div class="result-badge {isWin(match) ? 'win' : 'loss'}">{isWin(match) ? $_('matches.won') : $_('matches.lost')}</div>
            </div>
          </div>
          <div class="overview-card meta-card">
            <div class="meta-row"><span class="meta-label">{$_('matches.col_date')}</span><span class="meta-value">{formatDate(match.start_time)}</span></div>
            <div class="meta-row"><span class="meta-label">Duration</span><span class="meta-value">{formatDuration(match.duration)}</span></div>
            <div class="meta-row"><span class="meta-label">Game Mode</span><span class="meta-value">{getGameModeName(match.game_mode)}</span></div>
            <div class="meta-row"><span class="meta-label">Parse State</span><span class="meta-value parse-{match.parse_state.toLowerCase()}">{match.parse_state}</span></div>
          </div>
        </div>
        <div class="overview-compare-col">
          <div class="oc-label other">{compareLabel}</div>
          <div class="overview-card hero-card">
            <HeroIcon heroId={b.hero_id} size="large" />
            <div class="hero-info">
              <div class="hero-name">{getHeroName(b.hero_id)}</div>
              <div class="result-badge {isWin(b) ? 'win' : 'loss'}">{isWin(b) ? $_('matches.won') : $_('matches.lost')}</div>
            </div>
          </div>
          <div class="overview-card meta-card">
            <div class="meta-row"><span class="meta-label">{$_('matches.col_date')}</span><span class="meta-value">{formatDate(b.start_time)}</span></div>
            <div class="meta-row"><span class="meta-label">Duration</span><span class="meta-value">{formatDuration(b.duration)}</span></div>
            <div class="meta-row"><span class="meta-label">Game Mode</span><span class="meta-value">{getGameModeName(b.game_mode)}</span></div>
            <div class="meta-row"><span class="meta-label">Parse State</span><span class="meta-value parse-{b.parse_state.toLowerCase()}">{b.parse_state}</span></div>
          </div>
        </div>
      </div>
    {:else}
      <div class="overview-grid">
        <div class="overview-card hero-card">
          <HeroIcon heroId={match.hero_id} size="large" />
          <div class="hero-info">
            <div class="hero-name">{getHeroName(match.hero_id)}</div>
            <div class="result-badge {isWin(match) ? 'win' : 'loss'}">{isWin(match) ? $_('matches.won') : $_('matches.lost')}</div>
          </div>
        </div>
        <div class="overview-card meta-card">
          <div class="meta-row"><span class="meta-label">{$_('matches.col_date')}</span><span class="meta-value">{formatDate(match.start_time)}</span></div>
          <div class="meta-row"><span class="meta-label">Duration</span><span class="meta-value">{formatDuration(match.duration)}</span></div>
          <div class="meta-row"><span class="meta-label">Game Mode</span><span class="meta-value">{getGameModeName(match.game_mode)}</span></div>
          <div class="meta-row"><span class="meta-label">Parse State</span><span class="meta-value parse-{match.parse_state.toLowerCase()}">{match.parse_state}</span></div>
        </div>
      </div>
    {/if}

    <!-- Performance Stats -->
    <div class="stats-section">
      <h2 class="section-title">Performance</h2>
      {#if compareMatch}
        {@const b = compareMatch}
        <div class="stats-compare-table">
          <div class="sct-header">
            <div class="sct-label-col"></div>
            <div class="sct-col this">This game</div>
            <div class="sct-col other">{compareLabel}</div>
          </div>
          {#each [
            [$_('matches.col_kda'), `${match.kills}/${match.deaths}/${match.assists}`, `${b.kills}/${b.deaths}/${b.assists}`, null],
            [$_('matches.col_gpm'), match.gold_per_min, b.gold_per_min, true],
            [$_('matches.col_xpm'), match.xp_per_min,   b.xp_per_min,   true],
            ['Last Hits',           match.last_hits,     b.last_hits,    true],
            ['Denies',              match.denies,        b.denies,       true],
            ['Hero Dmg',            match.hero_damage,   b.hero_damage,  true],
            ['Tower Dmg',           match.tower_damage,  b.tower_damage, true],
            ['Healing',             match.hero_healing,  b.hero_healing, true],
            ['Duration',            formatDuration(match.duration), formatDuration(b.duration), null],
          ] as [lbl, va, vb, higherBetter]}
            {@const isNum = typeof va === 'number' && typeof vb === 'number'}
            {@const aBetter = isNum && higherBetter !== null ? (/** @type {any} */ (higherBetter) ? va > vb : va < vb) : false}
            {@const bBetter = isNum && higherBetter !== null ? (/** @type {any} */ (higherBetter) ? vb > va : vb < va) : false}
            <div class="sct-row">
              <div class="sct-label-col">{lbl}</div>
              <div class="sct-col" class:sct-better={aBetter}>{isNum ? (/** @type {number} */ (va)).toLocaleString() : va}</div>
              <div class="sct-col" class:sct-better={bBetter}>{isNum ? (/** @type {number} */ (vb)).toLocaleString() : vb}</div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-value kda">
              <span class="kills">{match.kills}</span>/<span class="deaths">{match.deaths}</span>/<span class="assists">{match.assists}</span>
            </div>
            <div class="stat-label">{$_('matches.col_kda')}</div>
          </div>
          <div class="stat-card"><div class="stat-value">{match.gold_per_min}</div><div class="stat-label">{$_('matches.col_gpm')}</div></div>
          <div class="stat-card"><div class="stat-value">{match.xp_per_min}</div><div class="stat-label">{$_('matches.col_xpm')}</div></div>
          <div class="stat-card"><div class="stat-value">{match.last_hits}</div><div class="stat-label">Last Hits</div></div>
          <div class="stat-card"><div class="stat-value">{match.denies}</div><div class="stat-label">Denies</div></div>
          <div class="stat-card"><div class="stat-value">{match.hero_damage.toLocaleString()}</div><div class="stat-label">Hero Dmg</div></div>
          <div class="stat-card"><div class="stat-value">{match.tower_damage.toLocaleString()}</div><div class="stat-label">Tower Dmg</div></div>
          <div class="stat-card"><div class="stat-value">{match.hero_healing.toLocaleString()}</div><div class="stat-label">Healing</div></div>
        </div>
      {/if}
    </div>

    <!-- Benchmark Ranking -->
    {#if matchBenchmark?.rows?.length > 0 || thisLhAt10 !== null}
      <div class="chart-section">
        <h2 class="section-title">Last Hitting Rank for this game</h2>

        {#if thisLhAt10 !== null}
          <div class="lh10-summary">
            <div class="lh10-value-block">
              <span class="lh10-number">{thisLhAt10}</span>
              <span class="lh10-label">last hits at 10 min</span>
            </div>
            {#if personalLhPercentile !== null}
              <div class="lh10-percentile-block">
                <span class="lh10-percentile">{personalLhPercentile}th percentile</span>
                <span class="lh10-percentile-label">vs your other games ({filteredLhHistory.length} games)</span>
              </div>
            {/if}
          </div>
        {/if}

        {#if personalLhHistory.length > 0}
          <div class="lh10-filters">
            <div class="lh10-filter-group">
              {#each [['all', 'All modes'], ['ranked', 'Ranked'], ['turbo', 'Turbo']] as [val, label]}
                <button class="lh10-filter-btn" class:active={lhFilterMode === val} onclick={() => lhFilterMode = val}>{label}</button>
              {/each}
            </div>
            <div class="lh10-filter-group">
              <button class="lh10-filter-btn" class:active={lhFilterHero === 'all'} onclick={() => lhFilterHero = 'all'}>All heroes</button>
              {#if match}
                <button class="lh10-filter-btn" class:active={lhFilterHero === 'this'} onclick={() => lhFilterHero = 'this'}>{getHeroName(match.hero_id)}</button>
              {/if}
              <button class="lh10-filter-btn" class:active={lhFilterHero === 'cores'} onclick={() => lhFilterHero = 'cores'}>All cores</button>
            </div>
          </div>
        {/if}

        {#if personalLhDistConfig !== null}
          <div class="chart-container" style="margin-bottom: 16px;">
            <Chart config={personalLhDistConfig} height="180px" />
          </div>
        {:else if personalLhHistory.length > 0}
          <p class="no-data-hint" style="margin-bottom: 16px;">No games match the current filters.</p>
        {/if}

        {#if matchBenchmark?.rows?.length > 0}
          <BenchmarkTable benchmarkData={matchBenchmark} />
        {/if}
      </div>
    {/if}

    <!-- Last Hits Chart -->
    <div class="chart-section">
      <h2 class="section-title">Last Hits Progression</h2>
      {#if csData.length > 0}
        <div class="chart-container">
          <Chart config={chartConfig()} height="320px" />
        </div>
      {:else}
        <div class="no-data-box">
          <p>No per-minute data available.</p>
          <p class="no-data-hint">Parse this match to see the last hits progression chart.</p>
        </div>
      {/if}
    </div>

    <!-- Networth Chart -->
    <div class="chart-section">
      <h2 class="section-title">Networth by Minute</h2>
      {#if nwData.length > 0}
        <div class="chart-container">
          <Chart config={nwChartConfig()} height="280px" />
        </div>
      {:else}
        <div class="no-data-box">
          <p>No networth data available.</p>
          <p class="no-data-hint">Parse this match to see the networth chart.</p>
        </div>
      {/if}
    </div>

    <!-- XP Chart -->
    <div class="chart-section">
      <h2 class="section-title">Experience by Minute</h2>
      {#if xpData.length > 0}
        <div class="chart-container">
          <Chart config={xpChartConfig()} height="280px" />
        </div>
      {:else}
        <div class="no-data-box">
          <p>No experience data available.</p>
          <p class="no-data-hint">Parse this match to see the XP chart.</p>
        </div>
      {/if}
    </div>

    <!-- Level Chart -->
    <div class="chart-section">
      <h2 class="section-title">Level by Minute</h2>
      {#if xpData.length > 0}
        <div class="chart-container">
          <Chart config={levelChartConfig()} height="260px" />
        </div>
      {:else}
        <div class="no-data-box">
          <p>No level data available.</p>
          <p class="no-data-hint">Parse this match to see the level chart.</p>
        </div>
      {/if}
    </div>

    <!-- Item Purchase Log -->
    {#if itemTimings.length > 0 || compareItemTimings.length > 0}
      <div class="items-section">
        <h2 class="section-title">Item Purchase Log</h2>
        {#if compareMatch}
          <div class="items-compare">
            <div class="items-compare-col">
              <div class="items-col-label this">This game</div>
              {#if itemTimings.length > 0}
                <div class="item-list">
                  {#each itemTimings as t}
                    <div class="item-row">
                      <span class="item-time">{formatSeconds(t.timing_seconds)}</span>
                      <span class="item-name">{getItemName(t.item_id)}</span>
                    </div>
                  {/each}
                </div>
              {:else}
                <p class="items-empty">No items recorded.</p>
              {/if}
            </div>
            <div class="items-compare-col">
              <div class="items-col-label other">{compareLabel}</div>
              {#if compareItemTimings.length > 0}
                <div class="item-list">
                  {#each compareItemTimings as t}
                    <div class="item-row">
                      <span class="item-time">{formatSeconds(t.timing_seconds)}</span>
                      <span class="item-name">{getItemName(t.item_id)}</span>
                    </div>
                  {/each}
                </div>
              {:else}
                <p class="items-empty">No items recorded.</p>
              {/if}
            </div>
          </div>
        {:else}
          <div class="item-list">
            {#each itemTimings as t}
              <div class="item-row">
                <span class="item-time">{formatSeconds(t.timing_seconds)}</span>
                <span class="item-name">{getItemName(t.item_id)}</span>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {:else if match?.parse_state === "Parsed"}
      <div class="items-section">
        <h2 class="section-title">Item Purchase Log</h2>
        <p class="items-empty">No item data recorded for this match.</p>
      </div>
    {/if}

    <!-- Goals -->
    {#if goalDetails.length > 0}
      <div class="goals-section">
        <h2 class="section-title">
          Goals
          <span class="goals-summary">{goalDetails.filter((g) => g.achieved).length}/{goalDetails.length} achieved</span>
        </h2>
        <div class="goals-list">
          {#each goalDetails as evaluation}
            <div class="goal-card {evaluation.achieved ? 'achieved' : 'not-achieved'}">
              <div class="goal-status">{evaluation.achieved ? "✓" : "✗"}</div>
              <div class="goal-info">
                <div class="goal-title">
                  {#if evaluation.goal.hero_id !== null}
                    <HeroIcon heroId={evaluation.goal.hero_id} size="small" showName={false} />
                    {getHeroName(evaluation.goal.hero_id)} —
                  {:else}
                    {$_('matches.any_hero')} —
                  {/if}
                  {#if evaluation.goal.metric === "ItemTiming"}
                    {evaluation.goal.item_id !== null ? getItemName(evaluation.goal.item_id) : "Item"} ({$_('matches.item_timing')})
                  {:else}
                    {getMetricLabel(evaluation.goal.metric)}
                  {/if}
                </div>
                <div class="goal-numbers">
                  {#if evaluation.goal.metric === "ItemTiming"}
                    <span class="goal-target">Target: by {formatSeconds(evaluation.goal.target_value)}</span>
                    <span class="goal-actual">Actual: {formatSeconds(evaluation.actual_value)}</span>
                  {:else}
                    <span class="goal-target">Target: {evaluation.goal.target_value} {getMetricUnit(evaluation.goal.metric)} by {evaluation.goal.target_time_minutes}m</span>
                    <span class="goal-actual">Actual: {evaluation.actual_value} {getMetricUnit(evaluation.goal.metric)}</span>
                  {/if}
                </div>
                {#if goalComparisons.get(evaluation.goal.id)}
                  {@const cmp = goalComparisons.get(evaluation.goal.id)}
                  <div class="goal-comparisons">
                    {#if cmp.last !== null}
                      {@const d = diffLabel(evaluation.actual_value, cmp.last, cmp.lowerIsBetter)}
                      <span class="cmp-chip">
                        <span class="cmp-label">Last</span>
                        <span class="cmp-value">{formatCompare(cmp.last, evaluation.goal.metric)}</span>
                        <span class="cmp-diff {d.cls}">{d.text}</span>
                        {#if cmp.lastMatchId && cmp.lastMatchId !== matchId}
                          <button class="cmp-compare-btn" onclick={() => selectCompareById(cmp.lastMatchId, 'Last match')}>⇄</button>
                        {/if}
                      </span>
                    {/if}
                    {#if cmp.best !== null}
                      {@const d = diffLabel(evaluation.actual_value, cmp.best, cmp.lowerIsBetter)}
                      <span class="cmp-chip">
                        <span class="cmp-label">Best</span>
                        <span class="cmp-value">{formatCompare(cmp.best, evaluation.goal.metric)}</span>
                        <span class="cmp-diff {d.cls}">{d.text}</span>
                        {#if cmp.bestMatchId && cmp.bestMatchId !== matchId}
                          <button class="cmp-compare-btn" onclick={() => selectCompareById(cmp.bestMatchId, 'Best match')}>⇄</button>
                        {/if}
                      </span>
                    {/if}
                    {#if cmp.bestForHero !== null}
                      {@const d = diffLabel(evaluation.actual_value, cmp.bestForHero, cmp.lowerIsBetter)}
                      <span class="cmp-chip">
                        <span class="cmp-label">Best for {getHeroName(match.hero_id)}</span>
                        <span class="cmp-value">{formatCompare(cmp.bestForHero, evaluation.goal.metric)}</span>
                        <span class="cmp-diff {d.cls}">{d.text}</span>
                        {#if cmp.bestForHeroMatchId && cmp.bestForHeroMatchId !== matchId}
                          <button class="cmp-compare-btn" onclick={() => selectCompareById(cmp.bestForHeroMatchId, `Best ${getHeroName(match.hero_id)} match`)}>⇄</button>
                        {/if}
                      </span>
                    {/if}
                  </div>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {:else if match.parse_state === "Parsed"}
      <div class="goals-section">
        <h2 class="section-title">Goals</h2>
        <p class="no-goals-text">{$_('matches.no_applicable_goals')}</p>
      </div>
    {/if}
  {/if}
</div>

<!-- Compare-to popup -->
{#if showComparePopup}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="popup-overlay" onclick={closeComparePopup} role="dialog" aria-modal="true">
    <div class="popup" onclick={(e) => e.stopPropagation()}>
      <div class="popup-header">
        <h3>Compare to…</h3>
        <button class="popup-close" onclick={closeComparePopup}>✕</button>
      </div>
      <div class="popup-options">
        <button class="popup-option-btn" disabled={!compareLastGame} onclick={() => compareLastGame && selectCompare(compareLastGame, 'Last game')}>
          <span class="option-icon">⏮</span>
          <div>
            <div class="option-title">Last game</div>
            {#if compareLastGame}<div class="option-sub">{getHeroName(compareLastGame.hero_id)} · {formatDate(compareLastGame.start_time)}</div>{/if}
          </div>
        </button>
        <button class="popup-option-btn" disabled={!compareLastSameHero} onclick={() => compareLastSameHero && selectCompare(compareLastSameHero, 'Last game with this hero')}>
          <span class="option-icon">🦸</span>
          <div>
            <div class="option-title">Last game with this hero</div>
            {#if compareLastSameHero}<div class="option-sub">{getHeroName(compareLastSameHero.hero_id)} · {formatDate(compareLastSameHero.start_time)}</div>
            {:else}<div class="option-sub muted">No other games with this hero</div>{/if}
          </div>
        </button>
      </div>
      <div class="popup-manual">
        <input
          class="popup-input"
          type="number"
          placeholder="Match ID…"
          bind:value={compareMatchIdInput}
          onkeydown={(e) => e.key === 'Enter' && loadCompareById()}
        />
        <button class="popup-go-btn" onclick={loadCompareById}>Go</button>
      </div>
      {#if compareError}<p class="popup-error">{compareError}</p>{/if}
    </div>
  </div>
{/if}

<style>
  .match-detail-content {
    max-width: 1100px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .page-header {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding: 20px 25px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
  }

  .back-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-secondary);
    text-decoration: none;
    padding: 8px 16px;
    border: 1px solid var(--border);
    border-radius: 4px;
    transition: all 0.2s;
    white-space: nowrap;
    background: transparent;
  }

  .back-btn:hover {
    color: var(--text-primary);
    border-color: var(--border-active);
  }

  .header-title {
    flex: 1;
  }

  .page-header h1 {
    font-family: 'Rajdhani', sans-serif;
    font-size: 28px;
    font-weight: 700;
    letter-spacing: 2px;
    color: var(--text-primary);
    text-transform: uppercase;
    margin: 0 0 6px 0;
  }

  .subtitle {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
    margin: 0;
  }

  .external-links {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .reparse-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 12px;
    padding: 10px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    background: transparent;
    color: var(--text-secondary);
    border: 1px solid var(--border);
  }

  .reparse-btn:hover:not(:disabled) {
    border-color: var(--gold);
    color: var(--gold);
    transform: translateY(-1px);
  }

  .reparse-btn:disabled {
    opacity: 0.5;
    cursor: default;
  }

  .reparse-btn.parsing {
    color: var(--gold);
    border-color: rgba(240, 180, 41, 0.4);
    animation: pulse 1.2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }

  .parse-error {
    font-size: 12px;
    color: var(--red, #f87171);
    margin: 4px 0 0 0;
  }

  .opendota-btn,
  .stratz-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 12px;
    border: none;
    padding: 10px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .opendota-btn {
    background: var(--gold);
    color: #080c10;
  }

  .opendota-btn:hover {
    background: var(--gold-bright);
    transform: translateY(-1px);
  }

  .stratz-btn {
    background: transparent;
    color: var(--text-secondary);
    border: 1px solid var(--border);
  }

  .stratz-btn:hover {
    border-color: var(--border-active);
    color: var(--text-primary);
    transform: translateY(-1px);
  }

  .loading {
    color: var(--text-muted);
    text-align: center;
    padding: 48px;
    font-size: 14px;
  }

  .error {
    color: var(--red);
    background: rgba(248, 113, 113, 0.1);
    border: 1px solid rgba(248, 113, 113, 0.25);
    border-radius: 4px;
    padding: 10px 14px;
    margin-bottom: 16px;
    font-size: 14px;
  }

  /* Overview grid */
  .overview-grid {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 1rem;
  }

  .overview-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .hero-card {
    display: flex;
    align-items: center;
    gap: 1.2rem;
  }

  .hero-info {
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
  }

  .hero-name {
    font-family: 'Rajdhani', sans-serif;
    font-size: 24px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: 1px;
  }

  .result-badge {
    font-family: 'Barlow Condensed', sans-serif;
    display: inline-block;
    padding: 5px 14px;
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 2px;
    border-radius: 4px;
    text-align: center;
    text-transform: uppercase;
  }

  .result-badge.win {
    background: rgba(74, 222, 128, 0.1);
    color: var(--green);
    border: 1px solid rgba(74, 222, 128, 0.3);
  }

  .result-badge.loss {
    background: rgba(248, 113, 113, 0.1);
    color: var(--red);
    border: 1px solid rgba(248, 113, 113, 0.3);
  }

  .meta-card {
    display: flex;
    flex-direction: column;
    gap: 0.8rem;
  }

  .meta-row {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .meta-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 2px;
    color: var(--text-muted);
    min-width: 90px;
  }

  .meta-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .parse-parsed { color: #60c040; }
  .parse-unparsed { color: #a0a0a0; }
  .parse-parsing { color: #ffc107; }
  .parse-failed { color: #ff6b6b; }

  /* Stats */
  .section-title {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 14px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 2px;
    color: var(--text-muted);
    margin: 0 0 16px 0;
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .stats-section,
  .chart-section,
  .goals-section,
  .items-section {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 1.5rem;
  }

  /* LH@10 summary row */
  .lh10-summary {
    display: flex;
    align-items: center;
    gap: 24px;
    margin-bottom: 16px;
    flex-wrap: wrap;
  }

  .lh10-value-block {
    display: flex;
    flex-direction: column;
    align-items: center;
    background: rgba(240, 180, 41, 0.08);
    border: 1.5px solid rgba(240, 180, 41, 0.35);
    border-radius: 8px;
    padding: 10px 20px;
    min-width: 90px;
  }

  .lh10-number {
    font-family: 'Rajdhani', sans-serif;
    font-size: 2.2rem;
    font-weight: 700;
    color: #f0b429;
    line-height: 1;
  }

  .lh10-label {
    font-size: 11px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-top: 2px;
  }

  .lh10-percentile-block {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .lh10-percentile {
    font-family: 'Rajdhani', sans-serif;
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .lh10-percentile-label {
    font-size: 12px;
    color: var(--text-muted);
  }

  .lh10-filters {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 12px;
  }

  .lh10-filter-group {
    display: flex;
    gap: 4px;
  }

  .lh10-filter-btn {
    padding: 4px 10px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--bg-elevated);
    color: var(--text-muted);
    font-size: 12px;
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 0.5px;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s, color 0.15s;
  }

  .lh10-filter-btn:hover {
    border-color: rgba(240, 180, 41, 0.4);
    color: var(--text-secondary);
  }

  .lh10-filter-btn.active {
    background: rgba(240, 180, 41, 0.12);
    border-color: rgba(240, 180, 41, 0.5);
    color: #f0b429;
  }

  .items-compare {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
  }

  .items-col-label {
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 1.5px;
    margin-bottom: 10px;
  }
  .items-col-label.this { color: #f0b429; border-bottom: 2px solid #f0b429; padding-bottom: 4px; }
  .items-col-label.other { color: rgba(96, 165, 250, 0.9); border-bottom: 2px solid rgba(96, 165, 250, 0.6); padding-bottom: 4px; }

  .item-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .item-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 5px 8px;
    border-radius: 4px;
    background: var(--bg-elevated);
    border: 1px solid transparent;
    transition: border-color 0.15s;
  }
  .item-row:hover { border-color: var(--border); }

  .item-time {
    font-size: 14px;
    font-family: monospace;
    color: #f0b429;
    min-width: 38px;
    font-weight: 600;
  }

  .item-name {
    font-size: 14px;
    color: var(--text-primary);
  }

  .items-empty {
    color: var(--text-muted);
    font-size: 14px;
    font-style: italic;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(110px, 1fr));
    gap: 0.75rem;
  }

  .stat-card {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 1rem 0.75rem;
    text-align: center;
    transition: border-color 0.2s;
  }

  .stat-card:hover {
    border-color: var(--border-active);
  }

  .stat-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 24px;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 0.3rem;
  }

  .stat-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 1.5px;
  }

  .kda {
    font-size: 20px;
  }

  .kda .kills { color: var(--green); }
  .kda .deaths { color: var(--red); }
  .kda .assists { color: var(--gold); }

  /* Chart */
  .chart-container {
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 0.5rem;
    background: var(--bg-elevated);
  }

  .no-data-box {
    text-align: center;
    padding: 2.5rem;
    color: var(--text-muted);
    font-size: 14px;
    border: 1px dashed var(--border);
    border-radius: 6px;
  }

  .no-data-hint {
    font-size: 12px;
    margin-top: 0.5rem;
    color: var(--text-muted);
  }

  /* Goals */
  .goals-summary {
    font-family: 'Rajdhani', sans-serif;
    font-size: 16px;
    font-weight: 600;
    color: var(--text-secondary);
    letter-spacing: 0;
    text-transform: none;
  }

  .goals-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .goal-card {
    display: flex;
    gap: 1rem;
    padding: 14px 18px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
    transition: border-color 0.2s;
  }

  .goal-card:hover {
    border-color: var(--border-active);
  }

  .goal-card.achieved {
    border-left: 3px solid var(--green);
  }

  .goal-card.not-achieved {
    border-left: 3px solid var(--red);
  }

  .goal-status {
    font-size: 16px;
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    flex-shrink: 0;
    align-self: center;
  }

  .goal-card.achieved .goal-status { color: var(--green); }
  .goal-card.not-achieved .goal-status { color: var(--red); }

  .goal-info {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .goal-title {
    font-family: 'Rajdhani', sans-serif;
    font-weight: 600;
    color: var(--text-primary);
    font-size: 16px;
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  .goal-numbers {
    display: flex;
    gap: 1.5rem;
    font-size: 14px;
  }

  .goal-target { color: var(--text-muted); }
  .goal-actual { color: var(--text-secondary); }

  .no-goals-text {
    color: var(--text-muted);
    text-align: center;
    padding: 1.5rem 0;
    margin: 0;
    font-size: 14px;
  }

  @media (max-width: 700px) {
    .overview-grid {
      grid-template-columns: 1fr;
    }

    .page-header {
      flex-wrap: wrap;
    }

    .stats-grid {
      grid-template-columns: repeat(auto-fill, minmax(90px, 1fr));
    }
  }

  /* Goal comparison chips */
  .goal-comparisons {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    margin-top: 6px;
  }

  .cmp-chip {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    background: rgba(255, 200, 80, 0.05);
    border: 1px solid rgba(255, 200, 80, 0.15);
    border-radius: 4px;
    padding: 3px 8px;
    font-size: 12px;
  }

  .cmp-label {
    color: var(--text-muted);
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 0.5px;
    text-transform: uppercase;
    font-size: 12px;
  }

  .cmp-value {
    color: var(--text-secondary);
  }

  .cmp-diff {
    font-weight: 700;
    font-family: 'Barlow Condensed', sans-serif;
  }
  .cmp-diff.better { color: #4ade80; }
  .cmp-diff.worse  { color: #f87171; }
  .cmp-diff.neutral { color: var(--text-muted); }

  .cmp-compare-btn {
    background: transparent;
    border: 1px solid rgba(255,200,80,0.25);
    border-radius: 3px;
    color: var(--text-muted);
    font-size: 12px;
    padding: 1px 5px;
    cursor: pointer;
    transition: all 0.15s;
    margin-left: 2px;
  }

  .cmp-compare-btn:hover {
    border-color: var(--gold);
    color: var(--gold);
  }

  /* Compare button */
  .compare-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 12px;
    padding: 10px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    background: transparent;
    color: var(--text-secondary);
    border: 1px solid var(--border);
  }

  .compare-btn:hover {
    border-color: var(--gold);
    color: var(--gold);
    transform: translateY(-1px);
  }

  /* Comparison banner */
  .cmp-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: rgba(240, 180, 41, 0.07);
    border: 1px solid rgba(240, 180, 41, 0.25);
    border-radius: 6px;
    padding: 10px 16px;
  }

  .cmp-banner-labels {
    display: flex;
    align-items: center;
    gap: 10px;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 14px;
    font-weight: 600;
    letter-spacing: 0.5px;
  }

  .cmp-banner-this { color: var(--gold); }
  .cmp-banner-vs   { color: var(--text-muted); font-size: 12px; }
  .cmp-banner-other { color: var(--text-secondary); }

  .cmp-banner-clear {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 1px;
    text-transform: uppercase;
    background: transparent;
    border: 1px solid rgba(240, 180, 41, 0.3);
    color: var(--text-muted);
    padding: 5px 12px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s;
  }

  .cmp-banner-clear:hover {
    border-color: var(--gold);
    color: var(--gold);
  }

  /* Side-by-side overview */
  .overview-compare {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .overview-compare-col {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .oc-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 2px;
    text-transform: uppercase;
    padding: 4px 0;
  }

  .oc-label.this  { color: var(--gold); border-bottom: 2px solid rgba(240,180,41,0.4); }
  .oc-label.other { color: var(--text-secondary); border-bottom: 1px solid var(--border); }

  /* Stats comparison table */
  .stats-compare-table {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
  }

  .sct-header, .sct-row {
    display: grid;
    grid-template-columns: 130px 1fr 1fr;
  }

  .sct-header {
    background: rgba(255,200,80,0.05);
    border-bottom: 1px solid rgba(255,200,80,0.12);
  }

  .sct-row {
    border-bottom: 1px solid rgba(255,200,80,0.06);
  }

  .sct-row:last-child { border-bottom: none; }

  .sct-row:nth-child(even) {
    background: rgba(255,255,255,0.015);
  }

  .sct-label-col {
    padding: 10px 14px;
    color: var(--text-muted);
    font-size: 14px;
    font-family: 'Barlow Condensed', sans-serif;
    letter-spacing: 0.5px;
    display: flex;
    align-items: center;
  }

  .sct-col {
    padding: 10px 14px;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 16px;
    font-weight: 600;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
  }

  .sct-header .sct-col {
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    padding: 8px 14px;
  }

  .sct-header .sct-col.this  { color: var(--gold); border-left: 2px solid rgba(240,180,41,0.4); }
  .sct-header .sct-col.other { color: var(--text-secondary); border-left: 1px solid rgba(255,200,80,0.12); }

  .sct-col:nth-child(2) { border-left: 2px solid rgba(240,180,41,0.2); }
  .sct-col:nth-child(3) { border-left: 1px solid rgba(255,200,80,0.08); }

  .sct-col.sct-better { color: #4ade80; }

  /* Compare popup */
  .popup-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
  }

  .popup {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 24px;
    width: 360px;
    max-width: 95vw;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .popup-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .popup-header h3 {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 16px;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--gold);
    margin: 0;
  }

  .popup-close {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 14px;
    padding: 4px 8px;
    transition: color 0.15s;
  }

  .popup-close:hover { color: var(--text-primary); }

  .popup-options {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .popup-option-btn {
    display: flex;
    align-items: center;
    gap: 12px;
    background: rgba(255, 200, 80, 0.04);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 12px 14px;
    text-align: left;
    cursor: pointer;
    transition: all 0.15s;
    color: var(--text-primary);
    width: 100%;
  }

  .popup-option-btn:hover:not(:disabled) {
    border-color: var(--gold);
    background: rgba(240, 180, 41, 0.08);
  }

  .popup-option-btn:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .option-icon {
    font-size: 18px;
    flex-shrink: 0;
  }

  .option-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .option-sub {
    font-size: 12px;
    color: var(--text-muted);
    margin-top: 2px;
  }

  .option-sub.muted { color: var(--text-muted); font-style: italic; }

  .popup-manual {
    display: flex;
    gap: 8px;
  }

  .popup-input {
    flex: 1;
    background: var(--bg-input, rgba(255,255,255,0.05));
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 8px 12px;
    color: var(--text-primary);
    font-size: 14px;
  }

  .popup-input:focus {
    outline: none;
    border-color: var(--gold);
  }

  .popup-go-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
    font-size: 12px;
    padding: 8px 16px;
    background: var(--gold);
    color: #080c10;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.15s;
  }

  .popup-go-btn:hover { background: var(--gold-bright, #f5c842); }

  .popup-error {
    font-size: 12px;
    color: var(--red, #f87171);
    margin: 0;
  }
</style>
