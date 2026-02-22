<script>
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { getHeroName } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";
  import Chart from "$lib/Chart.svelte";

  let matchId = $derived(parseInt($page.params.matchId));
  let match = $state(null);
  let csData = $state([]);
  let goalDetails = $state([]);
  let isLoading = $state(true);
  let error = $state("");

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      const [allMatches, cs, goals] = await Promise.all([
        invoke("get_matches"),
        invoke("get_match_cs", { matchId }),
        invoke("evaluate_goals_for_match", { matchId }),
      ]);

      match = allMatches.find((m) => m.match_id === matchId) ?? null;
      csData = cs;
      goalDetails = goals;

      if (!match) {
        error = "Match not found.";
      }
    } catch (e) {
      error = `Failed to load match: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  function formatDuration(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  function formatDate(timestamp) {
    return new Date(timestamp * 1000).toLocaleString();
  }

  function isWin(m) {
    const isRadiant = m.player_slot < 128;
    return (isRadiant && m.radiant_win) || (!isRadiant && !m.radiant_win);
  }

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

  function getMetricLabel(metric) {
    switch (metric) {
      case "Networth": return "Net Worth";
      case "Kills": return "Kills";
      case "LastHits": return "Last Hits";
      case "Level": return "Level";
      default: return metric;
    }
  }

  function getMetricUnit(metric) {
    switch (metric) {
      case "Networth": return "gold";
      case "Kills": return "kills";
      case "LastHits": return "CS";
      default: return "";
    }
  }

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
      // Linear ramp from 0 at minute 0 to target_value at target_time_minutes; null beyond
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

    return {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "Last Hits",
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
            label: "Denies",
            data: dnValues,
            borderColor: "rgba(163, 163, 163, 0.8)",
            backgroundColor: "rgba(163, 163, 163, 0.1)",
            borderWidth: 2,
            pointRadius: 2,
            pointHoverRadius: 5,
            fill: false,
            tension: 0.3,
          },
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
</script>

<div class="match-detail-content">
  <div class="page-header">
    <a href="/matches" class="back-btn">← Matches</a>
    <div class="header-title">
      <h1>Match Details</h1>
      {#if match}
        <p class="subtitle">#{match.match_id}</p>
      {/if}
    </div>
    {#if match}
      <button class="opendota-btn" onclick={() => openInOpenDota(match.match_id)}>
        View on OpenDota
      </button>
    {/if}
  </div>

  {#if error}
    <p class="error">{error}</p>
  {:else if isLoading}
    <div class="loading"><p>Loading match details...</p></div>
  {:else if match}
    <!-- Match Overview Card -->
    <div class="overview-grid">
      <div class="overview-card hero-card">
        <HeroIcon heroId={match.hero_id} size="large" />
        <div class="hero-info">
          <div class="hero-name">{getHeroName(match.hero_id)}</div>
          <div class="result-badge {isWin(match) ? 'win' : 'loss'}">
            {isWin(match) ? "VICTORY" : "DEFEAT"}
          </div>
        </div>
      </div>

      <div class="overview-card meta-card">
        <div class="meta-row">
          <span class="meta-label">Date</span>
          <span class="meta-value">{formatDate(match.start_time)}</span>
        </div>
        <div class="meta-row">
          <span class="meta-label">Duration</span>
          <span class="meta-value">{formatDuration(match.duration)}</span>
        </div>
        <div class="meta-row">
          <span class="meta-label">Game Mode</span>
          <span class="meta-value">{getGameModeName(match.game_mode)}</span>
        </div>
        <div class="meta-row">
          <span class="meta-label">Parse State</span>
          <span class="meta-value parse-{match.parse_state.toLowerCase()}">{match.parse_state}</span>
        </div>
      </div>
    </div>

    <!-- Performance Stats -->
    <div class="stats-section">
      <h2 class="section-title">Performance</h2>
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-value kda">
            <span class="kills">{match.kills}</span>/<span class="deaths">{match.deaths}</span>/<span class="assists">{match.assists}</span>
          </div>
          <div class="stat-label">K / D / A</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{match.gold_per_min}</div>
          <div class="stat-label">GPM</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{match.xp_per_min}</div>
          <div class="stat-label">XPM</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{match.last_hits}</div>
          <div class="stat-label">Last Hits</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{match.denies}</div>
          <div class="stat-label">Denies</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{match.hero_damage.toLocaleString()}</div>
          <div class="stat-label">Hero Dmg</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{match.tower_damage.toLocaleString()}</div>
          <div class="stat-label">Tower Dmg</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{match.hero_healing.toLocaleString()}</div>
          <div class="stat-label">Healing</div>
        </div>
      </div>
    </div>

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

    <!-- Goals -->
    {#if goalDetails.length > 0}
      <div class="goals-section">
        <h2 class="section-title">
          Goals
          <span class="goals-summary">
            {goalDetails.filter((g) => g.achieved).length}/{goalDetails.length} achieved
          </span>
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
                    Any Hero —
                  {/if}
                  {getMetricLabel(evaluation.goal.metric)}
                </div>
                <div class="goal-numbers">
                  <span class="goal-target">Target: {evaluation.goal.target_value} {getMetricUnit(evaluation.goal.metric)} by {evaluation.goal.target_time_minutes}m</span>
                  <span class="goal-actual">Actual: {evaluation.actual_value} {getMetricUnit(evaluation.goal.metric)}</span>
                </div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {:else if match.parse_state === "Parsed"}
      <div class="goals-section">
        <h2 class="section-title">Goals</h2>
        <p class="no-goals-text">No applicable goals for this match.</p>
      </div>
    {/if}
  {/if}
</div>

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
    font-size: 11px;
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
    font-size: 24px;
    font-weight: 700;
    letter-spacing: 2px;
    color: var(--text-primary);
    text-transform: uppercase;
    margin: 0 0 6px 0;
  }

  .subtitle {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
    margin: 0;
  }

  .opendota-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 11px;
    background: var(--gold);
    color: #080c10;
    border: none;
    padding: 10px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .opendota-btn:hover {
    background: var(--gold-bright);
    transform: translateY(-1px);
  }

  .loading {
    color: var(--text-muted);
    text-align: center;
    padding: 48px;
    font-size: 13px;
  }

  .error {
    color: var(--red);
    background: rgba(248, 113, 113, 0.1);
    border: 1px solid rgba(248, 113, 113, 0.25);
    border-radius: 4px;
    padding: 10px 14px;
    margin-bottom: 16px;
    font-size: 13px;
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
    font-size: 18px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: 1px;
  }

  .result-badge {
    font-family: 'Barlow Condensed', sans-serif;
    display: inline-block;
    padding: 4px 12px;
    font-size: 10px;
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
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 2px;
    color: var(--text-muted);
    min-width: 90px;
  }

  .meta-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 14px;
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
    font-size: 12px;
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
  .goals-section {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 1.5rem;
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
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 0.3rem;
  }

  .stat-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 1.5px;
  }

  .kda {
    font-size: 18px;
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
    font-size: 13px;
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
    font-size: 14px;
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
    font-size: 14px;
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  .goal-numbers {
    display: flex;
    gap: 1.5rem;
    font-size: 12px;
  }

  .goal-target { color: var(--text-muted); }
  .goal-actual { color: var(--text-secondary); }

  .no-goals-text {
    color: var(--text-muted);
    text-align: center;
    padding: 1.5rem 0;
    margin: 0;
    font-size: 13px;
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
</style>
