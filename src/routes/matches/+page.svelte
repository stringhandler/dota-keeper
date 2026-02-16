<script>
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import { getHeroName } from "$lib/heroes.js";

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

  // Pagination
  let currentPage = $state(1);
  let pageSize = $state(10);

  onMount(async () => {
    await loadData();

    // Listen for match state changes
    unlistenMatchStateChanged = await listen("match-state-changed", (event) => {
      const { match_id, state } = event.payload;

      // Find and update the match in the list
      const matchIndex = matches.findIndex(m => m.match_id === match_id);
      if (matchIndex !== -1) {
        matches[matchIndex].parse_state = state;
        // Force reactivity by creating a new array
        matches = [...matches];
      }
    });

    // Set up auto-refresh timer (every 30 seconds)
    autoRefreshTimer = setInterval(async () => {
      await autoRefreshAndParse();
    }, 30000); // 30 seconds
  });

  onDestroy(() => {
    // Clean up event listener
    if (unlistenMatchStateChanged) {
      unlistenMatchStateChanged();
    }

    // Clean up auto-refresh timer
    if (autoRefreshTimer) {
      clearInterval(autoRefreshTimer);
    }
  });

  async function autoRefreshAndParse() {
    try {
      // Refresh matches silently (don't show loading state)
      const newMatches = await invoke("refresh_matches");
      matches = newMatches;

      // Get the last 10 matches
      const recentMatches = matches.slice(0, 10);

      // Auto-parse unparsed or failed matches
      for (const match of recentMatches) {
        // Only parse if: Unparsed or Failed, and not currently parsing
        if ((match.parse_state === "Unparsed" || match.parse_state === "Failed") &&
            !parsingMatches.has(match.match_id)) {
          // Don't await - let it run in background
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
      error = `Failed to refresh matches: ${e}`;
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
    return new Date(timestamp * 1000).toLocaleDateString();
  }

  function isWin(match) {
    const isRadiant = match.player_slot < 128;
    return (isRadiant && match.radiant_win) || (!isRadiant && !match.radiant_win);
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
      setTimeout(() => {
        copiedMatchId = null;
      }, 2000);
    } catch (e) {
      console.error("Failed to copy match ID:", e);
    }
  }

  async function openInOpenDota(matchId) {
    try {
      const { open } = await import("@tauri-apps/plugin-opener");
      await open(`https://www.opendota.com/matches/${matchId}`);
    } catch (e) {
      console.error("Failed to open OpenDota link:", e);
    }
  }

  async function parseMatch(matchId) {
    parsingMatches = new Set([...parsingMatches, matchId]);
    try {
      await invoke("parse_match", { matchId, steamId: currentSteamId });
      // Reload matches to get updated state
      await loadMatches();
    } catch (e) {
      error = `Failed to parse match: ${e}`;
      console.error("Failed to parse match:", e);
    } finally {
      const newSet = new Set(parsingMatches);
      newSet.delete(matchId);
      parsingMatches = newSet;
    }
  }

  function getParseStateLabel(state) {
    switch (state) {
      case "Unparsed": return "Parse";
      case "Parsing": return "Parsing...";
      case "Parsed": return "Re-parse";
      case "Failed": return "Retry";
      default: return "Parse";
    }
  }

  function getParseStateColor(state) {
    switch (state) {
      case "Unparsed": return "unparsed";
      case "Parsing": return "parsing";
      case "Parsed": return "parsed";
      case "Failed": return "failed";
      default: return "unparsed";
    }
  }

  function getGameModeName(gameMode) {
    switch (gameMode) {
      case 0: return "Unknown";
      case 1: return "All Pick";
      case 2: return "Captain's Mode";
      case 3: return "Random Draft";
      case 4: return "Single Draft";
      case 5: return "All Random";
      case 6: return "Intro";
      case 7: return "Diretide";
      case 8: return "Reverse CM";
      case 9: return "Mid Only";
      case 10: return "Least Played";
      case 11: return "Limited Heroes";
      case 12: return "Compendium";
      case 13: return "Custom";
      case 14: return "CD";
      case 15: return "Balanced Draft";
      case 16: return "Ability Draft";
      case 18: return "AR/DM";
      case 19: return "1v1 Mid";
      case 20: return "Ranked";
      case 21: return "Turbo";
      case 22: return "Ranked All Pick";
      case 23: return "Turbo";
      default: return `Mode ${gameMode}`;
    }
  }

  // Pagination computed values
  $effect(() => {
    // Reset to page 1 when matches change
    if (matches.length > 0) {
      const maxPage = Math.ceil(matches.length / pageSize);
      if (currentPage > maxPage) {
        currentPage = 1;
      }
    }
  });

  function getPaginatedMatches() {
    const startIndex = (currentPage - 1) * pageSize;
    const endIndex = startIndex + pageSize;
    return matches.slice(startIndex, endIndex);
  }

  function getTotalPages() {
    return Math.ceil(matches.length / pageSize);
  }

  function goToPage(page) {
    const totalPages = getTotalPages();
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
    }
  }

  function nextPage() {
    goToPage(currentPage + 1);
  }

  function previousPage() {
    goToPage(currentPage - 1);
  }

  function changePageSize(newSize) {
    pageSize = newSize;
    currentPage = 1;
  }
</script>

<div class="matches-content">
  <div class="page-header">
    <div class="header-content">
      <h1>Matches</h1>
      <p class="subtitle">Your recent Dota 2 match history</p>
    </div>
    <button class="refresh-btn" onclick={handleRefresh} disabled={isRefreshing}>
      {isRefreshing ? "Refreshing..." : "Refresh Matches"}
    </button>
  </div>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if isLoading}
    <div class="loading">
      <p>Loading matches...</p>
    </div>
  {:else}
    <div class="content">
      {#if matches.length === 0}
        <p class="no-matches">No matches found. Click "Refresh Matches" to fetch your recent games.</p>
      {:else}
        <div class="table-wrapper">
          <table class="matches-table">
            <thead>
              <tr>
                <th>Match ID</th>
                <th>Date</th>
                <th>Hero</th>
                <th>Game Type</th>
                <th>Result</th>
                <th>Duration</th>
                <th>K/D/A</th>
                <th>GPM</th>
                <th>XPM</th>
                <th>Parse</th>
                <th>Goals</th>
              </tr>
            </thead>
            <tbody>
              {#each getPaginatedMatches() as match}
                <tr class={isWin(match) ? "win" : "loss"}>
                  <td class="match-id-cell">
                    <div class="match-id-content">
                      <span class="match-id-text">{match.match_id}</span>
                      <div class="match-id-actions">
                        <button
                          class="icon-btn copy-btn"
                          onclick={() => copyMatchId(match.match_id)}
                          aria-label="Copy match ID"
                          title="Copy match ID"
                        >
                          {#if copiedMatchId === match.match_id}
                            ✓
                          {:else}
                            📋
                          {/if}
                        </button>
                        <button
                          class="icon-btn opendota-btn"
                          onclick={() => openInOpenDota(match.match_id)}
                          aria-label="Open in OpenDota"
                          title="Open in OpenDota"
                        >
                          🔗
                        </button>
                      </div>
                    </div>
                  </td>
                  <td>{formatDate(match.start_time)}</td>
                  <td>{getHeroName(match.hero_id)}</td>
                  <td>{getGameModeName(match.game_mode)}</td>
                  <td class="result">{isWin(match) ? "Won" : "Lost"}</td>
                  <td>{formatDuration(match.duration)}</td>
                  <td>{match.kills}/{match.deaths}/{match.assists}</td>
                  <td>{match.gold_per_min}</td>
                  <td>{match.xp_per_min}</td>
                  <td>
                    {#if match.parse_state === "Parsing"}
                      <span class="parse-state {getParseStateColor(match.parse_state)}">
                        {getParseStateLabel(match.parse_state)}
                      </span>
                    {:else}
                      <button
                        class="parse-btn {getParseStateColor(match.parse_state)}"
                        onclick={() => parseMatch(match.match_id)}
                        disabled={parsingMatches.has(match.match_id)}
                      >
                        {getParseStateLabel(match.parse_state)}
                      </button>
                    {/if}
                  </td>
                  <td>
                    {#if match.parse_state === "Parsing" || parsingMatches.has(match.match_id)}
                      <span class="parsing-spinner" aria-label="Parsing in progress">⏳</span>
                    {:else if match.parse_state === "Unparsed"}
                      <span class="parse-needed" title="Parse match to see accurate goal evaluation">
                        Parse needed
                      </span>
                    {:else if match.goals_applicable > 0}
                      <button
                        class="goals-btn"
                        onclick={() => showGoalDetails(match)}
                        aria-label="View goal details"
                      >
                        {match.goals_achieved}/{match.goals_applicable}
                      </button>
                    {:else}
                      <span class="no-goals">-</span>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>

        <!-- Pagination Controls -->
        <div class="pagination">
          <div class="pagination-info">
            Showing {((currentPage - 1) * pageSize) + 1} to {Math.min(currentPage * pageSize, matches.length)} of {matches.length} matches
          </div>

          <div class="pagination-controls">
            <button
              class="pagination-btn"
              onclick={previousPage}
              disabled={currentPage === 1}
              aria-label="Previous page"
            >
              ← Previous
            </button>

            <div class="page-numbers">
              {#each Array.from({ length: getTotalPages() }, (_, i) => i + 1) as page}
                {#if getTotalPages() <= 7 || page === 1 || page === getTotalPages() || (page >= currentPage - 1 && page <= currentPage + 1)}
                  <button
                    class="page-btn {page === currentPage ? 'active' : ''}"
                    onclick={() => goToPage(page)}
                  >
                    {page}
                  </button>
                {:else if page === currentPage - 2 || page === currentPage + 2}
                  <span class="page-ellipsis">...</span>
                {/if}
              {/each}
            </div>

            <button
              class="pagination-btn"
              onclick={nextPage}
              disabled={currentPage === getTotalPages()}
              aria-label="Next page"
            >
              Next →
            </button>
          </div>

          <div class="page-size-selector">
            <label for="page-size">Per page:</label>
            <select id="page-size" bind:value={pageSize} onchange={() => changePageSize(pageSize)}>
              <option value={10}>10</option>
              <option value={25}>25</option>
              <option value={50}>50</option>
              <option value={100}>100</option>
            </select>
          </div>
        </div>
      {/if}
    </div>
  {/if}

  {#if selectedMatch}
    <div class="modal-overlay" onclick={closeGoalDetails}>
      <div class="modal-content" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h2>Goal Details</h2>
          <button class="close-btn" onclick={closeGoalDetails} aria-label="Close">×</button>
        </div>
        <div class="modal-body">
          <div class="match-summary">
            <p><strong>Hero:</strong> {getHeroName(selectedMatch.hero_id)}</p>
            <p><strong>Result:</strong> <span class={isWin(selectedMatch) ? "win-text" : "loss-text"}>{isWin(selectedMatch) ? "Won" : "Lost"}</span></p>
            <p><strong>Duration:</strong> {formatDuration(selectedMatch.duration)}</p>
          </div>

          {#if goalDetails.length === 0}
            <p class="no-applicable-goals">No applicable goals for this match.</p>
          {:else}
            <div class="goals-list">
              {#each goalDetails as evaluation}
                <div class="goal-detail-card {evaluation.achieved ? 'achieved' : 'not-achieved'}">
                  <div class="goal-status-indicator">
                    {evaluation.achieved ? "✓" : "✗"}
                  </div>
                  <div class="goal-detail-info">
                    <div class="goal-detail-title">
                      {evaluation.goal.hero_id !== null ? getHeroName(evaluation.goal.hero_id) : "Any Hero"}
                      - {getMetricLabel(evaluation.goal.metric)}
                    </div>
                    <div class="goal-detail-target">
                      Target: {evaluation.goal.target_value}
                      {getMetricUnit(evaluation.goal.metric) || ""}
                      by {evaluation.goal.target_time_minutes} min
                    </div>
                    <div class="goal-detail-actual">
                      Actual: {evaluation.actual_value}
                      {getMetricUnit(evaluation.goal.metric) || ""}
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
</div>

<style>
  .matches-content {
    max-width: 1400px;
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
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .header-content {
    flex: 1;
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

  .refresh-btn {
    border-radius: 3px;
    border: 2px solid rgba(139, 92, 46, 0.6);
    padding: 12px 24px;
    font-size: 1em;
    font-weight: bold;
    font-family: inherit;
    cursor: pointer;
    background: linear-gradient(180deg, rgba(60, 100, 40, 0.8) 0%, rgba(40, 80, 30, 0.8) 100%);
    color: #e0e0e0;
    text-transform: uppercase;
    letter-spacing: 1px;
    transition: all 0.3s ease;
    box-shadow:
      0 4px 15px rgba(0, 0, 0, 0.6),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
  }

  .refresh-btn:hover:not(:disabled) {
    background: linear-gradient(180deg, rgba(70, 120, 50, 0.9) 0%, rgba(50, 100, 40, 0.9) 100%);
    border-color: rgba(139, 92, 46, 0.8);
    box-shadow:
      0 6px 20px rgba(0, 0, 0, 0.8),
      0 0 20px rgba(100, 255, 100, 0.3);
    transform: translateY(-2px);
  }

  .refresh-btn:disabled {
    background: linear-gradient(180deg, rgba(40, 40, 50, 0.8) 0%, rgba(30, 30, 40, 0.8) 100%);
    cursor: not-allowed;
    opacity: 0.6;
  }

  .error {
    color: #ff6b6b;
    background-color: rgba(220, 53, 69, 0.2);
    border: 1px solid rgba(220, 53, 69, 0.4);
    border-radius: 3px;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
  }

  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 3rem;
    color: #a0a0a0;
    font-style: italic;
  }

  .content {
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

  .no-matches {
    color: #a0a0a0;
    font-style: italic;
    text-align: center;
    padding: 2rem 0;
  }

  .table-wrapper {
    overflow-x: auto;
    background:
      linear-gradient(135deg, rgba(25, 25, 35, 0.8) 0%, rgba(20, 20, 30, 0.9) 100%),
      repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.1) 3px, rgba(0, 0, 0, 0.1) 6px),
      repeating-linear-gradient(-45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.05) 3px, rgba(0, 0, 0, 0.05) 6px);
    background-size: 100%, 6px 6px, 6px 6px;
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 3px;
    box-shadow:
      0 4px 20px rgba(0, 0, 0, 0.5),
      inset 0 1px 0 rgba(255, 255, 255, 0.03);
  }

  .matches-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.9rem;
  }

  .matches-table th,
  .matches-table td {
    padding: 0.9rem 0.7rem;
    text-align: left;
    border-bottom: 1px solid rgba(139, 92, 46, 0.3);
  }

  .matches-table th {
    background:
      linear-gradient(180deg, rgba(30, 30, 40, 0.9) 0%, rgba(25, 25, 35, 0.9) 100%);
    font-weight: 600;
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 1px;
    font-size: 0.85rem;
  }

  .matches-table tr.win {
    background-color: rgba(96, 192, 64, 0.1);
  }

  .matches-table tr.loss {
    background-color: rgba(220, 53, 69, 0.1);
  }

  .matches-table tr.win .result {
    color: #60c040;
    font-weight: 600;
    text-shadow: 0 0 10px rgba(96, 192, 64, 0.3);
  }

  .matches-table tr.loss .result {
    color: #ff6b6b;
    font-weight: 600;
    text-shadow: 0 0 10px rgba(255, 107, 107, 0.3);
  }

  .matches-table tbody tr {
    transition: all 0.2s ease;
  }

  .matches-table tbody tr:hover {
    background-color: rgba(40, 40, 50, 0.5);
  }

  .goals-btn {
    background: linear-gradient(180deg, rgba(60, 80, 40, 0.8) 0%, rgba(40, 60, 30, 0.8) 100%);
    color: #e0e0e0;
    padding: 6px 12px;
    font-size: 0.9em;
    border-radius: 3px;
    cursor: pointer;
    border: 2px solid rgba(139, 92, 46, 0.6);
    font-weight: bold;
    transition: all 0.3s ease;
  }

  .goals-btn:hover {
    background: linear-gradient(180deg, rgba(70, 95, 50, 0.9) 0%, rgba(50, 75, 40, 0.9) 100%);
    box-shadow: 0 0 15px rgba(100, 255, 100, 0.3);
    transform: translateY(-1px);
  }

  .no-goals {
    color: #808080;
  }

  .parse-needed {
    color: #ffc107;
    font-size: 0.85rem;
    font-style: italic;
    text-shadow: 0 0 10px rgba(255, 193, 7, 0.3);
  }

  .parsing-spinner {
    display: inline-block;
    font-size: 1em;
    animation: spin 2s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .match-id-cell {
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
  }

  .match-id-content {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    justify-content: space-between;
  }

  .match-id-text {
    white-space: nowrap;
  }

  .match-id-actions {
    display: flex;
    gap: 0.25rem;
  }

  .icon-btn {
    background: rgba(30, 30, 40, 0.6);
    border: 1px solid rgba(139, 92, 46, 0.4);
    padding: 0.3em 0.5em;
    font-size: 0.9rem;
    cursor: pointer;
    border-radius: 3px;
    transition: all 0.2s ease;
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-btn:hover {
    background-color: rgba(60, 60, 70, 0.8);
    border-color: rgba(139, 92, 46, 0.6);
    box-shadow: 0 0 10px rgba(212, 175, 55, 0.2);
  }

  .copy-btn {
    min-width: 1.5rem;
  }

  .opendota-btn {
    min-width: 1.5rem;
  }

  .parse-btn {
    padding: 6px 14px;
    font-size: 0.85em;
    border-radius: 3px;
    cursor: pointer;
    transition: all 0.3s ease;
    border: 2px solid;
    font-weight: bold;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .parse-btn.unparsed {
    background: linear-gradient(180deg, rgba(60, 60, 70, 0.8) 0%, rgba(40, 40, 50, 0.8) 100%);
    border-color: rgba(100, 100, 110, 0.6);
    color: #e0e0e0;
  }

  .parse-btn.unparsed:hover:not(:disabled) {
    background: linear-gradient(180deg, rgba(70, 70, 80, 0.9) 0%, rgba(50, 50, 60, 0.9) 100%);
    transform: translateY(-1px);
  }

  .parse-btn.failed {
    background: linear-gradient(180deg, rgba(100, 40, 40, 0.8) 0%, rgba(80, 30, 30, 0.8) 100%);
    border-color: rgba(139, 46, 46, 0.6);
    color: #ff6b6b;
  }

  .parse-btn.failed:hover:not(:disabled) {
    background: linear-gradient(180deg, rgba(120, 50, 50, 0.9) 0%, rgba(100, 40, 40, 0.9) 100%);
    box-shadow: 0 0 15px rgba(255, 100, 100, 0.3);
  }

  .parse-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .parse-state {
    padding: 6px 14px;
    font-size: 0.85em;
    border-radius: 3px;
    font-weight: bold;
    display: inline-block;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .parse-state.parsing {
    color: #ffc107;
    text-shadow: 0 0 10px rgba(255, 193, 7, 0.5);
  }

  .parse-state.parsed {
    color: #60c040;
    text-shadow: 0 0 10px rgba(96, 192, 64, 0.5);
  }

  .parse-btn.parsed {
    background: linear-gradient(180deg, rgba(60, 100, 40, 0.8) 0%, rgba(40, 80, 30, 0.8) 100%);
    border-color: rgba(100, 200, 80, 0.6);
    color: #e0e0e0;
  }

  .parse-btn.parsed:hover:not(:disabled) {
    background: linear-gradient(180deg, rgba(70, 120, 50, 0.9) 0%, rgba(50, 100, 40, 0.9) 100%);
    box-shadow: 0 0 15px rgba(100, 255, 100, 0.3);
  }

  .parse-state.failed {
    color: #ff6b6b;
    text-shadow: 0 0 10px rgba(255, 107, 107, 0.5);
  }

  /* Modal styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.8);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
    padding: 1rem;
  }

  .modal-content {
    background:
      linear-gradient(135deg, rgba(25, 25, 35, 0.98) 0%, rgba(20, 20, 30, 0.98) 100%),
      repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.1) 3px, rgba(0, 0, 0, 0.1) 6px),
      repeating-linear-gradient(-45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.05) 3px, rgba(0, 0, 0, 0.05) 6px);
    background-size: 100%, 6px 6px, 6px 6px;
    border: 2px solid rgba(139, 92, 46, 0.5);
    border-radius: 8px;
    box-shadow:
      0 8px 40px rgba(0, 0, 0, 0.9),
      0 0 100px rgba(255, 100, 0, 0.2);
    max-width: 700px;
    width: 100%;
    max-height: 85vh;
    overflow-y: auto;
  }

  .modal-header {
    background:
      linear-gradient(180deg, rgba(30, 30, 40, 0.9) 0%, rgba(20, 20, 30, 0.9) 100%),
      repeating-linear-gradient(90deg, transparent, transparent 2px, rgba(139, 92, 46, 0.08) 2px, rgba(139, 92, 46, 0.08) 4px);
    background-size: 100%, 4px 4px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 2px solid rgba(139, 92, 46, 0.6);
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.5rem;
    color: #d4af37;
    text-shadow:
      0 0 20px rgba(212, 175, 55, 0.5),
      2px 2px 4px rgba(0, 0, 0, 0.8);
    letter-spacing: 2px;
  }

  .close-btn {
    background: rgba(60, 60, 70, 0.6);
    border: 2px solid rgba(139, 92, 46, 0.4);
    font-size: 1.5rem;
    line-height: 1;
    cursor: pointer;
    color: #d4af37;
    padding: 0;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
    transition: all 0.3s ease;
  }

  .close-btn:hover {
    color: #e0c050;
    background: rgba(70, 70, 80, 0.8);
    border-color: rgba(139, 92, 46, 0.6);
    box-shadow: 0 0 20px rgba(212, 175, 55, 0.3);
  }

  .modal-body {
    padding: 1.5rem;
  }

  .match-summary {
    background: rgba(30, 30, 40, 0.6);
    border: 1px solid rgba(139, 92, 46, 0.4);
    border-left: 3px solid rgba(139, 92, 46, 0.6);
    padding: 1rem;
    border-radius: 3px;
    margin-bottom: 1.5rem;
  }

  .match-summary p {
    margin: 0.5rem 0;
    color: #e0e0e0;
  }

  .win-text {
    color: #60c040;
    font-weight: 600;
    text-shadow: 0 0 10px rgba(96, 192, 64, 0.3);
  }

  .loss-text {
    color: #ff6b6b;
    font-weight: 600;
    text-shadow: 0 0 10px rgba(255, 107, 107, 0.3);
  }

  .no-applicable-goals {
    color: #a0a0a0;
    font-style: italic;
    text-align: center;
    padding: 2rem 0;
  }

  .goals-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .goal-detail-card {
    display: flex;
    gap: 1rem;
    padding: 1.2rem;
    border-radius: 3px;
    border: 2px solid;
    transition: all 0.3s ease;
  }

  .goal-detail-card.achieved {
    background: linear-gradient(90deg, rgba(60, 100, 40, 0.3) 0%, rgba(40, 80, 30, 0.3) 100%);
    border-color: rgba(100, 200, 80, 0.6);
    border-left: 4px solid #60c040;
  }

  .goal-detail-card.not-achieved {
    background: linear-gradient(90deg, rgba(100, 40, 40, 0.3) 0%, rgba(80, 30, 30, 0.3) 100%);
    border-color: rgba(139, 46, 46, 0.6);
    border-left: 4px solid #ff6b6b;
  }

  .goal-detail-card:hover {
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.6);
  }

  .goal-status-indicator {
    font-size: 1.2rem;
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .goal-detail-card.achieved .goal-status-indicator {
    color: #60c040;
    background-color: rgba(96, 192, 64, 0.3);
    box-shadow: 0 0 15px rgba(100, 255, 100, 0.3);
  }

  .goal-detail-card.not-achieved .goal-status-indicator {
    color: #ff6b6b;
    background-color: rgba(220, 53, 69, 0.3);
    box-shadow: 0 0 15px rgba(255, 100, 100, 0.3);
  }

  .goal-detail-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .goal-detail-title {
    font-weight: 600;
    font-size: 0.95rem;
    color: #e0e0e0;
  }

  .goal-detail-target,
  .goal-detail-actual {
    font-size: 0.9rem;
    color: #b0b0b0;
  }

  /* Pagination */
  .pagination {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1.5rem;
    padding: 20px;
    margin-top: 20px;
    background: rgba(30, 30, 40, 0.5);
    border: 2px solid rgba(139, 92, 46, 0.3);
    border-radius: 5px;
    flex-wrap: wrap;
  }

  .pagination-info {
    color: #a0a0a0;
    font-size: 0.9rem;
    white-space: nowrap;
  }

  .pagination-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    justify-content: center;
  }

  .pagination-btn {
    padding: 8px 16px;
    background: linear-gradient(180deg, rgba(60, 60, 70, 0.8) 0%, rgba(40, 40, 50, 0.8) 100%);
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 3px;
    color: #e0e0e0;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    white-space: nowrap;
  }

  .pagination-btn:hover:not(:disabled) {
    background: linear-gradient(180deg, rgba(70, 70, 80, 0.9) 0%, rgba(50, 50, 60, 0.9) 100%);
    border-color: rgba(139, 92, 46, 0.6);
    transform: translateY(-2px);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.4);
  }

  .pagination-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .page-numbers {
    display: flex;
    gap: 0.25rem;
    align-items: center;
  }

  .page-btn {
    min-width: 2.5rem;
    height: 2.5rem;
    padding: 0.5rem;
    background: rgba(30, 30, 40, 0.6);
    border: 2px solid rgba(139, 92, 46, 0.3);
    border-radius: 3px;
    color: #e0e0e0;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .page-btn:hover {
    background: rgba(40, 40, 50, 0.8);
    border-color: rgba(139, 92, 46, 0.5);
    color: #d4af37;
  }

  .page-btn.active {
    background: linear-gradient(180deg, rgba(60, 80, 40, 0.8) 0%, rgba(40, 60, 30, 0.8) 100%);
    border-color: #d4af37;
    color: #d4af37;
    box-shadow: 0 0 15px rgba(212, 175, 55, 0.3);
  }

  .page-ellipsis {
    color: #808080;
    padding: 0 0.25rem;
  }

  .page-size-selector {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    white-space: nowrap;
  }

  .page-size-selector label {
    color: #a0a0a0;
    font-size: 0.9rem;
    font-weight: 600;
  }

  .page-size-selector select {
    padding: 8px 12px;
    background-color: rgba(30, 30, 40, 0.8);
    border: 2px solid rgba(139, 92, 46, 0.4);
    border-radius: 3px;
    color: #e0e0e0;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .page-size-selector select:focus {
    border-color: rgba(139, 92, 46, 0.8);
    outline: none;
    box-shadow: 0 0 10px rgba(212, 175, 55, 0.2);
  }

  @media (max-width: 768px) {
    .pagination {
      flex-direction: column;
      align-items: stretch;
    }

    .pagination-info,
    .pagination-controls,
    .page-size-selector {
      justify-content: center;
    }

    .page-numbers {
      flex-wrap: wrap;
    }
  }
</style>
