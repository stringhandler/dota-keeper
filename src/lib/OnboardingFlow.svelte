<script>
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import { heroes } from "$lib/heroes.js";

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
        <p class="ob-sub">Pick up to {MAX_FAVORITES} heroes you play most.</p>
        <div class="hero-grid">
          {#each allHeroes as hero}
            <button
              class="hero-chip"
              class:selected={selectedHeroIds.has(hero.id)}
              class:disabled={!selectedHeroIds.has(hero.id) && selectedHeroIds.size >= MAX_FAVORITES}
              onclick={() => toggleHero(hero.id)}
            >
              {hero.name}
            </button>
          {/each}
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
  .hero-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    max-height: 280px;
    overflow-y: auto;
    margin-bottom: 12px;
    padding: 4px 0;
  }

  .hero-chip {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 5px 10px;
    font-size: 12px;
    font-family: 'Barlow', sans-serif;
    color: var(--text-secondary);
    cursor: pointer;
    transition: border-color 0.15s, color 0.15s, background 0.15s;
  }

  .hero-chip:hover:not(.disabled) {
    border-color: var(--border-active);
    color: var(--text-primary);
  }

  .hero-chip.selected {
    border-color: var(--gold);
    background: rgba(255, 200, 80, 0.12);
    color: var(--gold);
  }

  .hero-chip.disabled {
    opacity: 0.4;
    cursor: default;
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
      max-height: 220px;
    }
  }
</style>
