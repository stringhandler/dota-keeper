<script>
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { onMount, onDestroy } from "svelte";
  import { getHeroName } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";
  import { trackPageView } from "$lib/analytics.js";

  let isLoading = $state(true);
  let error = $state("");
  let matches = $state([]);
  let isRefreshing = $state(false);
  let selectedMatch = $state(null);
  let goalDetails = $state([]);
  let copiedMatchId = $state(null);
  let parsingMatches = $state(new Set());
  let currentSteamId = $state("");
  let unlistenMatchStateChanged = null;
  let autoRefreshTimer = null;

  // Filter
  let activeFilter = $state('all');

  // Pagination
  let currentPage = $state(1);
  let pageSize = $state(10);

  // Derived: unique heroes in matches list
  let trackedHeroes = $derived.by(() => {
    const heroMap = new Map();
    for (const m of matches) {
      if (!heroMap.has(m.hero_id)) {
        heroMap.set(m.hero_id, getHeroName(m.hero_id));
      }
    }
    return Array.from(heroMap.entries()).map(([id, name]) => ({ id, name }));
  });

  onMount(async () => {
    await loadData();

    unlistenMatchStateChanged = await listen("match-state-changed", (event) => {
      const { match_id, state } = event.payload;
      const matchIndex = matches.findIndex(m => m.match_id === match_id);
      if (matchIndex !== -1) {
        matches[matchIndex].parse_state = state;
        matches = [...matches];
      }
    });

    autoRefreshTimer = setInterval(async () => {
      await autoRefreshAndParse();
    }, 30000);

    // Track page view
    trackPageView("Matches");
  });

  onDestroy(() => {
    if (unlistenMatchStateChanged) unlistenMatchStateChanged();
    if (autoRefreshTimer) clearInterval(autoRefreshTimer);
  });

  async function autoRefreshAndParse() {
    try {
      const newMatches = await invoke("refresh_matches");
      matches = newMatches;
      const recentMatches = matches.slice(0, 10);
      for (const match of recentMatches) {
        if ((match.parse_state === "Unparsed" || match.parse_state === "Failed") &&
            !parsingMatches.has(match.match_id)) {
          parseMatch(match.match_id).catch(err => {
            console.error(`Auto-parse failed for match ${match.match_id}:`, err);
          });
        }
      }
    } catch (e) {
      console.error("Auto-refresh failed:", e);
    }
  }

  async function loadData() {
    try {
      const settings = await invoke("get_settings");
      currentSteamId = settings.steam_id || "";
      await loadMatches();
    } catch (e) {
      error = `Failed to load data: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  async function loadMatches() {
    try {
      matches = await invoke("get_matches");
    } catch (e) {
      console.error("Failed to load matches:", e);
    }
  }

  async function handleRefresh() {
    error = "";
    isRefreshing = true;
    try {
      matches = await invoke("refresh_matches");
    } catch (e) {
      const msg = String(e);
      if (msg.includes("500") || msg.includes("502") || msg.includes("503") || msg.includes("429")) {
        error = "OpenDota is temporarily unavailable — your cached matches are shown below.";
      } else if (msg.includes("Failed to fetch") || msg.includes("connection") || msg.includes("network")) {
        error = "Could not reach OpenDota. Check your internet connection.";
      } else {
        error = `Refresh failed: ${e}`;
      }
      await loadMatches();
    } finally {
      isRefreshing = false;
    }
  }

  function formatDuration(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  function formatDate(timestamp) {
    const d = new Date(timestamp * 1000);
    return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  }

  function isWin(match) {
    const isRadiant = match.player_slot < 128;
    return (isRadiant && match.radiant_win) || (!isRadiant && !match.radiant_win);
  }

  function isTurbo(gameMode) {
    return gameMode === 21 || gameMode === 23;
  }

  function isRanked(gameMode) {
    return gameMode === 20 || gameMode === 22;
  }

  function getModeTag(gameMode) {
    if (isTurbo(gameMode)) return { cls: 'mode-turbo', label: 'Turbo' };
    if (isRanked(gameMode)) return { cls: 'mode-ranked', label: 'Ranked' };
    return { cls: 'mode-other', label: getGameModeName(gameMode) };
  }

  async function showGoalDetails(match) {
    selectedMatch = match;
    try {
      goalDetails = await invoke("evaluate_goals_for_match", { matchId: match.match_id });
    } catch (e) {
      console.error("Failed to load goal details:", e);
      goalDetails = [];
    }
  }

  function closeGoalDetails() {
    selectedMatch = null;
    goalDetails = [];
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

  async function copyMatchId(matchId) {
    try {
      await navigator.clipboard.writeText(matchId.toString());
      copiedMatchId = matchId;
      setTimeout(() => { copiedMatchId = null; }, 2000);
    } catch (e) {
      console.error("Failed to copy match ID:", e);
    }
  }

  async function openInOpenDota(matchId) {
    try {
      await openUrl(`https://www.opendota.com/matches/${matchId}`);
    } catch (e) {
      console.error("Failed to open OpenDota link:", e);
    }
  }

  async function parseMatch(matchId) {
    parsingMatches = new Set([...parsingMatches, matchId]);
    try {
      await invoke("parse_match", { matchId, steamId: currentSteamId });
      await loadMatches();
    } catch (e) {
      error = `Failed to parse match: ${e}`;
    } finally {
      const newSet = new Set(parsingMatches);
      newSet.delete(matchId);
      parsingMatches = newSet;
    }
  }

  function getGameModeName(gameMode) {
    switch (gameMode) {
      case 0: return "Unknown";
      case 1: return "All Pick";
      case 2: return "Captain's Mode";
      case 3: return "Random Draft";
      case 5: return "All Random";
      case 20: return "Ranked";
      case 21: return "Turbo";
      case 22: return "Ranked";
      case 23: return "Turbo";
      default: return `Mode ${gameMode}`;
    }
  }

  // Filter logic
  function setFilter(filter) {
    activeFilter = filter;
    currentPage = 1;
  }

  function getFilteredMatches() {
    switch (activeFilter) {
      case 'wins':
        return matches.filter(m => isWin(m));
      case 'losses':
        return matches.filter(m => !isWin(m));
      case 'ranked':
        return matches.filter(m => isRanked(m.game_mode));
      case 'turbo':
        return matches.filter(m => isTurbo(m.game_mode));
      default:
        if (activeFilter.startsWith('hero-')) {
          const heroId = parseInt(activeFilter.split('-')[1]);
          return matches.filter(m => m.hero_id === heroId);
        }
        return matches;
    }
  }

  // Pagination
  $effect(() => {
    if (matches.length > 0) {
      const maxPage = Math.ceil(getFilteredMatches().length / pageSize);
      if (currentPage > maxPage) currentPage = 1;
    }
  });

  function getPaginatedMatches() {
    const filtered = getFilteredMatches();
    const startIndex = (currentPage - 1) * pageSize;
    return filtered.slice(startIndex, startIndex + pageSize);
  }

  function getTotalPages() {
    return Math.ceil(getFilteredMatches().length / pageSize);
  }

  function goToPage(page) {
    const totalPages = getTotalPages();
    if (page >= 1 && page <= totalPages) currentPage = page;
  }
</script>

<div class="matches-content">
  {#if error}
    <div class="error-banner">{error}</div>
  {/if}

  <!-- FILTER BAR -->
  <div class="match-filters">
    <button class="filter-chip" class:active={activeFilter === 'all'} onclick={() => setFilter('all')}>All</button>
    <button class="filter-chip" class:active={activeFilter === 'wins'} onclick={() => setFilter('wins')}>Wins</button>
    <button class="filter-chip" class:active={activeFilter === 'losses'} onclick={() => setFilter('losses')}>Losses</button>
    <button class="filter-chip" class:active={activeFilter === 'ranked'} onclick={() => setFilter('ranked')}>Ranked</button>
    <button class="filter-chip" class:active={activeFilter === 'turbo'} onclick={() => setFilter('turbo')}>Turbo</button>
    {#each trackedHeroes as hero}
      <button class="filter-chip" class:active={activeFilter === `hero-${hero.id}`} onclick={() => setFilter(`hero-${hero.id}`)}>
        {hero.name}
      </button>
    {/each}
    <div style="flex:1"></div>
    <button class="btn btn-primary" onclick={handleRefresh} disabled={isRefreshing}>
      ↻ {isRefreshing ? 'Refreshing...' : 'Refresh Matches'}
    </button>
  </div>

  {#if isLoading}
    <div class="loading-state">Loading matches...</div>
  {:else if matches.length === 0}
    <div class="empty-state">No matches found. Click "Refresh Matches" to fetch your recent games.</div>
  {:else}
    {@const filteredCount = getFilteredMatches().length}
    <div class="matches-table">
      <div class="table-head">
        <div class="th">Match ID</div>
        <div class="th">Date</div>
        <div class="th">Hero</div>
        <div class="th">Mode</div>
        <div class="th">Result</div>
        <div class="th">K/D/A</div>
        <div class="th">GPM</div>
        <div class="th">XPM</div>
        <div class="th">Goals</div>
      </div>

      {#each getPaginatedMatches() as match}
        <div class="match-row" onclick={() => showGoalDetails(match)}>
          <!-- Match ID + actions -->
          <div class="match-id-cell">
            <span class="match-id-text">{match.match_id}</span>
            <div class="match-id-actions">
              <a
                class="icon-btn"
                href="/matches/{match.match_id}"
                aria-label="View details"
                title="View details"
                onclick={(e) => e.stopPropagation()}
              >🔍</a>
              <button
                class="icon-btn"
                onclick={(e) => { e.stopPropagation(); copyMatchId(match.match_id); }}
                aria-label="Copy ID"
                title="Copy ID"
              >{copiedMatchId === match.match_id ? '✓' : '📋'}</button>
              <button
                class="icon-btn"
                onclick={(e) => { e.stopPropagation(); openInOpenDota(match.match_id); }}
                aria-label="OpenDota"
                title="Open in OpenDota"
              >🔗</button>
            </div>
          </div>

          <!-- Date -->
          <div class="td-text">{formatDate(match.start_time)}</div>

          <!-- Hero -->
          <div class="match-hero">
            <div class="hero-icon-wrap"><HeroIcon heroId={match.hero_id} size="small" showName={false} /></div>
            <span>{getHeroName(match.hero_id)}</span>
          </div>

          <!-- Mode -->
          <div>
            <span class="mode-tag {getModeTag(match.game_mode).cls}">{getModeTag(match.game_mode).label}</span>
          </div>

          <!-- Result -->
          <div class="{isWin(match) ? 'result-win' : 'result-loss'}">
            {isWin(match) ? 'WON' : 'LOST'}
          </div>

          <!-- K/D/A -->
          <div class="kda">{match.kills}/{match.deaths}/{match.assists}</div>

          <!-- GPM -->
          <div class="td-text">{match.gold_per_min}</div>

          <!-- XPM -->
          <div class="td-text">{match.xp_per_min}</div>

          <!-- Goals chip -->
          <div>
            {#if match.parse_state === "Parsing" || parsingMatches.has(match.match_id)}
              <span class="parsing-spinner">⏳</span>
            {:else if match.parse_state === "Unparsed"}
              <button
                class="parse-btn"
                onclick={(e) => { e.stopPropagation(); parseMatch(match.match_id); }}
                disabled={parsingMatches.has(match.match_id)}
              >Parse</button>
            {:else if match.goals_applicable > 0}
              <span class="goals-chip" class:has-goals={match.goals_achieved > 0}>
                {match.goals_achieved}/{match.goals_applicable}{match.goals_achieved > 0 ? ' ⚡' : ''}
              </span>
            {:else}
              <span class="no-goals-text">—</span>
            {/if}
          </div>
        </div>
      {/each}
    </div>

    <!-- PAGINATION -->
    <div class="pagination">
      <div class="pagination-info">
        Showing {((currentPage - 1) * pageSize) + 1}–{Math.min(currentPage * pageSize, filteredCount)} of {filteredCount}
        {#if activeFilter !== 'all'}<span class="filter-note">(filtered from {matches.length})</span>{/if}
      </div>

      <div class="pagination-controls">
        <button class="pagination-btn" onclick={() => goToPage(currentPage - 1)} disabled={currentPage === 1}>← Prev</button>

        <div class="page-numbers">
          {#each Array.from({ length: getTotalPages() }, (_, i) => i + 1) as p}
            {#if getTotalPages() <= 7 || p === 1 || p === getTotalPages() || (p >= currentPage - 1 && p <= currentPage + 1)}
              <button class="page-btn" class:active={p === currentPage} onclick={() => goToPage(p)}>{p}</button>
            {:else if p === currentPage - 2 || p === currentPage + 2}
              <span class="page-ellipsis">...</span>
            {/if}
          {/each}
        </div>

        <button class="pagination-btn" onclick={() => goToPage(currentPage + 1)} disabled={currentPage === getTotalPages()}>Next →</button>
      </div>

      <div class="page-size-selector">
        <label for="page-size">Per page:</label>
        <select id="page-size" class="form-select" bind:value={pageSize} onchange={() => { currentPage = 1; }}>
          <option value={10}>10</option>
          <option value={25}>25</option>
          <option value={50}>50</option>
          <option value={100}>100</option>
        </select>
      </div>
    </div>
  {/if}
</div>

<!-- GOAL DETAILS MODAL -->
{#if selectedMatch}
  <div class="modal-overlay" onclick={closeGoalDetails}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <div class="modal-title">
          <HeroIcon heroId={selectedMatch.hero_id} size="small" showName={false} />
          Goal Details — Match {selectedMatch.match_id}
        </div>
        <button class="modal-close" onclick={closeGoalDetails}>✕</button>
      </div>
      <div class="modal-body">
        <div class="match-summary">
          <span class="{isWin(selectedMatch) ? 'result-win' : 'result-loss'}">{isWin(selectedMatch) ? 'WON' : 'LOST'}</span>
          <span class="td-text">Duration: {formatDuration(selectedMatch.duration)}</span>
          <span class="td-text">KDA: {selectedMatch.kills}/{selectedMatch.deaths}/{selectedMatch.assists}</span>
        </div>

        {#if goalDetails.length === 0}
          <p class="no-applicable-goals">No applicable goals for this match.</p>
        {:else}
          <div class="goals-list">
            {#each goalDetails as evaluation}
              <div class="goal-eval-card" class:achieved={evaluation.achieved}>
                <div class="eval-status">{evaluation.achieved ? "✓" : "✗"}</div>
                <div class="eval-info">
                  <div class="eval-title">
                    {#if evaluation.goal.hero_id !== null}
                      <HeroIcon heroId={evaluation.goal.hero_id} size="small" showName={false} />
                      {getHeroName(evaluation.goal.hero_id)}
                    {:else}
                      Any Hero
                    {/if}
                    — {getMetricLabel(evaluation.goal.metric)}
                  </div>
                  <div class="eval-detail">
                    Target: {evaluation.goal.target_value} {getMetricUnit(evaluation.goal.metric)} by {evaluation.goal.target_time_minutes} min
                    · Actual: {evaluation.actual_value} {getMetricUnit(evaluation.goal.metric)}
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .matches-content { max-width: 1400px; margin: 0 auto; }

  /* Filter bar */
  .match-filters {
    display: flex;
    gap: 8px;
    margin-bottom: 20px;
    flex-wrap: wrap;
    align-items: center;
  }

  .empty-state {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 48px;
    text-align: center;
    color: var(--text-secondary);
    font-size: 13px;
  }

  /* Table */
  .matches-table {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
    margin-bottom: 16px;
  }

  .table-head,
  .match-row {
    display: grid;
    grid-template-columns: 160px 80px 160px 80px 70px 90px 60px 60px 90px;
    padding: 12px 20px;
    align-items: center;
    gap: 8px;
  }

  .table-head {
    border-bottom: 1px solid var(--border);
    background: var(--bg-elevated);
  }

  .th {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .match-row {
    border-bottom: 1px solid var(--border);
    cursor: pointer;
    transition: background 0.15s;
  }

  .match-row:last-child { border-bottom: none; }
  .match-row:hover { background: var(--bg-elevated); }

  /* Match ID cell */
  .match-id-cell {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .match-id-text {
    font-family: monospace;
    font-size: 11px;
    color: var(--text-muted);
  }

  .match-id-actions {
    display: flex;
    gap: 4px;
  }

  .icon-btn {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid var(--border);
    padding: 2px 5px;
    font-size: 11px;
    cursor: pointer;
    border-radius: 3px;
    transition: all 0.15s;
    line-height: 1.4;
    text-decoration: none;
    color: inherit;
  }

  .icon-btn:hover {
    border-color: var(--border-active);
    background: var(--bg-elevated);
  }

  /* Hero cell */
  .match-hero {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 500;
    overflow: hidden;
  }

  .match-hero span {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .hero-icon-wrap {
    width: 26px;
    height: 26px;
    border-radius: 4px;
    border: 1px solid var(--border);
    overflow: hidden;
    flex-shrink: 0;
  }

  /* Result */
  .result-win {
    color: var(--green);
    font-weight: 600;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 13px;
    letter-spacing: 1px;
  }

  .result-loss {
    color: var(--red);
    font-weight: 600;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 13px;
    letter-spacing: 1px;
  }

  .kda {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .td-text {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .no-goals-text {
    color: var(--text-muted);
    font-size: 13px;
  }

  .parsing-spinner {
    display: inline-block;
    animation: spin 2s linear infinite;
    font-size: 14px;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .parse-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 1px;
    text-transform: uppercase;
    padding: 4px 8px;
    border-radius: 3px;
    border: 1px solid var(--border);
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s;
  }

  .parse-btn:hover:not(:disabled) {
    border-color: var(--border-active);
    color: var(--gold);
  }

  .parse-btn:disabled { opacity: 0.4; cursor: not-allowed; }

  /* Pagination */
  .pagination {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    padding: 16px 20px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    flex-wrap: wrap;
  }

  .pagination-info {
    color: var(--text-secondary);
    font-size: 12px;
    white-space: nowrap;
  }

  .filter-note { color: var(--text-muted); margin-left: 4px; }

  .pagination-controls {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
    justify-content: center;
  }

  .pagination-btn {
    padding: 6px 14px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1px;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .pagination-btn:hover:not(:disabled) {
    border-color: var(--border-active);
    color: var(--text-primary);
  }

  .pagination-btn:disabled { opacity: 0.4; cursor: not-allowed; }

  .page-numbers { display: flex; gap: 4px; align-items: center; }

  .page-btn {
    min-width: 32px;
    height: 32px;
    padding: 0 6px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-secondary);
    font-size: 12px;
    cursor: pointer;
    transition: all 0.2s;
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
  }

  .page-btn:hover { border-color: var(--border-active); color: var(--gold); }

  .page-btn.active {
    background: rgba(240, 180, 41, 0.12);
    border-color: rgba(240, 180, 41, 0.4);
    color: var(--gold);
  }

  .page-ellipsis { color: var(--text-muted); padding: 0 4px; font-size: 12px; }

  .page-size-selector {
    display: flex;
    align-items: center;
    gap: 8px;
    white-space: nowrap;
  }

  .page-size-selector label {
    color: var(--text-secondary);
    font-size: 12px;
    font-family: 'Barlow Condensed', sans-serif;
  }

  /* Modal */
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(4px);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
    padding: 16px;
  }

  .modal-content {
    background: var(--bg-card);
    border: 1px solid var(--border-active);
    border-radius: 10px;
    max-width: 640px;
    width: 100%;
    max-height: 80vh;
    overflow-y: auto;
  }

  .modal-header {
    padding: 20px 24px 16px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .modal-title {
    font-family: 'Rajdhani', sans-serif;
    font-size: 16px;
    font-weight: 700;
    letter-spacing: 1.5px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .modal-close {
    width: 30px;
    height: 30px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    transition: all 0.2s;
  }

  .modal-close:hover { color: var(--text-primary); border-color: var(--border-active); }

  .modal-body { padding: 20px 24px; }

  .match-summary {
    display: flex;
    gap: 20px;
    align-items: center;
    margin-bottom: 16px;
    padding: 12px 16px;
    background: var(--bg-elevated);
    border-radius: 6px;
    font-size: 13px;
  }

  .no-applicable-goals {
    color: var(--text-secondary);
    font-style: italic;
    text-align: center;
    padding: 24px 0;
    font-size: 13px;
  }

  .goals-list { display: flex; flex-direction: column; gap: 8px; }

  .goal-eval-card {
    display: flex;
    gap: 12px;
    padding: 14px 16px;
    border-radius: 6px;
    border: 1px solid;
    align-items: flex-start;
  }

  .goal-eval-card.achieved {
    background: rgba(74, 222, 128, 0.06);
    border-color: rgba(74, 222, 128, 0.3);
  }

  .goal-eval-card:not(.achieved) {
    background: rgba(248, 113, 113, 0.06);
    border-color: rgba(248, 113, 113, 0.25);
  }

  .eval-status {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 14px;
    flex-shrink: 0;
  }

  .goal-eval-card.achieved .eval-status { color: var(--green); background: rgba(74, 222, 128, 0.15); }
  .goal-eval-card:not(.achieved) .eval-status { color: var(--red); background: rgba(248, 113, 113, 0.15); }

  .eval-info { flex: 1; }

  .eval-title {
    font-weight: 600;
    font-size: 13px;
    color: var(--text-primary);
    margin-bottom: 4px;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .eval-detail { font-size: 12px; color: var(--text-secondary); }
</style>
