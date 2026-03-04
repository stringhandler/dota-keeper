<script>
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import { heroes, getHeroIconUrl } from "$lib/heroes.js";

  let { steamId = "", onComplete = () => {} } = $props();

  let step = $state(1);
  const TOTAL_STEPS = 4;

  // Step 1 — goals
  let selectedGoals = $state(new Set());
  const GOAL_CARDS = [
    { id: "goals",      icon: "🎯", label: "Track my goals",         desc: "Set targets and see if I'm hitting them" },
    { id: "analysis",   icon: "📊", label: "Analyse my performance",  desc: "Understand trends in my CS, KDA, win rate" },
    { id: "mental",     icon: "🧠", label: "Monitor my mental game",  desc: "Spot tilt patterns and avoid burnout" },
    { id: "challenges", icon: "🏆", label: "Daily/weekly challenges", desc: "Stay motivated with structured targets" },
  ];

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
      <span class="step-label">Step {step} of {TOTAL_STEPS}</span>
    </div>

    <!-- Step 1 — Welcome -->
    {#if step === 1}
      <div class="step-content">
        <h1 class="ob-title">Welcome to Dota Keeper</h1>
        <p class="ob-sub">What do you want to get out of it?</p>
        <div class="goal-cards">
          {#each GOAL_CARDS as card}
            <button
              class="goal-card"
              class:selected={selectedGoals.has(card.id)}
              onclick={() => toggleGoal(card.id)}
            >
              <span class="goal-icon">{card.icon}</span>
              <div class="goal-text">
                <div class="goal-label">{card.label}</div>
                <div class="goal-desc">{card.desc}</div>
              </div>
              {#if selectedGoals.has(card.id)}
                <span class="checkmark">✓</span>
              {/if}
            </button>
          {/each}
        </div>
        <div class="step-actions">
          <button class="btn btn-primary" onclick={next}>Continue</button>
          <button class="btn-skip" onclick={skip}>Skip all</button>
        </div>
      </div>

    <!-- Step 2 — Heroes -->
    {:else if step === 2}
      <div class="step-content">
        <h2 class="ob-title">Favourite Heroes</h2>
        <p class="ob-sub">Pick up to {MAX_FAVORITES} heroes you play most. ({selectedHeroIds.size}/{MAX_FAVORITES} selected)</p>
        <input
          class="hero-search"
          type="text"
          placeholder="Search heroes…"
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
            <p class="no-results">No heroes match "{heroSearch}"</p>
          {/if}
        </div>
        <p class="hint-text">You can always update favourites in the Analysis page.</p>
        <div class="step-actions">
          <button class="btn btn-primary" onclick={next}>Continue</button>
          <button class="btn-skip" onclick={skip}>Skip</button>
        </div>
      </div>

    <!-- Step 3 — Mental health -->
    {:else if step === 3}
      <div class="step-content">
        <h2 class="ob-title">Monitor Your Mental Game</h2>
        <p class="ob-sub">After some sessions we'll ask 2 quick questions — like how calm you felt. It takes 5 seconds and helps you spot burnout before it hits your rank.</p>
        <ul class="feature-list">
          <li>All data stays on your device</li>
          <li>You can skip any check-in</li>
          <li>Disable tracking at any time in Settings</li>
        </ul>
        <div class="step-actions">
          {#if mentalEnabled}
            <div class="enabled-badge">✓ Enabled</div>
            <button class="btn btn-primary" onclick={next}>Continue</button>
          {:else}
            <button class="btn btn-primary" onclick={enableMental}>Yes, enable it</button>
            <button class="btn-skip" onclick={skip}>Maybe later</button>
          {/if}
        </div>
      </div>

    <!-- Step 4 — Backfill -->
    {:else if step === 4}
      <div class="step-content">
        <h2 class="ob-title">Pull In Your Match History</h2>
        <p class="ob-sub">Backfilling gives Dota Keeper historical data to build better goal suggestions and trend analysis.</p>
        <div class="step-actions">
          {#if backfillDone}
            <div class="enabled-badge">✓ Matches imported</div>
            <button class="btn btn-primary" onclick={finish}>Go to Dashboard</button>
          {:else if backfilling}
            <div class="spinner-row">
              <div class="spinner"></div>
              <span>Importing matches…</span>
            </div>
            <button class="btn btn-primary" onclick={finish}>Continue in background</button>
          {:else}
            <button class="btn btn-primary" onclick={startBackfill}>Yes, backfill now</button>
            <button class="btn-skip" onclick={finish}>Skip for now</button>
          {/if}
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
    font-size: 11px;
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
    font-size: 13px;
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
    font-size: 11px;
    font-weight: 700;
  }

  .hero-card-name {
    font-family: 'Barlow', sans-serif;
    font-size: 10px;
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
    font-size: 11px;
    color: var(--gold);
    line-height: 1;
    text-shadow: 0 0 4px rgba(0,0,0,0.8);
  }

  .no-results {
    grid-column: 1 / -1;
    font-size: 13px;
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
    font-size: 13px;
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
    font-size: 13px;
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
    font-size: 13px;
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
