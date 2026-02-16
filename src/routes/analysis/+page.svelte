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
  let heroFilter = $state("all"); // "all" or "favorites"

  // Favorites
  let favoriteHeroes = $state(new Set());

  // Analysis data
  let analysis = $state(null);

  // Game modes
  const gameModes = [
    { value: null, label: "All Modes" },
    { value: 22, label: "Ranked" },
    { value: 23, label: "Turbo" },
    { value: 2, label: "All Pick" },
  ];

  onMount(async () => {
    await loadFavorites();
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

  async function toggleFavorite(heroId) {
    try {
      const isFavorite = await invoke("toggle_favorite_hero", { heroId });

      if (isFavorite) {
        favoriteHeroes.add(heroId);
      } else {
        favoriteHeroes.delete(heroId);
      }

      // Trigger reactivity
      favoriteHeroes = new Set(favoriteHeroes);

      // Reload analysis to update sorting
      await loadAnalysis();
    } catch (e) {
      console.error("Failed to toggle favorite:", e);
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
      console.error("Failed to load analysis:", e);
    } finally {
      isLoading = false;
    }
  }

  async function handleFilterChange() {
    await loadAnalysis();
  }

  function getChangePercentage() {
    if (!analysis || !analysis.previous_period) return null;

    const current = analysis.current_period.average;
    const previous = analysis.previous_period.average;

    if (previous === 0) return null;

    return ((current - previous) / previous) * 100;
  }

  function getChangeIndicator() {
    const change = getChangePercentage();
    if (change === null) return "";
    if (change > 0) return "↗";
    if (change < 0) return "↘";
    return "→";
  }

  function getChangeClass() {
    const change = getChangePercentage();
    if (change === null) return "";
    if (change > 2) return "improving";
    if (change < -2) return "declining";
    return "stable";
  }

  function formatPercentage(value) {
    return value >= 0 ? `+${value.toFixed(1)}%` : `${value.toFixed(1)}%`;
  }

  // Get hero list sorted alphabetically
  const heroList = Object.entries(heroes)
    .map(([id, name]) => ({ value: parseInt(id), label: name }))
    .sort((a, b) => a.label.localeCompare(b.label));

  heroList.unshift({ value: null, label: "All Heroes" });

  // Filter and sort hero stats based on favorites
  function getFilteredHeroStats() {
    if (!analysis || !analysis.per_hero_stats) return [];

    let stats = [...analysis.per_hero_stats];

    // Apply filter
    if (heroFilter === "favorites") {
      stats = stats.filter(stat => favoriteHeroes.has(stat.hero_id));
    }

    // Sort: favorites first when showing all heroes
    if (heroFilter === "all") {
      stats.sort((a, b) => {
        const aFav = favoriteHeroes.has(a.hero_id) ? 1 : 0;
        const bFav = favoriteHeroes.has(b.hero_id) ? 1 : 0;

        // Favorites first
        if (aFav !== bFav) return bFav - aFav;

        // Within same favorite status, sort by average (descending)
        return b.average - a.average;
      });
    }

    return stats;
  }
</script>

<div class="analysis-content">
  <div class="page-header">
    <h1>Last Hits Analysis</h1>
    <p class="subtitle">Track your farming efficiency over time</p>
  </div>

  <div class="filters-section">
    <h2>Filters</h2>
    <div class="filters-grid">
      <div class="filter-group">
        <label for="time-minutes">Time Marker</label>
        <select id="time-minutes" bind:value={timeMinutes} onchange={handleFilterChange}>
          <option value={10}>10 minutes</option>
        </select>
      </div>

      <div class="filter-group">
        <label for="window-size">Window Size</label>
        <select id="window-size" bind:value={windowSize} onchange={handleFilterChange}>
          <option value={30}>30 games</option>
        </select>
      </div>

      <div class="filter-group">
        <label for="hero-filter">Hero</label>
        <select id="hero-filter" bind:value={selectedHeroId} onchange={handleFilterChange}>
          {#each heroList as hero}
            <option value={hero.value}>{hero.label}</option>
          {/each}
        </select>
      </div>

      <div class="filter-group">
        <label for="mode-filter">Game Mode</label>
        <select id="mode-filter" bind:value={selectedGameMode} onchange={handleFilterChange}>
          {#each gameModes as mode}
            <option value={mode.value}>{mode.label}</option>
          {/each}
        </select>
      </div>

      <div class="filter-group">
        <label for="hero-favorites-filter">Show</label>
        <select id="hero-favorites-filter" bind:value={heroFilter}>
          <option value="all">All Heroes</option>
          <option value="favorites">Favorites Only</option>
        </select>
      </div>
    </div>
  </div>

  {#if isLoading}
    <div class="loading">
      <p>Loading...</p>
    </div>
  {:else if error}
    <p class="error">{error}</p>
  {:else if analysis}
    {#if analysis.current_period.count === 0}
      <div class="no-data">
        <p>No data available for the selected filters.</p>
        <p class="hint">Make sure you have parsed matches with last hits data at {timeMinutes} minutes.</p>
      </div>
    {:else}
      <div class="stats-section">
        <h2>Current Period (Last {analysis.current_period.count} games)</h2>
        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-label">Average LH</div>
            <div class="stat-value">{analysis.current_period.average.toFixed(1)}</div>
          </div>

          <div class="stat-card">
            <div class="stat-label">Min</div>
            <div class="stat-value">{analysis.current_period.min}</div>
          </div>

          <div class="stat-card">
            <div class="stat-label">Max</div>
            <div class="stat-value">{analysis.current_period.max}</div>
          </div>

          <div class="stat-card">
            <div class="stat-label">Games</div>
            <div class="stat-value">{analysis.current_period.count}</div>
          </div>
        </div>
      </div>

      {#if analysis.previous_period && analysis.previous_period.count > 0}
        <div class="comparison-section">
          <h2>Comparison: Last {analysis.current_period.count} vs Previous {analysis.previous_period.count}</h2>
          <div class="comparison-grid">
            <div class="comparison-row">
              <div class="comparison-label">Current:</div>
              <div class="comparison-value">{analysis.current_period.average.toFixed(1)} avg LH</div>
            </div>

            <div class="comparison-row">
              <div class="comparison-label">Previous:</div>
              <div class="comparison-value">{analysis.previous_period.average.toFixed(1)} avg LH</div>
            </div>

            <div class="comparison-row highlight">
              <div class="comparison-label">Change:</div>
              <div class="comparison-value {getChangeClass()}">
                {(analysis.current_period.average - analysis.previous_period.average).toFixed(1)} LH
                ({formatPercentage(getChangePercentage())})
                <span class="indicator">{getChangeIndicator()}</span>
                <span class="status-text">
                  {#if getChangeClass() === 'improving'}
                    IMPROVING
                  {:else if getChangeClass() === 'declining'}
                    DECLINING
                  {:else}
                    STABLE
                  {/if}
                </span>
              </div>
            </div>
          </div>
        </div>
      {/if}

      {#if analysis.per_hero_stats && analysis.per_hero_stats.length > 0}
        <div class="hero-breakdown-section">
          <h2>Per-Hero Breakdown</h2>
          <div class="hero-list">
            {#each getFilteredHeroStats() as heroStat}
              <div class="hero-row-container">
                <button
                  class="favorite-btn"
                  class:is-favorite={favoriteHeroes.has(heroStat.hero_id)}
                  onclick={(e) => {
                    e.stopPropagation();
                    toggleFavorite(heroStat.hero_id);
                  }}
                  title={favoriteHeroes.has(heroStat.hero_id) ? "Remove from favorites" : "Add to favorites"}
                >
                  {favoriteHeroes.has(heroStat.hero_id) ? '★' : '☆'}
                </button>
                <a href="/analysis/{heroStat.hero_id}" class="hero-row">
                  <div class="hero-name">
                    <HeroIcon heroId={heroStat.hero_id} size="small" />
                  </div>
                  <div class="hero-avg">{heroStat.average.toFixed(1)} avg</div>
                  <div class="hero-count">({heroStat.count} games)</div>
                  {#if heroStat.trend_percentage !== 0}
                    <div class="hero-trend" class:positive={heroStat.trend_percentage > 0} class:negative={heroStat.trend_percentage < 0}>
                      {heroStat.trend_percentage > 0 ? '↗' : '↘'} {Math.abs(heroStat.trend_percentage).toFixed(1)}%
                    </div>
                  {/if}
                  <div class="hero-detail-btn">View Details →</div>
                </a>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    {/if}
  {/if}
</div>

<style>
  .analysis-content {
    max-width: 1400px;
    margin: 0 auto;
  }

  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 3rem;
    color: #d4af37;
    font-size: 1rem;
    letter-spacing: 2px;
    text-transform: uppercase;
  }

  .page-header {
    margin-bottom: 2rem;
    padding: 25px 30px;
    background:
      linear-gradient(180deg, rgba(30, 30, 40, 0.9) 0%, rgba(20, 20, 30, 0.9) 100%),
      repeating-linear-gradient(90deg, transparent, transparent 2px, rgba(139, 92, 46, 0.08) 2px, rgba(139, 92, 46, 0.08) 4px);
    background-size: 100%, 4px 4px;
    border: 2px solid rgba(139, 92, 46, 0.5);
    border-radius: 8px;
    box-shadow:
      0 4px 20px rgba(0, 0, 0, 0.5),
      inset 0 1px 0 rgba(255, 255, 255, 0.05);
  }

  .page-header h1 {
    margin: 0 0 0.5rem 0;
    font-size: 2em;
    color: #d4af37;
    text-shadow:
      0 0 20px rgba(212, 175, 55, 0.5),
      2px 2px 4px rgba(0, 0, 0, 0.8);
    letter-spacing: 3px;
    text-transform: uppercase;
  }

  .subtitle {
    color: #a0a0a0;
    margin: 0;
    font-size: 0.9rem;
    letter-spacing: 1px;
  }

  .error {
    color: #ff6b6b;
    background-color: rgba(220, 53, 69, 0.2);
    border: 1px solid rgba(220, 53, 69, 0.4);
    border-radius: 3px;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  .no-data {
    padding: 3rem;
    text-align: center;
    color: #a0a0a0;
    background: rgba(30, 30, 40, 0.5);
    border: 2px solid rgba(139, 92, 46, 0.3);
    border-radius: 8px;
  }

  .no-data p {
    margin: 0.5rem 0;
  }

  .hint {
    font-size: 0.9rem;
    color: #808080;
  }

  .filters-section {
    padding: 30px;
    background:
      linear-gradient(135deg, rgba(25, 25, 35, 0.8) 0%, rgba(20, 20, 30, 0.9) 100%),
      repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.1) 3px, rgba(0, 0, 0, 0.1) 6px);
    background-size: 100%, 6px 6px;
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 8px;
    margin-bottom: 2rem;
    box-shadow:
      0 4px 20px rgba(0, 0, 0, 0.5),
      inset 0 1px 0 rgba(255, 255, 255, 0.03);
  }

  .filters-section h2 {
    margin: 0 0 1.5rem 0;
    font-size: 1.2em;
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 2px;
  }

  .filters-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
  }

  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .filter-group label {
    font-weight: 600;
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 1px;
    font-size: 0.9em;
  }

  select {
    padding: 12px;
    background-color: rgba(30, 30, 40, 0.8);
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 3px;
    color: #e0e0e0;
    font-size: 1em;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  select:focus {
    border-color: rgba(139, 92, 46, 0.8);
    outline: none;
    box-shadow: 0 0 20px rgba(212, 175, 55, 0.3);
  }

  .stats-section,
  .comparison-section,
  .hero-breakdown-section {
    padding: 30px;
    background:
      linear-gradient(135deg, rgba(25, 25, 35, 0.8) 0%, rgba(20, 20, 30, 0.9) 100%),
      repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.1) 3px, rgba(0, 0, 0, 0.1) 6px);
    background-size: 100%, 6px 6px;
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 8px;
    margin-bottom: 2rem;
    box-shadow:
      0 4px 20px rgba(0, 0, 0, 0.5),
      inset 0 1px 0 rgba(255, 255, 255, 0.03);
  }

  .stats-section h2,
  .comparison-section h2,
  .hero-breakdown-section h2 {
    margin: 0 0 1.5rem 0;
    font-size: 1.2em;
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 2px;
    border-bottom: 2px solid rgba(139, 92, 46, 0.5);
    padding-bottom: 15px;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
  }

  .stat-card {
    padding: 25px;
    background: rgba(30, 30, 40, 0.6);
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 5px;
    text-align: center;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.4);
  }

  .stat-label {
    font-size: 0.9rem;
    color: #a0a0a0;
    text-transform: uppercase;
    letter-spacing: 1px;
    margin-bottom: 0.5rem;
  }

  .stat-value {
    font-size: 2rem;
    font-weight: bold;
    color: #d4af37;
    text-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
  }

  .comparison-grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .comparison-row {
    display: grid;
    grid-template-columns: 150px 1fr;
    align-items: center;
    padding: 15px 20px;
    background: rgba(30, 30, 40, 0.4);
    border-left: 3px solid rgba(139, 92, 46, 0.5);
    border-radius: 3px;
  }

  .comparison-row.highlight {
    background: rgba(40, 40, 50, 0.6);
    border-left-width: 4px;
  }

  .comparison-label {
    font-weight: 600;
    color: #a0a0a0;
    text-transform: uppercase;
    letter-spacing: 1px;
    font-size: 0.9rem;
  }

  .comparison-value {
    font-size: 1.1rem;
    font-weight: bold;
    color: #e0e0e0;
  }

  .comparison-value.improving {
    color: #60c040;
  }

  .comparison-value.declining {
    color: #ff6b6b;
  }

  .comparison-value.stable {
    color: #f0b840;
  }

  .indicator {
    margin-left: 0.5rem;
    font-size: 1.2rem;
  }

  .status-text {
    margin-left: 0.5rem;
    font-size: 0.9rem;
    font-weight: bold;
    letter-spacing: 1px;
  }

  .hero-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .hero-row-container {
    display: flex;
    align-items: stretch;
    gap: 0.5rem;
  }

  .favorite-btn {
    flex-shrink: 0;
    width: 45px;
    background: rgba(30, 30, 40, 0.6);
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 3px;
    color: #a0a0a0;
    font-size: 1.5rem;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .favorite-btn:hover {
    background: rgba(40, 40, 50, 0.8);
    border-color: #d4af37;
    color: #d4af37;
  }

  .favorite-btn.is-favorite {
    color: #d4af37;
    text-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
  }

  .hero-row {
    flex: 1;
    display: grid;
    grid-template-columns: 1fr auto auto auto auto;
    gap: 1rem;
    align-items: center;
    padding: 15px 20px;
    background: rgba(30, 30, 40, 0.4);
    border-left: 3px solid rgba(139, 92, 46, 0.5);
    border-radius: 3px;
    text-decoration: none;
    transition: all 0.3s ease;
    cursor: pointer;
  }

  .hero-row:hover {
    background: rgba(40, 40, 50, 0.6);
    border-left-color: #d4af37;
    transform: translateX(5px);
  }

  .hero-name {
    font-weight: 600;
    color: #e0e0e0;
    font-size: 1rem;
  }

  .hero-avg {
    font-size: 1.05rem;
    font-weight: bold;
    color: #d4af37;
  }

  .hero-count {
    font-size: 0.9rem;
    color: #a0a0a0;
  }

  .hero-trend {
    font-size: 1rem;
    font-weight: bold;
    padding: 5px 12px;
    border-radius: 3px;
  }

  .hero-trend.positive {
    color: #60c040;
    background: rgba(96, 192, 64, 0.2);
  }

  .hero-trend.negative {
    color: #ff6b6b;
    background: rgba(255, 107, 107, 0.2);
  }

  .hero-detail-btn {
    font-size: 0.85rem;
    color: #a0a0a0;
    font-weight: 600;
    transition: color 0.3s ease;
  }

  .hero-row:hover .hero-detail-btn {
    color: #d4af37;
  }
</style>
