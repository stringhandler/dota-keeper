<script>
  import { invoke } from "@tauri-apps/api/core";
  import { getVersion } from "@tauri-apps/api/app";
  import { check } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { onMount } from "svelte";
  import { trackPageView, updateAnalyticsConsent } from "$lib/analytics.js";

  let databasePath = $state("");
  let isLoading = $state(true);
  let error = $state("");
  let successMessage = $state("");
  let isBackfilling = $state(false);
  let isReparsing = $state(false);
  let isClearing = $state(false);
  let steamId = $state("");
  let suggestionDifficulty = $state("Medium");
  let customPercentage = $state(10);
  let isSavingDifficulty = $state(false);
  let analyticsConsent = $state("NotYet");
  let isSavingAnalytics = $state(false);
  let appVersion = $state("");
  let checkingUpdate = $state(false);
  let updateResult = $state(null);
  let updateError = $state("");
  let isInstalling = $state(false);

  onMount(async () => {
    await loadDatabasePath();
    await loadSteamId();
    await loadDifficulty();
    await loadAnalytics();
    await loadAppVersion();

    // Track page view
    trackPageView("Settings");
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

  async function loadDifficulty() {
    try {
      const settings = await invoke("get_settings");
      suggestionDifficulty = settings.suggestion_difficulty || "Medium";
      customPercentage = settings.suggestion_custom_percentage
        ? Math.round(settings.suggestion_custom_percentage * 100)
        : 10;
    } catch (e) {
      console.error("Failed to load difficulty setting:", e);
    }
  }

  async function loadAnalytics() {
    try {
      const settings = await invoke("get_settings");
      analyticsConsent = settings.analytics_consent ?? "NotYet";
    } catch (e) {
      console.error("Failed to load analytics setting:", e);
    }
  }

  async function saveDifficulty() {
    error = "";
    successMessage = "";
    isSavingDifficulty = true;
    try {
      const pct = suggestionDifficulty === "Custom" ? customPercentage / 100 : null;
      await invoke("save_suggestion_difficulty", {
        difficulty: suggestionDifficulty,
        customPercentage: pct,
      });
      successMessage = "Difficulty setting saved.";
    } catch (e) {
      error = `Failed to save difficulty: ${e}`;
    } finally {
      isSavingDifficulty = false;
    }
  }

  async function saveAnalytics(consent) {
    error = "";
    successMessage = "";
    isSavingAnalytics = true;
    try {
      await invoke("save_analytics_consent", { consent });
      analyticsConsent = consent;
      // Update the analytics module's cached state
      updateAnalyticsConsent(consent);

      if (consent === "Accepted") {
        successMessage = "Analytics enabled. Thank you for helping improve Dota Keeper!";
      } else if (consent === "Declined") {
        successMessage = "Analytics disabled. No data will be sent.";
      } else {
        successMessage = "Analytics preference reset.";
      }
    } catch (e) {
      error = `Failed to save analytics setting: ${e}`;
    } finally {
      isSavingAnalytics = false;
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

  async function loadAppVersion() {
    try {
      appVersion = await getVersion();
    } catch (e) {
      console.error("Failed to get app version:", e);
    }
  }

  async function checkForUpdates() {
    checkingUpdate = true;
    updateResult = null;
    updateError = "";
    try {
      const update = await check();
      if (update?.available) {
        updateResult = { available: true, version: update.version, update };
      } else {
        updateResult = { available: false };
      }
    } catch (e) {
      updateError = String(e);
    } finally {
      checkingUpdate = false;
    }
  }

  async function installUpdate() {
    if (!updateResult?.available) return;
    isInstalling = true;
    updateError = "";
    try {
      await updateResult.update.downloadAndInstall();
      await relaunch();
    } catch (e) {
      updateError = `Failed to install update: ${e}`;
      isInstalling = false;
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
    <h2>Goal Suggestions</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>Suggestion Difficulty</h3>
        <p class="setting-description">
          Controls how ambitious your CS goal suggestions will be. Medium targets 5–10% improvement over your recent average.
        </p>
        <div class="difficulty-controls">
          <select class="difficulty-select" bind:value={suggestionDifficulty}>
            <option value="Easy">Easy (3–5% improvement)</option>
            <option value="Medium">Medium (5–10% improvement)</option>
            <option value="Hard">Hard (10–15% improvement)</option>
            <option value="Custom">Custom</option>
          </select>
          {#if suggestionDifficulty === "Custom"}
            <div class="custom-pct">
              <label>
                Target improvement:
                <input
                  type="number"
                  min="1"
                  max="50"
                  bind:value={customPercentage}
                  class="pct-input"
                />%
              </label>
            </div>
          {/if}
        </div>
      </div>
      <button
        class="save-btn"
        onclick={saveDifficulty}
        disabled={isSavingDifficulty}
      >
        {isSavingDifficulty ? "Saving..." : "Save"}
      </button>
    </div>
  </div>

  <div class="settings-section">
    <h2>Privacy</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>Anonymous Analytics</h3>
        <p class="setting-description">
          Help improve Dota Keeper by sending anonymous usage data. We track page views, feature usage, and errors to understand how the app is used.
        </p>
        <p class="setting-description" style="margin-top: 0.5rem;">
          <strong>What we collect:</strong> App version, platform, page views, feature usage events (e.g. "goal created", "challenge accepted").
        </p>
        <p class="setting-description" style="margin-top: 0.5rem;">
          <strong>What we DON'T collect:</strong> Steam ID, match IDs, hero choices, goal descriptions, or any personally identifiable information.
        </p>
        <div class="analytics-options" style="margin-top: 1rem;">
          <label class="radio-label">
            <input
              type="radio"
              name="analytics"
              value="Accepted"
              checked={analyticsConsent === "Accepted"}
              onchange={() => saveAnalytics("Accepted")}
              disabled={isSavingAnalytics}
            />
            <span>Accept — Help improve Dota Keeper</span>
          </label>
          <label class="radio-label">
            <input
              type="radio"
              name="analytics"
              value="Declined"
              checked={analyticsConsent === "Declined"}
              onchange={() => saveAnalytics("Declined")}
              disabled={isSavingAnalytics}
            />
            <span>Decline — Don't send usage data</span>
          </label>
        </div>
      </div>
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

  <div class="settings-section">
    <h2>Updates</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>Application Updates</h3>
        <p class="setting-description">
          Manually check for a new version of Dota Keeper at any time.
        </p>
        {#if appVersion}
          <p class="version-label">Current version: <span class="version-value">v{appVersion}</span></p>
        {/if}
        {#if updateError}
          <p class="update-error">{updateError}</p>
        {:else if updateResult !== null}
          {#if updateResult.available}
            <p class="update-available">Version {updateResult.version} is available!</p>
          {:else}
            <p class="update-current">You're on the latest version (v{appVersion})</p>
          {/if}
        {/if}
      </div>
      <div class="update-actions">
        <button
          class="check-update-btn"
          onclick={checkForUpdates}
          disabled={checkingUpdate || isInstalling}
        >
          {checkingUpdate ? 'Checking...' : 'Check for Updates'}
        </button>
        {#if updateResult?.available}
          <button
            class="install-update-btn"
            onclick={installUpdate}
            disabled={isInstalling}
          >
            {isInstalling ? 'Installing...' : 'Install & Restart'}
          </button>
        {/if}
      </div>
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
  font-size: 1.5em;
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
  font-size: 1.1rem;
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

.difficulty-controls {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin-top: 0.75rem;
}

.difficulty-select {
  background: rgba(30, 30, 40, 0.9);
  color: #e0e0e0;
  border: 1px solid rgba(139, 92, 46, 0.5);
  border-radius: 3px;
  padding: 0.5rem 0.75rem;
  font-family: inherit;
  font-size: 0.95rem;
  cursor: pointer;
  max-width: 260px;
}

.custom-pct {
  color: #a0a0a0;
  font-size: 0.95rem;
}

.custom-pct label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.pct-input {
  width: 64px;
  background: rgba(30, 30, 40, 0.9);
  color: #e0e0e0;
  border: 1px solid rgba(139, 92, 46, 0.5);
  border-radius: 3px;
  padding: 0.4rem 0.5rem;
  font-family: inherit;
  font-size: 0.95rem;
  text-align: center;
}

.save-btn {
  border-radius: 3px;
  border: 2px solid rgba(139, 92, 46, 0.6);
  padding: 12px 24px;
  font-size: 1em;
  font-weight: bold;
  font-family: inherit;
  color: #e0e0e0;
  background: linear-gradient(180deg, rgba(60, 80, 40, 0.8) 0%, rgba(40, 60, 30, 0.8) 100%);
  transition: all 0.3s ease;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.6), inset 0 1px 0 rgba(255, 255, 255, 0.1);
  cursor: pointer;
  white-space: nowrap;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.save-btn:hover:not(:disabled) {
  background: linear-gradient(180deg, rgba(70, 95, 50, 0.9) 0%, rgba(50, 75, 40, 0.9) 100%);
  border-color: rgba(139, 92, 46, 0.8);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.8), 0 0 20px rgba(100, 255, 100, 0.2);
  transform: translateY(-2px);
}

.save-btn:disabled {
  background: linear-gradient(180deg, rgba(40, 40, 50, 0.8) 0%, rgba(30, 30, 40, 0.8) 100%);
  cursor: not-allowed;
  opacity: 0.6;
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

.version-label {
  margin: 0.5rem 0 0 0;
  color: #808080;
  font-size: 0.9rem;
}

.version-value {
  color: #a0a0a0;
  font-family: 'Courier New', monospace;
}

.update-error {
  margin: 0.75rem 0 0 0;
  color: #ff6b6b;
  background-color: rgba(220, 53, 69, 0.2);
  border: 1px solid rgba(220, 53, 69, 0.4);
  border-radius: 3px;
  padding: 0.5rem 0.75rem;
  font-size: 0.9rem;
}

.update-available {
  margin: 0.75rem 0 0 0;
  color: #f0b840;
  font-size: 0.95rem;
  font-weight: 600;
}

.update-current {
  margin: 0.75rem 0 0 0;
  color: #60c040;
  font-size: 0.95rem;
  font-weight: 600;
}

.update-actions {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  align-items: flex-end;
}

.check-update-btn {
  border-radius: 3px;
  border: 2px solid rgba(139, 92, 46, 0.6);
  padding: 12px 24px;
  font-size: 1em;
  font-weight: bold;
  font-family: inherit;
  color: #e0e0e0;
  background: linear-gradient(180deg, rgba(60, 80, 40, 0.8) 0%, rgba(40, 60, 30, 0.8) 100%);
  transition: all 0.3s ease;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.6), inset 0 1px 0 rgba(255, 255, 255, 0.1);
  cursor: pointer;
  white-space: nowrap;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.check-update-btn:hover:not(:disabled) {
  background: linear-gradient(180deg, rgba(70, 95, 50, 0.9) 0%, rgba(50, 75, 40, 0.9) 100%);
  border-color: rgba(139, 92, 46, 0.8);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.8), 0 0 20px rgba(100, 255, 100, 0.2);
  transform: translateY(-2px);
}

.check-update-btn:disabled {
  background: linear-gradient(180deg, rgba(40, 40, 50, 0.8) 0%, rgba(30, 30, 40, 0.8) 100%);
  cursor: not-allowed;
  opacity: 0.6;
}

.install-update-btn {
  border-radius: 3px;
  border: 2px solid rgba(50, 200, 100, 0.6);
  padding: 12px 24px;
  font-size: 1em;
  font-weight: bold;
  font-family: inherit;
  color: #e0e0e0;
  background: linear-gradient(180deg, rgba(40, 140, 70, 0.9) 0%, rgba(30, 110, 55, 0.9) 100%);
  transition: all 0.3s ease;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.6), inset 0 1px 0 rgba(255, 255, 255, 0.1);
  cursor: pointer;
  white-space: nowrap;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.install-update-btn:hover:not(:disabled) {
  background: linear-gradient(180deg, rgba(50, 170, 85, 1) 0%, rgba(40, 140, 70, 1) 100%);
  border-color: rgba(50, 200, 100, 0.8);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.8), 0 0 20px rgba(50, 200, 100, 0.3);
  transform: translateY(-2px);
}

.install-update-btn:disabled {
  background: linear-gradient(180deg, rgba(40, 40, 50, 0.8) 0%, rgba(30, 30, 40, 0.8) 100%);
  cursor: not-allowed;
  opacity: 0.6;
}

@media (max-width: 600px) {
  .update-actions {
    align-items: stretch;
  }

  .check-update-btn,
  .install-update-btn {
    width: 100%;
  }
}

.analytics-options {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  cursor: pointer;
  color: #e0e0e0;
  font-size: 0.95rem;
  padding: 0.5rem;
  border-radius: 4px;
  transition: background 0.2s;
}

.radio-label:hover {
  background: rgba(240, 180, 41, 0.05);
}

.radio-label input[type="radio"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
  accent-color: #d4af37;
}

.radio-label input[type="radio"]:disabled {
  cursor: not-allowed;
  opacity: 0.5;
}

.radio-label span {
  font-weight: 500;
}
</style>
