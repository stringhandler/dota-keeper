<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { getHeroName } from "$lib/heroes.js";

  let isLoading = $state(true);
  let error = $state("");

  // Either showing options or active progress
  let options = $state([]);
  let activeChallenge = $state(null);
  let progress = $state(null);
  let rerollsUsed = $state(0);
  let isAccepting = $state(false);
  let isRerolling = $state(false);
  let isSkipping = $state(false);

  const MAX_REROLLS = 2;

  onMount(async () => {
    await loadState();
  });

  async function loadState() {
    isLoading = true;
    error = "";
    try {
      // Check if there's already an active challenge
      progress = await invoke("get_weekly_challenge_progress_cmd");
      if (progress) {
        activeChallenge = progress.challenge;
      } else {
        // Load options
        options = await invoke("get_weekly_challenge_options_cmd");
        rerollsUsed = options.length > 0 ? options[0].reroll_generation : 0;
      }
    } catch (e) {
      error = `Failed to load challenges: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  async function acceptChallenge(optionId) {
    isAccepting = true;
    error = "";
    try {
      await invoke("accept_weekly_challenge_cmd", { optionId });
      await loadState();
    } catch (e) {
      error = `Failed to accept challenge: ${e}`;
    } finally {
      isAccepting = false;
    }
  }

  async function reroll() {
    isRerolling = true;
    error = "";
    try {
      options = await invoke("reroll_weekly_challenges_cmd");
      rerollsUsed = options.length > 0 ? options[0].reroll_generation : rerollsUsed + 1;
    } catch (e) {
      error = `Failed to reroll: ${e}`;
    } finally {
      isRerolling = false;
    }
  }

  async function skipWeek() {
    if (!confirm("Skip this week's challenge? You won't be able to accept a challenge until next week.")) return;
    isSkipping = true;
    error = "";
    try {
      await invoke("skip_weekly_challenge_cmd");
      await loadState();
    } catch (e) {
      error = `Failed to skip: ${e}`;
    } finally {
      isSkipping = false;
    }
  }

  function difficultyColor(type) {
    switch (type) {
      case "easy": return "#60c040";
      case "medium": return "#f0b840";
      case "hard": return "#ff6b6b";
      default: return "#a0a0a0";
    }
  }

  function difficultyLabel(type) {
    switch (type) {
      case "easy": return "Easy";
      case "medium": return "Medium";
      case "hard": return "Hard";
      default: return type;
    }
  }

  function progressPercent(current, target) {
    if (target <= 0) return 0;
    return Math.min(100, Math.round((current / target) * 100));
  }
</script>

<div class="challenges-content">
  <div class="page-header">
    <h1>Weekly Challenges</h1>
    <p class="subtitle">Accept a challenge each week to push your limits</p>
    <a href="/challenges/history" class="history-link">View History →</a>
  </div>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if isLoading}
    <p class="loading">Loading challenges...</p>
  {:else if progress && activeChallenge}
    <!-- Active challenge progress view -->
    <div class="active-challenge">
      <div class="active-header">
        <div class="difficulty-badge" style="color: {difficultyColor(activeChallenge.challenge_type)}">
          {difficultyLabel(activeChallenge.challenge_type)}
        </div>
        <h2>{activeChallenge.challenge_description}</h2>
        {#if activeChallenge.hero_id !== null}
          <p class="hero-label">Hero: {getHeroName(activeChallenge.hero_id)}</p>
        {/if}
      </div>

      <div class="progress-section">
        <div class="progress-numbers">
          <span class="current">{progress.current_value}</span>
          <span class="separator">/</span>
          <span class="target">{progress.target}</span>
        </div>
        <div class="progress-bar-container">
          <div
            class="progress-bar"
            class:completed={progress.completed}
            style="width: {progressPercent(progress.current_value, progress.target)}%"
          ></div>
        </div>
        <p class="progress-label">{progressPercent(progress.current_value, progress.target)}% complete</p>
      </div>

      <div class="meta-row">
        <div class="meta-item">
          <span class="meta-label">Games counted</span>
          <span class="meta-value">{progress.games_counted}</span>
        </div>
        <div class="meta-item">
          <span class="meta-label">Days remaining</span>
          <span class="meta-value">{progress.days_remaining}</span>
        </div>
        <div class="meta-item">
          <span class="meta-label">Status</span>
          <span class="meta-value" class:text-green={progress.completed}>
            {progress.completed ? "Completed!" : "In Progress"}
          </span>
        </div>
      </div>
    </div>
  {:else}
    <!-- Option selection view -->
    <div class="options-header">
      <p class="options-intro">Choose one of this week's challenges:</p>
      <div class="reroll-row">
        <button
          class="reroll-btn"
          onclick={reroll}
          disabled={isRerolling || rerollsUsed >= MAX_REROLLS}
        >
          {isRerolling ? "Rerolling..." : `🎲 Reroll (${MAX_REROLLS - rerollsUsed} left)`}
        </button>
        <button
          class="skip-btn"
          onclick={skipWeek}
          disabled={isSkipping}
        >
          {isSkipping ? "Skipping..." : "Skip This Week"}
        </button>
      </div>
    </div>

    {#if options.length === 0}
      <p class="no-data">No challenges available yet. Play some matches first to generate challenges.</p>
    {:else}
      <div class="options-grid">
        {#each options as option}
          <div class="option-card">
            <div class="option-top">
              <span class="option-difficulty" style="color: {difficultyColor(option.challenge_type)}">
                {difficultyLabel(option.challenge_type)}
              </span>
              <span class="option-type">{option.metric.replace(/_/g, ' ')}</span>
            </div>
            <p class="option-description">{option.challenge_description}</p>
            {#if option.hero_id !== null}
              <p class="option-hero">Hero: {getHeroName(option.hero_id)}</p>
            {/if}
            {#if option.challenge_target_games !== null}
              <p class="option-games">Across {option.challenge_target_games} games</p>
            {/if}
            <button
              class="accept-btn"
              onclick={() => acceptChallenge(option.id)}
              disabled={isAccepting}
            >
              {isAccepting ? "Accepting..." : "Accept"}
            </button>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .challenges-content {
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
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.05);
    position: relative;
  }

  .page-header h1 {
    margin: 0 0 0.5rem 0;
    font-size: 2em;
    color: #d4af37;
    text-shadow: 0 0 20px rgba(212, 175, 55, 0.5), 2px 2px 4px rgba(0, 0, 0, 0.8);
    letter-spacing: 3px;
    text-transform: uppercase;
  }

  .subtitle {
    color: #a0a0a0;
    margin: 0;
    font-size: 0.9rem;
    letter-spacing: 1px;
  }

  .history-link {
    position: absolute;
    top: 25px;
    right: 30px;
    color: #d4af37;
    text-decoration: none;
    font-size: 0.9rem;
    transition: color 0.2s;
  }

  .history-link:hover {
    color: #e0c050;
  }

  .error {
    color: #ff6b6b;
    background-color: rgba(220, 53, 69, 0.2);
    border: 1px solid rgba(220, 53, 69, 0.4);
    border-radius: 3px;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
  }

  .loading, .no-data {
    color: #a0a0a0;
    text-align: center;
    padding: 3rem;
    font-style: italic;
  }

  /* Options view */
  .options-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .options-intro {
    color: #a0a0a0;
    margin: 0;
    font-size: 0.95rem;
  }

  .reroll-row {
    display: flex;
    gap: 0.75rem;
  }

  .reroll-btn {
    background: linear-gradient(180deg, rgba(60, 80, 100, 0.8) 0%, rgba(40, 60, 80, 0.8) 100%);
    border: 2px solid rgba(139, 92, 46, 0.6);
    color: #e0e0e0;
    padding: 0.5rem 1.25rem;
    border-radius: 3px;
    font-family: inherit;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
    font-weight: bold;
  }

  .reroll-btn:hover:not(:disabled) {
    background: linear-gradient(180deg, rgba(70, 95, 120, 0.9) 0%, rgba(50, 75, 100, 0.9) 100%);
    border-color: rgba(139, 92, 46, 0.9);
  }

  .reroll-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .skip-btn {
    background: transparent;
    border: 1px solid rgba(139, 92, 46, 0.4);
    color: #808080;
    padding: 0.5rem 1rem;
    border-radius: 3px;
    font-family: inherit;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .skip-btn:hover:not(:disabled) {
    color: #a0a0a0;
    border-color: rgba(139, 92, 46, 0.6);
  }

  .skip-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .options-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
    gap: 1.5rem;
  }

  .option-card {
    padding: 1.5rem;
    background:
      linear-gradient(135deg, rgba(25, 25, 35, 0.8) 0%, rgba(20, 20, 30, 0.9) 100%);
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    transition: border-color 0.2s;
  }

  .option-card:hover {
    border-color: rgba(212, 175, 55, 0.5);
  }

  .option-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .option-difficulty {
    font-weight: bold;
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .option-type {
    font-size: 0.8rem;
    color: #606060;
    text-transform: capitalize;
  }

  .option-description {
    color: #e0e0e0;
    font-size: 1rem;
    margin: 0;
    line-height: 1.4;
  }

  .option-hero, .option-games {
    font-size: 0.85rem;
    color: #a0a0a0;
    margin: 0;
  }

  .accept-btn {
    margin-top: auto;
    background: linear-gradient(180deg, rgba(60, 100, 60, 0.9) 0%, rgba(40, 80, 40, 0.9) 100%);
    border: 1px solid rgba(100, 200, 100, 0.4);
    color: #e0e0e0;
    padding: 0.6rem 1.25rem;
    border-radius: 3px;
    font-family: inherit;
    font-size: 0.95rem;
    font-weight: bold;
    cursor: pointer;
    transition: all 0.2s;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .accept-btn:hover:not(:disabled) {
    background: linear-gradient(180deg, rgba(70, 120, 70, 1) 0%, rgba(50, 100, 50, 1) 100%);
    border-color: rgba(100, 220, 100, 0.6);
  }

  .accept-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Active challenge view */
  .active-challenge {
    padding: 2rem;
    background:
      linear-gradient(135deg, rgba(25, 25, 35, 0.8) 0%, rgba(20, 20, 30, 0.9) 100%);
    border: 2px solid rgba(139, 92, 46, 0.5);
    border-radius: 8px;
  }

  .active-header {
    margin-bottom: 2rem;
  }

  .difficulty-badge {
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 2px;
    font-weight: bold;
    margin-bottom: 0.5rem;
  }

  .active-header h2 {
    font-size: 1.4rem;
    color: #e0e0e0;
    margin: 0 0 0.5rem 0;
  }

  .hero-label {
    color: #a0a0a0;
    font-size: 0.9rem;
    margin: 0;
  }

  .progress-section {
    margin-bottom: 2rem;
  }

  .progress-numbers {
    font-size: 2rem;
    font-weight: bold;
    margin-bottom: 0.75rem;
  }

  .progress-numbers .current {
    color: #d4af37;
  }

  .progress-numbers .separator {
    color: #606060;
    margin: 0 0.25rem;
  }

  .progress-numbers .target {
    color: #808080;
  }

  .progress-bar-container {
    height: 12px;
    background: rgba(30, 30, 40, 0.8);
    border-radius: 6px;
    border: 1px solid rgba(139, 92, 46, 0.3);
    overflow: hidden;
    margin-bottom: 0.5rem;
  }

  .progress-bar {
    height: 100%;
    background: linear-gradient(90deg, rgba(212, 175, 55, 0.8) 0%, rgba(212, 175, 55, 1) 100%);
    border-radius: 6px;
    transition: width 0.5s ease;
  }

  .progress-bar.completed {
    background: linear-gradient(90deg, rgba(96, 192, 64, 0.8) 0%, rgba(96, 192, 64, 1) 100%);
  }

  .progress-label {
    color: #a0a0a0;
    font-size: 0.85rem;
    margin: 0;
  }

  .meta-row {
    display: flex;
    gap: 2rem;
    flex-wrap: wrap;
  }

  .meta-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .meta-label {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 1px;
    color: #606060;
  }

  .meta-value {
    font-size: 1.2rem;
    font-weight: bold;
    color: #d4af37;
  }

  .text-green {
    color: #60c040 !important;
  }
</style>
