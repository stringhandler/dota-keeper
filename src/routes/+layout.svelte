<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { check } from '@tauri-apps/plugin-updater';
  import { relaunch } from '@tauri-apps/plugin-process';
  import '../app.css';

  let isLoading = $state(true);
  let isLoggedIn = $state(false);
  let currentSteamId = $state("");
  let steamId = $state("");
  let error = $state("");
  let updateAvailable = $state(false);
  let updateVersion = $state("");
  let isUpdating = $state(false);

  onMount(async () => {
    await loadSettings();
    await checkForUpdates();
  });

  async function loadSettings() {
    try {
      const settings = await invoke("get_settings");
      if (settings.steam_id) {
        isLoggedIn = true;
        currentSteamId = settings.steam_id;
      }
    } catch (e) {
      error = `Failed to load settings: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  function extractSteamId(input) {
    const trimmed = input.trim();
    const profileMatch = trimmed.match(/steamcommunity\.com\/profiles\/(\d+)/);
    if (profileMatch) {
      return profileMatch[1];
    }
    return trimmed;
  }

  async function handleLogin(event) {
    event.preventDefault();
    error = "";

    if (!steamId.trim()) {
      error = "Please enter your Steam ID";
      return;
    }

    const extractedId = extractSteamId(steamId);

    try {
      const settings = await invoke("save_steam_id", { steamId: extractedId });
      isLoggedIn = true;
      currentSteamId = settings.steam_id;
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

  async function checkForUpdates() {
    try {
      const update = await check();
      if (update?.available) {
        updateAvailable = true;
        updateVersion = update.version;
        console.log(`Update available: ${update.version}`);
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

{#if isLoading}
  <div class="loading-screen">
    <p>Loading...</p>
  </div>
{:else if !isLoggedIn}
  <div class="login-screen">
    <h1>Dota Keeper</h1>
    <p class="subtitle">Track your Dota 2 progress</p>

    <form class="login-form" onsubmit={handleLogin}>
      <label for="steam-id">Enter your Steam ID</label>
      <input
        id="steam-id"
        type="text"
        placeholder="Steam ID or profile URL"
        bind:value={steamId}
      />
      <p class="hint">Your Steam ID or profile URL (e.g., steamcommunity.com/profiles/...)</p>
      <button type="submit">Save & Continue</button>
    </form>

    {#if error}
      <p class="error">{error}</p>
    {/if}
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
          <button class="update-btn" onclick={installUpdate} disabled={isUpdating}>
            {isUpdating ? 'Installing...' : 'Install & Restart'}
          </button>
        </div>
      </div>
    {/if}
    <aside class="sidebar">
      <div class="sidebar-header">
        <h1>Dota Keeper</h1>
        <p class="steam-info">
          <span class="label">Steam ID</span>
          <code>{currentSteamId}</code>
        </p>
      </div>

      <nav class="sidebar-nav">
        <a href="/" class="nav-item" class:active={isActive('/')}>
          <span class="nav-icon">📊</span>
          <span class="nav-label">Dashboard</span>
        </a>
        <a href="/matches" class="nav-item" class:active={isActive('/matches')}>
          <span class="nav-icon">⚔️</span>
          <span class="nav-label">Matches</span>
        </a>
        <a href="/analysis" class="nav-item" class:active={isActive('/analysis')}>
          <span class="nav-icon">📈</span>
          <span class="nav-label">Analysis</span>
        </a>
        <a href="/goals" class="nav-item" class:active={isActive('/goals')}>
          <span class="nav-icon">🎯</span>
          <span class="nav-label">Goals</span>
        </a>
        <a href="/settings" class="nav-item" class:active={isActive('/settings')}>
          <span class="nav-icon">⚙️</span>
          <span class="nav-label">Settings</span>
        </a>
      </nav>

      <div class="sidebar-footer">
        <button class="logout-btn" onclick={handleLogout}>Logout</button>
      </div>
    </aside>

    <main class="main-panel">
      <slot />
    </main>
  </div>
{/if}

<style>
  .loading-screen {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    color: #d4af37;
    font-size: 1.2rem;
    letter-spacing: 2px;
    text-transform: uppercase;
  }

  .login-screen {
    max-width: 500px;
    margin: 0 auto;
    padding: 3rem 2rem;
    padding-top: 10vh;
    text-align: center;
    background:
      linear-gradient(180deg, rgba(20, 20, 30, 0.95) 0%, rgba(15, 15, 25, 0.98) 100%),
      repeating-linear-gradient(90deg, transparent, transparent 3px, rgba(139, 92, 46, 0.03) 3px, rgba(139, 92, 46, 0.03) 6px),
      repeating-linear-gradient(0deg, transparent, transparent 3px, rgba(139, 92, 46, 0.03) 3px, rgba(139, 92, 46, 0.03) 6px);
    background-size: 100%, 6px 6px, 6px 6px;
    border: 2px solid rgba(139, 92, 46, 0.5);
    border-radius: 8px;
    box-shadow:
      0 0 30px rgba(0, 0, 0, 0.8),
      inset 0 0 50px rgba(0, 0, 0, 0.5),
      0 0 100px rgba(255, 100, 0, 0.1);
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .login-screen h1 {
    text-align: center;
    margin-bottom: 0.5rem;
    font-size: 2.5em;
    color: #d4af37;
    text-shadow:
      0 0 20px rgba(212, 175, 55, 0.5),
      2px 2px 4px rgba(0, 0, 0, 0.8),
      0 0 40px rgba(255, 200, 100, 0.3);
    letter-spacing: 3px;
  }

  .subtitle {
    color: #a0a0a0;
    margin-bottom: 2rem;
    letter-spacing: 1px;
  }

  .login-form {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .login-form label {
    text-align: left;
    font-weight: 600;
    margin-top: 1rem;
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 1px;
    font-size: 0.9em;
  }

  .hint {
    font-size: 0.85rem;
    color: #888;
    text-align: left;
    margin: 0.25rem 0 1rem 0;
  }

  input,
  button {
    border-radius: 3px;
    border: 2px solid rgba(139, 92, 46, 0.6);
    padding: 12px 24px;
    font-size: 1em;
    font-weight: bold;
    font-family: inherit;
    transition: all 0.3s ease;
    box-shadow:
      0 4px 15px rgba(0, 0, 0, 0.6),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
  }

  input {
    color: #e0e0e0;
    background-color: rgba(30, 30, 40, 0.8);
    border: 2px solid rgba(139, 92, 46, 0.4);
  }

  input:focus {
    border-color: rgba(139, 92, 46, 0.8);
    outline: none;
    box-shadow:
      0 0 20px rgba(212, 175, 55, 0.3),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
  }

  button {
    cursor: pointer;
    background: linear-gradient(180deg, rgba(60, 80, 40, 0.8) 0%, rgba(40, 60, 30, 0.8) 100%);
    color: #e0e0e0;
    text-transform: uppercase;
    letter-spacing: 1px;
    margin-top: 1rem;
  }

  button:hover {
    background: linear-gradient(180deg, rgba(70, 95, 50, 0.9) 0%, rgba(50, 75, 40, 0.9) 100%);
    border-color: rgba(139, 92, 46, 0.8);
    box-shadow:
      0 6px 20px rgba(0, 0, 0, 0.8),
      0 0 20px rgba(100, 255, 100, 0.2);
    transform: translateY(-2px);
  }

  button:active {
    transform: translateY(0);
  }

  .error {
    color: #ff6b6b;
    background-color: rgba(220, 53, 69, 0.2);
    border: 1px solid rgba(220, 53, 69, 0.4);
    border-radius: 3px;
    padding: 0.75rem 1rem;
    margin-top: 1rem;
    font-size: 0.9rem;
  }

  /* App Layout */
  .app-layout {
    display: flex;
    min-height: 100vh;
    background: #0a0a0f;
    position: relative;
  }

  /* Update Banner */
  .update-banner {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    background: linear-gradient(180deg, rgba(40, 80, 120, 0.95) 0%, rgba(30, 60, 100, 0.95) 100%);
    border-bottom: 2px solid rgba(100, 150, 200, 0.5);
    z-index: 1000;
    padding: 12px 20px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
  }

  .update-content {
    display: flex;
    align-items: center;
    gap: 1rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  .update-icon {
    font-size: 1.5rem;
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
    gap: 0.25rem;
  }

  .update-text strong {
    color: #fff;
    font-size: 1rem;
  }

  .update-text span {
    color: #d0d0d0;
    font-size: 0.875rem;
  }

  .update-btn {
    background: linear-gradient(180deg, rgba(50, 200, 100, 0.9) 0%, rgba(40, 160, 80, 0.9) 100%);
    color: white;
    border: 2px solid rgba(50, 200, 100, 0.6);
    padding: 8px 16px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.3s ease;
    margin: 0;
  }

  .update-btn:hover:not(:disabled) {
    background: linear-gradient(180deg, rgba(60, 220, 110, 1) 0%, rgba(50, 180, 90, 1) 100%);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(50, 200, 100, 0.4);
  }

  .update-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  /* Sidebar */
  .sidebar {
    width: 280px;
    background:
      linear-gradient(180deg, rgba(20, 20, 30, 0.95) 0%, rgba(15, 15, 25, 0.98) 100%),
      repeating-linear-gradient(90deg, transparent, transparent 3px, rgba(139, 92, 46, 0.03) 3px, rgba(139, 92, 46, 0.03) 6px),
      repeating-linear-gradient(0deg, transparent, transparent 3px, rgba(139, 92, 46, 0.03) 3px, rgba(139, 92, 46, 0.03) 6px);
    background-size: 100%, 6px 6px, 6px 6px;
    border-right: 2px solid rgba(139, 92, 46, 0.5);
    display: flex;
    flex-direction: column;
    box-shadow:
      2px 0 30px rgba(0, 0, 0, 0.8),
      inset -1px 0 0 rgba(139, 92, 46, 0.3);
  }

  .sidebar-header {
    padding: 30px 20px;
    border-bottom: 2px solid rgba(139, 92, 46, 0.5);
    background:
      linear-gradient(180deg, rgba(30, 30, 40, 0.9) 0%, rgba(20, 20, 30, 0.9) 100%),
      repeating-linear-gradient(90deg, transparent, transparent 2px, rgba(139, 92, 46, 0.08) 2px, rgba(139, 92, 46, 0.08) 4px);
    background-size: 100%, 4px 4px;
  }

  .sidebar-header h1 {
    margin: 0 0 1rem 0;
    font-size: 1.8em;
    color: #d4af37;
    text-shadow:
      0 0 20px rgba(212, 175, 55, 0.5),
      2px 2px 4px rgba(0, 0, 0, 0.8),
      0 0 40px rgba(255, 200, 100, 0.3);
    letter-spacing: 2px;
    text-align: center;
  }

  .steam-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    font-size: 0.8rem;
    text-align: center;
  }

  .steam-info .label {
    color: #808080;
    text-transform: uppercase;
    letter-spacing: 1px;
    font-size: 0.75rem;
  }

  .steam-info code {
    color: #90ff90;
    background-color: rgba(30, 30, 40, 0.8);
    padding: 0.4rem 0.6rem;
    border-radius: 3px;
    border: 1px solid rgba(139, 92, 46, 0.3);
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
  }

  .sidebar-nav {
    flex: 1;
    padding: 20px 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 15px 20px;
    color: #b0b0b0;
    text-decoration: none;
    font-weight: 600;
    font-size: 1rem;
    text-transform: uppercase;
    letter-spacing: 1px;
    border-left: 3px solid transparent;
    transition: all 0.3s ease;
    background: transparent;
  }

  .nav-item:hover {
    color: #d4af37;
    background: rgba(30, 30, 40, 0.6);
    border-left-color: rgba(139, 92, 46, 0.5);
  }

  .nav-item.active {
    color: #d4af37;
    background:
      linear-gradient(90deg, rgba(30, 30, 40, 0.8) 0%, rgba(20, 20, 30, 0.8) 100%);
    border-left-color: #d4af37;
    box-shadow: inset 0 0 20px rgba(212, 175, 55, 0.1);
  }

  .nav-icon {
    font-size: 1.4rem;
    filter: grayscale(100%);
    transition: filter 0.3s ease;
  }

  .nav-item:hover .nav-icon,
  .nav-item.active .nav-icon {
    filter: grayscale(0%);
  }

  .sidebar-footer {
    padding: 20px;
    border-top: 2px solid rgba(139, 92, 46, 0.5);
  }

  .logout-btn {
    width: 100%;
    background: linear-gradient(180deg, rgba(60, 60, 70, 0.8) 0%, rgba(40, 40, 50, 0.8) 100%);
    border: 2px solid rgba(139, 92, 46, 0.6);
    padding: 12px;
    text-align: center;
  }

  .logout-btn:hover {
    background: linear-gradient(180deg, rgba(70, 70, 80, 0.9) 0%, rgba(50, 50, 60, 0.9) 100%);
  }

  /* Main Panel */
  .main-panel {
    flex: 1;
    overflow-y: auto;
    padding: 20px;
  }
</style>
