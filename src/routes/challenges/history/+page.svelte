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
    max-width: 800px;
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
  }

  .back-link {
    display: inline-block;
    color: #d4af37;
    text-decoration: none;
    font-size: 0.9rem;
    margin-bottom: 0.75rem;
    transition: color 0.2s;
  }

  .back-link:hover {
    color: #e0c050;
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

  .error {
    color: #ff6b6b;
    background-color: rgba(220, 53, 69, 0.2);
    border: 1px solid rgba(220, 53, 69, 0.4);
    border-radius: 3px;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
  }

  .loading, .empty {
    color: #a0a0a0;
    text-align: center;
    padding: 3rem;
    font-style: italic;
  }

  .filter-tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
  }

  .tab {
    background: transparent;
    border: 1px solid rgba(139, 92, 46, 0.4);
    color: #808080;
    padding: 0.4rem 1rem;
    border-radius: 3px;
    font-family: inherit;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .tab:hover {
    color: #d4af37;
    border-color: rgba(212, 175, 55, 0.5);
  }

  .tab.active {
    background: rgba(212, 175, 55, 0.15);
    border-color: rgba(212, 175, 55, 0.6);
    color: #d4af37;
  }

  .group {
    margin-bottom: 2rem;
  }

  .group-label {
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 2px;
    color: #606060;
    margin: 0 0 0.75rem 0;
    border-bottom: 1px solid rgba(139, 92, 46, 0.2);
    padding-bottom: 0.5rem;
  }

  .history-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.9rem 1.25rem;
    margin-bottom: 0.5rem;
    background: rgba(20, 20, 30, 0.7);
    border: 1px solid rgba(139, 92, 46, 0.3);
    border-left: 3px solid rgba(139, 92, 46, 0.4);
    border-radius: 4px;
    transition: border-color 0.2s;
  }

  .history-item.completed {
    border-left-color: rgba(96, 192, 64, 0.7);
  }

  .history-item.failed {
    border-left-color: rgba(255, 107, 107, 0.5);
  }

  .item-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
  }

  .item-type-badge {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 1px;
    padding: 0.2rem 0.5rem;
    border-radius: 3px;
    background: rgba(139, 92, 46, 0.2);
    color: #a0a0a0;
    white-space: nowrap;
    border: 1px solid rgba(139, 92, 46, 0.3);
  }

  .item-type-badge.weekly {
    background: rgba(100, 100, 200, 0.2);
    border-color: rgba(100, 100, 200, 0.3);
    color: #a0a0ff;
  }

  .item-description {
    margin: 0;
    color: #c0c0c0;
    font-size: 0.95rem;
  }

  .item-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    white-space: nowrap;
  }

  .status-icon {
    font-size: 1.1rem;
  }

  .completed-date {
    font-size: 0.8rem;
    color: #606060;
  }
</style>
