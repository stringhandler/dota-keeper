<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { heroes, getHeroName } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";

  let goals = $state([]);
  let isLoading = $state(true);
  let error = $state("");
  let isSaving = $state(false);
  let items = $state([]);

  // Form state
  let editingGoal = $state(null);
  let formHeroId = $state("");
  let formMetric = $state("Networth");
  let formTargetValue = $state("");
  let formTargetTime = $state("");
  let formItemId = $state("");
  let formItemMinutes = $state("");
  let formItemSeconds = $state("");
  let formGameMode = $state("Ranked");

  // Get sorted hero list for dropdown
  const heroList = Object.entries(heroes)
    .map(([id, name]) => ({ id: parseInt(id), name }))
    .sort((a, b) => a.name.localeCompare(b.name));

  onMount(async () => {
    await loadGoals();
    await loadItems();
  });

  async function loadGoals() {
    try {
      goals = await invoke("get_goals");
    } catch (e) {
      error = `Failed to load goals: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  async function loadItems() {
    try {
      items = await invoke("get_all_items");
    } catch (e) {
      console.error("Failed to load items:", e);
    }
  }

  function resetForm() {
    editingGoal = null;
    formHeroId = "";
    formMetric = "Networth";
    formTargetValue = "";
    formTargetTime = "";
    formItemId = "";
    formItemMinutes = "";
    formItemSeconds = "";
    formGameMode = "Ranked";
  }

  function editGoal(goal) {
    editingGoal = goal;
    formHeroId = goal.hero_id !== null ? goal.hero_id.toString() : "";
    formMetric = goal.metric;
    formTargetValue = goal.target_value.toString();
    formTargetTime = goal.target_time_minutes.toString();
    formItemId = goal.item_id !== null ? goal.item_id.toString() : "";

    // For item timing goals, split seconds into minutes and seconds
    if (goal.metric === "ItemTiming" && goal.target_value) {
      formItemMinutes = Math.floor(goal.target_value / 60).toString();
      formItemSeconds = (goal.target_value % 60).toString();
    }

    formGameMode = goal.game_mode;
  }

  async function handleSubmit(event) {
    event.preventDefault();
    error = "";

    // Validation for item timing goals
    if (formMetric === "ItemTiming") {
      if (!formHeroId) {
        error = "Hero selection is required for Item Timing goals";
        return;
      }
      if (!formItemId) {
        error = "Item selection is required for Item Timing goals";
        return;
      }
      if (!formItemMinutes && formItemMinutes !== "0") {
        error = "Target time (minutes) is required";
        return;
      }

      // Convert minutes and seconds to total seconds
      const minutes = parseInt(formItemMinutes) || 0;
      const seconds = parseInt(formItemSeconds) || 0;

      if (minutes < 0 || seconds < 0 || seconds >= 60) {
        error = "Invalid time values (seconds must be 0-59)";
        return;
      }

      if (minutes === 0 && seconds === 0) {
        error = "Target time must be greater than 0";
        return;
      }
    } else {
      // Normal validation for other goals
      if (!formTargetValue || !formTargetTime) {
        error = "Please fill in all required fields";
        return;
      }
    }

    // Calculate target value based on metric type
    const targetValue = formMetric === "ItemTiming"
      ? (parseInt(formItemMinutes) || 0) * 60 + (parseInt(formItemSeconds) || 0)
      : parseInt(formTargetValue);
    const targetTime = formMetric === "ItemTiming" ? 0 : parseInt(formTargetTime);

    if (isNaN(targetValue) || targetValue <= 0) {
      error = "Target value must be a positive number";
      return;
    }

    if (formMetric !== "ItemTiming" && (isNaN(targetTime) || targetTime <= 0)) {
      error = "Target time must be a positive number";
      return;
    }

    isSaving = true;

    try {
      if (editingGoal) {
        await invoke("save_goal", {
          goal: {
            id: editingGoal.id,
            hero_id: formHeroId ? parseInt(formHeroId) : null,
            metric: formMetric,
            target_value: targetValue,
            target_time_minutes: targetTime,
            item_id: formItemId ? parseInt(formItemId) : null,
            game_mode: formGameMode,
            created_at: editingGoal.created_at,
          },
        });
      } else {
        await invoke("create_goal", {
          goal: {
            hero_id: formHeroId ? parseInt(formHeroId) : null,
            metric: formMetric,
            target_value: targetValue,
            target_time_minutes: targetTime,
            item_id: formItemId ? parseInt(formItemId) : null,
            game_mode: formGameMode,
          },
        });
      }

      resetForm();
      await loadGoals();
    } catch (e) {
      error = `Failed to save goal: ${e}`;
    } finally {
      isSaving = false;
    }
  }

  async function deleteGoal(goalId) {
    if (!confirm("Are you sure you want to delete this goal?")) {
      return;
    }

    try {
      await invoke("remove_goal", { goalId });
      await loadGoals();
    } catch (e) {
      error = `Failed to delete goal: ${e}`;
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
      case "Level": return "";
      default: return "";
    }
  }

  function getItemName(itemId) {
    const item = items.find(i => i.id === itemId);
    return item ? item.display_name : `Item ${itemId}`;
  }

  function formatGoalDescription(goal) {
    const heroName = goal.hero_id !== null ? getHeroName(goal.hero_id) : "Any Hero";

    if (goal.metric === "ItemTiming") {
      const itemName = goal.item_id !== null ? getItemName(goal.item_id) : "Unknown Item";
      const minutes = Math.floor(goal.target_value / 60);
      const seconds = goal.target_value % 60;
      const timeStr = seconds > 0 ? `${minutes}:${seconds.toString().padStart(2, '0')}` : `${minutes}:00`;
      return `${heroName}: ${itemName} by ${timeStr}`;
    } else {
      const metricLabel = getMetricLabel(goal.metric);
      const unit = getMetricUnit(goal.metric);
      const valueStr = unit ? `${goal.target_value} ${unit}` : `Level ${goal.target_value}`;
      return `${heroName}: ${valueStr} by ${goal.target_time_minutes} min`;
    }
  }
</script>

<div class="goals-content">
  <div class="page-header">
    <h1>Goals</h1>
    <p class="subtitle">Set performance targets for your games</p>
  </div>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  <div class="content">
    <section class="form-section">
      <h2>{editingGoal ? "Edit Goal" : "New Goal"}</h2>
      <form class="goal-form" onsubmit={handleSubmit}>
        <div class="form-group">
          <label for="hero">Hero</label>
          <select id="hero" bind:value={formHeroId}>
            <option value="">Any Hero</option>
            {#each heroList as hero}
              <option value={hero.id}>{hero.name}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label for="metric">Metric</label>
          <select id="metric" bind:value={formMetric}>
            <option value="Networth">Net Worth</option>
            <option value="Kills">Kills</option>
            <option value="LastHits">Last Hits</option>
            <option value="Level">Level</option>
            <option value="ItemTiming">Item Timing</option>
          </select>
        </div>

        {#if formMetric === "ItemTiming"}
          <div class="form-group">
            <label for="item">Item (required) <span class="required">*</span></label>
            <select id="item" bind:value={formItemId} required>
              <option value="">Select an item...</option>
              {#each items as item}
                <option value={item.id}>{item.display_name}</option>
              {/each}
            </select>
          </div>

          <div class="form-group">
            <label>Target Time <span class="required">*</span></label>
            <div class="time-inputs">
              <div class="time-input-group">
                <input
                  type="number"
                  min="0"
                  max="60"
                  placeholder="9"
                  bind:value={formItemMinutes}
                />
                <span class="time-label">minutes</span>
              </div>
              <span class="time-separator">:</span>
              <div class="time-input-group">
                <input
                  type="number"
                  min="0"
                  max="59"
                  placeholder="30"
                  bind:value={formItemSeconds}
                />
                <span class="time-label">seconds</span>
              </div>
            </div>
            <span class="help-text">Example: 9:30 for Armlet timing</span>
          </div>
        {:else}
          <div class="form-group">
            <label for="target-value">
              Target {getMetricLabel(formMetric)}
              {#if getMetricUnit(formMetric)}
                <span class="unit">({getMetricUnit(formMetric)})</span>
              {/if}
            </label>
            <input
              id="target-value"
              type="number"
              min="1"
              placeholder={formMetric === "Level" ? "e.g., 6" : "e.g., 5000"}
              bind:value={formTargetValue}
            />
          </div>

          <div class="form-group">
            <label for="target-time">Target Time (minutes)</label>
            <input
              id="target-time"
              type="number"
              min="1"
              max="120"
              placeholder="e.g., 10"
              bind:value={formTargetTime}
            />
          </div>
        {/if}

        <div class="form-group">
          <label for="game-mode">Game Mode</label>
          <select id="game-mode" bind:value={formGameMode}>
            <option value="Ranked">Ranked</option>
            <option value="Turbo">Turbo</option>
          </select>
        </div>

        <div class="form-actions">
          <button type="submit" class="save-btn" disabled={isSaving}>
            {isSaving ? "Saving..." : editingGoal ? "Update Goal" : "Create Goal"}
          </button>
          {#if editingGoal}
            <button type="button" class="cancel-btn" onclick={resetForm}>
              Cancel
            </button>
          {/if}
        </div>
      </form>
    </section>

    <section class="goals-section">
      <h2>Your Goals</h2>
      {#if isLoading}
        <p class="loading-text">Loading goals...</p>
      {:else if goals.length === 0}
        <p class="no-goals">No goals yet. Create your first goal above.</p>
      {:else}
        <div class="goals-list">
          {#each goals as goal}
            <div class="goal-card">
              <div class="goal-info">
                <div class="goal-description">
                  {#if goal.hero_id !== null}
                    <HeroIcon heroId={goal.hero_id} size="small" showName={false} />
                  {/if}
                  {formatGoalDescription(goal)}
                </div>
                <div class="goal-mode">{goal.game_mode}</div>
              </div>
              <div class="goal-actions">
                <a href="/goals/{goal.id}" class="view-btn">View Details</a>
                <button class="edit-btn" onclick={() => editGoal(goal)}>Edit</button>
                <button class="delete-btn" onclick={() => deleteGoal(goal.id)}>Delete</button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </section>
  </div>
</div>

<style>
  .goals-content {
    max-width: 900px;
    margin: 0 auto;
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
  }

  .content {
    display: grid;
    gap: 2rem;
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

  .form-section {
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

  .goal-form {
    display: grid;
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

  .form-group .unit {
    font-weight: 400;
    color: #a0a0a0;
  }

  .form-group .required {
    color: #ff6b6b;
    font-weight: bold;
  }

  .form-group .help-text {
    font-size: 0.85rem;
    color: #a0a0a0;
    font-style: italic;
    margin-top: 0.25rem;
    display: block;
  }

  .time-inputs {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .time-input-group {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .time-input-group input {
    width: 80px;
  }

  .time-label {
    font-size: 0.75rem;
    color: #a0a0a0;
    text-align: center;
  }

  .time-separator {
    font-size: 1.5rem;
    font-weight: bold;
    color: #d4af37;
    margin-bottom: 1.5rem;
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

  .form-actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  button {
    border-radius: 3px;
    border: 2px solid rgba(139, 92, 46, 0.6);
    padding: 12px 24px;
    font-size: 1em;
    font-weight: bold;
    font-family: inherit;
    cursor: pointer;
    text-transform: uppercase;
    letter-spacing: 1px;
    transition: all 0.3s ease;
    box-shadow:
      0 4px 15px rgba(0, 0, 0, 0.6),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
  }

  .save-btn {
    background: linear-gradient(180deg, rgba(60, 80, 40, 0.8) 0%, rgba(40, 60, 30, 0.8) 100%);
    color: #e0e0e0;
  }

  .save-btn:hover {
    background: linear-gradient(180deg, rgba(70, 95, 50, 0.9) 0%, rgba(50, 75, 40, 0.9) 100%);
    border-color: rgba(139, 92, 46, 0.8);
    box-shadow:
      0 6px 20px rgba(0, 0, 0, 0.8),
      0 0 20px rgba(100, 255, 100, 0.2);
    transform: translateY(-2px);
  }

  .save-btn:disabled {
    background: linear-gradient(180deg, rgba(40, 40, 50, 0.8) 0%, rgba(30, 30, 40, 0.8) 100%);
    cursor: not-allowed;
    opacity: 0.6;
  }

  .cancel-btn {
    background: linear-gradient(180deg, rgba(60, 60, 70, 0.8) 0%, rgba(40, 40, 50, 0.8) 100%);
    color: #e0e0e0;
  }

  .cancel-btn:hover {
    background: linear-gradient(180deg, rgba(70, 70, 80, 0.9) 0%, rgba(50, 50, 60, 0.9) 100%);
    transform: translateY(-2px);
  }

  .goals-section {
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

  .loading-text,
  .no-goals {
    color: #a0a0a0;
    font-style: italic;
  }

  .goals-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .goal-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.2rem;
    background: linear-gradient(90deg, rgba(30, 30, 40, 0.6) 0%, rgba(25, 25, 35, 0.6) 100%);
    border-radius: 3px;
    border: 2px solid rgba(80, 80, 90, 0.4);
    border-left: 3px solid rgba(139, 92, 46, 0.6);
    transition: all 0.3s ease;
  }

  .goal-card:hover {
    border-color: rgba(139, 92, 46, 0.6);
    background: linear-gradient(90deg, rgba(35, 35, 45, 0.8) 0%, rgba(30, 30, 40, 0.8) 100%);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.6);
  }

  .goal-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .goal-description {
    font-weight: 600;
    color: #e0e0e0;
    font-size: 1em;
  }

  .goal-mode {
    font-size: 0.85rem;
    color: #a0a0a0;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .goal-actions {
    display: flex;
    gap: 0.5rem;
  }

  .view-btn {
    background: linear-gradient(180deg, rgba(40, 80, 100, 0.8) 0%, rgba(30, 60, 80, 0.8) 100%);
    color: #e0e0e0;
    padding: 10px 20px;
    font-size: 0.9em;
    border: 2px solid rgba(92, 139, 139, 0.6);
    border-radius: 3px;
    text-decoration: none;
    display: inline-block;
    font-weight: bold;
    font-family: inherit;
    cursor: pointer;
    text-transform: uppercase;
    letter-spacing: 1px;
    transition: all 0.3s ease;
    box-shadow:
      0 4px 15px rgba(0, 0, 0, 0.6),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
  }

  .view-btn:hover {
    background: linear-gradient(180deg, rgba(50, 95, 120, 0.9) 0%, rgba(40, 75, 100, 0.9) 100%);
    box-shadow: 0 0 15px rgba(100, 150, 200, 0.3);
    transform: translateY(-2px);
  }

  .edit-btn {
    background: linear-gradient(180deg, rgba(100, 100, 40, 0.8) 0%, rgba(80, 80, 30, 0.8) 100%);
    color: #e0e0e0;
    padding: 10px 20px;
    font-size: 0.9em;
    border: 2px solid rgba(139, 92, 46, 0.6);
  }

  .edit-btn:hover {
    background: linear-gradient(180deg, rgba(120, 120, 50, 0.9) 0%, rgba(100, 100, 40, 0.9) 100%);
    box-shadow: 0 0 15px rgba(212, 175, 55, 0.3);
    transform: translateY(-2px);
  }

  .delete-btn {
    background: linear-gradient(180deg, rgba(100, 40, 40, 0.8) 0%, rgba(80, 30, 30, 0.8) 100%);
    color: #ff6b6b;
    padding: 10px 20px;
    font-size: 0.9em;
    border: 2px solid rgba(139, 46, 46, 0.6);
  }

  .delete-btn:hover {
    background: linear-gradient(180deg, rgba(120, 50, 50, 0.9) 0%, rgba(100, 40, 40, 0.9) 100%);
    box-shadow: 0 0 15px rgba(255, 100, 100, 0.3);
    transform: translateY(-2px);
  }
</style>
