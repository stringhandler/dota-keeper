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

  onMount(async () => {
    const favs = await invoke("get_favorite_heroes").catch(() => []);
    favoriteHeroIds = new Set(favs);
    await loadGoalData();
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
    if (g.metric === "ItemTiming") {
      const itemName = g.item_id !== null ? getItemName(g.item_id) : "Unknown Item";
      const timeStr = formatSeconds(g.target_value);
      return `${heroName}: ${itemName} by ${timeStr}`;
    }
    const unit = getMetricUnit(g.metric);
    const valueStr = unit ? `${g.target_value} ${unit}` : `Level ${g.target_value}`;
    return `${heroName}: ${valueStr} by ${g.target_time_minutes} min`;
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
      </section>
    </div>

    <!-- Histogram Section -->
    <section class="histogram-section">
      <h2>Distribution</h2>
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
                stroke="rgba(139, 92, 46, 0.2)"
                stroke-width="1"
              />
              <text
                x="70"
                y={y + 5}
                text-anchor="end"
                fill="#a0a0a0"
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
                stroke="#d4af37"
                stroke-width="3"
                stroke-dasharray="5,5"
              />
              <text
                x={targetX}
                y="40"
                text-anchor="middle"
                fill="#d4af37"
                font-size="14"
                font-weight="bold"
              >
                Target: {goal.metric === "ItemTiming" ? formatSeconds(goal.target_value) : goal.target_value}
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
                fill="rgba(220, 53, 69, 0.6)"
                stroke="rgba(220, 53, 69, 0.8)"
                stroke-width="1"
              />

              <!-- Achieved portion (top) -->
              <rect
                x={x}
                y={350 - barHeight}
                width={barWidth - 5}
                height={achievedHeight}
                fill="rgba(96, 192, 64, 0.6)"
                stroke="rgba(96, 192, 64, 0.8)"
                stroke-width="1"
              />

              <!-- X-axis label -->
              <text
                x={x + barWidth / 2 - 2.5}
                y="370"
                text-anchor="middle"
                fill="#a0a0a0"
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
              fill="#d4af37"
              font-size="14"
              font-weight="bold"
            >
              {goal.metric === "ItemTiming"
                ? `${goal.item_id !== null ? getItemName(goal.item_id) : "Item"} Timing (M:SS)`
                : `${getMetricLabel(goal.metric)} (${getMetricUnit(goal.metric) || "value"})`}
            </text>
            <text
              x="30"
              y="200"
              text-anchor="middle"
              fill="#d4af37"
              font-size="14"
              font-weight="bold"
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
          </div>
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
    padding: 3rem;
    gap: 1rem;
  }

  .loading p {
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

  .back-link {
    color: #d4af37;
    text-decoration: none;
    font-size: 0.9rem;
    transition: all 0.3s ease;
  }

  .back-link:hover {
    color: #e0c050;
    text-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
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

  .goal-description {
    color: #a0a0a0;
    margin: 0;
    font-size: 0.95rem;
    letter-spacing: 1px;
  }

  .header-row {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .edit-btn {
    background: transparent;
    border: 1px solid rgba(212, 175, 55, 0.5);
    color: #d4af37;
    padding: 0.35rem 0.85rem;
    border-radius: 3px;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .edit-btn:hover {
    background: rgba(212, 175, 55, 0.15);
    border-color: rgba(212, 175, 55, 0.8);
  }

  .edit-success {
    color: #60c040;
    font-size: 0.9rem;
    margin: 0;
  }

  .edit-form {
    margin-top: 0.75rem;
    padding: 1.25rem;
    background: rgba(20, 20, 30, 0.8);
    border: 1px solid rgba(212, 175, 55, 0.4);
    border-radius: 6px;
  }

  .form-row {
    display: flex;
    gap: 1.25rem;
    flex-wrap: wrap;
    margin-bottom: 0.75rem;
  }

  .form-row label {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    color: #a0a0a0;
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .form-select,
  .form-input {
    background: rgba(30, 30, 40, 0.9);
    color: #e0e0e0;
    border: 1px solid rgba(139, 92, 46, 0.5);
    border-radius: 3px;
    padding: 0.4rem 0.6rem;
    font-family: inherit;
    font-size: 0.95rem;
  }

  .form-input.small {
    width: 64px;
    text-align: center;
  }

  .time-inputs {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    color: #a0a0a0;
  }

  .form-error {
    color: #ff6b6b;
    font-size: 0.9rem;
    margin: 0 0 0.75rem 0;
  }

  .form-actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 1rem;
  }

  .save-edit-btn {
    background: linear-gradient(180deg, rgba(60, 100, 60, 0.9) 0%, rgba(40, 80, 40, 0.9) 100%);
    border: 1px solid rgba(100, 180, 100, 0.5);
    color: #e0e0e0;
    padding: 0.5rem 1.25rem;
    border-radius: 3px;
    font-family: inherit;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .save-edit-btn:hover:not(:disabled) {
    background: linear-gradient(180deg, rgba(70, 120, 70, 1) 0%, rgba(50, 100, 50, 1) 100%);
  }

  .save-edit-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .cancel-edit-btn {
    background: transparent;
    border: 1px solid rgba(139, 92, 46, 0.5);
    color: #a0a0a0;
    padding: 0.5rem 1.25rem;
    border-radius: 3px;
    font-family: inherit;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .cancel-edit-btn:hover:not(:disabled) {
    border-color: rgba(139, 92, 46, 0.8);
    color: #e0e0e0;
  }

  .content-grid {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: 2rem;
    margin-bottom: 2rem;
  }

  section {
    background:
      linear-gradient(135deg, rgba(25, 25, 35, 0.8) 0%, rgba(20, 20, 30, 0.9) 100%),
      repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.1) 3px, rgba(0, 0, 0, 0.1) 6px),
      repeating-linear-gradient(-45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.05) 3px, rgba(0, 0, 0, 0.05) 6px);
    background-size: 100%, 6px 6px, 6px 6px;
    border: 2px solid rgba(139, 92, 46, 0.4);
    padding: 25px;
    border-radius: 3px;
    box-shadow:
      0 4px 20px rgba(0, 0, 0, 0.5),
      inset 0 1px 0 rgba(255, 255, 255, 0.03);
  }

  h2 {
    font-size: 1.2em;
    margin: 0 0 1.5rem 0;
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 2px;
    text-shadow: 0 0 10px rgba(212, 175, 55, 0.3);
    border-bottom: 2px solid rgba(139, 92, 46, 0.5);
    padding-bottom: 10px;
  }

  .filters-form {
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-weight: 600;
    font-size: 0.9rem;
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  input,
  select {
    border-radius: 3px;
    border: 2px solid rgba(139, 92, 46, 0.4);
    padding: 12px 16px;
    font-size: 1em;
    font-family: inherit;
    background-color: rgba(30, 30, 40, 0.8);
    color: #e0e0e0;
    transition: all 0.3s ease;
  }

  input:focus,
  select:focus {
    border-color: rgba(139, 92, 46, 0.8);
    outline: none;
    box-shadow: 0 0 20px rgba(212, 175, 55, 0.3);
  }

  .reset-btn {
    border-radius: 3px;
    border: 2px solid rgba(139, 92, 46, 0.6);
    padding: 12px 24px;
    font-size: 1em;
    font-weight: bold;
    font-family: inherit;
    cursor: pointer;
    background: linear-gradient(180deg, rgba(60, 60, 70, 0.8) 0%, rgba(40, 40, 50, 0.8) 100%);
    color: #e0e0e0;
    text-transform: uppercase;
    letter-spacing: 1px;
    transition: all 0.3s ease;
  }

  .reset-btn:hover {
    background: linear-gradient(180deg, rgba(70, 70, 80, 0.9) 0%, rgba(50, 50, 60, 0.9) 100%);
    border-color: rgba(139, 92, 46, 0.8);
    box-shadow: 0 0 15px rgba(212, 175, 55, 0.3);
    transform: translateY(-2px);
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
  }

  .stat-card {
    background: rgba(30, 30, 40, 0.6);
    border: 2px solid rgba(80, 80, 90, 0.4);
    border-left: 3px solid rgba(139, 92, 46, 0.6);
    padding: 1rem;
    border-radius: 3px;
    transition: all 0.3s ease;
  }

  .stat-card:hover {
    border-color: rgba(139, 92, 46, 0.6);
    background: rgba(35, 35, 45, 0.8);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.6);
  }

  .stat-card.success {
    border-left-color: rgba(96, 192, 64, 0.8);
  }

  .stat-card.failure {
    border-left-color: rgba(220, 53, 69, 0.8);
  }

  .stat-label {
    font-size: 0.85rem;
    color: #a0a0a0;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 0.5rem;
  }

  .stat-value {
    font-size: 1.5rem;
    font-weight: bold;
    color: #e0e0e0;
  }

  .histogram-section {
    grid-column: 1 / -1;
  }

  .no-data {
    color: #a0a0a0;
    font-style: italic;
    text-align: center;
    padding: 2rem 0;
  }

  .histogram-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .histogram {
    width: 100%;
    height: auto;
    min-height: 400px;
  }

  .legend {
    display: flex;
    justify-content: center;
    gap: 2rem;
    padding: 1rem;
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .legend-color {
    width: 20px;
    height: 20px;
    border-radius: 3px;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .legend-color.achieved {
    background: rgba(96, 192, 64, 0.6);
  }

  .legend-color.failed {
    background: rgba(220, 53, 69, 0.6);
  }

  .legend-item span {
    color: #e0e0e0;
    font-size: 0.9rem;
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
