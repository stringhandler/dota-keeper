<script>
  import { invoke } from "@tauri-apps/api/core";
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
      const { open } = await import("@tauri-apps/plugin-opener");
      await open(`https://www.opendota.com/matches/${mid}`);
    } catch (e) {
      console.error("Failed to open OpenDota link:", e);
    }
  }

  // Chart config derived from csData
  const goalLineColors = [
    "rgba(255, 100, 100, 0.85)",
    "rgba(100, 180, 255, 0.85)",
    "rgba(180, 100, 255, 0.85)",
    "rgba(100, 230, 150, 0.85)",
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
            borderColor: "#d4af37",
            backgroundColor: "rgba(212, 175, 55, 0.15)",
            borderWidth: 2,
            pointRadius: 2,
            pointHoverRadius: 5,
            fill: true,
            tension: 0.3,
          },
          {
            label: "Denies",
            data: dnValues,
            borderColor: "#8b5e3c",
            backgroundColor: "rgba(139, 94, 60, 0.1)",
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
            labels: { color: "#e0e0e0" },
          },
          tooltip: {
            backgroundColor: "rgba(20, 20, 30, 0.95)",
            borderColor: "rgba(139, 92, 46, 0.5)",
            borderWidth: 1,
            titleColor: "#d4af37",
            bodyColor: "#e0e0e0",
          },
        },
        scales: {
          x: {
            ticks: { color: "#a0a0a0", maxTicksLimit: 15 },
            grid: { color: "rgba(139, 92, 46, 0.15)" },
            title: { display: true, text: "Game Time", color: "#808080" },
          },
          y: {
            ticks: { color: "#a0a0a0" },
            grid: { color: "rgba(139, 92, 46, 0.15)" },
            title: { display: true, text: "Count", color: "#808080" },
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
    background:
      linear-gradient(180deg, rgba(30, 30, 40, 0.9) 0%, rgba(20, 20, 30, 0.9) 100%),
      repeating-linear-gradient(90deg, transparent, transparent 2px, rgba(139, 92, 46, 0.08) 2px, rgba(139, 92, 46, 0.08) 4px);
    background-size: 100%, 4px 4px;
    border: 2px solid rgba(139, 92, 46, 0.5);
    border-radius: 8px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }

  .back-btn {
    color: #a0a0a0;
    text-decoration: none;
    font-size: 0.9rem;
    padding: 8px 14px;
    border: 1px solid rgba(139, 92, 46, 0.4);
    border-radius: 3px;
    transition: all 0.2s ease;
    white-space: nowrap;
    background: rgba(30, 30, 40, 0.6);
  }

  .back-btn:hover {
    color: #d4af37;
    border-color: rgba(139, 92, 46, 0.7);
    background: rgba(40, 40, 55, 0.8);
  }

  .header-title {
    flex: 1;
  }

  .page-header h1 {
    margin: 0 0 0.2rem 0;
    font-size: 1.8em;
    color: #d4af37;
    text-shadow: 0 0 20px rgba(212, 175, 55, 0.5), 2px 2px 4px rgba(0, 0, 0, 0.8);
    letter-spacing: 3px;
    text-transform: uppercase;
  }

  .subtitle {
    margin: 0;
    color: #808080;
    font-size: 0.85rem;
    font-family: 'Courier New', monospace;
  }

  .opendota-btn {
    padding: 10px 18px;
    font-size: 0.9em;
    font-weight: bold;
    font-family: inherit;
    cursor: pointer;
    background: linear-gradient(180deg, rgba(40, 55, 80, 0.8) 0%, rgba(30, 45, 65, 0.8) 100%);
    color: #e0e0e0;
    border: 2px solid rgba(80, 120, 180, 0.5);
    border-radius: 3px;
    letter-spacing: 0.5px;
    transition: all 0.3s ease;
    white-space: nowrap;
  }

  .opendota-btn:hover {
    border-color: rgba(100, 150, 220, 0.7);
    box-shadow: 0 0 15px rgba(80, 120, 200, 0.3);
    transform: translateY(-1px);
  }

  .loading {
    display: flex;
    justify-content: center;
    padding: 3rem;
    color: #a0a0a0;
    font-style: italic;
  }

  .error {
    color: #ff6b6b;
    background-color: rgba(220, 53, 69, 0.2);
    border: 1px solid rgba(220, 53, 69, 0.4);
    border-radius: 3px;
    padding: 0.75rem 1rem;
  }

  /* Overview grid */
  .overview-grid {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 1rem;
  }

  .overview-card {
    background:
      linear-gradient(135deg, rgba(25, 25, 35, 0.9) 0%, rgba(20, 20, 30, 0.95) 100%);
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 6px;
    padding: 1.5rem;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.4);
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
    font-size: 1.2rem;
    font-weight: 700;
    color: #e0e0e0;
    letter-spacing: 1px;
  }

  .result-badge {
    display: inline-block;
    padding: 4px 14px;
    font-size: 0.85rem;
    font-weight: 700;
    letter-spacing: 2px;
    border-radius: 3px;
    text-align: center;
  }

  .result-badge.win {
    background: rgba(96, 192, 64, 0.2);
    color: #60c040;
    border: 1px solid rgba(96, 192, 64, 0.5);
    text-shadow: 0 0 10px rgba(96, 192, 64, 0.4);
  }

  .result-badge.loss {
    background: rgba(220, 53, 69, 0.2);
    color: #ff6b6b;
    border: 1px solid rgba(220, 53, 69, 0.4);
    text-shadow: 0 0 10px rgba(255, 107, 107, 0.4);
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
    color: #808080;
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 1px;
    min-width: 90px;
  }

  .meta-value {
    color: #e0e0e0;
    font-size: 0.95rem;
  }

  .parse-parsed { color: #60c040; }
  .parse-unparsed { color: #a0a0a0; }
  .parse-parsing { color: #ffc107; }
  .parse-failed { color: #ff6b6b; }

  /* Stats */
  .section-title {
    margin: 0 0 1rem 0;
    font-size: 1.1rem;
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 2px;
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .stats-section,
  .chart-section,
  .goals-section {
    background:
      linear-gradient(135deg, rgba(25, 25, 35, 0.9) 0%, rgba(20, 20, 30, 0.95) 100%);
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 6px;
    padding: 1.5rem;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.4);
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(110px, 1fr));
    gap: 0.75rem;
  }

  .stat-card {
    background: rgba(30, 30, 42, 0.7);
    border: 1px solid rgba(139, 92, 46, 0.3);
    border-radius: 4px;
    padding: 1rem 0.75rem;
    text-align: center;
  }

  .stat-value {
    font-size: 1.4rem;
    font-weight: 700;
    color: #e0e0e0;
    margin-bottom: 0.3rem;
  }

  .stat-label {
    font-size: 0.75rem;
    color: #808080;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .kda {
    font-size: 1.2rem;
  }

  .kda .kills { color: #60c040; }
  .kda .deaths { color: #ff6b6b; }
  .kda .assists { color: #ffc107; }

  /* Chart */
  .chart-container {
    border: 1px solid rgba(139, 92, 46, 0.2);
    border-radius: 4px;
    padding: 0.5rem;
    background: rgba(15, 15, 25, 0.5);
  }

  .no-data-box {
    text-align: center;
    padding: 2.5rem;
    color: #808080;
    font-style: italic;
    border: 1px dashed rgba(139, 92, 46, 0.3);
    border-radius: 4px;
  }

  .no-data-hint {
    font-size: 0.85rem;
    margin-top: 0.5rem;
    color: #606060;
  }

  /* Goals */
  .goals-summary {
    font-size: 0.9rem;
    font-weight: 600;
    color: #a0a0a0;
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
    padding: 1rem 1.2rem;
    border-radius: 4px;
    border: 2px solid;
  }

  .goal-card.achieved {
    background: linear-gradient(90deg, rgba(60, 100, 40, 0.25) 0%, rgba(40, 80, 30, 0.2) 100%);
    border-color: rgba(96, 192, 64, 0.5);
    border-left: 4px solid #60c040;
  }

  .goal-card.not-achieved {
    background: linear-gradient(90deg, rgba(100, 40, 40, 0.25) 0%, rgba(80, 30, 30, 0.2) 100%);
    border-color: rgba(220, 53, 69, 0.4);
    border-left: 4px solid #ff6b6b;
  }

  .goal-status {
    font-size: 1.1rem;
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.8rem;
    height: 1.8rem;
    border-radius: 50%;
    flex-shrink: 0;
    align-self: center;
  }

  .goal-card.achieved .goal-status { color: #60c040; }
  .goal-card.not-achieved .goal-status { color: #ff6b6b; }

  .goal-info {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .goal-title {
    font-weight: 600;
    color: #e0e0e0;
    font-size: 0.95rem;
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  .goal-numbers {
    display: flex;
    gap: 1.5rem;
    font-size: 0.85rem;
  }

  .goal-target { color: #a0a0a0; }
  .goal-actual { color: #c0c0c0; }

  .no-goals-text {
    color: #808080;
    font-style: italic;
    text-align: center;
    padding: 1.5rem 0;
    margin: 0;
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
