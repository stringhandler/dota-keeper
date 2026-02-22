<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let isLoading = $state(true);
  let error = $state("");
  let history = $state([]);
  let activeFilter = $state("all"); // "all", "weekly", "daily"

  onMount(async () => {
    await loadHistory();
  });

  async function loadHistory() {
    isLoading = true;
    error = "";
    try {
      const filter = activeFilter === "all" ? null : activeFilter;
      history = await invoke("get_challenge_history_cmd", {
        challengeType: filter,
        limit: 100,
      });
    } catch (e) {
      error = `Failed to load history: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  async function setFilter(f) {
    activeFilter = f;
    await loadHistory();
  }

  function formatDate(dateStr) {
    // dateStr is "YYYY-MM-DD" (week start date)
    return new Date(dateStr + "T00:00:00").toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  }

  function formatTimestamp(ts) {
    if (!ts) return "—";
    return new Date(ts * 1000).toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
    });
  }

  // Group history items by week (using period_start_date)
  let grouped = $derived(() => {
    const groups = {};
    for (const item of history) {
      const key = item.challenge_type === "weekly"
        ? `Week of ${formatDate(item.period_start_date)}`
        : formatDate(item.period_start_date);
      if (!groups[key]) groups[key] = [];
      groups[key].push(item);
    }
    return Object.entries(groups).map(([label, items]) => ({ label, items }));
  });
</script>

<div class="history-content">
  <div class="page-header">
    <a href="/challenges" class="back-link">← Back to Challenges</a>
    <h1>Challenge History</h1>
    <p class="subtitle">Your completed and failed challenges</p>
  </div>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  <div class="filter-tabs">
    <button class="tab" class:active={activeFilter === "all"} onclick={() => setFilter("all")}>All</button>
    <button class="tab" class:active={activeFilter === "weekly"} onclick={() => setFilter("weekly")}>Weekly</button>
    <button class="tab" class:active={activeFilter === "daily"} onclick={() => setFilter("daily")}>Daily</button>
  </div>

  {#if isLoading}
    <p class="loading">Loading history...</p>
  {:else if history.length === 0}
    <p class="empty">No challenge history yet. Complete some challenges to see them here!</p>
  {:else}
    {#each grouped() as group}
      <div class="group">
        <h2 class="group-label">{group.label}</h2>
        {#each group.items as item}
          <div class="history-item" class:completed={item.status === "completed"} class:failed={item.status === "failed"}>
            <div class="item-left">
              <span class="item-type-badge" class:weekly={item.challenge_type === "weekly"}>
                {item.challenge_type}
              </span>
              <p class="item-description">{item.challenge_description}</p>
            </div>
            <div class="item-right">
              <span class="status-icon">
                {item.status === "completed" ? "✅" : "✗"}
              </span>
              {#if item.completed_at}
                <span class="completed-date">{formatTimestamp(item.completed_at)}</span>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/each}
  {/if}
</div>

<style>
  .history-content {
    max-width: 900px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: 28px;
  }

  .back-link {
    display: inline-block;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-secondary);
    text-decoration: none;
    margin-bottom: 12px;
    transition: color 0.2s;
  }

  .back-link:hover {
    color: var(--gold);
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
    font-size: 11px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
    margin: 0;
  }

  .error {
    color: var(--red);
    background: rgba(248, 113, 113, 0.1);
    border: 1px solid rgba(248, 113, 113, 0.25);
    border-radius: 4px;
    padding: 10px 14px;
    margin-bottom: 16px;
    font-size: 13px;
  }

  .loading, .empty {
    color: var(--text-muted);
    text-align: center;
    padding: 48px;
    font-size: 13px;
  }

  .filter-tabs {
    display: flex;
    gap: 8px;
    margin-bottom: 20px;
  }

  .tab {
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    font-size: 11px;
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-secondary);
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tab:hover {
    color: var(--text-primary);
    border-color: var(--border-active);
  }

  .tab.active {
    background: rgba(240, 180, 41, 0.1);
    border-color: var(--gold);
    color: var(--gold);
  }

  .group {
    margin-bottom: 28px;
  }

  .group-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 2px;
    color: var(--text-muted);
    margin: 0 0 12px 0;
    border-bottom: 1px solid var(--border);
    padding-bottom: 8px;
  }

  .history-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    padding: 14px 18px;
    margin-bottom: 8px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-left: 3px solid var(--border);
    border-radius: 6px;
    transition: border-color 0.2s;
  }

  .history-item:hover {
    border-color: var(--border-active);
  }

  .history-item.completed {
    border-left-color: var(--green);
  }

  .history-item.failed {
    border-left-color: var(--red);
  }

  .item-left {
    display: flex;
    align-items: center;
    gap: 12px;
    flex: 1;
  }

  .item-type-badge {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 9px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 1.5px;
    padding: 4px 8px;
    border-radius: 3px;
    background: rgba(240, 180, 41, 0.1);
    color: var(--gold);
    white-space: nowrap;
    border: 1px solid rgba(240, 180, 41, 0.2);
  }

  .item-type-badge.weekly {
    background: rgba(100, 130, 255, 0.1);
    border-color: rgba(100, 130, 255, 0.2);
    color: #8090ff;
  }

  .item-description {
    margin: 0;
    color: var(--text-primary);
    font-size: 14px;
  }

  .item-right {
    display: flex;
    align-items: center;
    gap: 12px;
    white-space: nowrap;
  }

  .status-icon {
    font-size: 16px;
  }

  .completed-date {
    font-size: 11px;
    color: var(--text-muted);
  }
</style>
