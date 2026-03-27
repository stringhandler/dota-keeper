<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { heroes, getHeroName } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";
  import ItemIcon from "$lib/ItemIcon.svelte";
  import HeroSelect from "$lib/HeroSelect.svelte";
  import Chart from "$lib/Chart.svelte";
  import { _ } from "svelte-i18n";

  let goalId = $derived($page.params.goalId);
  let goal = $state(/** @type {any} */ (null));
  let matchData = $state(/** @type {any[]} */ ([]));
  let items = $state(/** @type {any[]} */ ([]));
  let isLoading = $state(true);
  let error = $state("");
  let editSuccess = $state("");
  let favoriteHeroIds = $state(new Set());

  // Edit state
  let isEditing = $state(false);
  let isSaving = $state(false);
  let editHeroId = $state("");
  let editMetric = $state("Networth");
  let editTargetValue = $state("");
  let editTargetTime = $state("");
  let editItemId = $state("");
  let editItemMinutes = $state("");
  let editItemSeconds = $state("");
  let editGameMode = $state("All");
  let editFrequencyType = $state("Pct75");

  // Filters
  let selectedHeroId = $state("");
  let selectedPeriod = $state("");

  // Get sorted hero list for dropdown
  const allHeroesSorted = Object.entries(heroes)
    .map(([id, name]) => ({ id: parseInt(id), name }))
    .sort((a, b) => a.name.localeCompare(b.name));

  // Filtered data
  let filteredData = $derived.by(() => {
    let data = matchData;

    // Filter by hero
    if (selectedHeroId) {
      const heroIdNum = parseInt(selectedHeroId);
      data = data.filter((d) => d.hero_id === heroIdNum);
    }

    // Filter by period
    if (selectedPeriod) {
      const days = selectedPeriod === '7d' ? 7 : selectedPeriod === '30d' ? 30 : 365;
      const cutoff = (Date.now() / 1000) - days * 86400;
      data = data.filter((d) => d.start_time >= cutoff);
    }

    return data;
  });

  // Histogram calculations
  let histogram = $derived.by(() => {
    const data = filteredData;
    if (data.length === 0) return { bins: [], max: 0 };

    // Bin size is always a multiple of 10 (minimum 10), targeting ~15 bins
    const dataMax = Math.max(...data.map((d) => d.value));
    const rawBinSize = Math.ceil(dataMax / 15);
    const binSize = Math.max(10, Math.ceil(rawBinSize / 10) * 10);
    const binCount = Math.ceil((dataMax + 1) / binSize);

    const bins = [];
    for (let i = 0; i < binCount; i++) {
      const binStart = i * binSize;
      const binEnd = binStart + binSize;
      const matches = data.filter((d) => d.value >= binStart && (i === binCount - 1 ? d.value < binEnd + binSize : d.value < binEnd));

      bins.push({
        start: binStart,
        end: binEnd,
        count: matches.length,
        wonCount: matches.filter((m) => m.won).length,
        lostCount: matches.filter((m) => !m.won).length,
      });
    }

    const maxCount = Math.max(...bins.map((b) => b.count));

    return { bins, max: maxCount };
  });

  let stats = $derived.by(() => {
    const data = filteredData;
    if (data.length === 0)
      return {
        total: 0,
        achieved: 0,
        failed: 0,
        achievementRate: '0',
        avgValue: 0,
        minValue: 0,
        maxValue: 0,
      };

    const achieved = data.filter((d) => d.achieved).length;
    const values = data.map((d) => d.value);
    const wins = data.filter((d) => d.won);
    const losses = data.filter((d) => !d.won);

    return {
      total: data.length,
      achieved,
      failed: data.length - achieved,
      achievementRate: ((achieved / data.length) * 100).toFixed(1),
      avgValue: Math.round(values.reduce((sum, v) => sum + v, 0) / values.length),
      minValue: Math.min(...values),
      maxValue: Math.max(...values),
      avgWinValue: wins.length ? Math.round(wins.reduce((sum, d) => sum + d.value, 0) / wins.length) : null,
      avgLossValue: losses.length ? Math.round(losses.reduce((sum, d) => sum + d.value, 0) / losses.length) : null,
    };
  });

  // Last 10 games sorted chronologically (oldest first, newest last)
  let last10Games = $derived.by(() => {
    const data = filteredData;
    return [...data]
      .sort((a, b) => a.start_time - b.start_time)
      .slice(-10);
  });

  // Achievement status indicator — behaviour depends on frequency_type
  let achievementStatus = $derived.by(() => {
    if (stats.total === 0 || !goal) return null;
    const freq = goal.frequency_type ?? 'Pct75';

    if (freq === 'JustOnce') {
      const everAchieved = filteredData.some(d => d.achieved);
      return everAchieved
        ? { label: 'Achieved!', color: '#4ade80', desc: 'You hit this at least once.', target: null }
        : { label: 'Not yet', color: '#f97316', desc: 'Keep going — you just need one!', target: null };
    }

    if (freq === 'OnAverage') {
      const avg = stats.avgValue;
      const target = goal.target_value;
      const isItemTiming = goal.metric === 'ItemTiming';
      const passing = isItemTiming ? avg <= target : avg >= target;
      const diff = isItemTiming ? target - avg : avg - target;
      const pct = target > 0 ? Math.abs(diff / target) * 100 : 0;
      if (passing && pct > 10) return { label: 'Above average', color: '#60a5fa', desc: 'Avg comfortably exceeds target', target: null };
      if (passing)             return { label: 'On track', color: '#4ade80', desc: 'Average is meeting the goal', target: null };
      if (pct <= 10)           return { label: 'Close', color: '#fbbf24', desc: `Average is within 10% of target`, target: null };
      if (pct <= 20)           return { label: 'Below', color: '#f97316', desc: `Average is falling short`, target: null };
      return { label: 'Far off', color: '#f87171', desc: 'Average is well below target', target: null };
    }

    // Percentage-based types
    const targetRate = freq === 'Pct50' ? 50 : freq === 'Pct90' ? 90 : 75;
    const rate = parseFloat(stats.achievementRate);
    if (rate > targetRate + 10) return { label: 'Too Easy',  color: '#60a5fa', desc: 'Time to raise the bar!',      target: targetRate };
    if (rate >= targetRate)     return { label: 'Excellent', color: '#4ade80', desc: 'Well-calibrated goal!',        target: targetRate };
    if (rate >= targetRate - 10) return { label: 'Good',     color: '#fbbf24', desc: 'Close! Keep pushing.',         target: targetRate };
    if (rate >= targetRate - 20) return { label: 'Low',      color: '#f97316', desc: 'Consider lowering goal',       target: targetRate };
    return { label: 'Critical', color: '#f87171', desc: 'Goal too ambitious',  target: targetRate };
  });

  // Goal suggestion based on last 10 games pass rate
  let goalSuggestion = $derived.by(() => {
    const last10 = last10Games;
    if (last10.length < 5) return null;
    const last10Mean = last10.reduce((sum, g) => sum + g.value, 0) / last10.length;
    const passCount = last10.filter(g => g.achieved).length;
    const passRate = (passCount / last10.length) * 100;
    if (passRate >= 75) return null;

    // For ItemTiming (lower=better): allow more time; for others: raise the floor
    const suggested = goal?.metric === 'ItemTiming'
      ? Math.round(last10Mean / 0.875)
      : Math.round(last10Mean * 1.125);

    const roundedMean = Math.round(last10Mean);
    if (passRate < 50) {
      return { severity: 'critical', last10Mean: roundedMean, passRate: Math.round(passRate), suggested };
    } else if (passRate < 70) {
      return { severity: 'warning', last10Mean: roundedMean, passRate: Math.round(passRate), suggested };
    } else {
      return { severity: 'info', last10Mean: roundedMean, passRate: Math.round(passRate), suggested: null };
    }
  });

  // Y-axis range for the scatter chart
  let scatterYRange = $derived.by(() => {
    const last10 = last10Games;
    if (last10.length === 0) return { min: 0, max: 100 };
    const values = last10.map(g => g.value);
    const targetVal = goal?.target_value ?? 0;
    const allVals = [...values, targetVal];
    const min = Math.min(...allVals);
    const max = Math.max(...allVals);
    const padding = Math.max((max - min) * 0.25, 5);
    return { min: Math.floor(min - padding), max: Math.ceil(max + padding) };
  });

  // Chart.js config for histogram — rebuilt reactively from histogram + stats + goal
  let histogramChartConfig = $derived.by(() => {
    const bins = histogram.bins;
    if (!goal || bins.length === 0) return null;

    const overlayPlugin = {
      id: 'goalLines',
      /** @param {import('chart.js').Chart} chart */
      afterDraw(chart) {
        const ctx = chart.ctx;
        const xScale = chart.scales.x;
        const yScale = chart.scales.y;

        /** @param {number} value @param {string} color @param {number[]} dash @param {string} label @param {number} labelY */
        const drawVLine = (value, color, dash, label, labelY) => {
          const binIdx = bins.findIndex((b, i) =>
            value >= b.start && (i === bins.length - 1 ? value < b.end + b.end - b.start : value < b.end)
          );
          if (binIdx < 0) return;
          // For grouped bars there are 2 bars per bin group; center on the group
          const groupX = xScale.getPixelForValue(binIdx);
          ctx.save();
          ctx.strokeStyle = color;
          ctx.lineWidth = 2;
          ctx.setLineDash(dash);
          ctx.beginPath();
          ctx.moveTo(groupX, yScale.top);
          ctx.lineTo(groupX, yScale.bottom);
          ctx.stroke();
          ctx.fillStyle = color;
          ctx.font = 'bold 11px sans-serif';
          ctx.textAlign = 'center';
          ctx.fillText(label, groupX, labelY);
          ctx.restore();
        };

        /** @param {number} v */
        const fmt = (v) => goal.metric === 'ItemTiming' ? formatSeconds(v) : String(v);

        drawVLine(goal.target_value, '#f0b429', [5, 5], `Target: ${fmt(goal.target_value)}`, yScale.top - 4);

        if (stats.avgWinValue !== null && stats.avgWinValue !== undefined) {
          drawVLine(/** @type {number} */ (stats.avgWinValue), 'rgba(74,222,128,0.9)', [4, 3], `W avg: ${fmt(/** @type {number} */ (stats.avgWinValue))}`, yScale.top + 12);
        }
        if (stats.avgLossValue !== null && stats.avgLossValue !== undefined) {
          drawVLine(/** @type {number} */ (stats.avgLossValue), 'rgba(248,113,113,0.9)', [4, 3], `L avg: ${fmt(/** @type {number} */ (stats.avgLossValue))}`, yScale.top + 24);
        }
      }
    };

    return {
      type: 'bar',
      plugins: [overlayPlugin],
      data: {
        labels: bins.map(b => goal.metric === 'ItemTiming' ? formatSeconds(b.start) : String(b.start)),
        datasets: [
          {
            label: 'Wins',
            data: bins.map(b => b.wonCount),
            backgroundColor: 'rgba(74, 222, 128, 0.5)',
            borderColor: 'rgba(74, 222, 128, 0.7)',
            borderWidth: 1,
          },
          {
            label: 'Losses',
            data: bins.map(b => b.lostCount),
            backgroundColor: 'rgba(248, 113, 113, 0.5)',
            borderColor: 'rgba(248, 113, 113, 0.7)',
            borderWidth: 1,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        layout: { padding: { top: 36 } },
        plugins: {
          legend: { display: false },
          tooltip: {
            backgroundColor: '#101820',
            titleColor: '#f0b429',
            bodyColor: '#9a8e7c',
            borderColor: 'rgba(255, 200, 80, 0.3)',
            borderWidth: 1,
            padding: 10,
            displayColors: true,
            callbacks: {
              title: (/** @type {import('chart.js').TooltipItem<'bar'>[]} */ items) => {
                const b = bins[items[0].dataIndex];
                return goal.metric === 'ItemTiming'
                  ? `${formatSeconds(b.start)} – ${formatSeconds(b.end)}`
                  : `${b.start} – ${b.end - 1}`;
              },
              label: (/** @type {import('chart.js').TooltipItem<'bar'>} */ item) => `${item.dataset.label}: ${item.parsed.y} games`,
            },
          },
        },
        scales: {
          x: {
            grid: { color: 'rgba(255, 200, 80, 0.08)' },
            ticks: { color: '#726558', font: { size: 11 }, maxRotation: 45, minRotation: 0, autoSkip: true, maxTicksLimit: 15 },
            title: {
              display: true,
              text: goal.metric === 'ItemTiming'
                ? 'Item Timing (M:SS)'
                : `${getMetricLabel(goal.metric)} (${getMetricUnit(goal.metric) || 'value'})`,
              color: '#9a8e7c',
              font: { size: 11 },
            },
          },
          y: {
            beginAtZero: true,
            grid: { color: 'rgba(255, 200, 80, 0.08)' },
            ticks: { color: '#726558', font: { size: 11 }, stepSize: 1 },
            title: {
              display: true,
              text: 'Number of Games',
              color: '#9a8e7c',
              font: { size: 11 },
            },
          },
        },
      },
    };
  });

  let bannerDismissed = $state(false);
  let hoveredDot = $state(/** @type {any} */ (null));

  function dismissBanner() {
    bannerDismissed = true;
    try {
      localStorage.setItem(`banner_dismissed_${goalId}`, JSON.stringify({ count: matchData.length }));
    } catch (e) { /* ignore */ }
  }

  /** @param {number} suggestedValue */
  function applyGoalSuggestion(suggestedValue) {
    startEdit();
    if (goal?.metric === 'ItemTiming') {
      editItemMinutes = Math.floor(suggestedValue / 60).toString();
      editItemSeconds = (suggestedValue % 60).toString();
    } else {
      editTargetValue = suggestedValue.toString();
    }
  }

  /** @param {number} timestamp */
  function formatDate(timestamp) {
    return new Date(timestamp * 1000).toLocaleDateString();
  }

  onMount(async () => {
    const favs = await invoke("get_favorite_heroes").catch(() => []);
    favoriteHeroIds = new Set(favs);
    await loadGoalData();
    // Restore banner dismissal (re-shows after 3+ new games)
    try {
      const stored = localStorage.getItem(`banner_dismissed_${goalId}`);
      if (stored) {
        const { count } = JSON.parse(stored);
        if (matchData.length - count < 3) bannerDismissed = true;
      }
    } catch (e) { /* ignore */ }
  });

  async function loadGoalData() {
    isLoading = true;
    error = "";
    try {
      [goal, matchData, items] = await Promise.all([
        invoke("get_goal", { goalId: parseInt(goalId ?? '0') }),
        invoke("get_goal_histogram_data", { goalId: parseInt(goalId ?? '0') }),
        invoke("get_all_items"),
      ]);
    } catch (e) {
      error = `Failed to load goal data: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  /** @param {number} itemId */
  function getItemName(itemId) {
    const item = items.find(i => i.id === itemId);
    return item ? item.display_name : `Item ${itemId}`;
  }

  /** @param {number} totalSeconds */
  function formatSeconds(totalSeconds) {
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes}:${seconds.toString().padStart(2, "0")}`;
  }

  function resetFilters() {
    selectedHeroId = "";
    selectedPeriod = "";
  }

  /** @param {string} metric */
  function getMetricLabel(metric) {
    switch (metric) {
      case "Networth":
        return "Net Worth";
      case "Kills":
        return "Kills";
      case "LastHits":
        return "Last Hits";
      case "Denies":
        return "Denies";
      case "Level":
        return "Level";
      case "ItemTiming":
        return "Item Timing";
      case "PartnerNetworth":
        return "Partner Networth";
      default:
        return metric;
    }
  }

  /** @param {string} metric */
  function getMetricUnit(metric) {
    switch (metric) {
      case "Networth":
        return "gold";
      case "Kills":
        return "kills";
      case "LastHits":
        return "CS";
      case "Denies":
        return "denies";
      case "Level":
        return "";
      case "ItemTiming":
        return "M:SS";
      case "PartnerNetworth":
        return "gold";
      default:
        return "";
    }
  }

  /** @param {any} g */
  function getHeroLabel(g) {
    if (g.hero_scope) {
      const opt = HERO_GROUP_OPTIONS.find(o => o.value === g.hero_scope);
      return opt ? opt.label : g.hero_scope;
    }
    return g.hero_id !== null ? getHeroName(g.hero_id) : "Any Hero";
  }

  /** @param {any} g */
  function formatGoalDescription(g) {
    if (!g) return "";
    const heroName = getHeroLabel(g);
    const modeStr = g.game_mode && g.game_mode !== 'All' ? ` (${g.game_mode})` : "";
    if (g.metric === "ItemTiming") {
      const itemName = g.item_id !== null ? getItemName(g.item_id) : "Unknown Item";
      const timeStr = formatSeconds(g.target_value);
      return `${heroName}: ${itemName} by ${timeStr}${modeStr}`;
    }
    if (g.metric === "PartnerNetworth") {
      return `${heroName}: Partner ${g.target_value}g by ${g.target_time_minutes} min${modeStr}`;
    }
    const unit = getMetricUnit(g.metric);
    const valueStr = unit ? `${g.target_value} ${unit}` : `Level ${g.target_value}`;
    return `${heroName}: ${valueStr} by ${g.target_time_minutes} min${modeStr}`;
  }

  /** @param {any} value @param {string} metric */
  function formatStatValue(value, metric) {
    if (metric === "ItemTiming") return formatSeconds(value);
    return value;
  }

  const HERO_SCOPES = ["any_core", "any_carry", "any_support"];
  const HERO_GROUP_OPTIONS = [
    { value: "any_core",    label: "Any Core (pos 1/2/3)" },
    { value: "any_carry",   label: "Any Carry (pos 1)" },
    { value: "any_support", label: "Any Support (pos 4/5)" },
  ];

  /** @param {string} freq */
  function getFrequencyLabel(freq) {
    switch (freq) {
      case "JustOnce":  return "Just once";
      case "OnAverage": return "On average";
      case "Pct50":     return "50% of games";
      case "Pct75":     return "75% of games";
      case "Pct90":     return "90% of games";
      default:          return "75% of games";
    }
  }

  /** @param {string} val */
  function parseHeroValue(val) {
    if (!val) return { hero_id: null, hero_scope: null };
    if (HERO_SCOPES.includes(val)) return { hero_id: null, hero_scope: val };
    return { hero_id: parseInt(val), hero_scope: null };
  }

  function startEdit() {
    editHeroId = goal.hero_scope ?? (goal.hero_id !== null ? goal.hero_id.toString() : "");
    editMetric = goal.metric;
    editTargetValue = goal.target_value.toString();
    editTargetTime = goal.target_time_minutes.toString();
    editItemId = goal.item_id !== null ? goal.item_id.toString() : "";
    if (goal.metric === "ItemTiming") {
      editItemMinutes = Math.floor(goal.target_value / 60).toString();
      editItemSeconds = (goal.target_value % 60).toString();
    }
    editGameMode = goal.game_mode;
    editFrequencyType = goal.frequency_type ?? "Pct75";
    isEditing = true;
    editSuccess = "";
    error = "";
  }

  function cancelEdit() {
    isEditing = false;
    error = "";
  }

  async function saveEdit() {
    error = "";
    if (editMetric === "ItemTiming") {
      if (!editHeroId) { error = "Hero is required for Item Timing goals"; return; }
      if (!editItemId) { error = "Item is required for Item Timing goals"; return; }
    } else {
      if (!editTargetValue || !editTargetTime) { error = "Please fill in all required fields"; return; }
    }

    const targetValue = editMetric === "ItemTiming"
      ? (parseInt(editItemMinutes) || 0) * 60 + (parseInt(editItemSeconds) || 0)
      : parseInt(editTargetValue);
    const targetTime = editMetric === "ItemTiming" ? 0 : parseInt(editTargetTime);

    if (isNaN(targetValue) || targetValue <= 0) { error = "Target value must be positive"; return; }
    if (editMetric !== "ItemTiming" && (isNaN(targetTime) || targetTime <= 0)) { error = "Target time must be positive"; return; }

    isSaving = true;
    try {
      const { hero_id, hero_scope } = parseHeroValue(editHeroId);
      await invoke("save_goal", {
        goal: {
          id: goal.id,
          hero_id,
          hero_scope,
          metric: editMetric,
          target_value: targetValue,
          target_time_minutes: targetTime,
          item_id: editItemId ? parseInt(editItemId) : null,
          game_mode: editGameMode,
          frequency_type: editFrequencyType,
          created_at: goal.created_at,
        },
      });
      // Reload goal data so histogram reflects new target
      await loadGoalData();
      isEditing = false;
      editSuccess = "Goal updated successfully.";
    } catch (e) {
      error = `Failed to save goal: ${e}`;
    } finally {
      isSaving = false;
    }
  }
</script>

{#if isLoading}
  <div class="loading">
    <p>{$_('goals.loading')}</p>
  </div>
{:else if error}
  <div class="error-container">
    <p class="error">{error}</p>
    <a href="/goals" class="back-link">← {$_('nav.goals')}</a>
  </div>
{:else if goal}
  <div class="goal-detail-content">
    <div class="page-header">
      <div class="header-content">
        <a href="/goals" class="back-link">← {$_('nav.goals')}</a>
        <div class="header-row">
          <h1>Goal Details</h1>
          {#if !isEditing}
            <button class="edit-btn" onclick={startEdit}>✏️ {$_('goals.edit')}</button>
          {/if}
        </div>

        {#if editSuccess && !isEditing}
          <p class="edit-success">{editSuccess}</p>
        {/if}

        {#if isEditing}
          <div class="edit-form">
            {#if error}
              <p class="form-error">{error}</p>
            {/if}
            <div class="form-row">
              <label style="flex: 2; min-width: 200px;">
                {$_('goals.hero')}
                <HeroSelect bind:value={editHeroId} heroes={allHeroesSorted} favoriteIds={favoriteHeroIds} anyLabel={$_('goals.any_hero')} groupOptions={HERO_GROUP_OPTIONS} />
              </label>
              <label style="flex: 1;">
                {$_('goals.metric')}
                <select bind:value={editMetric} class="form-select">
                  <option value="LastHits">{$_('goals.metric_last_hits')}</option>
                  <option value="Denies">{$_('goals.metric_denies')}</option>
                  <option value="PartnerNetworth">{$_('goals.metric_partner_nw')}</option>
                  <option value="Networth">{$_('goals.metric_networth')}</option>
                  <option value="Kills">{$_('goals.metric_kills')}</option>
                  <option value="Level">{$_('goals.metric_level')}</option>
                  <option value="ItemTiming">{$_('goals.metric_item_timing')}</option>
                </select>
              </label>
              <label style="flex: 1;">
                {$_('goals.mode')}
                <select bind:value={editGameMode} class="form-select">
                  <option value="All">{$_('goals.mode_any')}</option>
                  <option value="Ranked">{$_('goals.mode_ranked')}</option>
                  <option value="Turbo">{$_('goals.mode_turbo')}</option>
                </select>
              </label>
              <label style="flex: 1;">
                How often?
                <select bind:value={editFrequencyType} class="form-select">
                  <option value="JustOnce">Just once</option>
                  <option value="OnAverage">On average</option>
                  <option value="Pct50">50% of games</option>
                  <option value="Pct75">75% of games</option>
                  <option value="Pct90">90% of games</option>
                </select>
              </label>
            </div>

            {#if editMetric === "ItemTiming"}
              <div class="form-row">
                <label>
                  {$_('goals.item')}
                  <select bind:value={editItemId} class="form-select">
                    <option value="">{$_('goals.item_select')}</option>
                    {#each items as item}
                      <option value={item.id}>{item.display_name}</option>
                    {/each}
                  </select>
                </label>
                <label>
                  Target Time
                  <div class="time-inputs">
                    <input type="number" min="0" bind:value={editItemMinutes} class="form-input small" placeholder="min" />
                    :
                    <input type="number" min="0" max="59" bind:value={editItemSeconds} class="form-input small" placeholder="sec" />
                  </div>
                </label>
              </div>
            {:else}
              <div class="form-row">
                <label>
                  Target Value
                  <input type="number" min="1" bind:value={editTargetValue} class="form-input" />
                </label>
                <label>
                  By Minute
                  <input type="number" min="1" bind:value={editTargetTime} class="form-input" />
                </label>
              </div>
            {/if}

            <div class="form-actions">
              <button class="save-edit-btn" onclick={saveEdit} disabled={isSaving}>
                {isSaving ? $_('goals.saving') : $_('goals.update')}
              </button>
              <button class="cancel-edit-btn" onclick={cancelEdit} disabled={isSaving}>
                {$_('goals.cancel')}
              </button>
            </div>
          </div>
        {:else}
          <p class="goal-description">
            {#if goal.hero_id !== null}
              <HeroIcon heroId={goal.hero_id} size="medium" showName={false} />
            {/if}
            {#if goal.metric === 'ItemTiming' && goal.item_id !== null}
              {@const itemEntry = items.find(i => i.id === goal.item_id)}
              {#if itemEntry}
                <ItemIcon itemName={itemEntry.name} displayName={itemEntry.display_name} size="medium" showName={false} />
              {/if}
            {/if}
            {formatGoalDescription(goal)}
          </p>
          <div class="goal-freq-badge">{getFrequencyLabel(goal.frequency_type)}</div>
        {/if}
      </div>
    </div>

    <!-- Filters Row -->
    <div class="filters-row">
      {#if goal.hero_id === null}
        <div class="filter-group">
          <div class="filter-label">{$_('goals_detail.hero_filter')}</div>
          <HeroSelect bind:value={selectedHeroId} heroes={allHeroesSorted} favoriteIds={favoriteHeroIds} anyLabel={$_('goals_detail.all_heroes')} />
        </div>
      {/if}
      <div class="filter-group">
        <div class="filter-label">{$_('goals_detail.period')}</div>
        <div class="period-buttons">
          {#each [['7d', 'goals_detail.period_7d'], ['30d', 'goals_detail.period_30d'], ['1y', 'goals_detail.period_1y']] as [val, tkey]}
            <button
              class="period-btn"
              class:active={selectedPeriod === val}
              onclick={() => selectedPeriod = selectedPeriod === val ? '' : val}
            >{$_(tkey)}</button>
          {/each}
        </div>
      </div>
      <button class="reset-btn" onclick={resetFilters}>{$_('goals_detail.reset')}</button>
    </div>

    <!-- Histogram Section -->
    <section class="histogram-section">
      <h2>{$_('goals_detail.distribution')}</h2>

      <!-- Goal suggestion banner -->
      {#if goalSuggestion && !bannerDismissed}
        {@const s = goalSuggestion}
        <div class="suggestion-banner banner-{s.severity}">
          <div class="banner-body">
            {#if s.severity === 'critical'}
              <span class="banner-icon">⚠️</span>
              <span class="banner-text">
                Your last {last10Games.length} games averaged <strong>{goal.metric === 'ItemTiming' ? formatSeconds(s.last10Mean) : s.last10Mean}</strong>, significantly below your goal of <strong>{goal.metric === 'ItemTiming' ? formatSeconds(goal.target_value) : goal.target_value}</strong>.
                Consider lowering your goal for more consistent progress.
              </span>
            {:else if s.severity === 'warning'}
              <span class="banner-icon">⚠️</span>
              <span class="banner-text">
                You're hitting this goal only <strong>{s.passRate}%</strong> of the time in your last {last10Games.length} games.
                Aim for ~75% success rate.
              </span>
            {:else}
              <span class="banner-icon">ℹ️</span>
              <span class="banner-text">
                You're close! Your recent average is <strong>{goal.metric === 'ItemTiming' ? formatSeconds(s.last10Mean) : s.last10Mean}</strong>. Keep pushing to consistently hit <strong>{goal.metric === 'ItemTiming' ? formatSeconds(goal.target_value) : goal.target_value}</strong>.
              </span>
            {/if}
          </div>
          <div class="banner-actions">
            {#if s.suggested !== null && s.severity !== 'info'}
              <button class="banner-btn banner-btn-primary" onclick={() => applyGoalSuggestion(s.suggested)}>
                Lower to {goal.metric === 'ItemTiming' ? formatSeconds(s.suggested) : s.suggested}
              </button>
            {/if}
            <button class="banner-btn banner-btn-dismiss" onclick={dismissBanner}>✕</button>
          </div>
        </div>
      {/if}

      {#if filteredData.length === 0}
        <p class="no-data">No matches found with the current filters.</p>
      {:else}
        <div class="histogram-container">
          {#if histogramChartConfig}
            <Chart config={histogramChartConfig} height="300px" />
          {/if}

          <div class="legend">
            <div class="legend-item">
              <div class="legend-color wins"></div>
              <span>Wins</span>
            </div>
            <div class="legend-item">
              <div class="legend-color losses"></div>
              <span>Losses</span>
            </div>
            <div class="legend-item">
              <div class="legend-line target-line"></div>
              <span>Target</span>
            </div>
            {#if stats.avgWinValue !== null}
              <div class="legend-item">
                <div class="legend-line win-avg-line"></div>
                <span>Win avg</span>
              </div>
            {/if}
            {#if stats.avgLossValue !== null}
              <div class="legend-item">
                <div class="legend-line loss-avg-line"></div>
                <span>Loss avg</span>
              </div>
            {/if}
          </div>

          <!-- Last N games scatter chart -->
          {#if last10Games.length > 0}
            {@const games = last10Games}
            {@const yr = scatterYRange}
            {@const chartW = 720}
            {@const chartH = 130}
            {@const lm = 60}
            {@const tm = 24}
            {@const bm = 28}
            {@const rm = 48}
            {@const svgW = lm + chartW + rm}
            {@const svgH = tm + chartH + bm}
            {@const ySpan = yr.max - yr.min || 1}

            <div class="scatter-section">
              <h3 class="scatter-title">Last {games.length} Games</h3>
              <div class="scatter-wrapper">
                <svg viewBox="0 0 {svgW} {svgH}" class="scatter-chart"
                  onmouseleave={() => hoveredDot = null}
                  role="img"
                  aria-label="Recent games performance scatter chart"
                >
                  <!-- Grid lines -->
                  {#each [0, 0.25, 0.5, 0.75, 1] as tick}
                    {@const y = tm + chartH * (1 - tick)}
                    {@const val = Math.round(yr.min + tick * ySpan)}
                    <line x1={lm} y1={y} x2={lm + chartW} y2={y} stroke="rgba(255,200,80,0.08)" stroke-width="1" />
                    <text x={lm - 6} y={y + 4} text-anchor="end" fill="var(--text-muted)" font-size="10">
                      {goal.metric === 'ItemTiming' ? formatSeconds(val) : val}
                    </text>
                  {/each}

                  <!-- Goal horizontal line -->
                  <line x1={lm} y1={tm + chartH * (1 - (goal.target_value - yr.min) / ySpan)} x2={lm + chartW} y2={tm + chartH * (1 - (goal.target_value - yr.min) / ySpan)} stroke="var(--gold)" stroke-width="2" />
                  <text x={lm + chartW + 4} y={tm + chartH * (1 - (goal.target_value - yr.min) / ySpan) + 4} fill="var(--gold)" font-size="10" font-weight="600">Goal</text>

                  <!-- Average horizontal line -->
                  <line x1={lm} y1={tm + chartH * (1 - (stats.avgValue - yr.min) / ySpan)} x2={lm + chartW} y2={tm + chartH * (1 - (stats.avgValue - yr.min) / ySpan)} stroke="rgba(251,191,36,0.6)" stroke-width="1.5" stroke-dasharray="5,4" />
                  <text x={lm + chartW + 4} y={tm + chartH * (1 - (stats.avgValue - yr.min) / ySpan) + 4} fill="rgba(251,191,36,0.8)" font-size="10">Avg</text>

                  <!-- Dots -->
                  {#each games as game, i}
                    {@const dotX = lm + (games.length <= 1 ? chartW / 2 : (i / (games.length - 1)) * chartW)}
                    {@const dotY = tm + chartH * (1 - (game.value - yr.min) / ySpan)}
                    {@const isHovered = hoveredDot?.game?.match_id === game.match_id}
                    <circle
                      cx={dotX}
                      cy={dotY}
                      r={isHovered ? 9 : 6}
                      fill={game.achieved ? 'rgba(74,222,128,0.9)' : 'rgba(248,113,113,0.9)'}
                      stroke={game.achieved ? '#4ade80' : '#f87171'}
                      stroke-width="1.5"
                      style="cursor: pointer; transition: r 0.1s"
                      onmouseenter={(e) => hoveredDot = { game, x: e.clientX, y: e.clientY }}
                      onclick={() => { window.location.href = `/matches/${game.match_id}`; }}
                      role="button"
                      tabindex="0"
                      aria-label="Game {i + 1}: {game.value} {getMetricUnit(goal.metric)} - {game.achieved ? 'passed' : 'failed'}"
                    />
                    <text x={dotX} y={svgH - 4} text-anchor="middle" fill="var(--text-muted)" font-size="9">{i + 1}</text>
                  {/each}

                  <!-- Axis labels -->
                  <text x={lm} y={svgH - 4} text-anchor="start" fill="var(--text-muted)" font-size="9" opacity="0.6">oldest</text>
                  <text x={lm + chartW} y={svgH - 4} text-anchor="end" fill="var(--text-muted)" font-size="9" opacity="0.6">newest</text>
                </svg>

                <!-- Hover tooltip -->
                {#if hoveredDot}
                  <div
                    class="dot-tooltip"
                    style="left: {hoveredDot.x + 14}px; top: {hoveredDot.y - 10}px"
                  >
                    <div class="tooltip-row tooltip-status {hoveredDot.game.achieved ? 'pass' : 'fail'}">
                      {hoveredDot.game.achieved ? '✓ Passed' : '✗ Failed'}
                    </div>
                    <div class="tooltip-row">
                      <span class="tooltip-label">Value</span>
                      <span>{goal.metric === 'ItemTiming' ? formatSeconds(hoveredDot.game.value) : hoveredDot.game.value} {getMetricUnit(goal.metric)}</span>
                    </div>
                    <div class="tooltip-row">
                      <span class="tooltip-label">Hero</span>
                      <span>{getHeroName(hoveredDot.game.hero_id)}</span>
                    </div>
                    <div class="tooltip-row">
                      <span class="tooltip-label">Date</span>
                      <span>{formatDate(hoveredDot.game.start_time)}</span>
                    </div>
                    <div class="tooltip-hint">Click to view match</div>
                  </div>
                {/if}
              </div>
            </div>
          {/if}
        </div>
      {/if}
    </section>

    <!-- Statistics Section -->
    <section class="stats-section">
      <h2>Statistics</h2>
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-label">Total Matches</div>
          <div class="stat-value">{stats.total}</div>
        </div>
        <div class="stat-card success">
          <div class="stat-label">Achieved</div>
          <div class="stat-value">{stats.achieved}</div>
        </div>
        <div class="stat-card failure">
          <div class="stat-label">Failed</div>
          <div class="stat-value">{stats.failed}</div>
        </div>
        <div class="stat-card">
          <div class="stat-label">Success Rate</div>
          <div class="stat-value">{stats.achievementRate}%</div>
        </div>
        <div class="stat-card">
          <div class="stat-label">Average</div>
          <div class="stat-value">{formatStatValue(stats.avgValue, goal.metric)} {goal.metric !== "ItemTiming" ? getMetricUnit(goal.metric) : ""}</div>
        </div>
        <div class="stat-card">
          <div class="stat-label">Range</div>
          <div class="stat-value">
            {formatStatValue(stats.minValue, goal.metric)} – {formatStatValue(stats.maxValue, goal.metric)}
          </div>
        </div>
      </div>

      {#if achievementStatus}
        <div class="achievement-rate-card">
          <div class="achievement-rate-row">
            <div class="achievement-rate-info">
              {#if goal.frequency_type === 'JustOnce'}
                <span class="achievement-rate-label">One-time goal</span>
                <span class="achievement-rate-count">({stats.achieved}/{stats.total} games hit target)</span>
              {:else if goal.frequency_type === 'OnAverage'}
                <span class="achievement-rate-label">Average</span>
                <span class="achievement-rate-value">{stats.avgValue}</span>
                <span class="achievement-rate-count">vs target {goal.target_value}</span>
              {:else}
                <span class="achievement-rate-label">Achievement Rate</span>
                <span class="achievement-rate-value">{stats.achievementRate}%</span>
                <span class="achievement-rate-count">({stats.achieved}/{stats.total} games)</span>
              {/if}
            </div>
            {#if achievementStatus.target !== null}
              <div class="achievement-target">Target: ~{achievementStatus.target}%</div>
            {/if}
          </div>
          <div class="achievement-status" style="color: {achievementStatus.color}">
            {achievementStatus.label} — {achievementStatus.desc}
          </div>
          {#if goal.frequency_type !== 'JustOnce' && goal.frequency_type !== 'OnAverage'}
            <div class="achievement-bar-track">
              <div class="achievement-bar-fill" style="width: {Math.min(100, parseFloat(stats.achievementRate))}%; background: {achievementStatus.color}"></div>
              <div class="achievement-bar-target"></div>
            </div>
          {/if}
        </div>
      {/if}
    </section>
  </div>
{/if}

<style>
  .goal-detail-content {
    max-width: 1400px;
    margin: 0 auto;
  }

  .loading,
  .error-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: 48px;
    gap: 16px;
  }

  .loading p {
    color: var(--text-muted);
    font-size: 13px;
  }

  .error {
    color: var(--red);
    background: rgba(248, 113, 113, 0.1);
    border: 1px solid rgba(248, 113, 113, 0.25);
    border-radius: 4px;
    padding: 10px 14px;
    font-size: 13px;
  }

  .back-link {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-secondary);
    text-decoration: none;
    transition: color 0.2s;
    display: inline-block;
    margin-bottom: 12px;
  }

  .back-link:hover {
    color: var(--gold);
  }

  .page-header {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 24px 28px;
    margin-bottom: 28px;
  }

  .header-content {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .page-header h1 {
    font-family: 'Rajdhani', sans-serif;
    font-size: 24px;
    font-weight: 700;
    letter-spacing: 2px;
    color: var(--text-primary);
    text-transform: uppercase;
    margin: 0;
  }

  .goal-description {
    font-size: 14px;
    color: var(--text-secondary);
    margin: 0;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .goal-freq-badge {
    display: inline-block;
    margin-top: 6px;
    font-size: 10px;
    font-family: 'Barlow Condensed', sans-serif;
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--text-secondary);
    background: rgba(154, 142, 124, 0.1);
    border: 1px solid rgba(154, 142, 124, 0.25);
    border-radius: 3px;
    padding: 2px 6px;
  }

  .header-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .edit-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 11px;
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-secondary);
    padding: 6px 14px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .edit-btn:hover {
    border-color: var(--border-active);
    color: var(--text-primary);
  }

  .edit-success {
    color: var(--green);
    font-size: 13px;
    margin: 0;
  }

  .edit-form {
    margin-top: 16px;
    padding: 20px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
  }

  .form-row {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
    margin-bottom: 12px;
  }

  .form-row label {
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 1.5px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .form-select,
  .form-input {
    background: var(--bg-elevated);
    color: var(--text-primary);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 8px 12px;
    font-family: inherit;
    font-size: 13px;
    outline: none;
    transition: border-color 0.2s;
  }

  .form-select:focus,
  .form-input:focus {
    border-color: var(--border-active);
  }

  .form-input.small {
    width: 64px;
    text-align: center;
  }

  .time-inputs {
    display: flex;
    align-items: center;
    gap: 6px;
    color: var(--text-secondary);
  }

  .form-error {
    color: var(--red);
    font-size: 13px;
    margin: 0 0 12px 0;
  }

  .form-actions {
    display: flex;
    gap: 10px;
    margin-top: 16px;
  }

  .save-edit-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 11px;
    background: var(--gold);
    color: #080c10;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .save-edit-btn:hover:not(:disabled) {
    background: var(--gold-bright);
    transform: translateY(-1px);
  }

  .save-edit-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none !important;
  }

  .cancel-edit-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 11px;
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-secondary);
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .cancel-edit-btn:hover:not(:disabled) {
    border-color: var(--border-active);
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

  .period-buttons {
    display: flex;
    gap: 4px;
  }

  .period-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1px;
    text-transform: uppercase;
    font-size: 11px;
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-secondary);
    padding: 6px 10px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .period-btn:hover {
    border-color: var(--border-active);
    color: var(--text-primary);
  }

  .period-btn.active {
    background: rgba(240, 180, 41, 0.12);
    border-color: var(--gold);
    color: var(--gold);
  }

  section {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 24px;
    transition: border-color 0.2s;
  }

  section:hover {
    border-color: var(--border-active);
  }

  h2 {
    font-family: 'Rajdhani', sans-serif;
    font-size: 18px;
    font-weight: 700;
    letter-spacing: 1px;
    color: var(--text-primary);
    text-transform: uppercase;
    margin: 0 0 20px 0;
    border-bottom: 1px solid var(--border);
    padding-bottom: 12px;
  }

  input,
  select {
    background: var(--bg-elevated);
    color: var(--text-primary);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 8px 12px;
    font-size: 13px;
    font-family: inherit;
    outline: none;
    transition: border-color 0.2s;
  }

  input:focus,
  select:focus {
    border-color: var(--border-active);
  }

  .reset-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 11px;
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-secondary);
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .reset-btn:hover {
    border-color: var(--border-active);
    color: var(--text-primary);
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .stat-card {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-left: 3px solid var(--border);
    padding: 16px;
    border-radius: 6px;
    transition: border-color 0.2s;
  }

  .stat-card:hover {
    border-color: var(--border-active);
  }

  .stat-card.success {
    border-left-color: var(--green);
  }

  .stat-card.failure {
    border-left-color: var(--red);
  }

  .stat-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
    margin-bottom: 8px;
  }

  .stat-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 24px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .histogram-section {
    margin-bottom: 24px;
  }

  .no-data {
    color: var(--text-muted);
    text-align: center;
    padding: 32px 0;
    font-size: 13px;
  }

  .histogram-container {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .legend {
    display: flex;
    justify-content: center;
    gap: 32px;
    padding: 16px;
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .legend-color {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    border: 1px solid var(--border);
  }

  .legend-color.wins {
    background: rgba(74, 222, 128, 0.6);
  }

  .legend-color.losses {
    background: rgba(248, 113, 113, 0.6);
  }

  .legend-item span {
    font-size: 13px;
    color: var(--text-secondary);
  }

  /* Achievement rate card */
  .achievement-rate-card {
    margin-top: 16px;
    padding: 14px 16px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
  }

  .achievement-rate-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 6px;
  }

  .achievement-rate-info {
    display: flex;
    align-items: baseline;
    gap: 8px;
  }

  .achievement-rate-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .achievement-rate-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 22px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .achievement-rate-count {
    font-size: 12px;
    color: var(--text-muted);
  }

  .achievement-target {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 1px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .achievement-status {
    font-size: 13px;
    font-weight: 600;
    margin-bottom: 10px;
  }

  .achievement-bar-track {
    position: relative;
    height: 5px;
    background: var(--border);
    border-radius: 3px;
    overflow: visible;
  }

  .achievement-bar-fill {
    height: 100%;
    border-radius: 3px;
    transition: width 0.4s ease;
    min-width: 2px;
  }

  .achievement-bar-target {
    position: absolute;
    left: 75%;
    top: -3px;
    width: 2px;
    height: 11px;
    background: rgba(255, 200, 80, 0.6);
    border-radius: 1px;
  }

  /* Suggestion banner */
  .suggestion-banner {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 16px;
    padding: 14px 16px;
    border-radius: 6px;
    margin-bottom: 20px;
    border-left: 4px solid;
  }

  .banner-critical {
    background: rgba(248, 113, 113, 0.08);
    border-left-color: #f87171;
  }

  .banner-warning {
    background: rgba(249, 115, 22, 0.08);
    border-left-color: #f97316;
  }

  .banner-info {
    background: rgba(96, 165, 250, 0.08);
    border-left-color: #60a5fa;
  }

  .banner-body {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    flex: 1;
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.5;
  }

  .banner-icon {
    font-size: 16px;
    flex-shrink: 0;
    margin-top: 1px;
  }

  .banner-text strong {
    color: var(--text-primary);
  }

  .banner-actions {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .banner-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1px;
    text-transform: uppercase;
    font-size: 11px;
    border-radius: 4px;
    cursor: pointer;
    padding: 6px 12px;
    transition: all 0.2s;
  }

  .banner-btn-primary {
    background: var(--gold);
    color: #080c10;
    border: none;
  }

  .banner-btn-primary:hover {
    background: var(--gold-bright);
    transform: translateY(-1px);
  }

  .banner-btn-dismiss {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-muted);
    padding: 6px 10px;
  }

  .banner-btn-dismiss:hover {
    border-color: var(--border-active);
    color: var(--text-primary);
  }

  /* Legend lines */
  .legend-line {
    width: 28px;
    height: 3px;
    border-radius: 2px;
  }

  .legend-line.target-line {
    background: var(--gold);
  }

  .legend-line.win-avg-line {
    background: repeating-linear-gradient(
      to right,
      rgba(74, 222, 128, 0.9) 0px,
      rgba(74, 222, 128, 0.9) 4px,
      transparent 4px,
      transparent 7px
    );
  }

  .legend-line.loss-avg-line {
    background: repeating-linear-gradient(
      to right,
      rgba(248, 113, 113, 0.9) 0px,
      rgba(248, 113, 113, 0.9) 4px,
      transparent 4px,
      transparent 7px
    );
  }

  /* Scatter chart */
  .scatter-section {
    margin-top: 24px;
    padding-top: 20px;
    border-top: 1px solid var(--border);
  }

  .scatter-title {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
    margin: 0 0 12px 0;
  }

  .scatter-wrapper {
    position: relative;
  }

  .scatter-chart {
    width: 100%;
    height: auto;
  }

  /* Hover tooltip */
  .dot-tooltip {
    position: fixed;
    z-index: 1000;
    background: var(--bg-elevated);
    border: 1px solid var(--border-active);
    border-radius: 6px;
    padding: 10px 12px;
    pointer-events: none;
    min-width: 160px;
    box-shadow: 0 4px 16px rgba(0,0,0,0.4);
  }

  .tooltip-row {
    display: flex;
    justify-content: space-between;
    gap: 12px;
    font-size: 12px;
    color: var(--text-secondary);
    margin-bottom: 4px;
  }

  .tooltip-label {
    color: var(--text-muted);
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    align-self: center;
  }

  .tooltip-status {
    font-weight: 700;
    font-size: 13px;
    margin-bottom: 6px;
    justify-content: flex-start;
  }

  .tooltip-status.pass {
    color: #4ade80;
  }

  .tooltip-status.fail {
    color: #f87171;
  }

  .tooltip-hint {
    margin-top: 6px;
    font-size: 10px;
    color: var(--text-muted);
    text-align: center;
    font-style: italic;
  }

  @media (max-width: 768px) {
    .stats-grid {
      grid-template-columns: repeat(2, 1fr);
    }

    .legend {
      gap: 6px;
      flex-wrap: wrap;
      justify-content: center;
      padding: 10px 0 4px;
    }

    .legend-item {
      background: var(--bg-elevated);
      border: 1px solid var(--border);
      border-radius: 20px;
      padding: 3px 10px 3px 6px;
      gap: 5px;
    }

    .legend-color {
      width: 10px;
      height: 10px;
      border-radius: 2px;
    }

    .legend-line {
      width: 18px;
    }

    .legend-item span {
      font-size: 11px;
    }

    :global(.chart-wrapper) {
      height: 240px !important;
    }
  }
</style>
