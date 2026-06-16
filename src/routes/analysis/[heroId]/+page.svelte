<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { getHeroName } from "$lib/heroes.js";
  import Chart from "$lib/Chart.svelte";
  import HeroIcon from "$lib/HeroIcon.svelte";
  import BenchmarkTable from "$lib/BenchmarkTable.svelte";
  import { _ } from "svelte-i18n";

  let isLoading = $state(true);
  let error = $state("");

  // Filters
  let timeMinutes = $state(10);
  let windowSize = $state(30);
  let selectedGameMode = $state(/** @type {number | null} */ (null));
  let winLossFilter = $state(/** @type {string} */ ("all")); // "all", "win", "loss"

  // Benchmark state
  let rankedBenchmark = $state(/** @type {any} */ (null));
  let turboBenchmark = $state(/** @type {any} */ (null));
  let benchmarkError = $state("");
  let rankedGameCount = $state(0);
  let turboGameCount = $state(0);
  let rankedWarning = $state("");
  let turboWarning = $state("");
  let activeBenchmark = $derived(selectedGameMode === 23 ? turboBenchmark : rankedBenchmark);
  let showBracketOverlays = $state(/** @type {Set<string>} */ (new Set()));

  const BRACKET_COLORS = {
    herald: "#9e9e9e",
    guardian: "#4caf50",
    crusader: "#00bcd4",
    archon: "#2196f3",
    legend: "#9c27b0",
    ancient: "#ff9800",
    divine: "#f44336",
    immortal: "#f0b429",
  };

  const ALL_BRACKETS = ["herald", "guardian", "crusader", "archon", "legend", "ancient", "divine", "immortal"];

  // Hero ID from URL
  let heroId = $derived(parseInt($page.params.heroId ?? '0'));
  let heroName = $derived(getHeroName(heroId));

  // Analysis data
  let analysis = $state(/** @type {any} */ (null));

  // Game modes
  let gameModes = $derived([
    { value: null, label: $_('analysis.all_modes') },
    { value: 22, label: $_('analysis.mode_ranked') },
    { value: 23, label: $_('analysis.mode_turbo') },
    { value: 2, label: $_('analysis.mode_all_pick') },
  ]);

  onMount(async () => {
    await loadAnalysis();
    await loadBenchmarks();
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

  async function loadBenchmarks() {
    benchmarkError = "";
    rankedWarning = "";
    turboWarning = "";
    rankedBenchmark = null;
    turboBenchmark = null;

    try {
      const hasBench = await invoke("has_benchmarks");
      if (!hasBench) {
        benchmarkError = "No benchmark data loaded yet. Restart the app to fetch benchmark data.";
        return;
      }

      const points = analysis?.current_period?.data_points;
      if (!points || points.length === 0) return;

      // Split data by mode and compute per-mode averages
      const rankedPoints = points.filter((/** @type {any} */ p) => p.game_mode === 22);
      const turboPoints = points.filter((/** @type {any} */ p) => p.game_mode === 23);
      rankedGameCount = rankedPoints.length;
      turboGameCount = turboPoints.length;

      const confidentGames = 30;

      // Ranked benchmark
      if (rankedGameCount > 0) {
        const rankedAvg = rankedPoints.reduce((/** @type {number} */ sum, /** @type {any} */ p) => sum + p.last_hits, 0) / rankedGameCount;
        if (rankedGameCount < confidentGames) {
          rankedWarning = `Based on only ${rankedGameCount} ranked game${rankedGameCount === 1 ? '' : 's'} — rank may be inaccurate.`;
        }
        try {
          rankedBenchmark = await invoke("get_hero_benchmark", {
            heroId,
            mode: "ranked",
            statName: "last_hits_10min",
            userValue: rankedAvg,
            userHeroId: heroId,
            userGameMode: 22,
          });
        } catch (e) {
          console.warn("Ranked benchmark load failed:", e);
        }
      }

      // Turbo benchmark
      if (turboGameCount > 0) {
        const turboAvg = turboPoints.reduce((/** @type {number} */ sum, /** @type {any} */ p) => sum + p.last_hits, 0) / turboGameCount;
        if (turboGameCount < confidentGames) {
          turboWarning = `Based on only ${turboGameCount} turbo game${turboGameCount === 1 ? '' : 's'} — rank may be inaccurate.`;
        }
        try {
          turboBenchmark = await invoke("get_hero_benchmark", {
            heroId,
            mode: "turbo",
            statName: "last_hits_10min",
            userValue: turboAvg,
            userHeroId: heroId,
            userGameMode: 23,
          });
        } catch (e) {
          console.warn("Turbo benchmark load failed:", e);
        }
      }

      if (!rankedBenchmark?.rows?.length && !turboBenchmark?.rows?.length && !rankedWarning && !turboWarning) {
        benchmarkError = "No benchmark data found for this hero.";
      }
    } catch (e) {
      benchmarkError = `Failed to load benchmarks: ${e}`;
      console.error("Failed to load benchmarks:", e);
    }
  }

  async function handleFilterChange() {
    await loadAnalysis();
    await loadBenchmarks();
  }

  /** @param {string} bracket */
  function toggleBracketOverlay(bracket) {
    const next = new Set(showBracketOverlays);
    if (next.has(bracket)) {
      next.delete(bracket);
    } else {
      next.add(bracket);
    }
    showBracketOverlays = next;
  }

  /** Generate normal distribution curve points for a bracket */
  function normalDistPoints(/** @type {number} */ mean, /** @type {number} */ sd, /** @type {{ min: number, max: number }[]} */ buckets) {
    return buckets.map(b => {
      const midpoint = (b.min + b.max) / 2;
      const z = (midpoint - mean) / sd;
      const density = Math.exp(-0.5 * z * z) / (sd * Math.sqrt(2 * Math.PI));
      return density;
    });
  }

  function calculateTrendLine() {
    if (!analysis || !analysis.current_period.data_points) return null;

    const points = analysis.current_period.data_points;
    if (points.length < 2) return null;

    // Calculate linear regression (y = mx + b)
    const n = points.length;
    let sumX = 0, sumY = 0, sumXY = 0, sumX2 = 0;

    points.forEach((/** @type {any} */ point, /** @type {number} */ index) => {
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

  /** @param {number} index */
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

  /** @param {number} timestamp */
  function formatDate(timestamp) {
    const date = new Date(timestamp * 1000);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  }

  function getDistributionBuckets() {
    if (!analysis || !analysis.current_period.data_points) return [];

    const points = analysis.current_period.data_points;
    if (points.length === 0) return [];

    const bucketSize = 10;
    const buckets = [];

    for (let currentMin = 0; currentMin < 100; currentMin += bucketSize) {
      const currentMax = currentMin + bucketSize;
      const count = points.filter((/** @type {any} */ p) => p.last_hits >= currentMin && p.last_hits < currentMax).length;
      buckets.push({
        min: currentMin,
        max: currentMax,
        count,
        label: `${currentMin}-${currentMax - 1}`
      });
    }

    return buckets;
  }

  function getMaxBucketCount() {
    const buckets = getDistributionBuckets();
    return Math.max(...buckets.map(b => b.count), 1);
  }

  function getFilteredDataPoints() {
    if (!analysis || !analysis.current_period.data_points) return [];
    let points = analysis.current_period.data_points;
    if (winLossFilter === "win") {
      points = points.filter((/** @type {any} */ p) => p.won === true);
    } else if (winLossFilter === "loss") {
      points = points.filter((/** @type {any} */ p) => p.won === false);
    }
    return points;
  }

  function getDistributionChartConfig() {
    if (!analysis || !analysis.current_period.data_points) return null;

    const buckets = getDistributionBuckets();
    const benchData = activeBenchmark;

    /** @type {any[]} */
    const datasets = [{
      label: 'Your Games',
      data: buckets.map((/** @type {any} */ b) => b.count),
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
      yAxisID: 'y',
    }];

    // Add bracket overlay curves
    if (benchData && benchData.rows.length > 0 && showBracketOverlays.size > 0) {
      const totalGames = analysis.current_period.count || 1;
      for (const row of benchData.rows) {
        if (!showBracketOverlays.has(row.bracket)) continue;
        const color = BRACKET_COLORS[/** @type {keyof typeof BRACKET_COLORS} */ (row.bracket)] || '#555';
        const densities = normalDistPoints(row.mean, row.std_dev, buckets);
        // Scale density to match histogram (density * totalGames * bucketWidth)
        const scaled = densities.map((/** @type {number} */ d) => d * totalGames * 10);
        datasets.push({
          label: row.bracket.charAt(0).toUpperCase() + row.bracket.slice(1),
          data: scaled,
          borderColor: color,
          backgroundColor: 'transparent',
          borderWidth: 1.5,
          borderDash: [4, 4],
          fill: false,
          tension: 0.4,
          pointRadius: 0,
          pointHoverRadius: 0,
          yAxisID: 'y',
        });
      }
    }

    return {
      type: 'line',
      data: {
        labels: buckets.map((/** @type {any} */ b) => b.label),
        datasets,
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            display: showBracketOverlays.size > 0,
            position: 'bottom',
            labels: {
              color: '#726558',
              font: { size: 10 },
              usePointStyle: true,
              padding: 10,
            }
          },
          tooltip: {
            backgroundColor: '#101820',
            titleColor: '#f0b429',
            bodyColor: '#9a8e7c',
            borderColor: 'rgba(255, 200, 80, 0.3)',
            borderWidth: 1,
            padding: 10,
            displayColors: true,
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
        labels: points.map((/** @type {any} */ p, /** @type {number} */ i) => `Game ${i + 1}`),
        datasets: [
          {
            label: 'Last Hits',
            data: points.map((/** @type {any} */ p) => p.last_hits),
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
            data: points.map((/** @type {any} */ _, /** @type {number} */ i) => getTrendLineValue(i)),
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
              /** @param {import('chart.js').TooltipItem<'line'>[]} context */
              title: function(context) {
                const point = points[context[0].dataIndex];
                return formatDate(point.start_time);
              },
              /** @param {import('chart.js').TooltipItem<'line'>} context */
              label: function(context) {
                if (context.datasetIndex === 0) {
                  return `Last Hits: ${context.parsed.y}`;
                } else {
                  return `Trend: ${(context.parsed.y ?? 0).toFixed(1)}`;
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
            min: 0,
            max: 120,
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

    <div class="filter-group">
      <div class="filter-label">Result</div>
      <select class="form-select" id="win-loss-filter" bind:value={winLossFilter} onchange={handleFilterChange}>
        <option value="all">All Games</option>
        <option value="win">Wins Only</option>
        <option value="loss">Losses Only</option>
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
      {@const allPoints = analysis.current_period.data_points}
      {@const rankedPts = allPoints.filter((/** @type {any} */ p) => p.game_mode === 22)}
      {@const turboPts = allPoints.filter((/** @type {any} */ p) => p.game_mode === 23)}
      {@const rankedWins = rankedPts.filter((/** @type {any} */ p) => p.won).length}
      {@const turboWins = turboPts.filter((/** @type {any} */ p) => p.won).length}
      {@const rankedWr = rankedPts.length > 0 ? (rankedWins / rankedPts.length * 100) : null}
      {@const turboWr = turboPts.length > 0 ? (turboWins / turboPts.length * 100) : null}
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
          {#if rankedWr != null}
            <div class="stat-item">
              <div class="stat-item-label">Ranked WR</div>
              <div class="big-stat" style="color:{rankedWr >= 50 ? 'var(--green)' : 'var(--red)'}">
                {rankedWr.toFixed(0)}%
              </div>
              <div class="stat-sub">{rankedWins}W {rankedPts.length - rankedWins}L</div>
            </div>
          {/if}
          {#if turboWr != null}
            <div class="stat-item">
              <div class="stat-item-label">Turbo WR</div>
              <div class="big-stat" style="color:{turboWr >= 50 ? 'var(--green)' : 'var(--red)'}">
                {turboWr.toFixed(0)}%
              </div>
              <div class="stat-sub">{turboWins}W {turboPts.length - turboWins}L</div>
            </div>
          {/if}
        </div>
      </div>

      <!-- BENCHMARK MEDALS -->
      {#if benchmarkError}
        <div class="analysis-card" style="margin-bottom:16px">
          <div class="analysis-card-title">Last Hitting Rank</div>
          <div class="benchmark-warning">{benchmarkError}</div>
        </div>
      {/if}
      {#if rankedBenchmark?.rows?.length > 0 || turboBenchmark?.rows?.length > 0 || rankedWarning || turboWarning}
        <div class="analysis-card" style="margin-bottom:16px">
          <div class="analysis-card-title">Last Hitting Rank</div>
          <div class="experimental-note">This analysis is experimental, could be wrong and might change in future.</div>
          <div class="medals-row">
            <div class="mode-medal">
              <div class="mode-label">Ranked ({rankedGameCount} games)</div>
              {#if rankedBenchmark?.rows?.length > 0}
                <BenchmarkTable benchmarkData={rankedBenchmark} compact={true} />
              {/if}
              {#if rankedWarning}
                <div class="benchmark-warning">{rankedWarning}</div>
              {/if}
            </div>
            <div class="mode-medal">
              <div class="mode-label">Turbo ({turboGameCount} games)</div>
              {#if turboBenchmark?.rows?.length > 0}
                <BenchmarkTable benchmarkData={turboBenchmark} compact={true} />
              {/if}
              {#if turboWarning}
                <div class="benchmark-warning">{turboWarning}</div>
              {/if}
            </div>
          </div>
        </div>
      {/if}

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

        <!-- Bracket overlay toggles -->
        {#if activeBenchmark?.rows?.length > 0}
          <div class="bracket-toggles">
            <span class="toggle-label">Compare with brackets:</span>
            {#each ALL_BRACKETS as bracket}
              {@const color = BRACKET_COLORS[/** @type {keyof typeof BRACKET_COLORS} */ (bracket)]}
              <button
                class="bracket-toggle"
                class:active={showBracketOverlays.has(bracket)}
                style="--bracket-color: {color}"
                onclick={() => toggleBracketOverlay(bracket)}
              >
                <span class="bracket-toggle-dot" style="background: {color}"></span>
                {bracket.charAt(0).toUpperCase() + bracket.slice(1)}
              </button>
            {/each}
          </div>
        {/if}

        {#if getDistributionChartConfig()}
          <Chart config={getDistributionChartConfig()} height="300px" />
        {/if}
      </div>

      <!-- BENCHMARK DETAIL TABLES -->
      {#if rankedBenchmark?.rows?.length > 0}
        <div class="analysis-card" style="margin-bottom:16px">
          <div class="analysis-card-title">Bracket Comparison (Ranked)</div>
          <BenchmarkTable benchmarkData={rankedBenchmark} />
        </div>
      {/if}
      {#if turboBenchmark?.rows?.length > 0}
        <div class="analysis-card" style="margin-bottom:16px">
          <div class="analysis-card-title">Bracket Comparison (Turbo)</div>
          <BenchmarkTable benchmarkData={turboBenchmark} />
        </div>
      {/if}

      <!-- TIMELINE CHART -->
      <div class="analysis-card" style="margin-bottom:16px">
        <div class="analysis-card-title">{$_('analysis.recent_timeline')}</div>
        {#if getTimelineChartConfig()}
          <Chart config={getTimelineChartConfig()} height="350px" />
        {/if}
      </div>

      <!-- BEST & WORST GAMES -->
      {@const points = analysis.current_period.data_points}
      {@const bestLH = [...points].sort((a, b) => b.last_hits - a.last_hits).slice(0, 3)}
      {@const worstLH = [...points].sort((a, b) => a.last_hits - b.last_hits).slice(0, 3)}
      {@const bestGPM = [...points].sort((a, b) => b.gold_per_min - a.gold_per_min).slice(0, 3)}
      {@const worstGPM = [...points].sort((a, b) => a.gold_per_min - b.gold_per_min).slice(0, 3)}
      {@const bestXPM = [...points].sort((a, b) => b.xp_per_min - a.xp_per_min).slice(0, 3)}
      {@const worstXPM = [...points].sort((a, b) => a.xp_per_min - b.xp_per_min).slice(0, 3)}
      {@const kdaOf = (/** @type {any} */ p) => p.deaths === 0 ? p.kills + p.assists : (p.kills + p.assists) / p.deaths}
      {@const bestKDA = [...points].sort((a, b) => kdaOf(b) - kdaOf(a)).slice(0, 3)}
      {@const worstKDA = [...points].sort((a, b) => kdaOf(a) - kdaOf(b)).slice(0, 3)}

      <div class="best-worst-grid">
        <!-- Best Games -->
        <div class="analysis-card">
          <div class="analysis-card-title" style="color:var(--green)">Best Games</div>

          <div class="bw-section">
            <div class="bw-label">Last Hits @{timeMinutes}m</div>
            {#each bestLH as p}
              <a href="/matches/{p.match_id}" class="bw-row best">
                <span class="bw-date">{formatDate(p.start_time)}</span>
                <span class="bw-value">{p.last_hits} LH</span>
                <span class="bw-result" class:win={p.won} class:loss={!p.won}>{p.won ? 'W' : 'L'}</span>
              </a>
            {/each}
          </div>

          <div class="bw-section">
            <div class="bw-label">GPM</div>
            {#each bestGPM as p}
              <a href="/matches/{p.match_id}" class="bw-row best">
                <span class="bw-date">{formatDate(p.start_time)}</span>
                <span class="bw-value">{p.gold_per_min} GPM</span>
                <span class="bw-result" class:win={p.won} class:loss={!p.won}>{p.won ? 'W' : 'L'}</span>
              </a>
            {/each}
          </div>

          <div class="bw-section">
            <div class="bw-label">XPM</div>
            {#each bestXPM as p}
              <a href="/matches/{p.match_id}" class="bw-row best">
                <span class="bw-date">{formatDate(p.start_time)}</span>
                <span class="bw-value">{p.xp_per_min} XPM</span>
                <span class="bw-result" class:win={p.won} class:loss={!p.won}>{p.won ? 'W' : 'L'}</span>
              </a>
            {/each}
          </div>

          <div class="bw-section">
            <div class="bw-label">KDA</div>
            {#each bestKDA as p}
              <a href="/matches/{p.match_id}" class="bw-row best">
                <span class="bw-date">{formatDate(p.start_time)}</span>
                <span class="bw-value">{p.kills}/{p.deaths}/{p.assists} ({kdaOf(p).toFixed(1)})</span>
                <span class="bw-result" class:win={p.won} class:loss={!p.won}>{p.won ? 'W' : 'L'}</span>
              </a>
            {/each}
          </div>
        </div>

        <!-- Worst Games -->
        <div class="analysis-card">
          <div class="analysis-card-title" style="color:var(--red)">Worst Games</div>

          <div class="bw-section">
            <div class="bw-label">Last Hits @{timeMinutes}m</div>
            {#each worstLH as p}
              <a href="/matches/{p.match_id}" class="bw-row worst">
                <span class="bw-date">{formatDate(p.start_time)}</span>
                <span class="bw-value">{p.last_hits} LH</span>
                <span class="bw-result" class:win={p.won} class:loss={!p.won}>{p.won ? 'W' : 'L'}</span>
              </a>
            {/each}
          </div>

          <div class="bw-section">
            <div class="bw-label">GPM</div>
            {#each worstGPM as p}
              <a href="/matches/{p.match_id}" class="bw-row worst">
                <span class="bw-date">{formatDate(p.start_time)}</span>
                <span class="bw-value">{p.gold_per_min} GPM</span>
                <span class="bw-result" class:win={p.won} class:loss={!p.won}>{p.won ? 'W' : 'L'}</span>
              </a>
            {/each}
          </div>

          <div class="bw-section">
            <div class="bw-label">XPM</div>
            {#each worstXPM as p}
              <a href="/matches/{p.match_id}" class="bw-row worst">
                <span class="bw-date">{formatDate(p.start_time)}</span>
                <span class="bw-value">{p.xp_per_min} XPM</span>
                <span class="bw-result" class:win={p.won} class:loss={!p.won}>{p.won ? 'W' : 'L'}</span>
              </a>
            {/each}
          </div>

          <div class="bw-section">
            <div class="bw-label">KDA</div>
            {#each worstKDA as p}
              <a href="/matches/{p.match_id}" class="bw-row worst">
                <span class="bw-date">{formatDate(p.start_time)}</span>
                <span class="bw-value">{p.kills}/{p.deaths}/{p.assists} ({kdaOf(p).toFixed(1)})</span>
                <span class="bw-result" class:win={p.won} class:loss={!p.won}>{p.won ? 'W' : 'L'}</span>
              </a>
            {/each}
          </div>
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
    font-size: 12px;
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
    font-size: 26px;
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
    font-size: 12px;
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
    font-size: 12px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .stat-sub {
    font-size: 12px;
    color: var(--text-muted);
    margin-top: 2px;
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

  /* ── EXPERIMENTAL NOTE ── */
  .experimental-note {
    font-size: 12px;
    font-style: italic;
    color: var(--text-muted);
    margin-bottom: 10px;
  }

  /* ── BENCHMARK WARNING ── */
  .benchmark-warning {
    font-size: 12px;
    color: var(--gold);
    padding: 8px 12px;
    background: rgba(240, 180, 41, 0.08);
    border-radius: 4px;
    border-left: 3px solid var(--gold);
  }

  /* ── BEST / WORST GAMES GRID ── */
  .best-worst-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .bw-section {
    margin-bottom: 14px;
  }

  .bw-section:last-child {
    margin-bottom: 0;
  }

  .bw-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    letter-spacing: 1.5px;
    color: var(--text-muted);
    text-transform: uppercase;
    margin-bottom: 4px;
  }

  .bw-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 5px 10px;
    border-radius: 4px;
    font-size: 14px;
    color: var(--text-secondary);
    text-decoration: none;
    transition: background 0.15s;
    border-left: 2px solid transparent;
  }

  .bw-row:hover {
    background: var(--bg-elevated);
  }

  .bw-row.best {
    border-left-color: var(--green);
  }

  .bw-row.worst {
    border-left-color: var(--red);
  }

  .bw-date {
    color: var(--text-muted);
    font-size: 12px;
    min-width: 60px;
  }

  .bw-value {
    font-family: 'Rajdhani', sans-serif;
    font-weight: 700;
    color: var(--gold);
    flex: 1;
  }

  .bw-result {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 1px;
    padding: 1px 6px;
    border-radius: 3px;
  }

  .bw-result.win {
    color: var(--green);
    background: rgba(74, 222, 128, 0.1);
  }

  .bw-result.loss {
    color: var(--red);
    background: rgba(248, 113, 113, 0.1);
  }

  /* ── NO DATA ── */
  .no-data {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 48px;
    text-align: center;
    color: var(--text-secondary);
    font-size: 14px;
  }

  .no-data p { margin-bottom: 8px; }
  .hint { font-size: 14px; color: var(--text-muted); }


  /* ── BENCHMARK MEDALS ROW ── */
  .medals-row {
    display: flex;
    gap: 24px;
    flex-wrap: wrap;
  }

  .mode-medal {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .mode-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  /* ── BRACKET OVERLAY TOGGLES ── */
  .bracket-toggles {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
    margin-bottom: 12px;
  }

  .toggle-label {
    font-size: 12px;
    color: var(--text-muted);
    margin-right: 4px;
  }

  .bracket-toggle {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 8px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--bg-surface);
    color: var(--text-secondary);
    font-size: 12px;
    font-family: 'Barlow Condensed', sans-serif;
    letter-spacing: 0.5px;
    cursor: pointer;
    transition: all 0.15s;
  }

  .bracket-toggle:hover {
    background: var(--bg-elevated);
  }

  .bracket-toggle.active {
    border-color: var(--bracket-color);
    background: color-mix(in srgb, var(--bracket-color) 15%, transparent);
  }

  .bracket-toggle-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }
</style>
