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
  let mentalHealthEnabled = $state(false);
  let isSavingMentalHealth = $state(false);
  let isClearingMoodData = $state(false);
  let showMentalHealthIntro = $state(false);
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
    await loadMentalHealth();
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

  async function loadMentalHealth() {
    try {
      const settings = await invoke("get_settings");
      mentalHealthEnabled = settings.mental_health_tracking_enabled ?? false;
    } catch (e) {
      console.error("Failed to load mental health setting:", e);
    }
  }

  async function toggleMentalHealth(enabled) {
    isSavingMentalHealth = true;
    error = "";
    successMessage = "";
    try {
      await invoke("save_mental_health_enabled", { enabled });
      mentalHealthEnabled = enabled;
      if (enabled) {
        const settings = await invoke("get_settings");
        if (!settings.mental_health_intro_shown) {
          showMentalHealthIntro = true;
        }
      }
    } catch (e) {
      error = `Failed to save mental health setting: ${e}`;
    } finally {
      isSavingMentalHealth = false;
    }
  }

  async function dismissMentalHealthIntro() {
    showMentalHealthIntro = false;
    try {
      await invoke("mark_mental_health_intro_shown");
    } catch (e) {
      console.error("Failed to mark intro shown:", e);
    }
  }

  async function clearMoodData() {
    const confirmed = confirm(
      "This will permanently delete all mood check-in history.\n\nYour match data is not affected."
    );
    if (!confirmed) return;
    isClearingMoodData = true;
    error = "";
    successMessage = "";
    try {
      await invoke("clear_mood_data");
      successMessage = "All mood check-in data has been cleared.";
    } catch (e) {
      error = `Failed to clear mood data: ${e}`;
    } finally {
      isClearingMoodData = false;
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

  async function handleLogout() {
    const confirmed = confirm(
      "Are you sure you want to log out?\n\n" +
      "Your match data will remain stored locally."
    );

    if (!confirmed) {
      return;
    }

    error = "";
    successMessage = "";

    try {
      await invoke("logout");
      // Reload the page to return to login screen
      window.location.reload();
    } catch (e) {
      error = `Failed to logout: ${e}`;
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

  <div class="settings-section">
    <h2>Mental Wellbeing</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>Post-game mood check-ins</h3>
        <p class="setting-description">
          Get occasional prompts to reflect on how you felt during a session. Helps you spot patterns like fatigue or frustration before they affect your gameplay.
        </p>
        <p class="setting-description">
          All data is stored locally on your device only. You can skip any check-in, and disable this at any time.
        </p>
        <div class="toggle-row">
          <button
            class="toggle-btn"
            class:active={mentalHealthEnabled}
            onclick={() => toggleMentalHealth(!mentalHealthEnabled)}
            disabled={isSavingMentalHealth}
          >
            {mentalHealthEnabled ? "On" : "Off"}
          </button>
          <span class="toggle-label">{mentalHealthEnabled ? "Check-ins enabled" : "Check-ins disabled"}</span>
        </div>
      </div>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>Clear all mood data</h3>
        <p class="setting-description">
          Permanently delete all mood check-in history. Your match data and goals are not affected.
        </p>
      </div>
      <button
        class="clear-btn"
        onclick={clearMoodData}
        disabled={isClearingMoodData}
      >
        {isClearingMoodData ? "Clearing..." : "Clear Mood Data"}
      </button>
    </div>
  </div>

  <div class="settings-section">
    <h2>Account</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>Log Out</h3>
        <p class="setting-description">
          Log out from your Steam account. Your match data and goals will remain stored locally and will be available when you log back in.
        </p>
      </div>
      <button
        class="logout-btn-destructive"
        onclick={handleLogout}
      >
        Log Out
      </button>
    </div>
  </div>
</div>

{#if showMentalHealthIntro}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={dismissMentalHealthIntro}>
    <div class="modal-card" onclick={(e) => e.stopPropagation()}>
      <h3 class="modal-title">About mood check-ins</h3>
      <p class="modal-body">
        Occasionally, after a session, we'll ask how you felt — using simple emoji scales. This helps you spot patterns like fatigue or frustration before they affect your gameplay.
      </p>
      <ul class="modal-list">
        <li>All data stays on your device</li>
        <li>You can skip any check-in</li>
        <li>Disable tracking at any time from this page</li>
      </ul>
      <button class="modal-confirm-btn" onclick={dismissMentalHealthIntro}>
        Got it, let's go
      </button>
    </div>
  </div>
{/if}

<style>
.settings-content {
  max-width: 1000px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 28px;
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

.success {
  color: var(--green);
  background: rgba(74, 222, 128, 0.1);
  border: 1px solid rgba(74, 222, 128, 0.25);
  border-radius: 4px;
  padding: 10px 14px;
  margin-bottom: 16px;
  font-size: 13px;
}

.settings-section {
  margin-bottom: 24px;
}

.settings-section h2 {
  font-family: 'Barlow Condensed', sans-serif;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 3px;
  color: var(--text-muted);
  text-transform: uppercase;
  margin-bottom: 12px;
}

.setting-item {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 18px 20px;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 20px;
  margin-bottom: 12px;
  transition: border-color 0.2s;
}

.setting-item:hover {
  border-color: var(--border-active);
}

.setting-info {
  flex: 1;
}

.setting-info h3 {
  font-family: 'Rajdhani', sans-serif;
  font-size: 16px;
  font-weight: 700;
  letter-spacing: 1px;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.setting-description {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin: 0 0 8px 0;
}

.database-path {
  font-family: 'Courier New', monospace;
  background: var(--bg-elevated);
  color: var(--teal);
  padding: 10px 12px;
  border-radius: 4px;
  border: 1px solid var(--border);
  word-break: break-all;
  font-size: 12px;
  margin: 0;
}

.database-path.loading {
  color: var(--text-muted);
  font-style: italic;
}

.open-folder-btn,
.save-btn,
.backfill-btn,
.reparse-btn,
.clear-btn,
.check-update-btn,
.install-update-btn {
  font-family: 'Barlow Condensed', sans-serif;
  font-weight: 600;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  font-size: 11px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  padding: 8px 16px;
  white-space: nowrap;
  flex-shrink: 0;
}

.open-folder-btn,
.save-btn,
.backfill-btn,
.reparse-btn,
.check-update-btn {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border);
}

.open-folder-btn:hover:not(:disabled),
.save-btn:hover:not(:disabled),
.backfill-btn:hover:not(:disabled),
.reparse-btn:hover:not(:disabled),
.check-update-btn:hover:not(:disabled) {
  border-color: var(--border-active);
  color: var(--text-primary);
}

.install-update-btn {
  background: var(--green);
  color: #080c10;
  border: none;
}

.install-update-btn:hover:not(:disabled) {
  background: #4ade80;
  transform: translateY(-1px);
}

.open-folder-btn:disabled,
.save-btn:disabled,
.backfill-btn:disabled,
.reparse-btn:disabled,
.clear-btn:disabled,
.check-update-btn:disabled,
.install-update-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.warning-text {
  margin: 8px 0 0 0;
  color: var(--gold);
  font-size: 12px;
  font-weight: 600;
}

.difficulty-controls {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-top: 10px;
}

.difficulty-select {
  background: var(--bg-elevated);
  color: var(--text-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 8px 12px;
  font-family: inherit;
  font-size: 13px;
  cursor: pointer;
  max-width: 280px;
  outline: none;
  transition: border-color 0.2s;
}

.difficulty-select:focus {
  border-color: var(--border-active);
}

.custom-pct {
  color: var(--text-secondary);
  font-size: 13px;
}

.custom-pct label {
  display: flex;
  align-items: center;
  gap: 8px;
}

.pct-input {
  width: 64px;
  background: var(--bg-elevated);
  color: var(--text-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 6px 8px;
  font-family: inherit;
  font-size: 13px;
  text-align: center;
  outline: none;
  transition: border-color 0.2s;
}

.pct-input:focus {
  border-color: var(--border-active);
}

.clear-btn {
  background: transparent;
  color: var(--red);
  border: 1px solid rgba(248, 113, 113, 0.5);
}

.clear-btn:hover:not(:disabled) {
  border-color: var(--red);
  background: rgba(248, 113, 113, 0.1);
}

@media (max-width: 600px) {
  .setting-item {
    flex-direction: column;
    align-items: stretch;
  }

  .open-folder-btn,
  .save-btn,
  .backfill-btn,
  .reparse-btn,
  .clear-btn,
  .check-update-btn,
  .install-update-btn {
    width: 100%;
  }
}

.version-label {
  margin: 8px 0 0 0;
  color: var(--text-muted);
  font-size: 12px;
}

.version-value {
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
}

.update-error {
  margin: 10px 0 0 0;
  color: var(--red);
  background: rgba(248, 113, 113, 0.1);
  border: 1px solid rgba(248, 113, 113, 0.25);
  border-radius: 4px;
  padding: 8px 12px;
  font-size: 12px;
}

.update-available {
  margin: 10px 0 0 0;
  color: var(--gold);
  font-size: 13px;
  font-weight: 600;
}

.update-current {
  margin: 10px 0 0 0;
  color: var(--green);
  font-size: 13px;
  font-weight: 600;
}

.update-actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: flex-end;
}

@media (max-width: 600px) {
  .update-actions {
    align-items: stretch;
  }
}

.analytics-options {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-top: 12px;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  color: var(--text-primary);
  font-size: 13px;
  padding: 8px;
  border-radius: 4px;
  transition: background 0.2s;
}

.radio-label:hover {
  background: rgba(255, 200, 80, 0.05);
}

.radio-label input[type="radio"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
  accent-color: var(--gold);
}

.radio-label input[type="radio"]:disabled {
  cursor: not-allowed;
  opacity: 0.5;
}

.radio-label span {
  font-weight: 500;
}

.logout-btn-destructive {
  font-family: 'Barlow Condensed', sans-serif;
  font-weight: 600;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  font-size: 11px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  padding: 8px 16px;
  white-space: nowrap;
  flex-shrink: 0;
  background: transparent;
  color: var(--red);
  border: 1px solid rgba(248, 113, 113, 0.5);
}

.logout-btn-destructive:hover {
  border-color: var(--red);
  background: rgba(248, 113, 113, 0.1);
  color: var(--red);
}

/* Mental health toggle */
.toggle-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 12px;
}

.toggle-btn {
  font-family: 'Barlow Condensed', sans-serif;
  font-weight: 600;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  font-size: 11px;
  border-radius: 4px;
  cursor: pointer;
  padding: 6px 18px;
  transition: all 0.2s;
  background: transparent;
  color: var(--text-muted);
  border: 1px solid var(--border);
  min-width: 56px;
}

.toggle-btn.active {
  background: rgba(74, 222, 128, 0.1);
  color: var(--green);
  border-color: rgba(74, 222, 128, 0.4);
}

.toggle-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toggle-label {
  font-size: 13px;
  color: var(--text-muted);
}

/* First-enable modal */
.modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-card {
  background: var(--bg-card);
  border: 1px solid var(--border-active);
  border-radius: 10px;
  padding: 28px 32px;
  max-width: 420px;
  width: 90%;
}

.modal-title {
  font-family: 'Rajdhani', sans-serif;
  font-size: 20px;
  font-weight: 700;
  letter-spacing: 1px;
  color: var(--text-primary);
  margin: 0 0 14px 0;
}

.modal-body {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin: 0 0 12px 0;
}

.modal-list {
  margin: 0 0 20px 0;
  padding-left: 18px;
  color: var(--text-muted);
  font-size: 13px;
  line-height: 1.8;
}

.modal-confirm-btn {
  font-family: 'Barlow Condensed', sans-serif;
  font-weight: 600;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  font-size: 12px;
  background: var(--gold);
  color: #080c10;
  border: none;
  border-radius: 4px;
  padding: 10px 24px;
  cursor: pointer;
  transition: all 0.2s;
  width: 100%;
}

.modal-confirm-btn:hover {
  background: #ffdf80;
}
</style>
