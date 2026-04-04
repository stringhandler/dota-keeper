<script>
  import { invoke } from "@tauri-apps/api/core";
  import { getVersion } from "@tauri-apps/api/app";
  import { check } from "@tauri-apps/plugin-updater";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { onMount, onDestroy } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { trackPageView, updateAnalyticsConsent } from "$lib/analytics.js";
  import { privacyStore } from "$lib/privacy-store.svelte.js";
  import { initSentry, disableSentry, captureException } from "$lib/sentry.js";
  import { showToast } from "$lib/toast.js";
  import { _ } from "svelte-i18n";

  let databasePath = $state("");
  let logsPath = $state("");
  let isLoading = $state(true);
  let error = $state("");
  let successMessage = $state("");
  let isBackfilling = $state(false);
  let isSyncingPatches = $state(false);
  let isReparsing = $state(false);
  let isSettingDirty = $state(false);
  let isClearing = $state(false);
  let steamId = $state("");
  let suggestionDifficulty = $state("Medium");
  let customPercentage = $state(10);
  let isSavingDifficulty = $state(false);
  let analyticsConsent = $state("NotYet");
  let isSavingAnalytics = $state(false);
  let mentalHealthEnabled = $state(false);
  let checkinFrequency = $state("every_game");
  let isSavingMentalHealth = $state(false);
  let isClearingMoodData = $state(false);
  let showMentalHealthIntro = $state(false);
  let privacyMode = $state(false);
  let backgroundParseEnabled = $state(true);
  let minBenchmarkGames = $state(5);
  let bgParseActive = $state(false);
  let bgParsePending = $state(0);
  let appVersion = $state("");
  let checkingUpdate = $state(false);
  let updateResult = $state(/** @type {{ available: boolean, version?: string, update?: import('@tauri-apps/plugin-updater').Update } | null} */ (null));
  let updateError = $state("");
  let isInstalling = $state(false);

  let isResetting = $state(false);
  let dataProvider = $state("opendota");
  let stratzApiKey = $state("");
  let opendotaApiKey = $state("");
  let isSavingProvider = $state(false);

  /** @type {(() => void) | undefined} */
  let unlistenBgParse;
  /** @type {(() => void) | undefined} */
  let unlistenBackfill;

  onMount(async () => {
    await loadDatabasePath();
    await loadLogsPath();
    await loadSteamId();
    await loadDifficulty();
    await loadAnalytics();
    await loadMentalHealth();
    await loadPrivacyMode();
    await loadBackgroundParse();
    await loadDataProvider();
    await loadMinBenchmarkGames();
    await loadAppVersion();

    // Sync initial status from backend
    try {
      const status = await invoke("get_background_parse_status");
      bgParseActive = status.active;
      bgParsePending = status.pending;
    } catch (e) {
      console.error("Failed to get background parse status:", e);
    }

    // Sync backfill status (may still be running from before tab switch)
    try {
      const backfillStatus = await invoke("get_backfill_status");
      isBackfilling = backfillStatus.active;
    } catch (e) {
      console.error("Failed to get backfill status:", e);
    }

    // Listen for live progress updates
    unlistenBgParse = await listen("background-parse-progress", (event) => {
      bgParseActive = event.payload.active;
      bgParsePending = event.payload.pending;
    });

    unlistenBackfill = await listen("backfill-progress", (event) => {
      isBackfilling = event.payload.active;
      if (event.payload.done) {
        showToast(event.payload.message, "success");
      }
    });

    // Track page view
    trackPageView("Settings");
  });

  onDestroy(() => {
    unlistenBgParse?.();
    unlistenBackfill?.();
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

  async function loadLogsPath() {
    try {
      logsPath = await invoke("get_logs_folder_path");
    } catch (e) {
      console.error("Failed to get logs path:", e);
    }
  }

  async function openLogsFolder() {
    error = "";
    successMessage = "";
    try {
      await invoke("open_logs_folder");
    } catch (e) {
      error = `Failed to open logs folder: ${e}`;
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

  async function loadDataProvider() {
    try {
      const settings = await invoke("get_settings");
      dataProvider = settings.data_provider || "opendota";
      stratzApiKey = settings.stratz_api_key || "";
      opendotaApiKey = settings.opendota_api_key || "";
    } catch (e) {
      console.error("Failed to load data provider settings:", e);
    }
  }

  /** @param {string} provider */
  async function saveDataProvider(provider) {
    dataProvider = provider;
    isSavingProvider = true;
    try {
      await invoke("save_data_provider", { provider });
      showToast("Data provider updated.", "success");
    } catch (e) {
      showToast(`Failed to save provider: ${e}`, "error");
    } finally {
      isSavingProvider = false;
    }
  }

  async function saveStratzApiKey() {
    isSavingProvider = true;
    try {
      await invoke("save_stratz_api_key", { apiKey: stratzApiKey || null });
      showToast("Stratz API key saved.", "success");
    } catch (e) {
      showToast(`Failed to save API key: ${e}`, "error");
    } finally {
      isSavingProvider = false;
    }
  }

  async function clearStratzApiKey() {
    isSavingProvider = true;
    try {
      await invoke("save_stratz_api_key", { apiKey: null });
      stratzApiKey = "";
      showToast("Stratz API key cleared.", "success");
    } catch (e) {
      showToast(`Failed to clear API key: ${e}`, "error");
    } finally {
      isSavingProvider = false;
    }
  }

  async function saveOpendotaApiKey() {
    isSavingProvider = true;
    try {
      await invoke("save_opendota_api_key", { apiKey: opendotaApiKey || null });
      showToast("OpenDota API key saved.", "success");
    } catch (e) {
      showToast(`Failed to save API key: ${e}`, "error");
    } finally {
      isSavingProvider = false;
    }
  }

  async function clearOpendotaApiKey() {
    isSavingProvider = true;
    try {
      await invoke("save_opendota_api_key", { apiKey: null });
      opendotaApiKey = "";
      showToast("OpenDota API key cleared.", "success");
    } catch (e) {
      showToast(`Failed to clear API key: ${e}`, "error");
    } finally {
      isSavingProvider = false;
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

  async function loadPrivacyMode() {
    try {
      const settings = await invoke("get_settings");
      privacyMode = settings.privacy_mode ?? false;
      privacyStore.privacyMode = privacyMode;
    } catch (e) {
      console.error("Failed to load privacy mode setting:", e);
    }
  }

  /** @param {boolean} enabled */
  async function togglePrivacyMode(enabled) {
    try {
      await invoke("save_privacy_mode", { enabled });
      privacyMode = enabled;
      privacyStore.privacyMode = enabled;
    } catch (e) {
      showToast(`Failed to save privacy mode: ${e}`, "error");
    }
  }

  async function loadMinBenchmarkGames() {
    try {
      const settings = await invoke("get_settings");
      minBenchmarkGames = settings.min_benchmark_games ?? 5;
    } catch (e) {
      console.error("Failed to load min benchmark games setting:", e);
    }
  }

  /** @param {number} value */
  async function saveMinBenchmarkGames(value) {
    try {
      await invoke("save_min_benchmark_games", { value });
      minBenchmarkGames = value;
      showToast("Minimum benchmark games saved");
    } catch (e) {
      showToast(`Failed to save: ${e}`, "error");
    }
  }

  async function loadBackgroundParse() {
    try {
      const settings = await invoke("get_settings");
      backgroundParseEnabled = settings.background_parse_enabled ?? true;
    } catch (e) {
      console.error("Failed to load background parse setting:", e);
    }
  }

  /** @param {boolean} enabled */
  async function toggleBackgroundParse(enabled) {
    try {
      await invoke("save_background_parse_enabled", { enabled });
      backgroundParseEnabled = enabled;
    } catch (e) {
      error = `Failed to save background parse setting: ${e}`;
    }
  }

  async function loadMentalHealth() {
    try {
      const settings = await invoke("get_settings");
      mentalHealthEnabled = settings.mental_health_tracking_enabled ?? false;
      checkinFrequency = settings.checkin_frequency ?? "every_game";
    } catch (e) {
      console.error("Failed to load mental health setting:", e);
    }
  }

  /** @param {string} frequency */
  async function saveCheckinFrequency(frequency) {
    checkinFrequency = frequency;
    try {
      await invoke("save_checkin_frequency", { frequency });
    } catch (e) {
      console.error("Failed to save check-in frequency:", e);
    }
  }

  /** @param {boolean} enabled */
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

  /** @param {string} consent */
  async function saveAnalytics(consent) {
    error = "";
    successMessage = "";
    isSavingAnalytics = true;
    try {
      await invoke("save_analytics_consent", { consent });
      analyticsConsent = consent;
      // Update the analytics module's cached state
      updateAnalyticsConsent(consent);

      // Keep Sentry in sync with analytics consent
      if (consent === "Accepted") {
        initSentry();
      } else {
        disableSentry();
      }

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

    try {
      await invoke("backfill_historical_matches", { steamId });
      isBackfilling = true;
      showToast("Backfill started. You can navigate away — it will continue in the background.", "success");
    } catch (e) {
      error = `Failed to start backfill: ${e}`;
    }
  }

  async function syncPatches() {
    isSyncingPatches = true;
    error = "";
    try {
      const updated = await invoke("sync_patches");
      showToast(`Patches synced. ${updated} match${updated !== 1 ? 'es' : ''} updated.`, "success");
    } catch (e) {
      error = `Failed to sync patches: ${e}`;
    } finally {
      isSyncingPatches = false;
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

  async function setReparseDirty() {
    error = "";
    successMessage = "";
    isSettingDirty = true;
    try {
      const result = await invoke("set_reparse_dirty_flag");
      successMessage = result;
    } catch (e) {
      error = `Failed to set reparse flag: ${e}`;
    } finally {
      isSettingDirty = false;
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
      await updateResult.update?.downloadAndInstall();
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

  async function handleFactoryReset() {
    const confirmed = confirm(
      "FACTORY RESET\n\n" +
      "This will permanently delete ALL data:\n" +
      "• All match history\n" +
      "• All goals and progress\n" +
      "• All settings (Steam ID, preferences)\n" +
      "• All mood check-ins and challenges\n\n" +
      "The app will close immediately after. This cannot be undone.\n\n" +
      "Are you absolutely sure?"
    );
    if (!confirmed) return;
    isResetting = true;
    try {
      await invoke("factory_reset");
    } catch (e) {
      error = `Factory reset failed: ${e}`;
      isResetting = false;
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
    <h1>{$_('settings.title')}</h1>
    <p class="subtitle">{$_('settings.subtitle')}</p>
  </div>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if successMessage}
    <p class="success">{successMessage}</p>
  {/if}

  <div class="settings-section">
    <h2>{$_('settings.section_database')}</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.db_location_title')}</h3>
        <p class="setting-description">
          {$_('settings.db_location_desc')}
        </p>
        {#if isLoading}
          <p class="database-path loading">{$_('settings.db_loading')}</p>
        {:else}
          <p class="database-path">{databasePath}</p>
        {/if}
      </div>
      <button class="open-folder-btn" onclick={openDatabaseFolder} disabled={isLoading}>
        {$_('settings.open_folder')}
      </button>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.logs_title')}</h3>
        <p class="setting-description">
          {$_('settings.logs_desc')}
        </p>
        {#if logsPath}
          <p class="database-path">{logsPath}</p>
        {/if}
      </div>
      <button class="open-folder-btn" onclick={openLogsFolder}>
        {$_('settings.open_logs_folder')}
      </button>
    </div>
  </div>

  <div class="settings-section">
    <h2>{$_('settings.section_suggestions')}</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.suggestion_difficulty_title')}</h3>
        <p class="setting-description">
          {$_('settings.suggestion_difficulty_desc')}
        </p>
        <div class="difficulty-controls">
          <select class="difficulty-select" bind:value={suggestionDifficulty}>
            <option value="Easy">{$_('settings.difficulty_easy')}</option>
            <option value="Medium">{$_('settings.difficulty_medium')}</option>
            <option value="Hard">{$_('settings.difficulty_hard')}</option>
            <option value="Custom">{$_('settings.difficulty_custom')}</option>
          </select>
          {#if suggestionDifficulty === "Custom"}
            <div class="custom-pct">
              <label>
                {$_('settings.target_improvement')}
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
        {isSavingDifficulty ? $_('settings.saving') : $_('settings.save')}
      </button>
    </div>
  </div>

  <div class="settings-section">
    <h2>Analysis</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>Minimum games for benchmark rank</h3>
        <p class="setting-description">
          Minimum number of games required before showing the Last Hitting Rank for a hero.
        </p>
        <div class="difficulty-controls">
          <select class="difficulty-select" bind:value={minBenchmarkGames} onchange={() => saveMinBenchmarkGames(minBenchmarkGames)}>
            <option value={3}>3 games</option>
            <option value={5}>5 games (default)</option>
            <option value={10}>10 games</option>
            <option value={15}>15 games</option>
            <option value={20}>20 games</option>
            <option value={30}>30 games</option>
          </select>
        </div>
      </div>
    </div>
  </div>

  <div class="settings-section">
    <h2>{$_('settings.section_privacy')}</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.privacy_mode_title')}</h3>
        <p class="setting-description">
          {$_('settings.privacy_mode_desc')}
        </p>
        <div class="toggle-row">
          <button
            class="toggle-btn"
            class:active={privacyMode}
            onclick={() => togglePrivacyMode(!privacyMode)}
          >
            {privacyMode ? $_('settings.enabled') : $_('settings.disabled')}
          </button>
        </div>
      </div>
    </div>
    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.analytics_title')}</h3>
        <p class="setting-description">
          {$_('settings.analytics_desc')}
        </p>
        <p class="setting-description" style="margin-top: 0.5rem;">
          <strong>{$_('settings.analytics_collect')}</strong> {$_('settings.analytics_collect_detail')}
        </p>
        <p class="setting-description" style="margin-top: 0.5rem;">
          <strong>{$_('settings.analytics_not_collect')}</strong> {$_('settings.analytics_not_collect_detail')}
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
            <span>{$_('settings.analytics_accept')}</span>
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
            <span>{$_('settings.analytics_decline')}</span>
          </label>
        </div>
        {#if analyticsConsent === "Accepted"}
          <div style="margin-top: 0.75rem;">
            <button
              type="button"
              class="btn btn-secondary"
              onclick={() => {
                captureException(new Error("Sentry test error from Dota Keeper settings"), { source: "manual_test" });
                showToast("Test error sent to Sentry", "success");
              }}
            >
              Send test error to Sentry
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <div class="settings-section">
    <h2>{$_('settings.section_matches')}</h2>

    <div class="setting-item">
      <div class="setting-info">
        <h3>Data Provider</h3>
        <p class="setting-description">
          Choose where match data is fetched from. Stratz requires a free API key.
        </p>
        <div class="analytics-options" style="margin-top: 0.75rem;">
          <label class="radio-label">
            <input
              type="radio"
              name="data_provider"
              value="opendota"
              checked={dataProvider === "opendota"}
              onchange={() => saveDataProvider("opendota")}
              disabled={isSavingProvider}
            />
            <span>OpenDota</span>
          </label>
          <label class="radio-label">
            <input
              type="radio"
              name="data_provider"
              value="stratz"
              checked={dataProvider === "stratz"}
              onchange={() => saveDataProvider("stratz")}
              disabled={isSavingProvider}
            />
            <span>Stratz</span>
          </label>
        </div>

        {#if dataProvider === "opendota"}
          <div style="margin-top: 0.75rem;">
            <p class="setting-description" style="margin-bottom: 0.5rem;">
              Optional API key — raises the rate limit. Get one at
              <button class="link-btn" onclick={() => openUrl("https://www.opendota.com/api-keys")}>opendota.com/api-keys</button>.
            </p>
            <div class="api-key-row">
              <input
                type="password"
                placeholder="Paste API key here (optional)"
                bind:value={opendotaApiKey}
                class="text-input api-key-input"
              />
              <button
                class="save-btn"
                onclick={saveOpendotaApiKey}
                disabled={isSavingProvider}
              >
                {isSavingProvider ? "Saving…" : "Save key"}
              </button>
              {#if opendotaApiKey}
                <button
                  class="clear-key-btn"
                  onclick={clearOpendotaApiKey}
                  disabled={isSavingProvider}
                >
                  Clear
                </button>
              {/if}
            </div>
          </div>
        {/if}

        {#if dataProvider === "stratz"}
          <div style="margin-top: 0.75rem;">
            <button
              class="btn btn-secondary"
              onclick={() => openUrl("https://stratz.com/api")}
              style="font-size: 12px;"
            >
              Get API key on stratz.com →
            </button>
          </div>
          <div class="api-key-row" style="margin-top: 0.75rem;">
            <input
              type="password"
              placeholder="Paste API key here"
              bind:value={stratzApiKey}
              class="text-input api-key-input"
            />
            <button
              class="save-btn"
              onclick={saveStratzApiKey}
              disabled={isSavingProvider}
            >
              {isSavingProvider ? "Saving…" : "Save key"}
            </button>
            {#if stratzApiKey}
              <button
                class="clear-key-btn"
                onclick={clearStratzApiKey}
                disabled={isSavingProvider}
              >
                Clear
              </button>
            {/if}
          </div>
        {/if}
      </div>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.backfill_title')}</h3>
        <p class="setting-description">
          {$_('settings.backfill_desc')}
        </p>
        <p class="warning-text">
          {$_('settings.backfill_warning')}
        </p>
      </div>
      <button
        class="backfill-btn"
        onclick={backfillMatches}
        disabled={isBackfilling || !steamId}
      >
        {isBackfilling ? $_('settings.backfilling') : $_('settings.backfill_btn')}
      </button>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>Sync Patch Data</h3>
        <p class="setting-description">
          Fetch the latest Dota 2 patch versions from OpenDota and assign them to your matches. Run this after a new patch releases to keep filters up to date.
        </p>
      </div>
      <button
        class="backfill-btn"
        onclick={syncPatches}
        disabled={isSyncingPatches}
      >
        {isSyncingPatches ? 'Syncing…' : 'Sync Patches'}
      </button>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.bg_parse_title')}</h3>
        <p class="setting-description">
          {$_('settings.bg_parse_desc')}
        </p>
        {#if bgParseActive}
          <p class="bg-parse-status">{$_('settings.bg_parse_status', { values: { count: bgParsePending, plural: bgParsePending === 1 ? '' : 'es' } })}</p>
        {/if}
        <div class="toggle-row">
          <button
            class="toggle-btn"
            class:active={backgroundParseEnabled}
            onclick={() => toggleBackgroundParse(!backgroundParseEnabled)}
          >
            {backgroundParseEnabled ? $_('settings.enabled') : $_('settings.disabled')}
          </button>
        </div>
      </div>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.reparse_title')}</h3>
        <p class="setting-description">
          {$_('settings.reparse_desc')}
        </p>
        <p class="warning-text">
          {$_('settings.reparse_warning')}
        </p>
      </div>
      <button
        class="reparse-btn"
        onclick={reparsePendingMatches}
        disabled={isReparsing || !steamId}
      >
        {isReparsing ? $_('settings.reparsing') : $_('settings.reparse_btn')}
      </button>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>Reparse All Matches</h3>
        <p class="setting-description">
          Mark all matches for reparsing on next app restart. Use this if parsed data seems incomplete or after an update.
        </p>
        <p class="warning-text">
          Requires an app restart. The background parser will reparse all matches automatically.
        </p>
      </div>
      <button
        class="reparse-btn"
        onclick={setReparseDirty}
        disabled={isSettingDirty}
      >
        {isSettingDirty ? 'Setting...' : 'Reparse All on Restart'}
      </button>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.clear_matches_title')}</h3>
        <p class="setting-description">
          {$_('settings.clear_matches_desc')}
        </p>
        <p class="warning-text">
          {$_('settings.clear_matches_warning')}
        </p>
      </div>
      <button
        class="clear-btn"
        onclick={clearAllMatches}
        disabled={isClearing}
      >
        {isClearing ? $_('settings.clearing') : $_('settings.clear_matches_btn')}
      </button>
    </div>
  </div>

  <div class="settings-section updates-section">
    <h2>{$_('settings.section_updates')}</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.updates_title')}</h3>
        <p class="setting-description">
          {$_('settings.updates_desc')}
        </p>
        {#if appVersion}
          <p class="version-label">{$_('settings.current_version')} <span class="version-value">v{appVersion}</span></p>
        {/if}
        {#if updateError}
          <p class="update-error">{updateError}</p>
        {:else if updateResult !== null}
          {#if updateResult.available}
            <p class="update-available">{$_('settings.update_available', { values: { version: updateResult.version } })}</p>
          {:else}
            <p class="update-current">{$_('settings.up_to_date', { values: { version: appVersion } })}</p>
          {/if}
        {/if}
      </div>
      <div class="update-actions">
        <button
          class="check-update-btn"
          onclick={checkForUpdates}
          disabled={checkingUpdate || isInstalling}
        >
          {checkingUpdate ? $_('settings.checking') : $_('settings.check_updates')}
        </button>
        {#if updateResult?.available}
          <button
            class="install-update-btn"
            onclick={installUpdate}
            disabled={isInstalling}
          >
            {isInstalling ? $_('settings.installing') : $_('settings.install_restart')}
          </button>
        {/if}
      </div>
    </div>
  </div>

  <div class="settings-section">
    <h2>{$_('settings.section_mental')}</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.mental_checkin_title')}</h3>
        <p class="setting-description">
          {$_('settings.mental_checkin_desc')}
        </p>
        <p class="setting-description">
          {$_('settings.mental_checkin_local')}
        </p>
        <div class="toggle-row">
          <button
            class="toggle-btn"
            class:active={mentalHealthEnabled}
            onclick={() => toggleMentalHealth(!mentalHealthEnabled)}
            disabled={isSavingMentalHealth}
          >
            {mentalHealthEnabled ? $_('settings.on') : $_('settings.off')}
          </button>
          <span class="toggle-label">{mentalHealthEnabled ? $_('settings.checkins_enabled') : $_('settings.checkins_disabled')}</span>
        </div>
      </div>
    </div>

    {#if mentalHealthEnabled}
      <div class="setting-item">
        <div class="setting-info">
          <h3>{$_('settings.frequency_title')}</h3>
          <p class="setting-description">
            {$_('settings.frequency_desc')}
          </p>
          <select
            class="frequency-select"
            value={checkinFrequency}
            onchange={(e) => saveCheckinFrequency(/** @type {HTMLSelectElement} */ (e.target).value)}
          >
            <option value="every_game">{$_('settings.freq_every')}</option>
            <option value="every_3">{$_('settings.freq_every_3')}</option>
            <option value="every_5">{$_('settings.freq_every_5')}</option>
            <option value="every_10">{$_('settings.freq_every_10')}</option>
            <option value="once_per_session">{$_('settings.freq_once_session')}</option>
            <option value="after_loss">{$_('settings.freq_after_loss')}</option>
          </select>
        </div>
      </div>
    {/if}

    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.clear_mood_title')}</h3>
        <p class="setting-description">
          {$_('settings.clear_mood_desc')}
        </p>
      </div>
      <button
        class="clear-btn"
        onclick={clearMoodData}
        disabled={isClearingMoodData}
      >
        {isClearingMoodData ? $_('settings.clearing') : $_('settings.clear_mood_btn')}
      </button>
    </div>
  </div>

  <div class="settings-section">
    <h2>{$_('settings.section_account')}</h2>
    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.logout_title')}</h3>
        <p class="setting-description">
          {$_('settings.logout_desc')}
        </p>
      </div>
      <button
        class="logout-btn-destructive"
        onclick={handleLogout}
      >
        {$_('settings.logout_btn')}
      </button>
    </div>

    <div class="setting-item">
      <div class="setting-info">
        <h3>{$_('settings.factory_reset_title')}</h3>
        <p class="setting-description">
          {$_('settings.factory_reset_desc')}
        </p>
        <p class="warning-text">
          {$_('settings.factory_reset_warning')}
        </p>
      </div>
      <button
        class="factory-reset-btn"
        onclick={handleFactoryReset}
        disabled={isResetting}
      >
        {isResetting ? $_('settings.resetting') : $_('settings.factory_reset_btn')}
      </button>
    </div>
  </div>
</div>

{#if showMentalHealthIntro}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={dismissMentalHealthIntro}>
    <div class="modal-card" onclick={(e) => e.stopPropagation()}>
      <h3 class="modal-title">{$_('settings.modal_checkin_title')}</h3>
      <p class="modal-body">
        {$_('settings.modal_checkin_body')}
      </p>
      <ul class="modal-list">
        <li>{$_('settings.modal_checkin_li1')}</li>
        <li>{$_('settings.modal_checkin_li2')}</li>
        <li>{$_('settings.modal_checkin_li3')}</li>
      </ul>
      <button class="modal-confirm-btn" onclick={dismissMentalHealthIntro}>
        {$_('settings.modal_checkin_confirm')}
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
  font-size: 12px;
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
  font-size: 14px;
}

.success {
  color: var(--green);
  background: rgba(74, 222, 128, 0.1);
  border: 1px solid rgba(74, 222, 128, 0.25);
  border-radius: 4px;
  padding: 10px 14px;
  margin-bottom: 16px;
  font-size: 14px;
}

.settings-section {
  margin-bottom: 24px;
}

.settings-section h2 {
  font-family: 'Barlow Condensed', sans-serif;
  font-size: 12px;
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
  font-size: 14px;
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
  font-size: 12px;
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
  font-size: 14px;
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
  font-size: 14px;
}

.custom-pct label {
  display: flex;
  align-items: center;
  gap: 8px;
}

.text-input {
  background: var(--bg-elevated);
  color: var(--text-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 6px 8px;
  font-family: inherit;
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
}

.text-input:focus {
  border-color: var(--border-active);
}

.pct-input {
  width: 64px;
  background: var(--bg-elevated);
  color: var(--text-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 6px 8px;
  font-family: inherit;
  font-size: 14px;
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

.clear-key-btn {
  font-family: 'Barlow Condensed', sans-serif;
  font-weight: 600;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  font-size: 12px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  padding: 8px 12px;
  white-space: nowrap;
  flex-shrink: 0;
  background: transparent;
  color: var(--text-muted);
  border: 1px solid rgba(248, 113, 113, 0.4);
  color: var(--red);
}

.clear-key-btn:hover:not(:disabled) {
  border-color: var(--red);
  background: rgba(248, 113, 113, 0.1);
}

.clear-key-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.link-btn {
  background: none;
  border: none;
  padding: 0;
  color: var(--teal);
  font-size: inherit;
  font-family: inherit;
  cursor: pointer;
  text-decoration: underline;
  text-underline-offset: 2px;
}

.link-btn:hover {
  color: var(--text-primary);
}

.api-key-row {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.api-key-input {
  flex: 1;
  min-width: 0;
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

  .api-key-row {
    flex-wrap: wrap;
  }

  .api-key-row .save-btn,
  .api-key-row .clear-key-btn {
    flex: 1;
  }

  .btn-secondary {
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
  font-size: 14px;
  font-weight: 600;
}

.update-current {
  margin: 10px 0 0 0;
  color: var(--green);
  font-size: 14px;
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

  .updates-section {
    display: none;
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
  font-size: 14px;
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

.factory-reset-btn {
  font-family: 'Barlow Condensed', sans-serif;
  font-weight: 600;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  font-size: 12px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  padding: 8px 16px;
  white-space: nowrap;
  flex-shrink: 0;
  background: rgba(248, 113, 113, 0.15);
  color: var(--red);
  border: 1px solid rgba(248, 113, 113, 0.6);
}

.factory-reset-btn:hover:not(:disabled) {
  background: rgba(248, 113, 113, 0.25);
  border-color: var(--red);
}

.factory-reset-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.logout-btn-destructive {
  font-family: 'Barlow Condensed', sans-serif;
  font-weight: 600;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  font-size: 12px;
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

.bg-parse-status {
  font-family: 'Barlow Condensed', sans-serif;
  font-size: 12px;
  letter-spacing: 1px;
  color: var(--text-muted);
  margin: 6px 0 4px;
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
  font-size: 12px;
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
  font-size: 14px;
  color: var(--text-muted);
}

.frequency-select {
  margin-top: 12px;
  padding: 8px 12px;
  background: var(--surface);
  color: var(--text);
  border: 1px solid var(--border);
  border-radius: 4px;
  font-family: 'Barlow Condensed', sans-serif;
  font-size: 14px;
  cursor: pointer;
  min-width: 200px;
}

.frequency-select:focus {
  outline: none;
  border-color: rgba(74, 222, 128, 0.4);
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
  font-size: 24px;
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
  font-size: 14px;
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
