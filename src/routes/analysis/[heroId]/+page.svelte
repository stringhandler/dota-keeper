<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { getHeroName } from "$lib/heroes.js";

  let isLoading = $state(true);
  let error = $state("");

  // Filters
  let timeMinutes = $state(10);
  let windowSize = $state(30);
  let selectedGameMode = $state(null);

  // Hero ID from URL
  let heroId = $derived(parseInt($page.params.heroId));
  let heroName = $derived(getHeroName(heroId));

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
    await loadAnalysis();
  });

  async function loadAnalysis() {
    isLoading = true;
    error = "";

    try {
      analysis = await invoke("get_last_hits_analysis_data", {
        timeMinutes,
        windowSize,
        heroId,
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

  function calculateTrendLine() {
    if (!analysis || !analysis.current_period.data_points) return null;

    const points = analysis.current_period.data_points;
    if (points.length < 2) return null;

    // Calculate linear regression (y = mx + b)
    const n = points.length;
    let sumX = 0, sumY = 0, sumXY = 0, sumX2 = 0;

    points.forEach((point, index) => {
      const x = index;
      const y = point.last_hits;
      sumX += x;
      sumY += y;
      sumXY += x * y;
      sumX2 += x * x;
    });

    const slope = (n * sumXY - sumX * sumY) / (n * sumX2 - sumX * sumX);
    const intercept = (sumY - slope * sumX) / n;

    return { slope, intercept };
  }

  function getTrendLineValue(index) {
    const trend = calculateTrendLine();
    if (!trend) return null;
    return trend.slope * index + trend.intercept;
  }

  function getTrendDescription() {
    const trend = calculateTrendLine();
    if (!trend) return "Not enough data";

    const slopePerGame = trend.slope;
    if (Math.abs(slopePerGame) < 0.5) return "Stable";
    if (slopePerGame > 0) return `Improving (+${slopePerGame.toFixed(2)} LH/game)`;
    return `Declining (${slopePerGame.toFixed(2)} LH/game)`;
  }

  function getTrendClass() {
    const trend = calculateTrendLine();
    if (!trend) return "";
    if (Math.abs(trend.slope) < 0.5) return "stable";
    if (trend.slope > 0) return "improving";
    return "declining";
  }

  function formatDate(timestamp) {
    const date = new Date(timestamp * 1000);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  }

  function getDistributionBuckets() {
    if (!analysis || !analysis.current_period.data_points) return [];

    const points = analysis.current_period.data_points;
    if (points.length === 0) return [];

    const min = Math.min(...points.map(p => p.last_hits));
    const max = Math.max(...points.map(p => p.last_hits));
    const range = max - min;
    const bucketSize = Math.max(5, Math.ceil(range / 10)); // At least 5 LH per bucket

    // Create buckets
    const buckets = [];
    let currentMin = Math.floor(min / bucketSize) * bucketSize;

    while (currentMin <= max) {
      const currentMax = currentMin + bucketSize;
      const count = points.filter(p => p.last_hits >= currentMin && p.last_hits < currentMax).length;

      if (count > 0 || buckets.length === 0) {
        buckets.push({
          min: currentMin,
          max: currentMax,
          count,
          label: `${currentMin}-${currentMax - 1}`
        });
      }

      currentMin = currentMax;
    }

    return buckets;
  }

  function getMaxBucketCount() {
    const buckets = getDistributionBuckets();
    return Math.max(...buckets.map(b => b.count), 1);
  }
</script>

<div class="hero-detail-content">
  <div class="page-header">
    <div class="header-content">
      <a href="/analysis" class="back-link">← Back to Analysis</a>
      <h1>{heroName} - Last Hits Analysis</h1>
      <p class="subtitle">Detailed performance tracking and trends</p>
    </div>
  </div>

  <div class="filters-section">
    <h2>Filters</h2>
    <div class="filters-grid">
      <div class="filter-group">
        <label for="time-minutes">Time Marker</label>
        <select id="time-minutes" bind:value={timeMinutes} on:change={handleFilterChange}>
          <option value={10}>10 minutes</option>
        </select>
      </div>

      <div class="filter-group">
        <label for="window-size">Window Size</label>
        <select id="window-size" bind:value={windowSize} on:change={handleFilterChange}>
          <option value={30}>30 games</option>
        </select>
      </div>

      <div class="filter-group">
        <label for="mode-filter">Game Mode</label>
        <select id="mode-filter" bind:value={selectedGameMode} on:change={handleFilterChange}>
          {#each gameModes as mode}
            <option value={mode.value}>{mode.label}</option>
          {/each}
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
        <p>No data available for {heroName} with the selected filters.</p>
        <p class="hint">Make sure you have parsed matches with this hero at {timeMinutes} minutes.</p>
      </div>
    {:else}
      <div class="stats-section">
        <h2>Summary Statistics</h2>
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

      <div class="trend-section">
        <h2>Trend Analysis</h2>
        <div class="trend-card {getTrendClass()}">
          <div class="trend-indicator">
            {#if getTrendClass() === 'improving'}
              <span class="arrow">↗</span>
            {:else if getTrendClass() === 'declining'}
              <span class="arrow">↘</span>
            {:else}
              <span class="arrow">→</span>
            {/if}
          </div>
          <div class="trend-info">
            <div class="trend-label">Trend</div>
            <div class="trend-value">{getTrendDescription()}</div>
          </div>
        </div>
      </div>

      <div class="distribution-section">
        <h2>Distribution (Last {analysis.current_period.count} games)</h2>
        <div class="distribution-chart">
          {#each getDistributionBuckets() as bucket}
            <div class="distribution-bar">
              <div class="bar-label">{bucket.label} LH</div>
              <div class="bar-container">
                <div
                  class="bar-fill"
                  style="width: {(bucket.count / getMaxBucketCount()) * 100}%"
                ></div>
                <span class="bar-count">{bucket.count}</span>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <div class="timeline-section">
        <h2>Recent Games</h2>
        <div class="timeline-chart">
          <div class="chart-container">
            {#each analysis.current_period.data_points as point, index}
              {@const trendValue = getTrendLineValue(index)}
              <div class="timeline-point" title="{formatDate(point.start_time)}: {point.last_hits} LH">
                <div class="point-marker" style="bottom: {(point.last_hits / analysis.current_period.max) * 100}%"></div>
                {#if trendValue !== null}
                  <div class="trend-marker" style="bottom: {(trendValue / analysis.current_period.max) * 100}%"></div>
                {/if}
              </div>
            {/each}
          </div>
          <div class="chart-axis">
            <div class="axis-label top">{analysis.current_period.max}</div>
            <div class="axis-label middle">{Math.round((analysis.current_period.max + analysis.current_period.min) / 2)}</div>
            <div class="axis-label bottom">{analysis.current_period.min}</div>
          </div>
        </div>
        <div class="chart-legend">
          <div class="legend-item">
            <span class="legend-dot actual"></span>
            <span>Actual LH</span>
          </div>
          <div class="legend-item">
            <span class="legend-dot trend"></span>
            <span>Trend Line</span>
          </div>
        </div>
      </div>

      <div class="games-list-section">
        <h2>Game Details</h2>
        <div class="games-table">
          <div class="table-header">
            <div class="col-date">Date</div>
            <div class="col-lh">Last Hits</div>
            <div class="col-deviation">vs Average</div>
          </div>
          {#each analysis.current_period.data_points as point}
            {@const deviation = point.last_hits - analysis.current_period.average}
            <div class="table-row">
              <div class="col-date">{formatDate(point.start_time)}</div>
              <div class="col-lh">{point.last_hits}</div>
              <div class="col-deviation" class:positive={deviation > 0} class:negative={deviation < 0}>
                {deviation >= 0 ? '+' : ''}{deviation.toFixed(1)}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  .hero-detail-content {
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

  .header-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .back-link {
    color: #a0a0a0;
    text-decoration: none;
    font-size: 0.9rem;
    transition: color 0.3s ease;
  }

  .back-link:hover {
    color: #d4af37;
  }

  .page-header h1 {
    margin: 0;
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

  .filters-section,
  .stats-section,
  .trend-section,
  .distribution-section,
  .timeline-section,
  .games-list-section {
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

  h2 {
    margin: 0 0 1.5rem 0;
    font-size: 1.2em;
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 2px;
    border-bottom: 2px solid rgba(139, 92, 46, 0.5);
    padding-bottom: 15px;
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

  .trend-card {
    display: flex;
    align-items: center;
    gap: 2rem;
    padding: 30px;
    background: rgba(30, 30, 40, 0.6);
    border: 3px solid rgba(139, 92, 46, 0.4);
    border-radius: 8px;
  }

  .trend-card.improving {
    border-color: rgba(96, 192, 64, 0.6);
    background: rgba(96, 192, 64, 0.1);
  }

  .trend-card.declining {
    border-color: rgba(255, 107, 107, 0.6);
    background: rgba(255, 107, 107, 0.1);
  }

  .trend-card.stable {
    border-color: rgba(240, 184, 64, 0.6);
    background: rgba(240, 184, 64, 0.1);
  }

  .trend-indicator {
    font-size: 3rem;
  }

  .trend-card.improving .arrow {
    color: #60c040;
  }

  .trend-card.declining .arrow {
    color: #ff6b6b;
  }

  .trend-card.stable .arrow {
    color: #f0b840;
  }

  .trend-info {
    flex: 1;
  }

  .trend-label {
    font-size: 0.9rem;
    color: #a0a0a0;
    text-transform: uppercase;
    letter-spacing: 1px;
    margin-bottom: 0.5rem;
  }

  .trend-value {
    font-size: 1.5rem;
    font-weight: bold;
    color: #e0e0e0;
  }

  .distribution-chart {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .distribution-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .bar-label {
    min-width: 100px;
    font-weight: 600;
    color: #a0a0a0;
    font-size: 0.9rem;
  }

  .bar-container {
    flex: 1;
    position: relative;
    height: 40px;
    background: rgba(30, 30, 40, 0.6);
    border: 1px solid rgba(139, 92, 46, 0.3);
    border-radius: 3px;
    overflow: hidden;
  }

  .bar-fill {
    height: 100%;
    background: linear-gradient(90deg, #d4af37 0%, #f0c050 50%, #d4af37 100%);
    box-shadow: 0 0 15px rgba(212, 175, 55, 0.5);
    transition: width 0.5s ease;
  }

  .bar-count {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-50%);
    font-weight: bold;
    color: #e0e0e0;
    font-size: 0.9rem;
  }

  .timeline-chart {
    display: flex;
    gap: 1rem;
  }

  .chart-container {
    flex: 1;
    display: flex;
    gap: 4px;
    height: 200px;
    background: rgba(30, 30, 40, 0.4);
    border: 2px solid rgba(139, 92, 46, 0.3);
    border-radius: 3px;
    padding: 10px;
    position: relative;
  }

  .timeline-point {
    flex: 1;
    position: relative;
    min-width: 8px;
  }

  .point-marker {
    position: absolute;
    width: 100%;
    height: 8px;
    background: #d4af37;
    border-radius: 50%;
    box-shadow: 0 0 8px rgba(212, 175, 55, 0.8);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .point-marker:hover {
    transform: scale(1.5);
    box-shadow: 0 0 15px rgba(212, 175, 55, 1);
  }

  .trend-marker {
    position: absolute;
    width: 100%;
    height: 2px;
    background: #60c040;
    box-shadow: 0 0 4px rgba(96, 192, 64, 0.6);
  }

  .chart-axis {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 10px 0;
    min-width: 50px;
  }

  .axis-label {
    font-size: 0.85rem;
    color: #a0a0a0;
    font-weight: 600;
  }

  .chart-legend {
    display: flex;
    gap: 2rem;
    margin-top: 1rem;
    justify-content: center;
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    color: #a0a0a0;
  }

  .legend-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
  }

  .legend-dot.actual {
    background: #d4af37;
    box-shadow: 0 0 8px rgba(212, 175, 55, 0.8);
  }

  .legend-dot.trend {
    background: #60c040;
    box-shadow: 0 0 8px rgba(96, 192, 64, 0.6);
  }

  .games-table {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .table-header {
    display: grid;
    grid-template-columns: 150px 1fr 1fr;
    gap: 1rem;
    padding: 15px 20px;
    background: rgba(40, 40, 50, 0.6);
    border-radius: 3px;
    font-weight: 600;
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 1px;
    font-size: 0.9rem;
  }

  .table-row {
    display: grid;
    grid-template-columns: 150px 1fr 1fr;
    gap: 1rem;
    padding: 15px 20px;
    background: rgba(30, 30, 40, 0.4);
    border-left: 3px solid rgba(139, 92, 46, 0.5);
    border-radius: 3px;
    color: #e0e0e0;
  }

  .col-date {
    color: #a0a0a0;
  }

  .col-lh {
    font-weight: bold;
    color: #d4af37;
  }

  .col-deviation {
    font-weight: bold;
  }

  .col-deviation.positive {
    color: #60c040;
  }

  .col-deviation.negative {
    color: #ff6b6b;
  }
</style>
