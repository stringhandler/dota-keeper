<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { getHeroName } from "$lib/heroes.js";
  import { _ } from "svelte-i18n";

  let isLoading = $state(true);
  let error = $state("");

  // Either showing options or active progress
  let options = $state(/** @type {any[]} */ ([]));
  let activeChallenge = $state(/** @type {any} */ (null));
  let progress = $state(/** @type {any} */ (null));
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

  /** @param {number} optionId */
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

  /** @param {string} type */
  function difficultyColor(type) {
    switch (type) {
      case "easy": return "#60c040";
      case "medium": return "#f0b840";
      case "hard": return "#ff6b6b";
      default: return "#a0a0a0";
    }
  }

  /** @param {string} type */
  function difficultyTKey(type) {
    switch (type) {
      case "easy": return 'challenges.difficulty_easy';
      case "medium": return 'challenges.difficulty_medium';
      case "hard": return 'challenges.difficulty_hard';
      default: return null;
    }
  }

  /** @param {number} current @param {number} target */
  function progressPercent(current, target) {
    if (target <= 0) return 0;
    return Math.min(100, Math.round((current / target) * 100));
  }

  /** @param {string} metric @param {number} target @param {number | null | undefined} targetGames @param {string} fallback */
  function challengeDesc(metric, target, targetGames, fallback) {
    const g = targetGames ?? 5;
    const keyMap = {
      kills: 'challenges.desc_kills',
      gpm: 'challenges.desc_gpm',
      low_deaths: 'challenges.desc_low_deaths',
      hero_damage: 'challenges.desc_hero_damage',
      cs_at_10: 'challenges.desc_cs_at_10',
      kills_total: 'challenges.desc_kills_total',
      avg_gpm: 'challenges.desc_avg_gpm',
      low_deaths_games: 'challenges.desc_low_deaths_games',
      hero_damage_total: 'challenges.desc_hero_damage_total',
      cs_at_10_avg: 'challenges.desc_cs_at_10_avg',
    };
    const key = /** @type {Record<string, string>} */ (keyMap)[metric];
    if (!key) return fallback;
    return $_( key, { values: { n: target, g } });
  }
</script>

<div class="challenges-content">
  <div class="page-header">
    <h1>{$_('challenges.title')}</h1>
    <p class="subtitle">{$_('challenges.subtitle')}</p>
    <a href="/challenges/history" class="history-link">{$_('challenges.view_history')}</a>
  </div>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if isLoading}
    <p class="loading">{$_('challenges.loading')}</p>
  {:else if progress && activeChallenge}
    <!-- Active challenge progress view -->
    <div class="active-challenge" class:challenge-completed={progress.completed}>
      {#if progress.completed}
        <div class="completed-banner">{$_('challenges.complete_banner')}</div>
      {/if}
      <div class="active-header">
        <div class="difficulty-badge" style="color: {difficultyColor(activeChallenge.challenge_type)}">
          {difficultyTKey(activeChallenge.challenge_type) ? $_(/** @type {string} */ (difficultyTKey(activeChallenge.challenge_type))) : activeChallenge.challenge_type}
        </div>
        <h2>{challengeDesc(activeChallenge.metric, activeChallenge.challenge_target, activeChallenge.challenge_target_games, activeChallenge.challenge_description)}</h2>
        {#if activeChallenge.hero_id !== null}
          <p class="hero-label">{$_('challenges.hero_label', { values: { name: getHeroName(activeChallenge.hero_id) } })}</p>
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
        <p class="progress-label">{$_('challenges.pct_complete', { values: { pct: progressPercent(progress.current_value, progress.target) } })}</p>
      </div>

      <div class="meta-row">
        {#if !progress.completed}
          <div class="meta-item">
            <span class="meta-label">{$_('challenges.games_counted')}</span>
            <span class="meta-value">{progress.games_counted}</span>
          </div>
        {/if}
        <div class="meta-item">
          <span class="meta-label">{progress.completed ? $_('challenges.resets_in') : $_('challenges.days_remaining')}</span>
          <span class="meta-value">{progress.days_remaining}d</span>
        </div>
        <div class="meta-item">
          <span class="meta-label">{$_('challenges.status')}</span>
          <span class="meta-value" class:text-green={progress.completed}>
            {progress.completed ? $_('challenges.status_completed') : $_('challenges.status_in_progress')}
          </span>
        </div>
      </div>
    </div>
  {:else}
    <!-- Option selection view -->
    <div class="options-header">
      <p class="options-intro">{$_('challenges.choose')}</p>
      <div class="reroll-row">
        <button
          class="reroll-btn"
          onclick={reroll}
          disabled={isRerolling || rerollsUsed >= MAX_REROLLS}
        >
          {isRerolling ? $_('challenges.rerolling') : $_('challenges.reroll', { values: { count: MAX_REROLLS - rerollsUsed } })}
        </button>
        <button
          class="skip-btn"
          onclick={skipWeek}
          disabled={isSkipping}
        >
          {isSkipping ? $_('challenges.skipping') : $_('challenges.skip_week')}
        </button>
      </div>
    </div>

    {#if options.length === 0}
      <p class="no-data">{$_('challenges.no_challenges')}</p>
    {:else}
      <div class="options-grid">
        {#each options as option}
          <div class="option-card">
            <div class="option-top">
              <span class="option-difficulty" style="color: {difficultyColor(option.challenge_type)}">
                {difficultyTKey(option.challenge_type) ? $_(/** @type {string} */ (difficultyTKey(option.challenge_type))) : option.challenge_type}
              </span>
              <span class="option-type">{option.metric.replace(/_/g, ' ')}</span>
            </div>
            <p class="option-description">{challengeDesc(option.metric, option.challenge_target, option.challenge_target_games, option.challenge_description)}</p>
            {#if option.hero_id !== null}
              <p class="option-hero">{$_('challenges.hero_label', { values: { name: getHeroName(option.hero_id) } })}</p>
            {/if}
            {#if option.challenge_target_games !== null}
              <p class="option-games">{$_('challenges.across_games', { values: { count: option.challenge_target_games } })}</p>
            {/if}
            <button
              class="accept-btn"
              onclick={() => acceptChallenge(option.id)}
              disabled={isAccepting}
            >
              {isAccepting ? $_('challenges.accepting') : $_('challenges.accept')}
            </button>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .challenges-content {
    max-width: 1000px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: 28px;
    position: relative;
  }

  .page-header h1 {
    font-family: 'Rajdhani', sans-serif;
    font-size: 24px;
    font-weight: 700;
    letter-spacing: 2px;
    color: var(--text-primary);
    text-transform: uppercase;
    margin: 0 0 6px 0;
  }

  .subtitle {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
    margin: 0;
  }

  .history-link {
    position: absolute;
    top: 0;
    right: 0;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-secondary);
    text-decoration: none;
    transition: color 0.2s;
  }

  .history-link:hover {
    color: var(--gold);
  }

  .error {
    color: var(--red);
    background: rgba(248, 113, 113, 0.1);
    border: 1px solid rgba(248, 113, 113, 0.25);
    border-radius: 4px;
    padding: 10px 14px;
    margin-bottom: 16px;
    font-size: 14px;
  }

  .loading, .no-data {
    color: var(--text-muted);
    text-align: center;
    padding: 48px;
    font-size: 14px;
  }

  /* Options view */
  .options-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    flex-wrap: wrap;
    gap: 16px;
  }

  .options-intro {
    color: var(--text-secondary);
    margin: 0;
    font-size: 14px;
  }

  .reroll-row {
    display: flex;
    gap: 10px;
  }

  .reroll-btn,
  .skip-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 12px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
    padding: 8px 16px;
  }

  .reroll-btn {
    background: transparent;
    color: var(--text-secondary);
    border: 1px solid var(--border);
  }

  .reroll-btn:hover:not(:disabled) {
    border-color: var(--border-active);
    color: var(--text-primary);
  }

  .reroll-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .skip-btn {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-muted);
  }

  .skip-btn:hover:not(:disabled) {
    color: var(--text-secondary);
    border-color: var(--border-active);
  }

  .skip-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .options-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 16px;
  }

  .option-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    transition: border-color 0.2s;
  }

  .option-card:hover {
    border-color: var(--border-active);
  }

  .option-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .option-difficulty {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 700;
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 2px;
  }

  .option-type {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 1.5px;
  }

  .option-description {
    font-family: 'Rajdhani', sans-serif;
    color: var(--text-primary);
    font-size: 16px;
    font-weight: 700;
    margin: 0;
    line-height: 1.4;
  }

  .option-hero, .option-games {
    font-size: 12px;
    color: var(--text-secondary);
    margin: 0;
  }

  .accept-btn {
    margin-top: auto;
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 12px;
    background: var(--gold);
    color: #080c10;
    border: none;
    padding: 10px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .accept-btn:hover:not(:disabled) {
    background: var(--gold-bright);
    transform: translateY(-1px);
  }

  .accept-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none !important;
  }

  /* Active challenge view */
  .active-challenge {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 28px;
    overflow: hidden;
  }

  .active-challenge.challenge-completed {
    border-color: rgba(74, 222, 128, 0.35);
  }

  .completed-banner {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 2px;
    text-transform: uppercase;
    color: var(--green);
    background: rgba(74, 222, 128, 0.08);
    border-bottom: 1px solid rgba(74, 222, 128, 0.2);
    padding: 10px 28px;
    margin: -28px -28px 20px -28px;
  }

  .active-header {
    margin-bottom: 24px;
  }

  .difficulty-badge {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 2px;
    font-weight: 700;
    margin-bottom: 8px;
    display: inline-block;
  }

  .active-header h2 {
    font-family: 'Rajdhani', sans-serif;
    font-size: 24px;
    font-weight: 700;
    letter-spacing: 1px;
    color: var(--text-primary);
    margin: 0 0 8px 0;
  }

  .hero-label {
    color: var(--text-secondary);
    font-size: 14px;
    margin: 0;
  }

  .progress-section {
    margin-bottom: 28px;
  }

  .progress-numbers {
    font-family: 'Rajdhani', sans-serif;
    font-size: 36px;
    font-weight: 700;
    margin-bottom: 12px;
  }

  .progress-numbers .current {
    color: var(--gold);
  }

  .progress-numbers .separator {
    color: var(--text-muted);
    margin: 0 4px;
  }

  .progress-numbers .target {
    color: var(--text-secondary);
  }

  .progress-bar-container {
    height: 8px;
    background: var(--bg-elevated);
    border-radius: 4px;
    border: 1px solid var(--border);
    overflow: hidden;
    margin-bottom: 8px;
  }

  .progress-bar {
    height: 100%;
    background: linear-gradient(90deg, var(--gold-dim), var(--gold-bright));
    border-radius: 4px;
    transition: width 0.5s ease;
  }

  .progress-bar.completed {
    background: linear-gradient(90deg, #16a34a, var(--green));
  }

  .progress-label {
    color: var(--text-muted);
    font-size: 12px;
    margin: 0;
  }

  .meta-row {
    display: flex;
    gap: 32px;
    flex-wrap: wrap;
  }

  .meta-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .meta-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 2px;
    color: var(--text-muted);
  }

  .meta-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .text-green {
    color: var(--green) !important;
  }
</style>
