<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let databasePath = $state("");
  let isLoading = $state(true);
  let error = $state("");
  let successMessage = $state("");
  let isBackfilling = $state(false);
  let isReparsing = $state(false);
  let isClearing = $state(false);
  let steamId = $state("");

  onMount(async () => {
    await loadDatabasePath();
    await loadSteamId();
  });

  async function loadDatabasePath() {
    try {
      databasePath = await invoke("get_database_folder_path");
    } catch (e) {
      error = `Failed to get database path: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  async function loadSteamId() {
    try {
      const settings = await invoke("get_settings");
      steamId = settings.steam_id || "";
    } catch (e) {
      console.error("Failed to load Steam ID:", e);
    }
  }

  async function openDatabaseFolder() {
    error = "";
    successMessage = "";
    try {
      await invoke("open_database_folder");
    } catch (e) {
      error = `Failed to open database folder: ${e}`;
    }
  }

  async function backfillMatches() {
    if (!steamId) {
      error = "No Steam ID configured";
      return;
    }

    error = "";
    successMessage = "";
    isBackfilling = true;

    try {
      const result = await invoke("backfill_historical_matches", { steamId });
      successMessage = result;
    } catch (e) {
      error = `Failed to backfill matches: ${e}`;
    } finally {
      isBackfilling = false;
    }
  }

  async function reparsePendingMatches() {
    if (!steamId) {
      error = "No Steam ID configured";
      return;
    }

    error = "";
    successMessage = "";
    isReparsing = true;

    try {
      const result = await invoke("reparse_pending_matches", { steamId });
      successMessage = result;
    } catch (e) {
      error = `Failed to reparse matches: ${e}`;
    } finally {
      isReparsing = false;
    }
  }

  async function clearAllMatches() {
    const confirmed = confirm(
      "Are you sure you want to clear ALL matches?\n\n" +
      "This will permanently delete all match data, including parsed CS data.\n" +
      "This action cannot be undone!"
    );

    if (!confirmed) {
      return;
    }

    error = "";
    successMessage = "";
    isClearing = true;

    try {
      const result = await invoke("clear_matches");
      successMessage = result;
    } catch (e) {
      error = `Failed to clear matches: ${e}`;
    } finally {
      isClearing = false;
    }
  }
</script>

<div class="settings-content">
  <div class="page-header">
    <h1>Settings</h1>
    <p class="subtitle">Manage your application settings</p>
  </div>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if successMessage}
    <p class="success">{successMessage}</p>
  {/if}

  <div class="settings-section">
    <h2>Database</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>Database Location</h3>
        <p class="setting-description">
          Your Dota 2 match data and goals are stored in a SQLite database.
        </p>
        {#if isLoading}
          <p class="database-path loading">Loading...</p>
        {:else}
          <p class="database-path">{databasePath}</p>
        {/if}
      </div>
      <button class="open-folder-btn" onclick={openDatabaseFolder} disabled={isLoading}>
        Open Folder
      </button>
    </div>
  </div>

  <div class="settings-section">
    <h2>Match History</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>Backfill Historical Matches</h3>
        <p class="setting-description">
          Download and parse 100 matches from before your oldest stored match. This helps build up your historical data for better analysis and trends.
        </p>
        <p class="warning-text">
          ⚠️ This process may take several minutes as it fetches and parses each match individually.
        </p>
      </div>
      <button
        class="backfill-btn"
        onclick={backfillMatches}
        disabled={isBackfilling || !steamId}
      >
        {isBackfilling ? 'Backfilling...' : 'Backfill 100 Matches'}
      </button>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>Reparse Pending Matches</h3>
        <p class="setting-description">
          Retry parsing all matches that failed or haven't been parsed yet. This is useful for matches that couldn't be parsed initially due to API issues.
        </p>
        <p class="warning-text">
          ⚠️ This will attempt to parse all unparsed/failed matches, which may take several minutes.
        </p>
      </div>
      <button
        class="reparse-btn"
        onclick={reparsePendingMatches}
        disabled={isReparsing || !steamId}
      >
        {isReparsing ? 'Reparsing...' : 'Reparse Pending'}
      </button>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>Clear All Matches</h3>
        <p class="setting-description">
          Permanently delete all match data from the database. This will remove all matches and their associated parsed data.
        </p>
        <p class="warning-text">
          ⚠️ WARNING: This action cannot be undone! All match history will be permanently deleted.
        </p>
      </div>
      <button
        class="clear-btn"
        onclick={clearAllMatches}
        disabled={isClearing}
      >
        {isClearing ? 'Clearing...' : 'Clear All Matches'}
      </button>
    </div>
  </div>
</div>

<style>
.settings-content {
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
  box-shadow:
    0 4px 20px rgba(0, 0, 0, 0.5),
    inset 0 1px 0 rgba(255, 255, 255, 0.05);
}

.page-header h1 {
  margin: 0 0 0.5rem 0;
  font-size: 2.5em;
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
  font-size: 1rem;
  letter-spacing: 1px;
}

.error {
  color: #ff6b6b;
  background-color: rgba(220, 53, 69, 0.2);
  border: 1px solid rgba(220, 53, 69, 0.4);
  border-radius: 3px;
  padding: 0.75rem 1rem;
  margin-bottom: 1.5rem;
}

.success {
  color: #60c040;
  background-color: rgba(96, 192, 64, 0.2);
  border: 1px solid rgba(96, 192, 64, 0.4);
  border-radius: 3px;
  padding: 0.75rem 1rem;
  margin-bottom: 1.5rem;
}

.settings-section {
  padding: 30px;
  background:
    linear-gradient(135deg, rgba(25, 25, 35, 0.8) 0%, rgba(20, 20, 30, 0.9) 100%),
    repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.1) 3px, rgba(0, 0, 0, 0.1) 6px),
    repeating-linear-gradient(-45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.05) 3px, rgba(0, 0, 0, 0.05) 6px);
  background-size: 100%, 6px 6px, 6px 6px;
  border: 2px solid rgba(139, 92, 46, 0.4);
  border-radius: 8px;
  margin-bottom: 2rem;
  box-shadow:
    0 4px 20px rgba(0, 0, 0, 0.5),
    inset 0 1px 0 rgba(255, 255, 255, 0.03);
}

.settings-section h2 {
  margin-bottom: 1.5rem;
  font-size: 1.8em;
  color: #d4af37;
  text-transform: uppercase;
  letter-spacing: 2px;
  text-shadow: 0 0 10px rgba(212, 175, 55, 0.3);
  border-bottom: 2px solid rgba(139, 92, 46, 0.5);
  padding-bottom: 15px;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1.5rem;
  padding: 25px;
  background:
    linear-gradient(135deg, rgba(25, 25, 35, 0.8) 0%, rgba(20, 20, 30, 0.9) 100%),
    repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.1) 3px, rgba(0, 0, 0, 0.1) 6px),
    repeating-linear-gradient(-45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.05) 3px, rgba(0, 0, 0, 0.05) 6px);
  background-size: 100%, 6px 6px, 6px 6px;
  border: 2px solid rgba(139, 92, 46, 0.4);
  border-left: 3px solid rgba(139, 92, 46, 0.6);
  border-radius: 3px;
  box-shadow:
    0 4px 20px rgba(0, 0, 0, 0.5),
    inset 0 1px 0 rgba(255, 255, 255, 0.03);
}

.setting-info {
  flex: 1;
}

.setting-info h3 {
  margin: 0 0 0.75rem 0;
  font-size: 1.3rem;
  font-weight: 600;
  color: #d4af37;
  text-transform: uppercase;
  letter-spacing: 1px;
  text-shadow: 0 0 10px rgba(212, 175, 55, 0.2);
}

.setting-description {
  margin: 0 0 1rem 0;
  color: #a0a0a0;
  font-size: 0.95rem;
  line-height: 1.5;
}

.database-path {
  font-family: 'Courier New', monospace;
  background-color: rgba(30, 30, 40, 0.8);
  color: #90ff90;
  padding: 0.75rem;
  border-radius: 3px;
  border: 1px solid rgba(139, 92, 46, 0.4);
  word-break: break-all;
  font-size: 0.85rem;
  margin: 0;
  box-shadow: inset 0 2px 5px rgba(0, 0, 0, 0.5);
}

.database-path.loading {
  color: #808080;
  font-style: italic;
}

.open-folder-btn {
  border-radius: 3px;
  border: 2px solid rgba(139, 92, 46, 0.6);
  padding: 12px 24px;
  font-size: 1em;
  font-weight: bold;
  font-family: inherit;
  color: #e0e0e0;
  background: linear-gradient(180deg, rgba(60, 80, 40, 0.8) 0%, rgba(40, 60, 30, 0.8) 100%);
  transition: all 0.3s ease;
  box-shadow:
    0 4px 15px rgba(0, 0, 0, 0.6),
    inset 0 1px 0 rgba(255, 255, 255, 0.1);
  cursor: pointer;
  white-space: nowrap;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.open-folder-btn:hover {
  background: linear-gradient(180deg, rgba(70, 95, 50, 0.9) 0%, rgba(50, 75, 40, 0.9) 100%);
  border-color: rgba(139, 92, 46, 0.8);
  box-shadow:
    0 6px 20px rgba(0, 0, 0, 0.8),
    0 0 20px rgba(100, 255, 100, 0.2);
  transform: translateY(-2px);
}

.open-folder-btn:active {
  transform: translateY(0);
}

.open-folder-btn:disabled {
  background: linear-gradient(180deg, rgba(40, 40, 50, 0.8) 0%, rgba(30, 30, 40, 0.8) 100%);
  cursor: not-allowed;
  opacity: 0.6;
}

.warning-text {
  margin: 0.5rem 0 0 0;
  color: #f0b840;
  font-size: 0.9rem;
  font-weight: 600;
}

.backfill-btn {
  border-radius: 3px;
  border: 2px solid rgba(139, 92, 46, 0.6);
  padding: 12px 24px;
  font-size: 1em;
  font-weight: bold;
  font-family: inherit;
  color: #e0e0e0;
  background: linear-gradient(180deg, rgba(60, 80, 100, 0.8) 0%, rgba(40, 60, 80, 0.8) 100%);
  transition: all 0.3s ease;
  box-shadow:
    0 4px 15px rgba(0, 0, 0, 0.6),
    inset 0 1px 0 rgba(255, 255, 255, 0.1);
  cursor: pointer;
  white-space: nowrap;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.backfill-btn:hover:not(:disabled) {
  background: linear-gradient(180deg, rgba(70, 95, 120, 0.9) 0%, rgba(50, 75, 100, 0.9) 100%);
  border-color: rgba(139, 92, 46, 0.8);
  box-shadow:
    0 6px 20px rgba(0, 0, 0, 0.8),
    0 0 20px rgba(100, 150, 200, 0.2);
  transform: translateY(-2px);
}

.backfill-btn:active:not(:disabled) {
  transform: translateY(0);
}

.backfill-btn:disabled {
  background: linear-gradient(180deg, rgba(40, 40, 50, 0.8) 0%, rgba(30, 30, 40, 0.8) 100%);
  cursor: not-allowed;
  opacity: 0.6;
}

.reparse-btn {
  border-radius: 3px;
  border: 2px solid rgba(139, 92, 46, 0.6);
  padding: 12px 24px;
  font-size: 1em;
  font-weight: bold;
  font-family: inherit;
  color: #e0e0e0;
  background: linear-gradient(180deg, rgba(100, 60, 80, 0.8) 0%, rgba(80, 40, 60, 0.8) 100%);
  transition: all 0.3s ease;
  box-shadow:
    0 4px 15px rgba(0, 0, 0, 0.6),
    inset 0 1px 0 rgba(255, 255, 255, 0.1);
  cursor: pointer;
  white-space: nowrap;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.reparse-btn:hover:not(:disabled) {
  background: linear-gradient(180deg, rgba(120, 70, 95, 0.9) 0%, rgba(100, 50, 75, 0.9) 100%);
  border-color: rgba(139, 92, 46, 0.8);
  box-shadow:
    0 6px 20px rgba(0, 0, 0, 0.8),
    0 0 20px rgba(200, 100, 150, 0.2);
  transform: translateY(-2px);
}

.reparse-btn:active:not(:disabled) {
  transform: translateY(0);
}

.reparse-btn:disabled {
  background: linear-gradient(180deg, rgba(40, 40, 50, 0.8) 0%, rgba(30, 30, 40, 0.8) 100%);
  cursor: not-allowed;
  opacity: 0.6;
}

@media (max-width: 600px) {
  .setting-item {
    flex-direction: column;
    align-items: stretch;
  }

  .open-folder-btn,
  .backfill-btn,
  .reparse-btn,
  .clear-btn {
    width: 100%;
  }
}

.clear-btn {
  border-radius: 3px;
  border: 2px solid rgba(139, 46, 46, 0.6);
  padding: 12px 24px;
  font-size: 1em;
  font-weight: bold;
  font-family: inherit;
  color: #e0e0e0;
  background: linear-gradient(180deg, rgba(120, 40, 40, 0.8) 0%, rgba(100, 30, 30, 0.8) 100%);
  transition: all 0.3s ease;
  box-shadow:
    0 4px 15px rgba(0, 0, 0, 0.6),
    inset 0 1px 0 rgba(255, 255, 255, 0.1);
  cursor: pointer;
  white-space: nowrap;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.clear-btn:hover:not(:disabled) {
  background: linear-gradient(180deg, rgba(150, 50, 50, 0.9) 0%, rgba(130, 40, 40, 0.9) 100%);
  border-color: rgba(139, 46, 46, 0.8);
  box-shadow:
    0 6px 20px rgba(0, 0, 0, 0.8),
    0 0 20px rgba(255, 100, 100, 0.3);
  transform: translateY(-2px);
}

.clear-btn:active:not(:disabled) {
  transform: translateY(0);
}

.clear-btn:disabled {
  background: linear-gradient(180deg, rgba(40, 40, 50, 0.8) 0%, rgba(30, 30, 40, 0.8) 100%);
  cursor: not-allowed;
  opacity: 0.6;
}
</style>
