<script>
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import { heroes, getHeroIconUrl } from "$lib/heroes.js";
  import { _ } from "svelte-i18n";
  import { openUrl } from "@tauri-apps/plugin-opener";

  let { steamId = "", onComplete = () => {} } = $props();

  let step = $state(1);
  const TOTAL_STEPS = 5;

  // Step 1 — goals
  let selectedGoals = $state(new Set());
  const GOAL_CARDS = [
    { id: "goals",      icon: "🎯", labelKey: "onboarding.goal_track",      descKey: "onboarding.goal_track_desc" },
    { id: "analysis",   icon: "📊", labelKey: "onboarding.goal_analyse",     descKey: "onboarding.goal_analyse_desc" },
    { id: "mental",     icon: "🧠", labelKey: "onboarding.goal_mental",      descKey: "onboarding.goal_mental_desc" },
    { id: "challenges", icon: "🏆", labelKey: "onboarding.goal_challenges",  descKey: "onboarding.goal_challenges_desc" },
  ];

  /** @param {string} id */
  function toggleGoal(id) {
    const next = new Set(selectedGoals);
    next.has(id) ? next.delete(id) : next.add(id);
    selectedGoals = next;
  }

  // Step 2 — heroes
  const MAX_FAVORITES = 5;
  const allHeroes = Object.entries(heroes)
    .map(([id, name]) => ({ id: parseInt(id), name }))
    .sort((a, b) => a.name.localeCompare(b.name));
  let selectedHeroIds = $state(new Set());
  let heroSearch = $state("");
  let filteredHeroes = $derived(
    heroSearch.trim() === ""
      ? allHeroes
      : allHeroes.filter(h => h.name.toLowerCase().includes(heroSearch.toLowerCase()))
  );

  /** @param {number} id */
  async function toggleHero(id) {
    if (selectedHeroIds.has(id)) {
      const next = new Set(selectedHeroIds);
      next.delete(id);
      selectedHeroIds = next;
      await invoke("toggle_favorite_hero", { heroId: id });
    } else if (selectedHeroIds.size < MAX_FAVORITES) {
      const next = new Set(selectedHeroIds);
      next.add(id);
      selectedHeroIds = next;
      await invoke("toggle_favorite_hero", { heroId: id });
    }
  }

  // Step 3 — mental health
  let mentalEnabled = $state(false);

  async function enableMental() {
    mentalEnabled = true;
    await invoke("save_mental_health_enabled", { enabled: true });
    await invoke("mark_mental_health_intro_shown");
  }

  // Step 4 — backfill
  let backfilling = $state(false);
  let backfillDone = $state(false);

  async function startBackfill() {
    backfilling = true;
    try {
      await invoke("backfill_historical_matches", { steamId });
      backfillDone = true;
    } catch (e) {
      console.error("Backfill failed:", e);
    } finally {
      backfilling = false;
    }
  }

  async function finish() {
    await invoke("complete_onboarding");
    onComplete();
    if (selectedGoals.has("goals")) {
      goto("/goals");
    } else {
      goto("/");
    }
  }

  function next() { step = Math.min(step + 1, TOTAL_STEPS); }
  function skip() {
    if (step === TOTAL_STEPS) { finish(); } else { step++; }
  }
</script>

<div class="onboarding-overlay">
  <div class="onboarding-card">
    <!-- Progress dots -->
    <div class="progress-dots">
      {#each Array(TOTAL_STEPS) as _, i}
        <div class="dot" class:active={i + 1 === step} class:done={i + 1 < step}></div>
      {/each}
      <span class="step-label">{$_('onboarding.step_label', { values: { step, total: TOTAL_STEPS } })}</span>
    </div>

    <!-- Step 1 — Welcome -->
    {#if step === 1}
      <div class="step-content">
        <h1 class="ob-title">{$_('onboarding.step1_title')}</h1>
        <p class="ob-sub">{$_('onboarding.step1_sub')}</p>
        <div class="goal-cards">
          {#each GOAL_CARDS as card}
            <button
              class="goal-card"
              class:selected={selectedGoals.has(card.id)}
              onclick={() => toggleGoal(card.id)}
            >
              <span class="goal-icon">{card.icon}</span>
              <div class="goal-text">
                <div class="goal-label">{$_(card.labelKey)}</div>
                <div class="goal-desc">{$_(card.descKey)}</div>
              </div>
              {#if selectedGoals.has(card.id)}
                <span class="checkmark">✓</span>
              {/if}
            </button>
          {/each}
        </div>
        <div class="step-actions">
          <button class="btn btn-primary" onclick={next}>{$_('onboarding.continue')}</button>
          <button class="btn-skip" onclick={skip}>{$_('onboarding.skip_all')}</button>
        </div>
      </div>

    <!-- Step 2 — Heroes -->
    {:else if step === 2}
      <div class="step-content">
        <h2 class="ob-title">{$_('onboarding.step2_title')}</h2>
        <p class="ob-sub">{$_('onboarding.step2_sub', { values: { max: MAX_FAVORITES, selected: selectedHeroIds.size } })}</p>
        <input
          class="hero-search"
          type="text"
          placeholder={$_('onboarding.search_heroes')}
          bind:value={heroSearch}
          autocomplete="off"
          spellcheck="false"
        />
        <div class="hero-grid">
          {#each filteredHeroes as hero (hero.id)}
            {@const iconUrl = getHeroIconUrl(hero.id)}
            {@const isSelected = selectedHeroIds.has(hero.id)}
            {@const isDisabled = !isSelected && selectedHeroIds.size >= MAX_FAVORITES}
            <button
              class="hero-card"
              class:selected={isSelected}
              class:disabled={isDisabled}
              onclick={() => toggleHero(hero.id)}
              title={hero.name}
            >
              {#if iconUrl}
                <img
                  src={iconUrl}
                  alt={hero.name}
                  class="hero-card-img"
                  loading="lazy"
                />
              {:else}
                <div class="hero-card-fallback">{hero.name.substring(0, 2).toUpperCase()}</div>
              {/if}
              <span class="hero-card-name">{hero.name}</span>
              {#if isSelected}
                <span class="hero-card-check">✓</span>
              {/if}
            </button>
          {/each}
          {#if filteredHeroes.length === 0}
            <p class="no-results">{$_('onboarding.no_heroes_match', { values: { query: heroSearch } })}</p>
          {/if}
        </div>
        <p class="hint-text">{$_('onboarding.heroes_hint')}</p>
        <div class="step-actions">
          <button class="btn btn-primary" onclick={next}>{$_('onboarding.continue')}</button>
          <button class="btn-skip" onclick={skip}>{$_('onboarding.skip')}</button>
        </div>
      </div>

    <!-- Step 3 — Mental health -->
    {:else if step === 3}
      <div class="step-content">
        <h2 class="ob-title">{$_('onboarding.step3_title')}</h2>
        <p class="ob-sub">{$_('onboarding.step3_sub')}</p>
        <ul class="feature-list">
          <li>{$_('onboarding.feature_local')}</li>
          <li>{$_('onboarding.feature_skip')}</li>
          <li>{$_('onboarding.feature_disable')}</li>
        </ul>
        <div class="step-actions">
          {#if mentalEnabled}
            <div class="enabled-badge">{$_('onboarding.enabled_badge')}</div>
            <button class="btn btn-primary" onclick={next}>{$_('onboarding.continue')}</button>
          {:else}
            <button class="btn btn-primary" onclick={enableMental}>{$_('onboarding.yes_enable')}</button>
            <button class="btn-skip" onclick={skip}>{$_('onboarding.maybe_later')}</button>
          {/if}
        </div>
      </div>

    <!-- Step 4 — Backfill -->
    {:else if step === 4}
      <div class="step-content">
        <h2 class="ob-title">{$_('onboarding.step4_title')}</h2>
        <p class="ob-sub">{$_('onboarding.step4_sub')}</p>
        <div class="step-actions">
          {#if backfillDone}
            <div class="enabled-badge">{$_('onboarding.matches_imported')}</div>
            <button class="btn btn-primary" onclick={next}>{$_('onboarding.continue')}</button>
          {:else if backfilling}
            <div class="spinner-row">
              <div class="spinner"></div>
              <span>{$_('onboarding.importing')}</span>
            </div>
            <button class="btn btn-primary" onclick={next}>{$_('onboarding.continue_bg')}</button>
          {:else}
            <button class="btn btn-primary" onclick={startBackfill}>{$_('onboarding.backfill_now')}</button>
            <button class="btn-skip" onclick={skip}>{$_('onboarding.skip_now')}</button>
          {/if}
        </div>
      </div>

    <!-- Step 5 — Community -->
    {:else if step === 5}
      <div class="step-content">
        <h2 class="ob-title">{$_('onboarding.step5_title')}</h2>
        <p class="ob-sub">{$_('onboarding.step5_sub')}</p>
        <div class="community-links">
          <button class="community-btn discord" onclick={() => openUrl('https://discord.gg/pgNrDSXV')}>
            <svg class="community-icon" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z"/>
            </svg>
            <div class="community-btn-text">
              <span class="community-btn-label">{$_('onboarding.join_discord')}</span>
              <span class="community-btn-sub">{$_('onboarding.join_discord_sub')}</span>
            </div>
          </button>
          <button class="community-btn twitter" onclick={() => openUrl('https://twitter.com/dotakeeperapp')}>
            <svg class="community-icon" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-4.714-6.231-5.401 6.231H2.744l7.737-8.835L1.254 2.25H8.08l4.259 5.632L18.244 2.25zm-1.161 17.52h1.833L7.084 4.126H5.117L17.083 19.77z"/>
            </svg>
            <div class="community-btn-text">
              <span class="community-btn-label">{$_('onboarding.follow_twitter')}</span>
              <span class="community-btn-sub">{$_('onboarding.follow_twitter_sub')}</span>
            </div>
          </button>
        </div>
        <div class="step-actions">
          <button class="btn btn-primary" onclick={finish}>{$_('onboarding.go_dashboard')}</button>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .onboarding-overlay {
    position: fixed;
    inset: 0;
    background: rgba(8, 12, 16, 0.96);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 16px;
  }

  .onboarding-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 36px 40px;
    width: 100%;
    max-width: 560px;
    max-height: 90vh;
    overflow-y: auto;
  }

  .progress-dots {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 28px;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    transition: background 0.2s;
  }

  .dot.active {
    background: var(--gold);
    border-color: var(--gold);
  }

  .dot.done {
    background: var(--gold-dim);
    border-color: var(--gold-dim);
  }

  .step-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-left: 4px;
  }

  .ob-title {
    font-family: 'Rajdhani', sans-serif;
    font-size: 28px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 8px;
  }

  .ob-sub {
    color: var(--text-secondary);
    font-size: 14px;
    line-height: 1.5;
    margin: 0 0 24px;
  }

  .goal-cards {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-bottom: 28px;
  }

  .goal-card {
    display: flex;
    align-items: center;
    gap: 14px;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 14px 16px;
    cursor: pointer;
    text-align: left;
    transition: border-color 0.15s, background 0.15s;
    color: var(--text-primary);
  }

  .goal-card:hover {
    border-color: var(--border-active);
  }

  .goal-card.selected {
    border-color: var(--gold);
    background: rgba(255, 200, 80, 0.07);
  }

  .goal-icon {
    font-size: 22px;
    flex-shrink: 0;
  }

  .goal-text {
    flex: 1;
  }

  .goal-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    font-size: 15px;
    letter-spacing: 0.5px;
  }

  .goal-desc {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .checkmark {
    color: var(--gold);
    font-size: 16px;
    flex-shrink: 0;
  }

  /* Hero grid */
  .hero-search {
    width: 100%;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 8px 12px;
    font-family: 'Barlow', sans-serif;
    font-size: 14px;
    color: var(--text-primary);
    outline: none;
    margin-bottom: 10px;
    box-sizing: border-box;
    transition: border-color 0.15s;
  }

  .hero-search:focus {
    border-color: var(--border-active);
  }

  .hero-search::placeholder {
    color: var(--text-muted);
  }

  .hero-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
    gap: 6px;
    max-height: 300px;
    overflow-y: auto;
    margin-bottom: 12px;
    padding: 2px 0;
  }

  .hero-grid::-webkit-scrollbar {
    width: 4px;
  }

  .hero-grid::-webkit-scrollbar-track {
    background: transparent;
  }

  .hero-grid::-webkit-scrollbar-thumb {
    background: rgba(255, 200, 80, 0.2);
    border-radius: 2px;
  }

  .hero-card {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 4px 4px 6px;
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s;
    overflow: hidden;
  }

  .hero-card:hover:not(.disabled) {
    border-color: var(--border-active);
    background: var(--bg-surface);
  }

  .hero-card.selected {
    border-color: var(--gold);
    background: rgba(255, 200, 80, 0.1);
  }

  .hero-card.disabled {
    opacity: 0.35;
    cursor: default;
  }

  .hero-card-img {
    width: 100%;
    aspect-ratio: 16/9;
    object-fit: cover;
    object-position: center top;
    border-radius: 2px;
    display: block;
  }

  .hero-card-fallback {
    width: 100%;
    aspect-ratio: 16/9;
    border-radius: 2px;
    background: var(--bg-card);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 12px;
    font-weight: 700;
  }

  .hero-card-name {
    font-family: 'Barlow', sans-serif;
    font-size: 12px;
    color: var(--text-secondary);
    text-align: center;
    line-height: 1.2;
    word-break: break-word;
    width: 100%;
  }

  .hero-card.selected .hero-card-name {
    color: var(--gold);
  }

  .hero-card-check {
    position: absolute;
    top: 3px;
    right: 4px;
    font-size: 12px;
    color: var(--gold);
    line-height: 1;
    text-shadow: 0 0 4px rgba(0,0,0,0.8);
  }

  .no-results {
    grid-column: 1 / -1;
    font-size: 14px;
    color: var(--text-muted);
    text-align: center;
    padding: 20px 0;
    margin: 0;
  }

  .hint-text {
    font-size: 12px;
    color: var(--text-muted);
    margin: 0 0 24px;
  }

  /* Mental health */
  .feature-list {
    color: var(--text-secondary);
    font-size: 14px;
    line-height: 1.8;
    padding-left: 20px;
    margin: 0 0 28px;
  }

  .enabled-badge {
    color: var(--green);
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 14px;
    font-weight: 600;
    letter-spacing: 1px;
    margin-bottom: 12px;
  }

  /* Actions */
  .step-actions {
    display: flex;
    flex-direction: column;
    gap: 10px;
    align-items: flex-start;
  }

  .btn-skip {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 14px;
    cursor: pointer;
    padding: 0;
    text-decoration: underline;
    text-underline-offset: 3px;
  }

  .btn-skip:hover {
    color: var(--text-secondary);
  }

  /* Spinner */
  .spinner-row {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--text-secondary);
    font-size: 14px;
    margin-bottom: 12px;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid var(--border);
    border-top-color: var(--gold);
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* Community links */
  .community-links {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-bottom: 28px;
  }

  .community-btn {
    display: flex;
    align-items: center;
    gap: 16px;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 16px 18px;
    cursor: pointer;
    text-align: left;
    transition: border-color 0.15s, background 0.15s;
    color: var(--text-primary);
    width: 100%;
  }

  .community-btn:hover {
    border-color: var(--border-active);
    background: var(--bg-elevated);
  }

  .community-btn.discord:hover {
    border-color: #5865f2;
    background: rgba(88, 101, 242, 0.08);
  }

  .community-btn.twitter:hover {
    border-color: #e7e9ea;
    background: rgba(231, 233, 234, 0.06);
  }

  .community-icon {
    width: 28px;
    height: 28px;
    flex-shrink: 0;
    color: var(--text-muted);
    transition: color 0.15s;
  }

  .community-btn.discord:hover .community-icon {
    color: #5865f2;
  }

  .community-btn.twitter:hover .community-icon {
    color: #e7e9ea;
  }

  .community-btn-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .community-btn-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    font-size: 15px;
    letter-spacing: 0.5px;
  }

  .community-btn-sub {
    font-size: 12px;
    color: var(--text-secondary);
  }

  /* Mobile */
  @media (max-width: 640px) {
    .onboarding-card {
      padding: 24px 20px;
    }

    .ob-title {
      font-size: 22px;
    }

    .hero-grid {
      max-height: 240px;
      grid-template-columns: repeat(auto-fill, minmax(70px, 1fr));
    }
  }
</style>
