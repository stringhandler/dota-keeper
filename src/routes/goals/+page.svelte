<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { heroes, getHeroName } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";
  import ItemIcon from "$lib/ItemIcon.svelte";
  import HeroSelect from "$lib/HeroSelect.svelte";
  import { trackPageView, trackEvent } from "$lib/analytics.js";
  import { showToast } from "$lib/toast.js";
  import { _ } from "svelte-i18n";
  import SkeletonLine from "$lib/SkeletonLine.svelte";

  let pendingDeleteId = $state(/** @type {number | null} */ (null));
  let goals = $state(/** @type {any[]} */ ([]));
  let isLoading = $state(true);
  let error = $state("");
  let isSaving = $state(false);
  let items = $state(/** @type {any[]} */ ([]));
  let showFormMobile = $state(false);

  // Form state
  let editingGoal = $state(/** @type {any} */ (null));
  let formHeroId = $state("");
  let formMetric = $state("LastHits");
  let formTargetValue = $state("");
  let formTargetTime = $state("10");
  let formItemId = $state("");
  let formItemMinutes = $state("");
  let formItemSeconds = $state("");
  let formGameMode = $state("All");
  let formFrequencyType = $state("Pct75");

  // Analysis data for contextual warnings
  let analysisData = $state(/** @type {any} */ (null));

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

  /** @param {number} heroId */
  function getHeroAverage(heroId) {
    if (!analysisData?.per_hero_stats) return null;
    const stat = analysisData.per_hero_stats.find((/** @type {any} */ s) => s.hero_id === heroId);
    return stat ? stat.average : null;
  }

  /** @param {any} goal */
  function getContextualWarning(goal) {
    if (goal.metric !== 'LastHits' || goal.hero_id === null || goal.hero_scope !== null) return null;
    const avg = getHeroAverage(goal.hero_id);
    if (avg === null) return null;
    if (avg >= goal.target_value) {
      return `Your avg ${getHeroName(goal.hero_id)} CS is ${avg.toFixed(0)} — consider raising this goal`;
    }
    return null;
  }

  const HERO_SCOPES = ["any_core", "any_carry", "any_support"];
  const HERO_GROUP_OPTIONS = [
    { value: "any_core",    label: "Any Core (pos 1/2/3)" },
    { value: "any_carry",   label: "Any Carry (pos 1)" },
    { value: "any_support", label: "Any Support (pos 4/5)" },
  ];

  /** @param {string} val */
  function parseHeroValue(val) {
    if (!val) return { hero_id: null, hero_scope: null };
    if (HERO_SCOPES.includes(val)) return { hero_id: null, hero_scope: val };
    return { hero_id: parseInt(val), hero_scope: null };
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
    formFrequencyType = "Pct75";
    showFormMobile = false;
  }

  /** @param {any} goal */
  function editGoal(goal) {
    editingGoal = goal;
    formHeroId = goal.hero_scope ?? (goal.hero_id !== null ? goal.hero_id.toString() : "");
    formMetric = goal.metric;
    formTargetValue = goal.target_value.toString();
    formTargetTime = goal.target_time_minutes.toString();
    formItemId = goal.item_id !== null ? goal.item_id.toString() : "";
    if (goal.metric === "ItemTiming" && goal.target_value) {
      formItemMinutes = Math.floor(goal.target_value / 60).toString();
      formItemSeconds = (goal.target_value % 60).toString();
    }
    formGameMode = goal.game_mode;
    formFrequencyType = goal.frequency_type ?? "Pct75";
    // Scroll to form
    document.querySelector('.create-form')?.scrollIntoView({ behavior: 'smooth' });
  }

  /** @param {SubmitEvent} event */
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
      const { hero_id, hero_scope } = parseHeroValue(formHeroId);
      if (editingGoal) {
        await invoke("save_goal", {
          goal: {
            id: editingGoal.id,
            hero_id,
            hero_scope,
            metric: formMetric,
            target_value: targetValue,
            target_time_minutes: targetTime,
            item_id: formItemId ? parseInt(formItemId) : null,
            game_mode: formGameMode,
            frequency_type: formFrequencyType,
            created_at: editingGoal.created_at,
          },
        });
        trackEvent("goal_updated", {
          metric: formMetric,
          game_mode: formGameMode,
          hero_scope,
          target_value: targetValue,
          target_time_minutes: targetTime,
          has_item: !!formItemId,
        });
        showToast($_('goals.toast_updated'));
      } else {
        await invoke("create_goal", {
          goal: {
            hero_id,
            hero_scope,
            metric: formMetric,
            target_value: targetValue,
            target_time_minutes: targetTime,
            item_id: formItemId ? parseInt(formItemId) : null,
            game_mode: formGameMode,
            frequency_type: formFrequencyType,
          },
        });
        trackEvent("goal_created", {
          metric: formMetric,
          game_mode: formGameMode,
          hero_scope,
          target_value: targetValue,
          target_time_minutes: targetTime,
          has_item: !!formItemId,
        });
        showToast($_('goals.toast_created'));
      }
      resetForm();
      await loadGoals();
    } catch (e) {
      error = `Failed to save goal: ${e}`;
    } finally {
      isSaving = false;
    }
  }

  /** @param {number} goalId */
  async function confirmDelete(goalId) {
    pendingDeleteId = goalId;
  }

  async function cancelDelete() {
    pendingDeleteId = null;
  }

  /** @param {number} goalId */
  async function deleteGoal(goalId) {
    pendingDeleteId = null;
    try {
      await invoke("remove_goal", { goalId });
      await loadGoals();
      showToast($_('goals.toast_deleted'));
    } catch (e) {
      error = `Failed to delete goal: ${e}`;
      showToast(`Failed to delete goal: ${e}`, 'error');
    }
  }

  /** @param {string} metric */
  function getMetricLabel(metric) {
    switch (metric) {
      case "Networth": return "Net Worth";
      case "Kills": return "Kills";
      case "Deaths": return "Deaths";
      case "LastHits": return "Last Hits";
      case "Denies": return "Denies";
      case "Level": return "Level";
      case "ItemTiming": return "Item Timing";
      case "PartnerNetworth": return "Partner Networth";
      default: return metric;
    }
  }

  /** @param {string} metric */
  function getMetricUnit(metric) {
    switch (metric) {
      case "Networth": return "gold";
      case "Kills": return "kills";
      case "Deaths": return "deaths";
      case "LastHits": return "CS";
      case "Denies": return "denies";
      case "PartnerNetworth": return "gold";
      default: return "";
    }
  }

  /** @param {number} itemId */
  function getItemName(itemId) {
    const item = items.find(i => i.id === itemId);
    return item ? item.display_name : `Item ${itemId}`;
  }

  /** @param {any} goal */
  function getHeroLabel(goal) {
    if (goal.hero_scope) {
      const g = HERO_GROUP_OPTIONS.find(o => o.value === goal.hero_scope);
      return g ? g.label : goal.hero_scope;
    }
    return goal.hero_id !== null ? getHeroName(goal.hero_id) : "Any Hero";
  }

  /** @param {any} goal */
  function formatGoalDescription(goal) {
    const heroName = getHeroLabel(goal);
    if (goal.metric === "ItemTiming") {
      const itemName = goal.item_id !== null ? getItemName(goal.item_id) : "Unknown Item";
      const minutes = Math.floor(goal.target_value / 60);
      const seconds = goal.target_value % 60;
      const timeStr = seconds > 0 ? `${minutes}:${seconds.toString().padStart(2, '0')}` : `${minutes}:00`;
      return `${heroName} — ${itemName} by ${timeStr}`;
    } else if (goal.metric === "Deaths") {
      return `${heroName} — at most ${goal.target_value} deaths by ${goal.target_time_minutes} min`;
    } else if (goal.metric === "PartnerNetworth") {
      return `${heroName} — Partner: ${goal.target_value}g by ${goal.target_time_minutes} min`;
    } else {
      const metricLabel = getMetricLabel(goal.metric);
      const unit = getMetricUnit(goal.metric);
      const valueStr = unit ? `${goal.target_value} ${unit}` : `Level ${goal.target_value}`;
      return `${heroName} — ${valueStr} by ${goal.target_time_minutes} min`;
    }
  }

  /** @param {string} freq */
  function getFrequencyLabel(freq) {
    switch (freq) {
      case "JustOnce":   return "Just once";
      case "OnAverage":  return "On average";
      case "Pct50":      return "50% of games";
      case "Pct75":      return "75% of games";
      case "Pct90":      return "90% of games";
      default:           return "75% of games";
    }
  }

  /** @param {string} metric */
  function getGoalTypeTag(metric) {
    switch (metric) {
      case "LastHits": return { tkey: 'goals.tag_cs', cls: 'tag-cs' };
      case "Denies": return { tkey: 'goals.tag_deny', cls: 'tag-cs' };
      case "ItemTiming": return { tkey: 'goals.tag_item', cls: 'tag-item' };
      case "Kills": return { tkey: 'goals.tag_kill', cls: 'tag-kill' };
      case "Deaths": return { tkey: 'goals.tag_death', cls: 'tag-kill' };
      case "Networth": return { tkey: 'goals.tag_nw', cls: 'tag-nw' };
      case "PartnerNetworth": return { tkey: 'goals.tag_support', cls: 'tag-nw' };
      default: return { tkey: null, cls: '' };
    }
  }
</script>

<div class="goals-content">
  {#if error}
    <div class="error-banner">{error}</div>
  {/if}

  <!-- NEW GOAL TOGGLE (mobile only) -->
  <div class="mobile-new-goal-row">
    <button class="btn btn-primary" onclick={() => { showFormMobile = !showFormMobile; editingGoal = null; }}>
      {showFormMobile ? $_('goals.cancel_new') : $_('goals.new_goal')}
    </button>
  </div>

  <!-- INLINE CREATE FORM -->
  <div class="create-form" class:form-hidden-mobile={!showFormMobile && !editingGoal}>
    <div class="create-form-title">
      {editingGoal ? $_('goals.edit_title') : $_('goals.create_title')}
    </div>

    <form onsubmit={handleSubmit}>
      <div class="form-row">
        <div class="fg">
          <div class="form-label">{$_('goals.hero')}</div>
          <HeroSelect bind:value={formHeroId} heroes={allHeroesSorted} favoriteIds={favoriteHeroIds} anyLabel={$_('goals.any_hero')} groupOptions={HERO_GROUP_OPTIONS} />
        </div>

        <div class="fg">
          <div class="form-label">{$_('goals.metric')}</div>
          <select class="form-select" bind:value={formMetric}>
            <option value="LastHits">{$_('goals.metric_last_hits')}</option>
            <option value="Denies">{$_('goals.metric_denies')}</option>
            <option value="PartnerNetworth">{$_('goals.metric_partner_nw')}</option>
            <option value="Networth">{$_('goals.metric_networth')}</option>
            <option value="Kills">{$_('goals.metric_kills')}</option>
            <option value="Deaths">{$_('goals.metric_deaths')}</option>
            <option value="Level">{$_('goals.metric_level')}</option>
            <option value="ItemTiming">{$_('goals.metric_item_timing')}</option>
          </select>
        </div>

        {#if formMetric === "ItemTiming"}
          <div class="fg">
            <div class="form-label">{$_('goals.item')} <span class="req">*</span></div>
            <select class="form-select" bind:value={formItemId} required>
              <option value="">{$_('goals.item_select')}</option>
              {#each items as item}
                <option value={item.id}>{item.display_name}</option>
              {/each}
            </select>
          </div>
          <div class="fg fg-narrow">
            <div class="form-label">{$_('goals.minutes')} <span class="req">*</span></div>
            <input class="form-input" type="number" min="0" max="60" placeholder="9" bind:value={formItemMinutes} />
          </div>
          <div class="fg fg-narrow">
            <div class="form-label">{$_('goals.seconds')}</div>
            <input class="form-input" type="number" min="0" max="59" placeholder="30" bind:value={formItemSeconds} />
          </div>
        {:else}
          <div class="fg">
            <div class="form-label">{$_('goals.target', { values: { metric: getMetricLabel(formMetric) } })}</div>
            <input class="form-input" type="number" min="1"
              placeholder={formMetric === "Level" ? "e.g. 6" : formMetric === "Deaths" ? "e.g. <4" : "e.g. 50"}
              bind:value={formTargetValue} />
          </div>
          <div class="fg fg-narrow">
            <div class="form-label">{$_('goals.by_min')}</div>
            <input class="form-input" type="number" min="1" max="120" placeholder="10" bind:value={formTargetTime} />
          </div>
        {/if}

        <div class="fg fg-narrow">
          <div class="form-label">{$_('goals.mode')}</div>
          <select class="form-select" bind:value={formGameMode}>
            <option value="All">{$_('goals.mode_any')}</option>
            <option value="Ranked">{$_('goals.mode_ranked')}</option>
            <option value="Turbo">{$_('goals.mode_turbo')}</option>
          </select>
        </div>

        <div class="fg">
          <div class="form-label">How often?</div>
          <select class="form-select" bind:value={formFrequencyType}>
            <option value="JustOnce">Just once</option>
            <option value="OnAverage">On average</option>
            <option value="Pct50">50% of games</option>
            <option value="Pct75">75% of games</option>
            <option value="Pct90">90% of games</option>
          </select>
        </div>

        <div class="fg fg-action">
          <div class="form-label">&nbsp;</div>
          <button type="submit" class="btn btn-primary" disabled={isSaving}>
            {isSaving ? $_('goals.saving') : editingGoal ? $_('goals.update') : $_('goals.add_goal')}
          </button>
          {#if editingGoal}
            <button type="button" class="btn btn-ghost" onclick={resetForm}>{$_('goals.cancel')}</button>
          {/if}
        </div>
      </div>
    </form>
  </div>

  <!-- GOALS LIST -->
  <div class="section-header">
    <div class="section-title">{$_('goals.active_goals', { values: { count: goals.length } })}</div>
    <!-- Archive All: future feature placeholder -->
    <button class="btn btn-ghost" title="Archive all goals (coming soon)" disabled>{$_('goals.archive_all')}</button>
  </div>

  {#if isLoading}
    <div class="skeleton-goals">
      {#each Array(3) as _}
        <div class="skeleton-goal-card">
          <div class="skeleton-goal-header">
            <SkeletonLine width="140px" height="14px" />
            <SkeletonLine width="60px" height="11px" />
          </div>
          <SkeletonLine width="100%" height="8px" />
          <SkeletonLine width="80px" height="11px" />
        </div>
      {/each}
    </div>
  {:else if goals.length === 0}
    <div class="no-goals">
      {$_('goals.empty')}
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
            {:else if goal.hero_scope === 'any_support'}
              🛡️
            {:else if goal.hero_scope === 'any_core'}
              ⚔️
            {:else if goal.hero_scope === 'any_carry'}
              🏹
            {:else}
              🌟
            {/if}
          </div>
          <div class="goal-info">
            <div class="goal-name" class:goal-name-inline={goal.metric === 'ItemTiming'}>
              {#if goal.metric === 'ItemTiming'}
                {@const heroName = getHeroLabel(goal)}
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
              <span class="goal-tag {tag.cls}">{tag.tkey ? $_(tag.tkey) : goal.metric}</span>
              <span>{goal.game_mode === 'All' ? $_('goals.mode_any') : goal.game_mode}</span>
              <span class="goal-tag tag-freq">{getFrequencyLabel(goal.frequency_type)}</span>
              {#if warning}
                <span class="warning-tag">⚠ {warning}</span>
              {/if}
            </div>
          </div>
          <div class="goal-actions" onclick={(e) => e.stopPropagation()}>
            {#if pendingDeleteId === goal.id}
              <span class="delete-confirm-label">{$_('goals.delete_confirm')}</span>
              <button class="btn btn-ghost" style="font-size:10px;padding:5px 10px;color:var(--red);border-color:rgba(248,113,113,0.4)"
                onclick={() => deleteGoal(goal.id)}>{$_('goals.delete_yes')}</button>
              <button class="btn btn-ghost" style="font-size:10px;padding:5px 10px"
                onclick={cancelDelete}>{$_('goals.delete_no')}</button>
            {:else}
              <button class="btn btn-ghost" style="font-size:10px;padding:5px 10px" onclick={() => editGoal(goal)}>
                {$_('goals.edit')}
              </button>
              <button class="btn btn-ghost" style="font-size:10px;padding:5px 10px;color:var(--red);border-color:rgba(248,113,113,0.25)"
                onclick={() => confirmDelete(goal.id)}>
                {$_('goals.delete')}
              </button>
            {/if}
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

  .fg-action > div { /* spacer label */ font-size: 12px; visibility: hidden; }

  .form-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
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
    font-size: 14px;
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

  .delete-confirm-label {
    font-size: 12px;
    color: var(--red);
    font-family: 'Barlow Condensed', sans-serif;
    letter-spacing: 0.5px;
  }

  /* Goal type tags */
  .goal-tag {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
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

  .tag-freq {
    color: var(--text-secondary);
    border-color: rgba(154, 142, 124, 0.3);
    background: rgba(154, 142, 124, 0.08);
  }

  /* Contextual warning */
  .warning-tag {
    color: var(--gold);
    font-style: italic;
    font-size: 12px;
  }

  /* Inline icon layout for item timing goals */
  .goal-name-inline {
    display: flex;
    align-items: center;
    gap: 5px;
    flex-wrap: wrap;
  }

  /* ── MOBILE NEW GOAL TOGGLE ── */
  .mobile-new-goal-row {
    display: none;
    margin-bottom: 16px;
  }

  .mobile-new-goal-row .btn {
    width: 100%;
    justify-content: center;
    padding: 12px;
  }

  @media (max-width: 640px) {
    .mobile-new-goal-row { display: block; }

    .form-hidden-mobile { display: none; }
  }

  .skeleton-goals { display: flex; flex-direction: column; gap: 8px; }

  .skeleton-goal-card {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 16px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
  }

  .skeleton-goal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
</style>
