<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { heroes, getHeroName } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";
  import ItemIcon from "$lib/ItemIcon.svelte";
  import HeroSelect from "$lib/HeroSelect.svelte";
  import { trackPageView, trackEvent } from "$lib/analytics.js";

  let goals = $state([]);
  let isLoading = $state(true);
  let error = $state("");
  let isSaving = $state(false);
  let items = $state([]);

  // Form state
  let editingGoal = $state(null);
  let formHeroId = $state("");
  let formMetric = $state("LastHits");
  let formTargetValue = $state("");
  let formTargetTime = $state("10");
  let formItemId = $state("");
  let formItemMinutes = $state("");
  let formItemSeconds = $state("");
  let formGameMode = $state("Ranked");

  // Analysis data for contextual warnings
  let analysisData = $state(null);

  const allHeroesSorted = Object.entries(heroes)
    .map(([id, name]) => ({ id: parseInt(id), name }))
    .sort((a, b) => a.name.localeCompare(b.name));

  let favoriteHeroIds = $state(new Set());

  onMount(async () => {
    const favs = await invoke("get_favorite_heroes").catch(() => []);
    favoriteHeroIds = new Set(favs);
    await Promise.all([loadGoals(), loadItems(), loadAnalysisForWarnings()]);

    // Track page view
    trackPageView("Goals");
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

  async function loadAnalysisForWarnings() {
    try {
      analysisData = await invoke("get_last_hits_analysis_data", {
        timeMinutes: 10,
        windowSize: 30,
        heroId: null,
        gameMode: null,
      });
    } catch (e) {
      // non-fatal
    }
  }

  function getHeroAverage(heroId) {
    if (!analysisData?.per_hero_stats) return null;
    const stat = analysisData.per_hero_stats.find(s => s.hero_id === heroId);
    return stat ? stat.average : null;
  }

  function getContextualWarning(goal) {
    if (goal.metric !== 'LastHits' || goal.hero_id === null) return null;
    const avg = getHeroAverage(goal.hero_id);
    if (avg === null) return null;
    if (avg >= goal.target_value) {
      return `Your avg ${getHeroName(goal.hero_id)} CS is ${avg.toFixed(0)} — consider raising this goal`;
    }
    return null;
  }

  function resetForm() {
    editingGoal = null;
    formHeroId = "";
    formMetric = "LastHits";
    formTargetValue = "";
    formTargetTime = "10";
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
    if (goal.metric === "ItemTiming" && goal.target_value) {
      formItemMinutes = Math.floor(goal.target_value / 60).toString();
      formItemSeconds = (goal.target_value % 60).toString();
    }
    formGameMode = goal.game_mode;
    // Scroll to form
    document.querySelector('.create-form')?.scrollIntoView({ behavior: 'smooth' });
  }

  async function handleSubmit(event) {
    event.preventDefault();
    error = "";

    if (formMetric === "ItemTiming") {
      if (!formHeroId) { error = "Hero is required for Item Timing goals"; return; }
      if (!formItemId) { error = "Item selection is required"; return; }
      if (!formItemMinutes && formItemMinutes !== "0") { error = "Target time (minutes) is required"; return; }
      const minutes = parseInt(formItemMinutes) || 0;
      const seconds = parseInt(formItemSeconds) || 0;
      if (minutes < 0 || seconds < 0 || seconds >= 60) { error = "Invalid time (seconds must be 0–59)"; return; }
      if (minutes === 0 && seconds === 0) { error = "Target time must be > 0"; return; }
    } else {
      if (!formTargetValue || !formTargetTime) { error = "Please fill in all required fields"; return; }
    }

    const targetValue = formMetric === "ItemTiming"
      ? (parseInt(formItemMinutes) || 0) * 60 + (parseInt(formItemSeconds) || 0)
      : parseInt(formTargetValue);
    const targetTime = formMetric === "ItemTiming" ? 0 : parseInt(formTargetTime);

    if (isNaN(targetValue) || targetValue <= 0) { error = "Target value must be a positive number"; return; }
    if (formMetric !== "ItemTiming" && (isNaN(targetTime) || targetTime <= 0)) { error = "Target time must be a positive number"; return; }

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
        trackEvent("goal_updated", { metric: formMetric, game_mode: formGameMode });
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
        trackEvent("goal_created", { metric: formMetric, game_mode: formGameMode });
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
    if (!confirm("Delete this goal?")) return;
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
      case "Denies": return "Denies";
      case "Level": return "Level";
      case "ItemTiming": return "Item Timing";
      case "PartnerNetworth": return "Partner Networth";
      default: return metric;
    }
  }

  function getMetricUnit(metric) {
    switch (metric) {
      case "Networth": return "gold";
      case "Kills": return "kills";
      case "LastHits": return "CS";
      case "Denies": return "denies";
      case "PartnerNetworth": return "gold";
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
      return `${heroName} — ${itemName} by ${timeStr}`;
    } else if (goal.metric === "PartnerNetworth") {
      return `${heroName} — Partner: ${goal.target_value}g by ${goal.target_time_minutes} min`;
    } else {
      const metricLabel = getMetricLabel(goal.metric);
      const unit = getMetricUnit(goal.metric);
      const valueStr = unit ? `${goal.target_value} ${unit}` : `Level ${goal.target_value}`;
      return `${heroName} — ${valueStr} by ${goal.target_time_minutes} min`;
    }
  }

  function getGoalTypeTag(metric) {
    switch (metric) {
      case "LastHits": return { label: 'CS Goal', cls: 'tag-cs' };
      case "Denies": return { label: 'Deny Goal', cls: 'tag-cs' };
      case "ItemTiming": return { label: 'Item Goal', cls: 'tag-item' };
      case "Kills": return { label: 'Kill Goal', cls: 'tag-kill' };
      case "Networth": return { label: 'NW Goal', cls: 'tag-nw' };
      case "PartnerNetworth": return { label: 'Support Goal', cls: 'tag-nw' };
      default: return { label: `${metric} Goal`, cls: '' };
    }
  }
</script>

<div class="goals-content">
  {#if error}
    <div class="error-banner">{error}</div>
  {/if}

  <!-- INLINE CREATE FORM -->
  <div class="create-form">
    <div class="create-form-title">
      {editingGoal ? 'Edit Goal' : 'Create New Goal'}
    </div>

    <form onsubmit={handleSubmit}>
      <div class="form-row">
        <div class="fg">
          <div class="form-label">Hero</div>
          <HeroSelect bind:value={formHeroId} heroes={allHeroesSorted} favoriteIds={favoriteHeroIds} anyLabel="Any Hero" />
        </div>

        <div class="fg">
          <div class="form-label">Metric</div>
          <select class="form-select" bind:value={formMetric}>
            <option value="LastHits">Last Hits</option>
            <option value="Denies">Denies</option>
            <option value="PartnerNetworth">Partner Networth</option>
            <option value="Networth">Net Worth</option>
            <option value="Kills">Kills</option>
            <option value="Level">Level</option>
            <option value="ItemTiming">Item Timing</option>
          </select>
        </div>

        {#if formMetric === "ItemTiming"}
          <div class="fg">
            <div class="form-label">Item <span class="req">*</span></div>
            <select class="form-select" bind:value={formItemId} required>
              <option value="">Select item...</option>
              {#each items as item}
                <option value={item.id}>{item.display_name}</option>
              {/each}
            </select>
          </div>
          <div class="fg fg-narrow">
            <div class="form-label">Minutes <span class="req">*</span></div>
            <input class="form-input" type="number" min="0" max="60" placeholder="9" bind:value={formItemMinutes} />
          </div>
          <div class="fg fg-narrow">
            <div class="form-label">Seconds</div>
            <input class="form-input" type="number" min="0" max="59" placeholder="30" bind:value={formItemSeconds} />
          </div>
        {:else}
          <div class="fg">
            <div class="form-label">Target {getMetricLabel(formMetric)}</div>
            <input class="form-input" type="number" min="1"
              placeholder={formMetric === "Level" ? "e.g. 6" : "e.g. 50"}
              bind:value={formTargetValue} />
          </div>
          <div class="fg fg-narrow">
            <div class="form-label">By (min)</div>
            <input class="form-input" type="number" min="1" max="120" placeholder="10" bind:value={formTargetTime} />
          </div>
        {/if}

        <div class="fg fg-narrow">
          <div class="form-label">Mode</div>
          <select class="form-select" bind:value={formGameMode}>
            <option value="Ranked">Ranked</option>
            <option value="Turbo">Turbo</option>
          </select>
        </div>

        <div class="fg fg-action">
          <div class="form-label">&nbsp;</div>
          <button type="submit" class="btn btn-primary" disabled={isSaving}>
            {isSaving ? 'Saving...' : editingGoal ? 'Update' : 'Add Goal'}
          </button>
          {#if editingGoal}
            <button type="button" class="btn btn-ghost" onclick={resetForm}>Cancel</button>
          {/if}
        </div>
      </div>
    </form>
  </div>

  <!-- GOALS LIST -->
  <div class="section-header">
    <div class="section-title">Active Goals ({goals.length})</div>
    <!-- Archive All: future feature placeholder -->
    <button class="btn btn-ghost" title="Archive all goals (coming soon)" disabled>Archive All</button>
  </div>

  {#if isLoading}
    <div class="loading-state">Loading goals...</div>
  {:else if goals.length === 0}
    <div class="no-goals">
      No goals yet. Use the form above to create your first goal.
    </div>
  {:else}
    <div class="goals-grid">
      {#each goals as goal}
        {@const warning = getContextualWarning(goal)}
        {@const tag = getGoalTypeTag(goal.metric)}
        <div class="goal-row" onclick={() => { window.location.href = `/goals/${goal.id}`; }}>
          <div class="hero-avatar">
            {#if goal.hero_id !== null}
              <HeroIcon heroId={goal.hero_id} size="small" showName={false} />
            {:else}
              🌟
            {/if}
          </div>
          <div class="goal-info">
            <div class="goal-name" class:goal-name-inline={goal.metric === 'ItemTiming'}>
              {#if goal.metric === 'ItemTiming'}
                {@const heroName = goal.hero_id !== null ? getHeroName(goal.hero_id) : 'Any Hero'}
                {@const itemEntry = items.find(i => i.id === goal.item_id)}
                {@const minutes = Math.floor(goal.target_value / 60)}
                {@const seconds = goal.target_value % 60}
                {@const timeStr = `${minutes}:${seconds.toString().padStart(2, '0')}`}
                <span>{heroName} —</span>
                {#if itemEntry}
                  <ItemIcon itemName={itemEntry.name} displayName={itemEntry.display_name} size="small" showName={true} />
                {:else}
                  <span>Unknown Item</span>
                {/if}
                <span>by {timeStr}</span>
              {:else}
                {formatGoalDescription(goal)}
              {/if}
            </div>
            <div class="goal-progress-bar">
              <div class="goal-fill" style="width:0%"></div>
            </div>
            <div class="goal-meta">
              <span class="goal-tag {tag.cls}">{tag.label}</span>
              <span>{goal.game_mode}</span>
              {#if warning}
                <span class="warning-tag">⚠ {warning}</span>
              {/if}
            </div>
          </div>
          <div class="goal-actions" onclick={(e) => e.stopPropagation()}>
            <button class="btn btn-ghost" style="font-size:10px;padding:5px 10px" onclick={() => editGoal(goal)}>
              Edit
            </button>
            <button class="btn btn-ghost" style="font-size:10px;padding:5px 10px;color:var(--red);border-color:rgba(248,113,113,0.25)"
              onclick={() => deleteGoal(goal.id)}>
              Delete
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .goals-content {
    max-width: 1000px;
    margin: 0 auto;
  }

  /* ── CREATE FORM ── */
  .create-form {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 22px 24px;
    margin-bottom: 24px;
  }

  .create-form-title {
    font-family: 'Rajdhani', sans-serif;
    font-size: 15px;
    font-weight: 700;
    letter-spacing: 2px;
    color: var(--text-primary);
    text-transform: uppercase;
    margin-bottom: 16px;
  }

  .form-row {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
    align-items: flex-end;
  }

  .fg {
    display: flex;
    flex-direction: column;
    gap: 5px;
    flex: 1;
    min-width: 120px;
  }

  .fg-narrow { flex: 0 0 80px; min-width: 80px; }

  .fg-action {
    display: flex;
    flex-direction: column;
    gap: 5px;
    flex-shrink: 0;
  }

  /* Align buttons with the inputs */
  .fg-action .btn { align-self: flex-start; }

  .fg-action > div { /* spacer label */ font-size: 10px; visibility: hidden; }

  .form-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .req { color: var(--red); }

  /* ── NO GOALS ── */
  .no-goals {
    background: var(--bg-card);
    border: 1px dashed var(--border);
    border-radius: 8px;
    padding: 40px;
    text-align: center;
    color: var(--text-muted);
    font-size: 13px;
  }

  /* ── GOALS GRID ── */
  .goals-grid {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  /* Override the goal-row grid to have 4 columns on goals page (no dots, add actions) */
  :global(.goals-grid .goal-row) {
    grid-template-columns: 40px 1fr auto;
  }

  /* Goal actions inside card */
  .goal-actions {
    display: flex;
    gap: 6px;
    align-items: center;
    flex-shrink: 0;
  }

  /* Goal type tags */
  .goal-tag {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    letter-spacing: 1px;
    text-transform: uppercase;
    font-weight: 600;
    padding: 1px 6px;
    border-radius: 3px;
    border: 1px solid;
  }

  .tag-cs {
    color: var(--teal);
    border-color: rgba(45, 212, 191, 0.3);
    background: rgba(45, 212, 191, 0.08);
  }

  .tag-item {
    color: var(--gold);
    border-color: rgba(240, 180, 41, 0.3);
    background: rgba(240, 180, 41, 0.08);
  }

  .tag-kill {
    color: var(--red);
    border-color: rgba(248, 113, 113, 0.3);
    background: rgba(248, 113, 113, 0.08);
  }

  .tag-nw {
    color: var(--green);
    border-color: rgba(74, 222, 128, 0.3);
    background: rgba(74, 222, 128, 0.08);
  }

  /* Contextual warning */
  .warning-tag {
    color: var(--gold);
    font-style: italic;
    font-size: 10px;
  }

  /* Inline icon layout for item timing goals */
  .goal-name-inline {
    display: flex;
    align-items: center;
    gap: 5px;
    flex-wrap: wrap;
  }
</style>
