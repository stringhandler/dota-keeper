<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { getHeroName, heroes } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";

  let isLoading = $state(true);
  let error = $state("");

  // Filters
  let timeMinutes = $state(10);
  let windowSize = $state(30);
  let selectedHeroId = $state(null);
  let selectedGameMode = $state(null);

  // Favorites
  let favoriteHeroes = $state(new Set());

  // Analysis data
  let analysis = $state(null);

  // Goals (for insight generation)
  let goals = $state([]);

  const gameModes = [
    { value: null, label: "All Modes" },
    { value: 22, label: "Ranked" },
    { value: 23, label: "Turbo" },
    { value: 2, label: "All Pick" },
  ];

  onMount(async () => {
    await Promise.all([
      loadFavorites(),
      loadGoals(),
    ]);
    await loadAnalysis();
  });

  async function loadFavorites() {
    try {
      const favorites = await invoke("get_favorite_heroes");
      favoriteHeroes = new Set(favorites);
    } catch (e) {
      console.error("Failed to load favorite heroes:", e);
    }
  }

  async function loadGoals() {
    try {
      goals = await invoke("get_goals");
    } catch (e) {
      console.error("Failed to load goals for insight:", e);
    }
  }

  async function toggleFavorite(heroId) {
    try {
      const isFavorite = await invoke("toggle_favorite_hero", { heroId });
      if (isFavorite) {
        favoriteHeroes.add(heroId);
      } else {
        favoriteHeroes.delete(heroId);
      }
      favoriteHeroes = new Set(favoriteHeroes);
      await loadAnalysis();
    } catch (e) {
      error = `Failed to toggle favorite: ${e}`;
    }
  }

  async function loadAnalysis() {
    isLoading = true;
    error = "";
    try {
      analysis = await invoke("get_last_hits_analysis_data", {
        timeMinutes,
        windowSize,
        heroId: selectedHeroId,
        gameMode: selectedGameMode,
      });
    } catch (e) {
      error = `Failed to load analysis: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  function getChangePercentage() {
    if (!analysis || !analysis.previous_period) return null;
    const current = analysis.current_period.average;
    const previous = analysis.previous_period.average;
    if (previous === 0) return null;
    return ((current - previous) / previous) * 100;
  }

  function getChangeClass() {
    const change = getChangePercentage();
    if (change === null) return "stable";
    if (change > 2) return "improving";
    if (change < -2) return "declining";
    return "stable";
  }

  function getChangeIndicator() {
    const change = getChangePercentage();
    if (change === null) return "→";
    if (change > 0) return "↗";
    if (change < 0) return "↘";
    return "→";
  }

  function formatPercentage(value) {
    return value >= 0 ? `+${value.toFixed(1)}%` : `${value.toFixed(1)}%`;
  }

  // Sorted hero lists
  let allHeroesSorted = Object.entries(heroes)
    .map(([id, name]) => ({ value: parseInt(id), label: name }))
    .sort((a, b) => a.label.localeCompare(b.label));

  let favoriteHeroList = $derived(allHeroesSorted.filter(h => favoriteHeroes.has(h.value)));
  let otherHeroList = $derived(allHeroesSorted.filter(h => !favoriteHeroes.has(h.value)));

  // Sorted hero stats: favorites first, then by average
  function getSortedHeroStats() {
    if (!analysis || !analysis.per_hero_stats) return [];
    return [...analysis.per_hero_stats].sort((a, b) => {
      const aFav = favoriteHeroes.has(a.hero_id) ? 1 : 0;
      const bFav = favoriteHeroes.has(b.hero_id) ? 1 : 0;
      if (aFav !== bFav) return bFav - aFav;
      return b.average - a.average;
    });
  }

  // Get the max average for bar scaling
  function getMaxAverage() {
    const stats = getSortedHeroStats();
    if (stats.length === 0) return 1;
    return Math.max(...stats.map(s => s.average));
  }

  // Generate insight text
  function generateInsight() {
    const stats = getSortedHeroStats();
    if (stats.length === 0) return null;

    // Check heroes against goals
    for (const hs of stats) {
      const heroGoal = goals.find(g =>
        g.hero_id === hs.hero_id && g.metric === 'LastHits'
      );
      if (heroGoal) {
        const heroName = getHeroName(hs.hero_id);
        if (hs.average >= heroGoal.target_value) {
          return `Your ${heroName} CS avg (${hs.average.toFixed(0)}) already beats your ${heroGoal.target_value} CS goal. Consider raising the target.`;
        } else {
          const diff = (heroGoal.target_value - hs.average).toFixed(0);
          return `Focus ${heroName} farming — you're ${diff} CS below your ${heroGoal.target_value} goal on avg.`;
        }
      }
    }

    // Generic insight
    const top = stats[0];
    if (top) {
      return `${getHeroName(top.hero_id)} is your strongest farming hero with ${top.average.toFixed(1)} avg CS @ ${timeMinutes} min.`;
    }
    return null;
  }

  // Top hero for trend card
  function getTopHeroStat() {
    const stats = getSortedHeroStats();
    return stats.length > 0 ? stats[0] : null;
  }
</script>

<div class="analysis-content">
  <!-- HORIZONTAL FILTERS ROW -->
  <div class="filters-row">
    <div class="filter-group">
      <div class="filter-label">Time Marker</div>
      <select class="form-select" bind:value={timeMinutes} onchange={loadAnalysis}>
        <option value={10}>10 minutes</option>
        <option value={15}>15 minutes</option>
        <option value={20}>20 minutes</option>
      </select>
    </div>

    <div class="filter-group">
      <div class="filter-label">Sample Size</div>
      <select class="form-select" bind:value={windowSize} onchange={loadAnalysis}>
        <option value={20}>20 games</option>
        <option value={30}>30 games</option>
        <option value={50}>50 games</option>
      </select>
    </div>

    <div class="filter-group">
      <div class="filter-label">Hero</div>
      <select class="form-select" bind:value={selectedHeroId} onchange={loadAnalysis}>
        <option value={null}>All Heroes</option>
        {#if favoriteHeroList.length > 0}
          <optgroup label="⭐ Favorites">
            {#each favoriteHeroList as hero}
              <option value={hero.value}>{hero.label}</option>
            {/each}
          </optgroup>
        {/if}
        <optgroup label="All Heroes">
          {#each otherHeroList as hero}
            <option value={hero.value}>{hero.label}</option>
          {/each}
        </optgroup>
      </select>
    </div>

    <div class="filter-group">
      <div class="filter-label">Game Mode</div>
      <select class="form-select" bind:value={selectedGameMode} onchange={loadAnalysis}>
        {#each gameModes as mode}
          <option value={mode.value}>{mode.label}</option>
        {/each}
      </select>
    </div>
  </div>

  {#if isLoading}
    <div class="loading-state">Loading analysis...</div>
  {:else if error}
    <div class="error-banner">{error}</div>
  {:else if !analysis || analysis.current_period.count === 0}
    <div class="no-data">
      <p>No data for the selected filters.</p>
      <p class="hint">Make sure you have parsed matches with data at {timeMinutes} minutes.</p>
    </div>
  {:else}
    {@const changeClass = getChangeClass()}
    {@const changePct = getChangePercentage()}
    {@const insight = generateInsight()}
    {@const heroStats = getSortedHeroStats()}
    {@const maxAvg = getMaxAverage()}
    {@const topHero = getTopHeroStat()}

    <div class="analysis-grid">

      <!-- CARD 1: Main stat -->
      <div class="analysis-card">
        <div class="analysis-card-title">Last Hits @ {timeMinutes} min — Last {analysis.current_period.count} games</div>
        <div class="big-stat">{analysis.current_period.average.toFixed(1)}</div>
        <div class="trend-label {changeClass}">
          {getChangeIndicator()}
          {changeClass === 'improving' ? 'Improving' : changeClass === 'declining' ? 'Declining' : 'Stable'}
        </div>
        {#if analysis.previous_period && analysis.previous_period.count > 0}
          {@const diff = analysis.current_period.average - analysis.previous_period.average}
          <div class="change-chip" class:change-pos={diff >= 0} class:change-neg={diff < 0}>
            {diff >= 0 ? '+' : ''}{diff.toFixed(1)} LH ({formatPercentage(changePct)}) vs prev {analysis.previous_period.count}
          </div>
        {/if}
        <!-- Decorative sparkline -->
        <div class="mini-chart">
          <svg viewBox="0 0 300 80" preserveAspectRatio="none">
            <defs>
              <linearGradient id="chartGrad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stop-color="#f0b429" stop-opacity="0.25"/>
                <stop offset="100%" stop-color="#f0b429" stop-opacity="0"/>
              </linearGradient>
            </defs>
            <path d="M0,40 L30,28 L70,48 L110,22 L150,40 L190,32 L230,55 L270,38 L300,42"
                  fill="none" stroke="#f0b429" stroke-width="1.5"/>
            <path d="M0,40 L30,28 L70,48 L110,22 L150,40 L190,32 L230,55 L270,38 L300,42 L300,80 L0,80Z"
                  fill="url(#chartGrad)"/>
          </svg>
        </div>
      </div>

      <!-- CARD 2: Hero breakdown -->
      <div class="analysis-card">
        <div class="analysis-card-title">By Hero — Avg LH @ {timeMinutes} min</div>
        {#if heroStats.length === 0}
          <p style="color:var(--text-muted);font-size:12px">No per-hero data available.</p>
        {:else}
          <div class="hero-breakdown">
            {#each heroStats as hs}
              <div class="hero-stat-row">
                <div class="hero-stat-name">
                  <button
                    class="fav-btn"
                    class:is-fav={favoriteHeroes.has(hs.hero_id)}
                    onclick={() => toggleFavorite(hs.hero_id)}
                    title={favoriteHeroes.has(hs.hero_id) ? "Remove from favorites" : "Add to favorites"}
                  >{favoriteHeroes.has(hs.hero_id) ? '★' : '☆'}</button>
                  <a href="/analysis/{hs.hero_id}" class="hero-link" title="View {getHeroName(hs.hero_id)} details">
                    {getHeroName(hs.hero_id)}
                  </a>
                </div>
                <div class="hero-stat-bar">
                  <div class="hero-stat-fill" style="width:{Math.round((hs.average / maxAvg) * 100)}%"></div>
                </div>
                <div class="hero-stat-val">{hs.average.toFixed(1)}</div>
              </div>
            {/each}
          </div>
          {#if insight}
            <div class="insight-box">
              <div class="insight-label">💡 Insight</div>
              <div class="insight-text">{insight}</div>
            </div>
          {/if}
        {/if}
      </div>

      <!-- CARD 3: Performance range -->
      <div class="analysis-card">
        <div class="analysis-card-title">Performance Range — {analysis.current_period.count} Games</div>
        <div class="range-stats">
          <div class="range-stat">
            <div class="range-label">Avg</div>
            <div class="range-value" style="color:var(--gold)">{analysis.current_period.average.toFixed(1)}</div>
          </div>
          <div class="range-stat">
            <div class="range-label">Best</div>
            <div class="range-value" style="color:var(--green)">{analysis.current_period.max}</div>
          </div>
          <div class="range-stat">
            <div class="range-label">Worst</div>
            <div class="range-value" style="color:var(--red)">{analysis.current_period.min}</div>
          </div>
        </div>
        <div class="range-bar-wrap" style="margin-top:16px">
          <div class="range-bar">
            <div class="range-bar-fill-red" style="width:20%"></div>
            <div class="range-bar-fill-gold" style="width:40%"></div>
            <div class="range-bar-fill-green" style="width:40%"></div>
            <div class="range-marker" style="left:calc({(analysis.current_period.max - analysis.current_period.min) > 0 ? Math.round(((analysis.current_period.average - analysis.current_period.min) / (analysis.current_period.max - analysis.current_period.min)) * 100) : 50}% - 1px)" title="Your average"></div>
          </div>
          <div class="range-labels">
            <span>{analysis.current_period.min}</span>
            <span>Avg: {analysis.current_period.average.toFixed(1)}</span>
            <span>{analysis.current_period.max}</span>
          </div>
        </div>
      </div>

      <!-- CARD 4: Top hero trend -->
      {#if topHero}
        <div class="analysis-card">
          <div class="analysis-card-title">{getHeroName(topHero.hero_id)} — Individual Trend</div>
          <div class="big-stat" style="font-size:36px">{topHero.average.toFixed(1)}</div>
          <div class="trend-label {topHero.trend_percentage > 2 ? 'improving' : topHero.trend_percentage < -2 ? 'declining' : 'stable'}">
            {topHero.trend_percentage > 2 ? '↗ Improving' : topHero.trend_percentage < -2 ? '↘ Declining' : '→ Stable'}
          </div>
          <div class="mini-chart" style="margin-top:14px">
            <svg viewBox="0 0 300 100" preserveAspectRatio="none">
              <defs>
                <linearGradient id="chartGrad2" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="0%" stop-color="#2dd4bf" stop-opacity="0.2"/>
                  <stop offset="100%" stop-color="#2dd4bf" stop-opacity="0"/>
                </linearGradient>
              </defs>
              <path d="M0,60 L50,35 L100,70 L150,45 L200,25 L250,40 L300,35"
                    fill="none" stroke="#2dd4bf" stroke-width="1.5"/>
              <path d="M0,60 L50,35 L100,70 L150,45 L200,25 L250,40 L300,35 L300,100 L0,100Z"
                    fill="url(#chartGrad2)"/>
            </svg>
          </div>
          <div class="hero-stat-meta">
            {topHero.count} games
            {#if topHero.trend_percentage !== 0}
              · {topHero.trend_percentage > 0 ? '+' : ''}{topHero.trend_percentage.toFixed(1)}% trend
            {/if}
          </div>
          <a href="/analysis/{topHero.hero_id}" class="btn btn-ghost" style="margin-top:12px;font-size:10px">
            View Full Details →
          </a>
        </div>
      {/if}

    </div>
  {/if}
</div>

<style>
  .analysis-content {
    max-width: 1400px;
    margin: 0 auto;
  }

  /* ── FILTERS ROW ── */
  .filters-row {
    display: flex;
    gap: 12px;
    margin-bottom: 24px;
    flex-wrap: wrap;
    align-items: flex-end;
  }

  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .filter-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  /* ── 2-COLUMN GRID ── */
  .analysis-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  /* ── CHANGE CHIP ── */
  .change-chip {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 13px;
    font-weight: 600;
    padding: 6px 12px;
    border-radius: 4px;
    margin-top: 10px;
    display: inline-block;
  }

  .change-neg {
    background: rgba(248, 113, 113, 0.1);
    color: var(--red);
    border: 1px solid rgba(248, 113, 113, 0.2);
  }

  .change-pos {
    background: rgba(74, 222, 128, 0.1);
    color: var(--green);
    border: 1px solid rgba(74, 222, 128, 0.2);
  }

  /* ── MINI CHART ── */
  .mini-chart {
    height: 72px;
    margin-top: 16px;
    overflow: hidden;
  }

  .mini-chart svg { width: 100%; height: 100%; }

  /* ── HERO BREAKDOWN ── */
  .hero-breakdown {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .fav-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 12px;
    cursor: pointer;
    padding: 0;
    line-height: 1;
    transition: color 0.2s;
    flex-shrink: 0;
  }

  .fav-btn:hover { color: var(--gold); }
  .fav-btn.is-fav { color: var(--gold); }

  .hero-link {
    color: var(--text-secondary);
    font-size: 12px;
    transition: color 0.2s;
    text-decoration: none;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .hero-link:hover { color: var(--gold); }

  /* Override hero-stat-name width since we have fav button + name */
  :global(.hero-stat-name) {
    width: 110px;
  }

  /* ── RANGE CARD ── */
  .range-stats {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
    margin-top: 4px;
  }

  .range-stat { display: flex; flex-direction: column; gap: 4px; }

  .range-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    letter-spacing: 1.5px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .range-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 26px;
    font-weight: 700;
  }

  .range-bar-wrap { }

  .range-bar {
    height: 24px;
    background: rgba(255, 255, 255, 0.04);
    border-radius: 4px;
    display: flex;
    overflow: hidden;
    position: relative;
  }

  .range-bar-fill-red { background: rgba(248, 113, 113, 0.4); }
  .range-bar-fill-gold { background: rgba(240, 180, 41, 0.4); }
  .range-bar-fill-green { background: rgba(74, 222, 128, 0.4); }

  .range-marker {
    position: absolute;
    top: 3px;
    bottom: 3px;
    width: 2px;
    background: white;
    border-radius: 1px;
  }

  .range-labels {
    display: flex;
    justify-content: space-between;
    font-size: 10px;
    color: var(--text-muted);
    margin-top: 4px;
    font-family: 'Barlow Condensed', sans-serif;
  }

  /* ── TREND CARD ── */
  .hero-stat-meta {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 8px;
    font-family: 'Barlow Condensed', sans-serif;
  }

  /* ── NO DATA ── */
  .no-data {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 48px;
    text-align: center;
    color: var(--text-secondary);
    font-size: 13px;
  }

  .no-data p { margin-bottom: 8px; }
  .hint { font-size: 12px; color: var(--text-muted); }
</style>
