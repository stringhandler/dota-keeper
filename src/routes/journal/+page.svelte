<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { getHeroName, heroes } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";
  import { trackPageView } from "$lib/analytics.js";

  /** @typedef {{ match_id: number, hero_id: number, start_time: number, metric_value: number, kills: number, deaths: number, assists: number, gold_per_min: number, xp_per_min: number, hero_damage: number, radiant_win: boolean, player_slot: number, game_mode: number }} JournalEntry */
  /** @typedef {{ top: JournalEntry[], bottom: JournalEntry[] }} PerformanceJournal */

  const METRICS = [
    { key: "kda",    label: "KDA" },
    { key: "gpm",    label: "GPM" },
    { key: "xpm",    label: "XPM" },
    { key: "damage", label: "Damage" },
    { key: "deaths", label: "Deaths" },
    { key: "lh10",   label: "LH@10" },
  ];

  let selectedMetric = $state("kda");
  let selectedHeroId = $state(/** @type {number | null} */ (null));
  let journal = $state(/** @type {PerformanceJournal | null} */ (null));
  let isLoading = $state(false);
  let error = $state("");

  // Unique heroes derived from journal entries (used to populate the hero filter)
  let knownHeroes = $state(/** @type {{ hero_id: number, name: string }[]} */ ([]));

  onMount(async () => {
    await load();
    trackPageView("Journal");
  });

  async function load() {
    isLoading = true;
    error = "";
    try {
      journal = /** @type {PerformanceJournal} */ (
        await invoke("get_performance_journal", {
          metric: selectedMetric,
          heroId: selectedHeroId,
          limit: 10,
        })
      );
      // Build hero list from all entries (top + bottom combined, de-duplicated)
      if (journal) {
        const all = [...journal.top, ...journal.bottom];
        const seen = new Set();
        knownHeroes = all
          .filter((e) => {
            if (seen.has(e.hero_id)) return false;
            seen.add(e.hero_id);
            return true;
          })
          .map((e) => ({ hero_id: e.hero_id, name: getHeroName(e.hero_id) }))
          .sort((a, b) => a.name.localeCompare(b.name));
      }
    } catch (e) {
      error = `Failed to load journal: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  /** @param {string} metric */
  function selectMetric(metric) {
    selectedMetric = metric;
    load();
  }

  function onHeroChange() {
    load();
  }

  /** @param {number} ts */
  function formatDate(ts) {
    return new Date(ts * 1000).toLocaleDateString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }

  /** @param {JournalEntry} entry */
  function isWin(entry) {
    const isRadiant = entry.player_slot < 128;
    return (isRadiant && entry.radiant_win) || (!isRadiant && !entry.radiant_win);
  }

  /** @param {number} value */
  function formatMetric(value) {
    if (selectedMetric === "kda" || selectedMetric === "deaths") {
      return value.toFixed(1);
    }
    if (selectedMetric === "damage") {
      return value >= 1000 ? (value / 1000).toFixed(1) + "k" : value.toFixed(0);
    }
    return value.toFixed(0);
  }

  // For deaths metric, flip the section labels
  let topLabel = $derived(selectedMetric === "deaths" ? "Best Games (Fewest Deaths)" : "Best Games");
  let bottomLabel = $derived(selectedMetric === "deaths" ? "Worst Games (Most Deaths)" : "Worst Games");
</script>

<div class="page">
  <header class="page-header">
    <h1 class="page-title">Performance Journal</h1>
    <p class="page-subtitle">Your best and worst performances, broken down by metric</p>
  </header>

  <!-- Metric selector -->
  <div class="controls">
    <div class="metric-buttons">
      {#each METRICS as m}
        <button
          class="metric-btn"
          class:active={selectedMetric === m.key}
          onclick={() => selectMetric(m.key)}
          type="button"
        >
          {m.label}
        </button>
      {/each}
    </div>

    <!-- Hero filter -->
    <div class="hero-filter">
      <label class="filter-label" for="hero-select">Hero</label>
      <select
        id="hero-select"
        class="hero-select"
        bind:value={selectedHeroId}
        onchange={onHeroChange}
      >
        <option value={null}>All Heroes</option>
        {#each knownHeroes as h}
          <option value={h.hero_id}>{h.name}</option>
        {/each}
      </select>
    </div>
  </div>

  {#if error}
    <div class="error-banner">{error}</div>
  {/if}

  {#if isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <span>Loading journal…</span>
    </div>
  {:else if journal}
    <div class="journal-grid">
      <!-- Best Games -->
      <section class="journal-section">
        <h2 class="section-title section-title--best">{topLabel}</h2>
        {#if journal.top.length === 0}
          <p class="empty-hint">No data yet. Play some matches!</p>
        {:else}
          <ul class="entry-list">
            {#each journal.top as entry}
              <li class="entry-card">
                <a href="/matches/{entry.match_id}" class="entry-link">
                  <div class="entry-hero">
                    <HeroIcon heroId={entry.hero_id} size="small" showName={true} />
                  </div>
                  <div class="entry-info">
                    <span class="entry-date">{formatDate(entry.start_time)}</span>
                    <span class="entry-kda">{entry.kills}/{entry.deaths}/{entry.assists}</span>
                  </div>
                  <div class="entry-right">
                    <span class="metric-value">{formatMetric(entry.metric_value)}</span>
                    <span class="wl-badge" class:win={isWin(entry)} class:loss={!isWin(entry)}>
                      {isWin(entry) ? "W" : "L"}
                    </span>
                  </div>
                </a>
              </li>
            {/each}
          </ul>
        {/if}
      </section>

      <!-- Worst Games -->
      <section class="journal-section">
        <h2 class="section-title section-title--worst">{bottomLabel}</h2>
        {#if journal.bottom.length === 0}
          <p class="empty-hint">No data yet.</p>
        {:else}
          <ul class="entry-list">
            {#each journal.bottom as entry}
              <li class="entry-card">
                <a href="/matches/{entry.match_id}" class="entry-link">
                  <div class="entry-hero">
                    <HeroIcon heroId={entry.hero_id} size="small" showName={true} />
                  </div>
                  <div class="entry-info">
                    <span class="entry-date">{formatDate(entry.start_time)}</span>
                    <span class="entry-kda">{entry.kills}/{entry.deaths}/{entry.assists}</span>
                  </div>
                  <div class="entry-right">
                    <span class="metric-value">{formatMetric(entry.metric_value)}</span>
                    <span class="wl-badge" class:win={isWin(entry)} class:loss={!isWin(entry)}>
                      {isWin(entry) ? "W" : "L"}
                    </span>
                  </div>
                </a>
              </li>
            {/each}
          </ul>
        {/if}
      </section>
    </div>
  {/if}
</div>

<style>
  .page {
    padding: 1.5rem;
    max-width: 900px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: 1.5rem;
  }

  .page-title {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 2rem;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--text-primary);
    margin: 0 0 0.25rem;
  }

  .page-subtitle {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 0.95rem;
    color: var(--text-muted);
    margin: 0;
    letter-spacing: 0.3px;
  }

  /* Controls */
  .controls {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .metric-buttons {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .metric-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 0.85rem;
    font-weight: 600;
    letter-spacing: 0.5px;
    text-transform: uppercase;
    padding: 0.4rem 0.85rem;
    border-radius: 6px;
    border: 1px solid var(--border);
    background: var(--bg-card);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s;
  }

  .metric-btn:hover {
    border-color: var(--gold);
    color: var(--gold);
  }

  .metric-btn.active {
    background: var(--gold);
    border-color: var(--gold);
    color: #000;
  }

  .hero-filter {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .filter-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.5px;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .hero-select {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 0.9rem;
    padding: 0.35rem 0.7rem;
    border-radius: 6px;
    border: 1px solid var(--border);
    background: var(--bg-card);
    color: var(--text-primary);
    cursor: pointer;
    min-width: 140px;
  }

  .hero-select:focus {
    outline: none;
    border-color: var(--gold);
  }

  /* Error / loading */
  .error-banner {
    background: rgba(220, 60, 60, 0.15);
    border: 1px solid var(--red);
    border-radius: 8px;
    padding: 0.75rem 1rem;
    color: var(--red);
    font-size: 0.9rem;
    margin-bottom: 1.25rem;
  }

  .loading-state {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 2rem;
    color: var(--text-muted);
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 1rem;
    letter-spacing: 0.5px;
  }

  .spinner {
    width: 20px;
    height: 20px;
    border: 2px solid var(--border);
    border-top-color: var(--gold);
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
    flex-shrink: 0;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* Journal grid */
  .journal-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
  }

  @media (max-width: 640px) {
    .journal-grid {
      grid-template-columns: 1fr;
    }
  }

  .journal-section {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 1rem;
  }

  .section-title {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 1rem;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
    margin: 0 0 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
  }

  .section-title--best {
    color: var(--green);
  }

  .section-title--worst {
    color: var(--red);
  }

  .empty-hint {
    color: var(--text-muted);
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 0.9rem;
    padding: 0.5rem 0;
  }

  /* Entry list */
  .entry-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .entry-card {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 8px;
    transition: border-color 0.15s;
  }

  .entry-card:hover {
    border-color: var(--gold);
  }

  .entry-link {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.6rem 0.75rem;
    text-decoration: none;
    color: inherit;
  }

  .entry-hero {
    flex-shrink: 0;
  }

  .entry-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
    min-width: 0;
  }

  .entry-date {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 0.78rem;
    color: var(--text-muted);
    letter-spacing: 0.3px;
  }

  .entry-kda {
    font-family: 'Rajdhani', sans-serif;
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .entry-right {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.25rem;
    flex-shrink: 0;
  }

  .metric-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 1.1rem;
    font-weight: 700;
    color: var(--gold);
  }

  .wl-badge {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 0.7rem;
    font-weight: 700;
    letter-spacing: 0.5px;
    padding: 0.1rem 0.4rem;
    border-radius: 4px;
  }

  .wl-badge.win {
    background: rgba(80, 200, 120, 0.2);
    color: var(--green);
    border: 1px solid var(--green);
  }

  .wl-badge.loss {
    background: rgba(220, 60, 60, 0.15);
    color: var(--red);
    border: 1px solid var(--red);
  }
</style>
