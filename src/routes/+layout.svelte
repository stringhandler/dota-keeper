<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { check } from '@tauri-apps/plugin-updater';
  import { relaunch } from '@tauri-apps/plugin-process';
  import { getAnalyticsConsent, identifyUser } from "$lib/analytics.js";
  import AnalyticsConsentModal from "$lib/AnalyticsConsentModal.svelte";
  import TitleBar from "$lib/TitleBar.svelte";
  import WindowResize from "$lib/WindowResize.svelte";
  import '../app.css';

  let isLoading = $state(true);
  let isLoggedIn = $state(false);
  let currentSteamId = $state("");
  let steamId = $state("");
  let error = $state("");
  let updateAvailable = $state(false);
  let updateVersion = $state("");
  let isUpdating = $state(false);
  let dailyProgress = $state(null);
  let showConsentModal = $state(false);

  onMount(async () => {
    await loadSettings();
    await checkForUpdates();

    // Check analytics consent on every startup
    const consent = await getAnalyticsConsent();
    if (consent === "NotYet") {
      showConsentModal = true;
    } else if (consent === "Accepted") {
      // Identify user for analytics once on app start
      await identifyUser();
    }
  });

  async function loadSettings() {
    try {
      const settings = await invoke("get_settings");
      if (settings.steam_id) {
        isLoggedIn = true;
        currentSteamId = settings.steam_id;
        await loadDailyChallenge();
      }
    } catch (e) {
      error = `Failed to load settings: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  async function loadDailyChallenge() {
    try {
      dailyProgress = await invoke("get_daily_challenge_progress_cmd");
    } catch (e) {
      // non-fatal
    }
  }

  function extractSteamId(input) {
    const trimmed = input.trim();
    const profileMatch = trimmed.match(/steamcommunity\.com\/profiles\/(\d+)/);
    if (profileMatch) return profileMatch[1];
    return trimmed;
  }

  async function handleLogin(event) {
    event.preventDefault();
    error = "";
    if (!steamId.trim()) { error = "Please enter your Steam ID"; return; }
    const extractedId = extractSteamId(steamId);
    try {
      const settings = await invoke("save_steam_id", { steamId: extractedId });
      isLoggedIn = true;
      currentSteamId = settings.steam_id;
      await loadDailyChallenge();
    } catch (e) {
      error = `Failed to save Steam ID: ${e}`;
    }
  }

  async function handleLogout() {
    try {
      await invoke("logout");
      isLoggedIn = false;
      currentSteamId = "";
      steamId = "";
    } catch (e) {
      error = `Failed to logout: ${e}`;
    }
  }

  function isActive(path) {
    return $page.url.pathname === path;
  }

  function isActivePath(prefix) {
    return $page.url.pathname.startsWith(prefix);
  }

  function getPageTitle(pathname) {
    if (pathname === '/') return 'Dashboard';
    if (pathname.startsWith('/matches')) return 'Match History';
    if (pathname.startsWith('/analysis')) return 'Performance Analysis';
    if (pathname.startsWith('/goals')) return 'Goal Management';
    if (pathname.startsWith('/challenges')) return 'Challenges';
    if (pathname.startsWith('/settings')) return 'Settings';
    return 'Dota Keeper';
  }

  async function checkForUpdates() {
    try {
      const update = await check();
      if (update?.available) {
        updateAvailable = true;
        updateVersion = update.version;
      }
    } catch (e) {
      console.error('Failed to check for updates:', e);
    }
  }

  async function installUpdate() {
    if (!updateAvailable) return;
    isUpdating = true;
    try {
      const update = await check();
      if (update?.available) {
        await update.downloadAndInstall();
        await relaunch();
      }
    } catch (e) {
      error = `Failed to install update: ${e}`;
      isUpdating = false;
    }
  }
</script>

<WindowResize />
<TitleBar />

{#if isLoading}
  <div class="loading-screen">
    <p>Loading...</p>
  </div>
{:else if !isLoggedIn}
  <div class="login-screen">
    <div class="login-box">
      <div class="login-brand">Dota Keeper</div>
      <p class="login-sub">Track your Dota 2 progress</p>

      <form class="login-form" onsubmit={handleLogin}>
        <label class="form-label" for="steam-id">Steam ID</label>
        <input
          class="form-input"
          id="steam-id"
          type="text"
          placeholder="Steam ID or profile URL"
          bind:value={steamId}
        />
        <p class="login-hint">Your Steam ID or steamcommunity.com/profiles/... URL</p>
        <button type="submit" class="btn btn-primary" style="width:100%;justify-content:center;padding:12px;">
          Save & Continue
        </button>
      </form>

      {#if error}
        <div class="error-banner" style="margin-top:12px">{error}</div>
      {/if}
    </div>
  </div>
{:else}
  <div class="app-layout">
    {#if updateAvailable}
      <div class="update-banner">
        <div class="update-content">
          <span class="update-icon">🔄</span>
          <div class="update-text">
            <strong>Update Available!</strong>
            <span>Version {updateVersion} is ready to install</span>
          </div>
          <button class="btn btn-primary" onclick={installUpdate} disabled={isUpdating}>
            {isUpdating ? 'Installing...' : 'Install & Restart'}
          </button>
        </div>
      </div>
    {/if}

    <!-- SIDEBAR -->
    <aside class="sidebar">
      <div class="brand">
        <div class="brand-name">Dota Keeper</div>
        <div class="brand-id">Steam ID</div>
        <div class="steam-badge">{currentSteamId}</div>
      </div>

      <nav class="nav">
        <a href="/" class="nav-item" class:active={isActive('/')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
          Dashboard
        </a>
        <a href="/matches" class="nav-item" class:active={isActivePath('/matches')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
          Matches
        </a>
        <a href="/analysis" class="nav-item" class:active={isActivePath('/analysis')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
          </svg>
          Analysis
        </a>
        <a href="/goals" class="nav-item" class:active={isActivePath('/goals')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
          </svg>
          Goals
        </a>
        <a href="/challenges" class="nav-item" class:active={isActivePath('/challenges')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z" />
          </svg>
          Challenges
        </a>
        <a href="/settings" class="nav-item" class:active={isActive('/settings')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          Settings
        </a>
      </nav>

      <div class="sidebar-footer">
        <div class="rank-pill">
          <div class="rank-star">★</div>
          <div>
            <div class="rank-text">Rank</div>
            <div class="rank-value">N/A</div>
          </div>
        </div>
      </div>
    </aside>

    <!-- MAIN -->
    <div class="main">
      <!-- TOP BAR -->
      <div class="topbar">
        <div class="page-title">{getPageTitle($page.url.pathname)}</div>
        <div class="topbar-actions">
          {#if dailyProgress}
            <button class="challenge-badge" onclick={() => goto('/challenges')}>
              ⚡ Daily Challenge: {dailyProgress.current_value}/{dailyProgress.target}
            </button>
          {:else}
            <button class="challenge-badge" onclick={() => goto('/challenges')}>
              ⚡ Challenges
            </button>
          {/if}
          <a href="/goals" class="btn btn-primary">+ New Goal</a>
        </div>
      </div>

      <!-- CONTENT -->
      <div class="content-area">
        <slot />
      </div>
    </div>
  </div>
{/if}

<!-- Analytics Consent Modal -->
{#if showConsentModal}
  <AnalyticsConsentModal onClose={() => showConsentModal = false} />
{/if}

<style>
  /* ── LOADING / LOGIN ── */
  .loading-screen {
    display: flex;
    justify-content: center;
    align-items: center;
    flex: 1;
    background: var(--bg-base);
    color: var(--gold);
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 12px;
    letter-spacing: 3px;
    text-transform: uppercase;
  }

  .login-screen {
    display: flex;
    justify-content: center;
    align-items: center;
    flex: 1;
    background: var(--bg-base);
  }

  .login-box {
    width: 420px;
    padding: 40px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
  }

  .login-brand {
    font-family: 'Rajdhani', sans-serif;
    font-size: 28px;
    font-weight: 700;
    letter-spacing: 4px;
    color: var(--gold);
    text-transform: uppercase;
    margin-bottom: 6px;
  }

  .login-sub {
    font-size: 13px;
    color: var(--text-secondary);
    margin-bottom: 28px;
  }

  .login-form {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .login-hint {
    font-size: 11px;
    color: var(--text-muted);
    margin-bottom: 8px;
  }

  /* ── APP LAYOUT ── */
  .app-layout {
    display: flex;
    flex: 1;
    min-height: 0;
    overflow: hidden;
    background: var(--bg-base);
  }

  /* ── UPDATE BANNER ── */
  .update-banner {
    position: fixed;
    top: 32px; left: 0; right: 0;
    background: linear-gradient(90deg, rgba(30, 80, 140, 0.97) 0%, rgba(20, 60, 110, 0.97) 100%);
    border-bottom: 1px solid rgba(100, 150, 220, 0.4);
    z-index: 1000;
    padding: 10px 20px;
  }

  .update-content {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .update-icon {
    font-size: 1.1rem;
    animation: rotate 2s linear infinite;
  }

  @keyframes rotate {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .update-text {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    font-size: 12px;
  }

  .update-text strong { color: #fff; }
  .update-text span { color: #c0c0c0; }

  /* ── SIDEBAR ── */
  .sidebar {
    width: var(--sidebar-w);
    background: var(--bg-card);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    position: relative;
    z-index: 10;
    flex-shrink: 0;
  }

  .sidebar::after {
    content: '';
    position: absolute;
    top: 0; right: -1px;
    width: 1px; height: 100%;
    background: linear-gradient(to bottom, transparent, var(--gold), transparent);
    opacity: 0.3;
  }

  .brand {
    padding: 24px 20px 20px;
    border-bottom: 1px solid var(--border);
  }

  .brand-name {
    font-family: 'Rajdhani', sans-serif;
    font-size: 17px;
    font-weight: 700;
    letter-spacing: 3px;
    color: var(--gold);
    text-transform: uppercase;
  }

  .brand-id {
    font-size: 10px;
    color: var(--text-muted);
    letter-spacing: 1px;
    margin-top: 6px;
    font-family: 'Barlow Condensed', sans-serif;
    text-transform: uppercase;
  }

  .steam-badge {
    margin-top: 6px;
    background: rgba(240, 180, 41, 0.06);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 5px 8px;
    font-size: 10px;
    color: var(--gold-dim);
    letter-spacing: 0.5px;
    font-family: monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* ── NAV ── */
  .nav {
    flex: 1;
    padding: 14px 0;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 11px 20px;
    cursor: pointer;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 2px;
    color: var(--text-secondary);
    text-transform: uppercase;
    position: relative;
    transition: all 0.2s;
    user-select: none;
    text-decoration: none;
  }

  .nav-item::before {
    content: '';
    position: absolute;
    left: 0; top: 0; bottom: 0;
    width: 2px;
    background: var(--gold);
    transform: scaleY(0);
    transition: transform 0.2s;
  }

  .nav-item.active {
    color: var(--gold);
    background: rgba(240, 180, 41, 0.06);
  }

  .nav-item.active::before { transform: scaleY(1); }

  .nav-item:hover:not(.active) {
    color: var(--text-primary);
    background: rgba(255, 255, 255, 0.03);
  }

  .nav-icon {
    width: 17px;
    height: 17px;
    opacity: 0.7;
    flex-shrink: 0;
  }

  .nav-item.active .nav-icon { opacity: 1; }

  /* ── SIDEBAR FOOTER ── */
  .sidebar-footer {
    padding: 14px 16px;
    border-top: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .rank-pill {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 10px 12px;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .rank-star {
    color: var(--gold);
    font-size: 16px;
    flex-shrink: 0;
  }

  .rank-text {
    font-size: 9px;
    color: var(--text-muted);
    font-family: 'Barlow Condensed', sans-serif;
    letter-spacing: 1.5px;
    text-transform: uppercase;
  }

  .rank-value {
    font-family: 'Rajdhani', sans-serif;
    font-weight: 700;
    color: var(--text-primary);
    font-size: 14px;
  }

  /* ── MAIN AREA ── */
  .main {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  /* ── TOP BAR ── */
  .topbar {
    padding: 0 32px;
    height: 60px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--bg-card);
    flex-shrink: 0;
  }

  .page-title {
    font-family: 'Rajdhani', sans-serif;
    font-size: 18px;
    font-weight: 700;
    letter-spacing: 3px;
    color: var(--text-primary);
    text-transform: uppercase;
  }

  .topbar-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .challenge-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    background: rgba(240, 180, 41, 0.08);
    border: 1px solid rgba(240, 180, 41, 0.25);
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 11px;
    font-family: 'Barlow Condensed', sans-serif;
    font-weight: 600;
    letter-spacing: 1px;
    color: var(--gold);
    cursor: pointer;
    transition: all 0.2s;
  }

  .challenge-badge:hover { background: rgba(240, 180, 41, 0.14); }

  /* ── CONTENT AREA ── */
  .content-area {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 28px 32px;
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
  }
</style>
