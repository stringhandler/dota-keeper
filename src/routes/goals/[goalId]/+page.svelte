<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { heroes, getHeroName } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";

  let goalId = $derived($page.params.goalId);
  let goal = $state(null);
  let matchData = $state([]);
  let items = $state([]);
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
  let editGameMode = $state("Ranked");

  // Filters
  let selectedHeroId = $state("");
  let startDate = $state("");
  let endDate = $state("");

  // Get sorted hero list for dropdown
  const allHeroesSorted = Object.entries(heroes)
    .map(([id, name]) => ({ id: parseInt(id), name }))
    .sort((a, b) => a.name.localeCompare(b.name));

  let favoriteHeroList = $derived(allHeroesSorted.filter(h => favoriteHeroIds.has(h.id)));
  let otherHeroList = $derived(allHeroesSorted.filter(h => !favoriteHeroIds.has(h.id)));

  // Filtered data
  let filteredData = $derived(() => {
    let data = matchData;

    // Filter by hero
    if (selectedHeroId) {
      const heroIdNum = parseInt(selectedHeroId);
      data = data.filter((d) => d.hero_id === heroIdNum);
    }

    // Filter by date range
    if (startDate) {
      const startTimestamp = new Date(startDate).getTime() / 1000;
      data = data.filter((d) => d.start_time >= startTimestamp);
    }

    if (endDate) {
      const endTimestamp = new Date(endDate).getTime() / 1000 + 86400; // End of day
      data = data.filter((d) => d.start_time < endTimestamp);
    }

    return data;
  });

  // Histogram calculations
  let histogram = $derived(() => {
    const data = filteredData();
    if (data.length === 0) return { bins: [], max: 0 };

    // Calculate histogram bins
    const values = data.map((d) => d.value);
    const minValue = Math.min(...values);
    const maxValue = Math.max(...values);

    // Create bins
    const binCount = Math.min(20, Math.max(5, Math.ceil(Math.sqrt(data.length))));
    const binSize = Math.max(1, Math.ceil((maxValue - minValue + 1) / binCount));

    const bins = [];
    for (let i = 0; i < binCount; i++) {
      const binStart = minValue + i * binSize;
      const binEnd = binStart + binSize;
      const matches = data.filter((d) => d.value >= binStart && (i === binCount - 1 ? d.value <= binEnd : d.value < binEnd));
      const achievedCount = matches.filter((m) => m.achieved).length;

      bins.push({
        start: binStart,
        end: binEnd,
        count: matches.length,
        achievedCount: achievedCount,
        failedCount: matches.length - achievedCount,
      });
    }

    const maxCount = Math.max(...bins.map((b) => b.count));

    return { bins, max: maxCount };
  });

  let stats = $derived(() => {
    const data = filteredData();
    if (data.length === 0)
      return {
        total: 0,
        achieved: 0,
        failed: 0,
        achievementRate: 0,
        avgValue: 0,
        minValue: 0,
        maxValue: 0,
      };

    const achieved = data.filter((d) => d.achieved).length;
    const values = data.map((d) => d.value);

    return {
      total: data.length,
      achieved,
      failed: data.length - achieved,
      achievementRate: ((achieved / data.length) * 100).toFixed(1),
      avgValue: Math.round(values.reduce((sum, v) => sum + v, 0) / values.length),
      minValue: Math.min(...values),
      maxValue: Math.max(...values),
    };
  });

  // Last 10 games sorted chronologically (oldest first, newest last)
  let last10Games = $derived(() => {
    const data = filteredData();
    return [...data]
      .sort((a, b) => a.start_time - b.start_time)
      .slice(-10);
  });

  // Achievement status indicator
  let achievementStatus = $derived(() => {
    const rate = parseFloat(stats().achievementRate);
    if (stats().total === 0) return null;
    if (rate > 85) return { label: 'Too Easy', color: '#60a5fa', desc: 'Time to raise the bar!' };
    if (rate >= 75) return { label: 'Excellent', color: '#4ade80', desc: 'Well-calibrated goal!' };
    if (rate >= 65) return { label: 'Good', color: '#fbbf24', desc: 'Close! Keep pushing.' };
    if (rate >= 50) return { label: 'Low', color: '#f97316', desc: 'Consider lowering goal' };
    return { label: 'Critical', color: '#f87171', desc: 'Goal too ambitious' };
  });

  // Goal suggestion based on last 10 games pass rate
  let goalSuggestion = $derived(() => {
    const last10 = last10Games();
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
  let scatterYRange = $derived(() => {
    const last10 = last10Games();
    if (last10.length === 0) return { min: 0, max: 100 };
    const values = last10.map(g => g.value);
    const targetVal = goal?.target_value ?? 0;
    const allVals = [...values, targetVal];
    const min = Math.min(...allVals);
    const max = Math.max(...allVals);
    const padding = Math.max((max - min) * 0.25, 5);
    return { min: Math.floor(min - padding), max: Math.ceil(max + padding) };
  });

  let bannerDismissed = $state(false);
  let hoveredDot = $state(null);

  function dismissBanner() {
    bannerDismissed = true;
    try {
      localStorage.setItem(`banner_dismissed_${goalId}`, JSON.stringify({ count: matchData.length }));
    } catch (e) { /* ignore */ }
  }

  function applyGoalSuggestion(suggestedValue) {
    startEdit();
    if (goal?.metric === 'ItemTiming') {
      editItemMinutes = Math.floor(suggestedValue / 60).toString();
      editItemSeconds = (suggestedValue % 60).toString();
    } else {
      editTargetValue = suggestedValue.toString();
    }
  }

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
        invoke("get_goal", { goalId: parseInt(goalId) }),
        invoke("get_goal_histogram_data", { goalId: parseInt(goalId) }),
        invoke("get_all_items"),
      ]);
    } catch (e) {
      error = `Failed to load goal data: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  function getItemName(itemId) {
    const item = items.find(i => i.id === itemId);
    return item ? item.display_name : `Item ${itemId}`;
  }

  function formatSeconds(totalSeconds) {
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes}:${seconds.toString().padStart(2, "0")}`;
  }

  function resetFilters() {
    selectedHeroId = "";
    startDate = "";
    endDate = "";
  }

  function getMetricLabel(metric) {
    switch (metric) {
      case "Networth":
        return "Net Worth";
      case "Kills":
        return "Kills";
      case "LastHits":
        return "Last Hits";
      case "Level":
        return "Level";
      case "ItemTiming":
        return "Item Timing";
      default:
        return metric;
    }
  }

  function getMetricUnit(metric) {
    switch (metric) {
      case "Networth":
        return "gold";
      case "Kills":
        return "kills";
      case "LastHits":
        return "CS";
      case "Level":
        return "";
      case "ItemTiming":
        return "M:SS";
      default:
        return "";
    }
  }

  function formatGoalDescription(g) {
    if (!g) return "";
    const heroName = g.hero_id !== null ? getHeroName(g.hero_id) : "Any Hero";
    const modeStr = g.game_mode ? ` (${g.game_mode})` : "";
    if (g.metric === "ItemTiming") {
      const itemName = g.item_id !== null ? getItemName(g.item_id) : "Unknown Item";
      const timeStr = formatSeconds(g.target_value);
      return `${heroName}: ${itemName} by ${timeStr}${modeStr}`;
    }
    const unit = getMetricUnit(g.metric);
    const valueStr = unit ? `${g.target_value} ${unit}` : `Level ${g.target_value}`;
    return `${heroName}: ${valueStr} by ${g.target_time_minutes} min${modeStr}`;
  }

  function formatStatValue(value, metric) {
    if (metric === "ItemTiming") return formatSeconds(value);
    return value;
  }

  function startEdit() {
    editHeroId = goal.hero_id !== null ? goal.hero_id.toString() : "";
    editMetric = goal.metric;
    editTargetValue = goal.target_value.toString();
    editTargetTime = goal.target_time_minutes.toString();
    editItemId = goal.item_id !== null ? goal.item_id.toString() : "";
    if (goal.metric === "ItemTiming") {
      editItemMinutes = Math.floor(goal.target_value / 60).toString();
      editItemSeconds = (goal.target_value % 60).toString();
    }
    editGameMode = goal.game_mode;
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
      await invoke("save_goal", {
        goal: {
          id: goal.id,
          hero_id: editHeroId ? parseInt(editHeroId) : null,
          metric: editMetric,
          target_value: targetValue,
          target_time_minutes: targetTime,
          item_id: editItemId ? parseInt(editItemId) : null,
          game_mode: editGameMode,
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
    <p>Loading goal data...</p>
  </div>
{:else if error}
  <div class="error-container">
    <p class="error">{error}</p>
    <a href="/goals" class="back-link">← Back to Goals</a>
  </div>
{:else if goal}
  <div class="goal-detail-content">
    <div class="page-header">
      <div class="header-content">
        <a href="/goals" class="back-link">← Back to Goals</a>
        <div class="header-row">
          <h1>Goal Details</h1>
          {#if !isEditing}
            <button class="edit-btn" onclick={startEdit}>✏️ Edit</button>
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
              <label>
                Hero
                <select bind:value={editHeroId} class="form-select">
                  <option value="">Any Hero</option>
                  {#if favoriteHeroList.length > 0}
                    <optgroup label="⭐ Favorites">
                      {#each favoriteHeroList as hero}
                        <option value={String(hero.id)}>{hero.name}</option>
                      {/each}
                    </optgroup>
                  {/if}
                  <optgroup label="All Heroes">
                    {#each otherHeroList as hero}
                      <option value={String(hero.id)}>{hero.name}</option>
                    {/each}
                  </optgroup>
                </select>
              </label>
              <label>
                Metric
                <select bind:value={editMetric} class="form-select">
                  <option value="LastHits">Last Hits (CS)</option>
                  <option value="Networth">Net Worth</option>
                  <option value="Kills">Kills</option>
                  <option value="Level">Level</option>
                  <option value="ItemTiming">Item Timing</option>
                </select>
              </label>
              <label>
                Game Mode
                <select bind:value={editGameMode} class="form-select">
                  <option value="Ranked">Ranked</option>
                  <option value="Turbo">Turbo</option>
                </select>
              </label>
            </div>

            {#if editMetric === "ItemTiming"}
              <div class="form-row">
                <label>
                  Item
                  <select bind:value={editItemId} class="form-select">
                    <option value="">Select item...</option>
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
                {isSaving ? "Saving..." : "Save Changes"}
              </button>
              <button class="cancel-edit-btn" onclick={cancelEdit} disabled={isSaving}>
                Cancel
              </button>
            </div>
          </div>
        {:else}
          <p class="goal-description">
            {#if goal.hero_id !== null}
              <HeroIcon heroId={goal.hero_id} size="medium" showName={false} />
            {/if}
            {formatGoalDescription(goal)}
          </p>
        {/if}
      </div>
    </div>

    <div class="content-grid">
      <!-- Filters Section -->
      <section class="filters-section">
        <h2>Filters</h2>
        <div class="filters-form">
          <div class="form-group">
            <label for="hero-filter">Hero</label>
            <select id="hero-filter" bind:value={selectedHeroId}>
              <option value="">All Heroes</option>
              {#if favoriteHeroList.length > 0}
                <optgroup label="⭐ Favorites">
                  {#each favoriteHeroList as hero}
                    <option value={hero.id}>{hero.name}</option>
                  {/each}
                </optgroup>
              {/if}
              <optgroup label="All Heroes">
                {#each otherHeroList as hero}
                  <option value={hero.id}>{hero.name}</option>
                {/each}
              </optgroup>
            </select>
          </div>

          <div class="form-group">
            <label for="start-date">Start Date</label>
            <input id="start-date" type="date" bind:value={startDate} />
          </div>

          <div class="form-group">
            <label for="end-date">End Date</label>
            <input id="end-date" type="date" bind:value={endDate} />
          </div>

          <button class="reset-btn" onclick={resetFilters}>Reset Filters</button>
        </div>
      </section>

      <!-- Statistics Section -->
      <section class="stats-section">
        <h2>Statistics</h2>
        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-label">Total Matches</div>
            <div class="stat-value">{stats().total}</div>
          </div>
          <div class="stat-card success">
            <div class="stat-label">Achieved</div>
            <div class="stat-value">{stats().achieved}</div>
          </div>
          <div class="stat-card failure">
            <div class="stat-label">Failed</div>
            <div class="stat-value">{stats().failed}</div>
          </div>
          <div class="stat-card">
            <div class="stat-label">Success Rate</div>
            <div class="stat-value">{stats().achievementRate}%</div>
          </div>
          <div class="stat-card">
            <div class="stat-label">Average</div>
            <div class="stat-value">{formatStatValue(stats().avgValue, goal.metric)} {goal.metric !== "ItemTiming" ? getMetricUnit(goal.metric) : ""}</div>
          </div>
          <div class="stat-card">
            <div class="stat-label">Range</div>
            <div class="stat-value">
              {formatStatValue(stats().minValue, goal.metric)} – {formatStatValue(stats().maxValue, goal.metric)}
            </div>
          </div>
        </div>

        {#if achievementStatus()}
          <div class="achievement-rate-card">
            <div class="achievement-rate-row">
              <div class="achievement-rate-info">
                <span class="achievement-rate-label">Achievement Rate</span>
                <span class="achievement-rate-value">{stats().achievementRate}%</span>
                <span class="achievement-rate-count">({stats().achieved}/{stats().total} games)</span>
              </div>
              <div class="achievement-target">Target: ~75%</div>
            </div>
            <div class="achievement-status" style="color: {achievementStatus().color}">
              {achievementStatus().label} — {achievementStatus().desc}
            </div>
            <div class="achievement-bar-track">
              <div class="achievement-bar-fill" style="width: {Math.min(100, parseFloat(stats().achievementRate))}%; background: {achievementStatus().color}"></div>
              <div class="achievement-bar-target"></div>
            </div>
          </div>
        {/if}
      </section>
    </div>

    <!-- Histogram Section -->
    <section class="histogram-section">
      <h2>Distribution</h2>

      <!-- Goal suggestion banner -->
      {#if goalSuggestion() && !bannerDismissed}
        {@const s = goalSuggestion()}
        <div class="suggestion-banner banner-{s.severity}">
          <div class="banner-body">
            {#if s.severity === 'critical'}
              <span class="banner-icon">⚠️</span>
              <span class="banner-text">
                Your last {last10Games().length} games averaged <strong>{goal.metric === 'ItemTiming' ? formatSeconds(s.last10Mean) : s.last10Mean}</strong>, significantly below your goal of <strong>{goal.metric === 'ItemTiming' ? formatSeconds(goal.target_value) : goal.target_value}</strong>.
                Consider lowering your goal for more consistent progress.
              </span>
            {:else if s.severity === 'warning'}
              <span class="banner-icon">⚠️</span>
              <span class="banner-text">
                You're hitting this goal only <strong>{s.passRate}%</strong> of the time in your last {last10Games().length} games.
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

      {#if filteredData().length === 0}
        <p class="no-data">No matches found with the current filters.</p>
      {:else}
        <div class="histogram-container">
          <svg class="histogram" viewBox="0 0 1000 400" preserveAspectRatio="xMidYMid meet">
            <!-- Y-axis labels and grid lines -->
            {#each [0, 0.25, 0.5, 0.75, 1] as tick}
              {@const y = 350 - tick * 300}
              {@const value = Math.round(tick * histogram().max)}
              <line
                x1="80"
                y1={y}
                x2="950"
                y2={y}
                stroke="rgba(255, 200, 80, 0.1)"
                stroke-width="1"
              />
              <text
                x="70"
                y={y + 5}
                text-anchor="end"
                fill="var(--text-muted)"
                font-size="12"
              >
                {value}
              </text>
            {/each}

            <!-- Target line -->
            {#if goal && histogram().bins.length > 0}
              {@const minVal = histogram().bins[0].start}
              {@const maxVal = histogram().bins[histogram().bins.length - 1].end}
              {@const range = maxVal - minVal}
              {@const targetX = 80 + ((goal.target_value - minVal) / range) * 870}
              <line
                x1={targetX}
                y1="50"
                x2={targetX}
                y2="350"
                stroke="var(--gold)"
                stroke-width="3"
                stroke-dasharray="5,5"
              />
              <text
                x={targetX}
                y="40"
                text-anchor="middle"
                fill="var(--gold)"
                font-size="14"
                font-weight="bold"
              >
                Target: {goal.metric === "ItemTiming" ? formatSeconds(goal.target_value) : goal.target_value}
              </text>
            {/if}

            <!-- Average line -->
            {#if goal && histogram().bins.length > 0 && stats().total > 0}
              {@const minVal = histogram().bins[0].start}
              {@const maxVal = histogram().bins[histogram().bins.length - 1].end}
              {@const range = maxVal - minVal}
              {@const avgX = range > 0 ? 80 + ((stats().avgValue - minVal) / range) * 870 : 465}
              <line
                x1={avgX}
                y1="50"
                x2={avgX}
                y2="350"
                stroke="rgba(251, 191, 36, 0.7)"
                stroke-width="2"
                stroke-dasharray="6,4"
              />
              <text
                x={avgX}
                y="45"
                text-anchor="middle"
                fill="rgba(251, 191, 36, 0.85)"
                font-size="12"
              >
                Avg: {goal.metric === "ItemTiming" ? formatSeconds(stats().avgValue) : stats().avgValue}
              </text>
            {/if}

            <!-- Histogram bars -->
            {#each histogram().bins as bin, i}
              {@const barWidth = 850 / histogram().bins.length}
              {@const x = 90 + i * barWidth}
              {@const barHeight = histogram().max > 0 ? (bin.count / histogram().max) * 300 : 0}
              {@const achievedHeight = histogram().max > 0 ? (bin.achievedCount / histogram().max) * 300 : 0}
              {@const failedHeight = histogram().max > 0 ? (bin.failedCount / histogram().max) * 300 : 0}

              <!-- Failed portion (bottom) -->
              <rect
                x={x}
                y={350 - failedHeight}
                width={barWidth - 5}
                height={failedHeight}
                fill="rgba(248, 113, 113, 0.5)"
                stroke="rgba(248, 113, 113, 0.7)"
                stroke-width="1"
              />

              <!-- Achieved portion (top) -->
              <rect
                x={x}
                y={350 - barHeight}
                width={barWidth - 5}
                height={achievedHeight}
                fill="rgba(74, 222, 128, 0.5)"
                stroke="rgba(74, 222, 128, 0.7)"
                stroke-width="1"
              />

              <!-- X-axis label -->
              <text
                x={x + barWidth / 2 - 2.5}
                y="370"
                text-anchor="middle"
                fill="var(--text-muted)"
                font-size="10"
              >
                {goal.metric === "ItemTiming" ? formatSeconds(bin.start) : bin.start}
              </text>
            {/each}

            <!-- Axis labels -->
            <text
              x="500"
              y="395"
              text-anchor="middle"
              fill="var(--text-secondary)"
              font-size="13"
              font-weight="600"
            >
              {goal.metric === "ItemTiming"
                ? `${goal.item_id !== null ? getItemName(goal.item_id) : "Item"} Timing (M:SS)`
                : `${getMetricLabel(goal.metric)} (${getMetricUnit(goal.metric) || "value"})`}
            </text>
            <text
              x="30"
              y="200"
              text-anchor="middle"
              fill="var(--text-secondary)"
              font-size="13"
              font-weight="600"
              transform="rotate(-90 30 200)"
            >
              Number of Matches
            </text>
          </svg>

          <div class="legend">
            <div class="legend-item">
              <div class="legend-color achieved"></div>
              <span>Achieved</span>
            </div>
            <div class="legend-item">
              <div class="legend-color failed"></div>
              <span>Failed</span>
            </div>
            <div class="legend-item">
              <div class="legend-line target-line"></div>
              <span>Target</span>
            </div>
            <div class="legend-item">
              <div class="legend-line avg-line"></div>
              <span>Average</span>
            </div>
          </div>

          <!-- Last N games scatter chart -->
          {#if last10Games().length > 0}
            {@const games = last10Games()}
            {@const yr = scatterYRange()}
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
                  <line x1={lm} y1={tm + chartH * (1 - (stats().avgValue - yr.min) / ySpan)} x2={lm + chartW} y2={tm + chartH * (1 - (stats().avgValue - yr.min) / ySpan)} stroke="rgba(251,191,36,0.6)" stroke-width="1.5" stroke-dasharray="5,4" />
                  <text x={lm + chartW + 4} y={tm + chartH * (1 - (stats().avgValue - yr.min) / ySpan) + 4} fill="rgba(251,191,36,0.8)" font-size="10">Avg</text>

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

  .content-grid {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: 24px;
    margin-bottom: 24px;
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

  .filters-form {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-group label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 1.5px;
    color: var(--text-muted);
    text-transform: uppercase;
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
    grid-column: 1 / -1;
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

  .histogram {
    width: 100%;
    height: auto;
    min-height: 400px;
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

  .legend-color.achieved {
    background: rgba(74, 222, 128, 0.6);
  }

  .legend-color.failed {
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

  .legend-line.avg-line {
    background: rgba(251, 191, 36, 0.7);
    background: repeating-linear-gradient(
      to right,
      rgba(251, 191, 36, 0.7) 0px,
      rgba(251, 191, 36, 0.7) 6px,
      transparent 6px,
      transparent 10px
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
    .content-grid {
      grid-template-columns: 1fr;
    }

    .stats-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>
