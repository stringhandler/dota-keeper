<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { getHeroName } from "$lib/heroes.js";
  import Chart from "$lib/Chart.svelte";
  import HeroIcon from "$lib/HeroIcon.svelte";
  import { _ } from "svelte-i18n";

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
  let gameModes = $derived([
    { value: null, label: $_('analysis.all_modes') },
    { value: 22, label: $_('analysis.mode_ranked') },
    { value: 23, label: $_('analysis.mode_turbo') },
    { value: 2, label: $_('analysis.mode_all_pick') },
  ]);

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

  function getDistributionChartConfig() {
    if (!analysis || !analysis.current_period.data_points) return null;

    const buckets = getDistributionBuckets();

    return {
      type: 'line',
      data: {
        labels: buckets.map(b => b.label),
        datasets: [{
          label: 'Game Count',
          data: buckets.map(b => b.count),
          borderColor: '#f0b429',
          backgroundColor: 'rgba(240, 180, 41, 0.15)',
          borderWidth: 2,
          fill: true,
          tension: 0.4,
          pointRadius: 5,
          pointHoverRadius: 7,
          pointBackgroundColor: '#f0b429',
          pointBorderColor: 'rgba(255, 255, 255, 0.6)',
          pointBorderWidth: 2,
          pointHoverBackgroundColor: '#f0b429',
          pointHoverBorderColor: '#fff',
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            display: false
          },
          tooltip: {
            backgroundColor: '#101820',
            titleColor: '#f0b429',
            bodyColor: '#9a8e7c',
            borderColor: 'rgba(255, 200, 80, 0.3)',
            borderWidth: 1,
            padding: 10,
            displayColors: false,
            callbacks: {
              label: function(context) {
                return `${context.parsed.y} games`;
              }
            }
          }
        },
        scales: {
          x: {
            grid: {
              color: 'rgba(255, 200, 80, 0.08)',
            },
            ticks: {
              color: '#726558',
              font: { size: 11 }
            },
            title: {
              display: true,
              text: 'Last Hits',
              color: '#9a8e7c',
              font: { size: 11 }
            }
          },
          y: {
            beginAtZero: true,
            grid: {
              color: 'rgba(255, 200, 80, 0.08)',
            },
            ticks: {
              color: '#726558',
              font: { size: 11 },
              stepSize: 1
            },
            title: {
              display: true,
              text: 'Number of Games',
              color: '#9a8e7c',
              font: { size: 11 }
            }
          }
        }
      }
    };
  }

  function getTimelineChartConfig() {
    if (!analysis || !analysis.current_period.data_points) return null;

    const points = analysis.current_period.data_points;
    const trendLine = calculateTrendLine();

    return {
      type: 'line',
      data: {
        labels: points.map((p, i) => `Game ${i + 1}`),
        datasets: [
          {
            label: 'Last Hits',
            data: points.map(p => p.last_hits),
            borderColor: '#f0b429',
            backgroundColor: 'rgba(240, 180, 41, 0.08)',
            borderWidth: 2,
            fill: false,
            tension: 0,
            pointRadius: 4,
            pointHoverRadius: 6,
            pointBackgroundColor: '#f0b429',
            pointBorderColor: 'rgba(255, 255, 255, 0.6)',
            pointBorderWidth: 1,
          },
          trendLine ? {
            label: 'Trend',
            data: points.map((_, i) => getTrendLineValue(i)),
            borderColor: 'rgba(74, 222, 128, 0.7)',
            backgroundColor: 'transparent',
            borderWidth: 1.5,
            borderDash: [5, 5],
            fill: false,
            tension: 0,
            pointRadius: 0,
            pointHoverRadius: 0,
          } : null
        ].filter(Boolean)
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        interaction: {
          mode: 'index',
          intersect: false,
        },
        plugins: {
          legend: {
            display: true,
            position: 'bottom',
            labels: {
              color: '#726558',
              font: { size: 11 },
              usePointStyle: true,
              padding: 15
            }
          },
          tooltip: {
            backgroundColor: '#101820',
            titleColor: '#f0b429',
            bodyColor: '#9a8e7c',
            borderColor: 'rgba(255, 200, 80, 0.3)',
            borderWidth: 1,
            padding: 10,
            callbacks: {
              title: function(context) {
                const point = points[context[0].dataIndex];
                return formatDate(point.start_time);
              },
              label: function(context) {
                if (context.datasetIndex === 0) {
                  return `Last Hits: ${context.parsed.y}`;
                } else {
                  return `Trend: ${context.parsed.y.toFixed(1)}`;
                }
              }
            }
          }
        },
        scales: {
          x: {
            grid: { color: 'rgba(255, 200, 80, 0.08)' },
            ticks: {
              color: '#726558',
              font: { size: 10 },
              maxRotation: 45,
              minRotation: 0,
              autoSkip: true,
              maxTicksLimit: 15
            },
            title: {
              display: true,
              text: 'Game Sequence',
              color: '#9a8e7c',
              font: { size: 11 }
            }
          },
          y: {
            grid: { color: 'rgba(255, 200, 80, 0.08)' },
            ticks: {
              color: '#726558',
              font: { size: 11 }
            },
            title: {
              display: true,
              text: 'Last Hits',
              color: '#9a8e7c',
              font: { size: 11 }
            }
          }
        }
      }
    };
  }
</script>

<div class="hero-detail-content">
  <!-- PAGE HEADER -->
  <div class="detail-header">
    <a href="/analysis" class="back-link">{$_('analysis.back_to_analysis')}</a>
    <div class="detail-title">
      <HeroIcon heroId={heroId} size="large" showName={false} />
      <div>
        <div class="hero-name">{heroName}</div>
        <div class="section-title">{$_('analysis.last_hits_analysis')}</div>
      </div>
    </div>
  </div>

  <!-- FILTERS ROW -->
  <div class="filters-row">
    <div class="filter-group">
      <div class="filter-label">{$_('analysis.filter_time')}</div>
      <select class="form-select" id="time-minutes" bind:value={timeMinutes} onchange={handleFilterChange}>
        <option value={10}>10 minutes</option>
      </select>
    </div>

    <div class="filter-group">
      <div class="filter-label">{$_('analysis.filter_sample')}</div>
      <select class="form-select" id="window-size" bind:value={windowSize} onchange={handleFilterChange}>
        <option value={30}>30 games</option>
      </select>
    </div>

    <div class="filter-group">
      <div class="filter-label">{$_('analysis.filter_mode')}</div>
      <select class="form-select" id="mode-filter" bind:value={selectedGameMode} onchange={handleFilterChange}>
        {#each gameModes as mode}
          <option value={mode.value}>{mode.label}</option>
        {/each}
      </select>
    </div>
  </div>

  {#if isLoading}
    <div class="loading-state">{$_('analysis.loading')}</div>
  {:else if error}
    <div class="error-banner">{error}</div>
  {:else if analysis}
    {#if analysis.current_period.count === 0}
      <div class="no-data">
        <p>{$_('analysis.no_data')}</p>
        <p class="hint">{$_('analysis.no_data_hint', { values: { minutes: timeMinutes } })}</p>
      </div>
    {:else}
      <!-- SUMMARY STATS -->
      <div class="analysis-card" style="margin-bottom:16px">
        <div class="analysis-card-title">{$_('analysis.summary_stats')}</div>
        <div class="stats-grid">
          <div class="stat-item">
            <div class="stat-item-label">{$_('analysis.avg_lh')}</div>
            <div class="big-stat">{analysis.current_period.average.toFixed(1)}</div>
          </div>
          <div class="stat-item">
            <div class="stat-item-label">{$_('analysis.min_val')}</div>
            <div class="big-stat" style="color:var(--red)">{analysis.current_period.min}</div>
          </div>
          <div class="stat-item">
            <div class="stat-item-label">{$_('analysis.max_val')}</div>
            <div class="big-stat" style="color:var(--green)">{analysis.current_period.max}</div>
          </div>
          <div class="stat-item">
            <div class="stat-item-label">Games</div>
            <div class="big-stat" style="color:var(--teal)">{analysis.current_period.count}</div>
          </div>
        </div>
      </div>

      <!-- TREND -->
      <div class="analysis-card" style="margin-bottom:16px">
        <div class="analysis-card-title">{$_('analysis.trend_analysis')}</div>
        <div class="trend-row">
          <span class="trend-arrow {getTrendClass()}">
            {#if getTrendClass() === 'improving'}↗{:else if getTrendClass() === 'declining'}↘{:else}→{/if}
          </span>
          <div>
            <div class="trend-label {getTrendClass()}">{getTrendDescription()}</div>
          </div>
        </div>
      </div>

      <!-- DISTRIBUTION CHART -->
      <div class="analysis-card" style="margin-bottom:16px">
        <div class="analysis-card-title">{$_('analysis.distribution', { values: { count: analysis.current_period.count } })}</div>
        {#if getDistributionChartConfig()}
          <Chart config={getDistributionChartConfig()} height="300px" />
        {/if}
      </div>

      <!-- TIMELINE CHART -->
      <div class="analysis-card" style="margin-bottom:16px">
        <div class="analysis-card-title">{$_('analysis.recent_timeline')}</div>
        {#if getTimelineChartConfig()}
          <Chart config={getTimelineChartConfig()} height="350px" />
        {/if}
      </div>

      <!-- GAME DETAILS TABLE -->
      <div class="analysis-card">
        <div class="analysis-card-title">{$_('analysis.game_details')}</div>
        <div class="games-table">
          <div class="table-header">
            <div class="col-date">{$_('analysis.col_date')}</div>
            <div class="col-lh">{$_('analysis.col_last_hits')}</div>
            <div class="col-deviation">{$_('analysis.col_vs_avg')}</div>
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

  /* ── HEADER ── */
  .detail-header {
    margin-bottom: 20px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .back-link {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-muted);
    transition: color 0.2s;
  }

  .back-link:hover { color: var(--gold); }

  .detail-title {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .hero-name {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 22px;
    font-weight: 700;
    letter-spacing: 1px;
    color: var(--text-primary);
  }

  /* ── FILTERS ROW ── */
  .filters-row {
    display: flex;
    gap: 12px;
    margin-bottom: 20px;
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

  /* ── STATS GRID ── */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;
  }

  .stat-item {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .stat-item-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  /* ── TREND ROW ── */
  .trend-row {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .trend-arrow {
    font-size: 2.5rem;
    line-height: 1;
  }

  .trend-arrow.improving { color: var(--green); }
  .trend-arrow.declining { color: var(--red); }
  .trend-arrow.stable { color: var(--teal); }

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

  /* ── GAMES TABLE ── */
  .games-table {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .table-header {
    display: grid;
    grid-template-columns: 150px 1fr 1fr;
    gap: 1rem;
    padding: 10px 16px;
    background: var(--bg-elevated);
    border-radius: 4px;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .table-row {
    display: grid;
    grid-template-columns: 150px 1fr 1fr;
    gap: 1rem;
    padding: 10px 16px;
    background: var(--bg-surface);
    border-left: 2px solid var(--border);
    border-radius: 4px;
    font-size: 13px;
    color: var(--text-secondary);
    transition: background 0.15s;
  }

  .table-row:hover { background: var(--bg-elevated); }

  .col-date { color: var(--text-muted); }

  .col-lh {
    font-family: 'Rajdhani', sans-serif;
    font-weight: 700;
    color: var(--gold);
  }

  .col-deviation { font-weight: 600; }
  .col-deviation.positive { color: var(--green); }
  .col-deviation.negative { color: var(--red); }
</style>
