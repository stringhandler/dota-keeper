<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { getVersion } from '@tauri-apps/api/app';
  import { check } from '@tauri-apps/plugin-updater';
  import { relaunch } from '@tauri-apps/plugin-process';
  import { listen } from '@tauri-apps/api/event';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { getAnalyticsConsent, identifyUser } from "$lib/analytics.js";
  import { initSentry } from "$lib/sentry.js";
  import AnalyticsConsentModal from "$lib/AnalyticsConsentModal.svelte";
  import OnboardingFlow from "$lib/OnboardingFlow.svelte";
  import TitleBar from "$lib/TitleBar.svelte";
  import WindowResize from "$lib/WindowResize.svelte";
  import BottomNav from "$lib/BottomNav.svelte";
  import FeedbackModal from "$lib/FeedbackModal.svelte";
  import WhatsNewModal from "$lib/WhatsNewModal.svelte";
  import { releaseNotes } from "$lib/whatsNew.js";
  import Toast from "$lib/Toast.svelte";
  import { showToast } from "$lib/toast.js";
  import { privacyStore, loadPrivacyMode } from "$lib/privacy-store.svelte.js";
  import { PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY } from '$env/static/public';
  import { setupI18n, setLocale, resolveLocale, locale } from "$lib/i18n.js";
  import { _ } from "svelte-i18n";
  import '../app.css';

  // Initialise i18n once on app load
  setupI18n();

  let isLoading = $state(true);
  let isMobile = $state(false);
  let isLoggedIn = $state(false);
  let showOnboarding = $state(false);
  let currentSteamId = $state("");
  let steamId = $state("");
  let error = $state("");
  let currentRankTier = $state(null);
  let updateAvailable = $state(false);
  let updateVersion = $state("");
  let isUpdating = $state(false);
  let showConsentModal = $state(false);
  let steamLoginPending = $state(false);
  let showFeedbackModal = $state(false);
  let appVersion = $state("");
  let showWhatsNew = $state(false);

  onMount(async () => {
    const checkMobile = () => { isMobile = window.innerWidth < 640; };
    checkMobile();
    window.addEventListener('resize', checkMobile);

    await loadSettings();
    appVersion = await getVersion();

    const lastSeen = localStorage.getItem('last_seen_version');
    if (lastSeen !== null && lastSeen !== appVersion && releaseNotes[appVersion]) {
      showWhatsNew = true;
    }
    localStorage.setItem('last_seen_version', appVersion);

    // Auto-detect OS language on first run (no saved preference)
    if (!localStorage.getItem('locale')) {
      try {
        const osLocale = await invoke("get_os_locale");
        setLocale(resolveLocale(osLocale));
        // Don't persist — only save when the user explicitly picks a language
        localStorage.removeItem('locale');
      } catch (_) {}
    }

    await checkForUpdates();

    if (!PUBLIC_SUPABASE_URL || !PUBLIC_SUPABASE_ANON_KEY) {
      showToast("Feedback is not configured. Set PUBLIC_SUPABASE_URL and PUBLIC_SUPABASE_ANON_KEY.", "error", 8000);
    }

    // Check analytics consent on every startup
    const consent = await getAnalyticsConsent();
    if (consent === "NotYet") {
      showConsentModal = true;
    } else if (consent === "Accepted") {
      // Identify user for analytics once on app start
      await identifyUser();
    }
  });

  let displaySteamId = $derived(
    privacyStore.privacyMode && currentSteamId
      ? currentSteamId.slice(0, 5) + "••••••••••"
      : currentSteamId
  );

  /** @param {number | null} rankTier */
  function getMedalLabel(rankTier) {
    if (!rankTier) return null;
    const NAMES = ["", "Herald", "Guardian", "Crusader", "Archon", "Legend", "Ancient", "Divine", "Immortal"];
    const major = Math.floor(rankTier / 10);
    const stars = rankTier % 10;
    if (major === 8) return "Immortal";
    if (major >= 1 && major <= 7) return stars > 0 ? `${NAMES[major]} ${stars}` : NAMES[major];
    return null;
  }

  async function loadSettings() {
    try {
      const settings = await invoke("get_settings");
      if (settings.steam_id) {
        isLoggedIn = true;
        currentSteamId = settings.steam_id;
      }
      privacyStore.privacyMode = settings.privacy_mode ?? false;
      if (settings.analytics_consent === "Accepted") {
        initSentry();
      }
    } catch (e) {
      error = `Failed to load settings: ${e}`;
    } finally {
      isLoading = false;
    }
    // Load medal stats independently so failures don't block the app
    try {
      const medalStats = await invoke("get_medal_stats");
      currentRankTier = medalStats.current_rank_tier;
    } catch (_) {}
  }

  /** @param {string} input */
  function extractSteamId(input) {
    const trimmed = input.trim();
    const profileMatch = trimmed.match(/steamcommunity\.com\/profiles\/(\d+)/);
    if (profileMatch) return profileMatch[1];
    return trimmed;
  }

  /** @param {string} id64 */
  async function saveAndLogin(id64) {
    const settings = await invoke("save_steam_id", { steamId: id64 });
    isLoggedIn = true;
    currentSteamId = settings.steam_id;
    if (!settings.onboarding_completed) {
      showOnboarding = true;
    }
  }

  /** @param {SubmitEvent} event */
  async function handleLogin(event) {
    event.preventDefault();
    error = "";
    if (!steamId.trim()) { error = "Please enter your Steam ID"; return; }
    try {
      await saveAndLogin(extractSteamId(steamId));
    } catch (e) {
      error = `Failed to save Steam ID: ${e}`;
    }
  }

  async function handleSteamLogin() {
    steamLoginPending = true;
    error = "";
    /** @type {(() => void) | undefined} */
    let unlisten;
    try {
      const url = await invoke("start_steam_login");
      unlisten = await listen("steam-login-complete", async (event) => {
        unlisten?.();
        steamLoginPending = false;
        const payload = event.payload;
        if (payload.steam_id) {
          try {
            await saveAndLogin(payload.steam_id);
          } catch (e) {
            error = `Failed to save Steam ID: ${e}`;
          }
        } else {
          error = payload.error || "Steam login failed";
        }
      });
      await openUrl(url);
    } catch (e) {
      unlisten?.();
      steamLoginPending = false;
      error = `Steam login failed: ${e}`;
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

  /** @param {string} path */
  function isActive(path) {
    return $page.url.pathname === path;
  }

  /** @param {string} prefix */
  function isActivePath(prefix) {
    return $page.url.pathname.startsWith(prefix);
  }

  /** @param {string} pathname */
  function getPageTitle(pathname) {
    if (pathname === '/') return $_('page_title.dashboard');
    if (pathname.startsWith('/matches')) return $_('page_title.matches');
    if (pathname.startsWith('/analysis')) return $_('page_title.analysis');
    if (pathname.startsWith('/goals')) return $_('page_title.goals');
    if (pathname.startsWith('/challenges')) return $_('page_title.challenges');
    if (pathname.startsWith('/mental-health')) return $_('page_title.mental_health');
    if (pathname.startsWith('/medals')) return $_('page_title.medals');
    if (pathname.startsWith('/settings')) return $_('page_title.settings');
    return $_('page_title.app');
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

{#if !isMobile}
  <WindowResize />
  <TitleBar />
{/if}

{#if isLoading}
  <div class="loading-screen">
    <p>{$_('layout.loading')}</p>
  </div>
{:else if !isLoggedIn}
  <div class="login-screen">
    <div class="login-box">
      <div class="login-brand">{$_('login.brand')}</div>
      <p class="login-sub">{$_('login.subtitle')}</p>

      <form class="login-form" onsubmit={handleLogin}>
        <label class="form-label" for="steam-id">{$_('login.steam_id')}</label>
        <input
          class="form-input"
          id="steam-id"
          type="text"
          autocomplete="off"
          autocorrect="off"
          autocapitalize="none"
          placeholder={$_('login.placeholder')}
          bind:value={steamId}
        />
        <p class="login-hint">{$_('login.hint')}</p>
        <button type="submit" class="btn btn-primary" style="width:100%;justify-content:center;padding:12px;">
          {$_('login.save_continue')}
        </button>
      </form>

      <div class="login-divider">
        <span>{$_('login.or')}</span>
      </div>

      <button
        type="button"
        class="btn btn-steam"
        onclick={handleSteamLogin}
        disabled={steamLoginPending}
      >
        {#if steamLoginPending}
          {$_('login.waiting_steam')}
        {:else}
          <svg viewBox="0 0 233 233" width="18" height="18" fill="currentColor" aria-hidden="true">
            <path d="M116.5 0C52.1 0 0 52.1 0 116.5c0 55.1 38.5 101.3 90.4 113.1l30.5-73.2c-1.3.1-2.5.1-3.8.1-26.7 0-48.4-21.7-48.4-48.4s21.7-48.4 48.4-48.4 48.4 21.7 48.4 48.4c0 23.6-16.9 43.3-39.4 47.5L96.7 228c6.4 1.6 13 2.5 19.8 2.5C180.9 230.5 233 178.4 233 114S180.9 0 116.5 0zM106.6 160.1l-14.5 34.8C63.7 183.6 43 152.3 43 116.5c0-40.5 32.9-73.5 73.5-73.5s73.5 32.9 73.5 73.5-32.9 73.5-73.5 73.5c-3.2 0-6.4-.2-9.5-.6l-.4-29.3zm9.9-93.6c-27.6 0-50 22.4-50 50s22.4 50 50 50 50-22.4 50-50-22.4-50-50-50zm0 80.7c-17 0-30.7-13.8-30.7-30.7s13.8-30.7 30.7-30.7 30.7 13.8 30.7 30.7-13.8 30.7-30.7 30.7z"/>
          </svg>
          {$_('login.sign_in_steam')}
        {/if}
      </button>

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
            <strong>{$_('layout.update_available')}</strong>
            <span>{$_('layout.update_ready', { values: { version: updateVersion } })}</span>
          </div>
          <button class="btn btn-primary" onclick={installUpdate} disabled={isUpdating}>
            {isUpdating ? $_('layout.installing') : $_('layout.install_restart')}
          </button>
        </div>
      </div>
    {/if}

    <!-- SIDEBAR (hidden on mobile — replaced by BottomNav) -->
    <aside class="sidebar" class:sidebar-hidden={isMobile}>
      <div class="brand">
        <div class="brand-name">Dota Keeper</div>
        {#if appVersion}<div class="app-version">v{appVersion}</div>{/if}
        <div class="brand-id">{$_('layout.steam_id_label')}</div>
        <div class="steam-badge">{displaySteamId}</div>
      </div>

      <nav class="nav">
        <a href="/" class="nav-item" class:active={isActive('/')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
          {$_('nav.dashboard')}
        </a>
        <a href="/matches" class="nav-item" class:active={isActivePath('/matches')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
          {$_('nav.matches')}
        </a>
        <a href="/analysis" class="nav-item" class:active={isActivePath('/analysis')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
          </svg>
          {$_('nav.analysis')}
        </a>
        <a href="/goals" class="nav-item" class:active={isActivePath('/goals')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
          </svg>
          {$_('nav.goals')}
        </a>
        <a href="/challenges" class="nav-item" class:active={isActivePath('/challenges')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z" />
          </svg>
          {$_('nav.challenges')}
        </a>
        <a href="/mental-health" class="nav-item" class:active={isActivePath('/mental-health')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z" />
          </svg>
          {$_('nav.mind')}
        </a>
        <a href="/medals" class="nav-item" class:active={isActivePath('/medals')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
          </svg>
          {$_('nav.medals')}
        </a>
        <a href="/settings" class="nav-item" class:active={isActive('/settings')}>
          <svg class="nav-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          {$_('nav.settings')}
        </a>
      </nav>

      <div class="sidebar-footer">
        <a href="/medals" class="rank-pill" style="text-decoration:none;">
          <div class="rank-star">★</div>
          <div>
            <div class="rank-text">{$_('layout.rank')}</div>
            <div class="rank-value">{getMedalLabel(currentRankTier) ?? $_('layout.rank_na')}</div>
          </div>
        </a>
        <!-- Language switcher -->
        <div class="lang-switcher">
          <button class="lang-btn" class:active={$locale === 'en'} onclick={() => setLocale('en')}>EN</button>
          <button class="lang-btn" class:active={$locale === 'ru'} onclick={() => setLocale('ru')}>RU</button>
        </div>
        <button class="feedback-link" onclick={() => showFeedbackModal = true} type="button">
          <svg class="feedback-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M7 8h10M7 12h6m-6 4h4M5 4h14a2 2 0 012 2v10a2 2 0 01-2 2H7l-4 4V6a2 2 0 012-2z" />
          </svg>
          {$_('layout.feedback')}
        </button>
      </div>
    </aside>

    <!-- MAIN -->
    <div class="main">
      <!-- TOP BAR -->
      <div class="topbar">
        <div class="page-title">{getPageTitle($page.url.pathname)}</div>
        <div class="topbar-actions">
          <a href="/goals" class="btn btn-primary new-goal-btn">{$_('layout.new_goal')}</a>
        </div>
      </div>

      <!-- CONTENT -->
      <div class="content-area" class:content-area-mobile={isMobile}>
        <slot />
      </div>
    </div>
  </div>

  {#if isMobile}
    <BottomNav />
    <button class="feedback-fab" onclick={() => showFeedbackModal = true} type="button" aria-label="Send feedback">
      <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5" width="18" height="18">
        <path stroke-linecap="round" stroke-linejoin="round" d="M7 8h10M7 12h6m-6 4h4M5 4h14a2 2 0 012 2v10a2 2 0 01-2 2H7l-4 4V6a2 2 0 012-2z" />
      </svg>
    </button>
  {/if}
{/if}

<!-- First-run onboarding overlay -->
{#if showOnboarding}
  <OnboardingFlow steamId={currentSteamId} onComplete={() => showOnboarding = false} />
{/if}

<!-- Analytics Consent Modal -->
{#if showConsentModal}
  <AnalyticsConsentModal onClose={() => showConsentModal = false} />
{/if}

<!-- What's New Modal -->
{#if showWhatsNew}
  <WhatsNewModal version={appVersion} notes={releaseNotes[appVersion]} onClose={() => showWhatsNew = false} />
{/if}

<!-- Feedback Modal -->
{#if showFeedbackModal}
  <FeedbackModal onClose={() => showFeedbackModal = false} />
{/if}

<!-- Global toast notifications -->
<Toast />

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
    padding-top: var(--sat);
  }

  .login-screen {
    display: flex;
    justify-content: center;
    align-items: center;
    flex: 1;
    background: var(--bg-base);
    padding-top: var(--sat);
  }

  .login-box {
    width: 420px;
    padding: 40px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
  }

  @media (max-width: 640px) {
    .login-box {
      width: calc(100% - 32px);
      max-width: 420px;
      padding: 28px 20px;
    }

    .login-brand {
      font-size: 22px;
    }
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

  .login-divider {
    display: flex;
    align-items: center;
    gap: 10px;
    margin: 18px 0 4px;
    color: var(--text-muted);
    font-size: 11px;
    font-family: 'Barlow Condensed', sans-serif;
    letter-spacing: 1px;
    text-transform: uppercase;
  }

  .login-divider::before,
  .login-divider::after {
    content: '';
    flex: 1;
    height: 1px;
    background: var(--border);
  }

  .btn-steam {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    padding: 11px;
    background: rgba(23, 47, 80, 0.6);
    border: 1px solid rgba(100, 150, 220, 0.35);
    border-radius: 6px;
    color: #c6d4df;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 1px;
    text-transform: uppercase;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
  }

  .btn-steam:hover:not(:disabled) {
    background: rgba(23, 47, 80, 0.9);
    border-color: rgba(100, 150, 220, 0.6);
  }

  .btn-steam:disabled {
    opacity: 0.5;
    cursor: default;
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

  .app-version {
    font-size: 10px;
    color: var(--text-secondary);
    letter-spacing: 1px;
    font-family: 'Barlow Condensed', sans-serif;
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

  /* ── LANGUAGE SWITCHER ── */
  .lang-switcher {
    display: flex;
    gap: 4px;
  }

  .lang-btn {
    flex: 1;
    padding: 5px 8px;
    background: none;
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-muted);
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 1px;
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s;
  }

  .lang-btn:hover {
    color: var(--text-secondary);
    border-color: rgba(240, 180, 41, 0.3);
  }

  .lang-btn.active {
    color: var(--gold);
    border-color: rgba(240, 180, 41, 0.5);
    background: rgba(240, 180, 41, 0.06);
  }

  /* ── FEEDBACK ── */
  .feedback-link {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    background: none;
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text-muted);
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s;
    width: 100%;
  }

  .feedback-link:hover {
    color: var(--text-secondary);
    border-color: rgba(240, 180, 41, 0.3);
  }

  .feedback-icon {
    width: 14px;
    height: 14px;
    opacity: 0.7;
    flex-shrink: 0;
  }

  /* Floating button on mobile */
  .feedback-fab {
    position: fixed;
    right: 14px;
    bottom: calc(var(--sab, 0px) + 68px);
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    color: var(--text-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    z-index: 99;
    transition: color 0.15s, border-color 0.15s;
  }

  .feedback-fab:hover {
    color: var(--gold);
    border-color: rgba(240, 180, 41, 0.5);
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
    padding: var(--sat) 32px 0;
    height: calc(60px + var(--sat));
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

  /* ── CONTENT AREA ── */
  .content-area {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 28px 32px;
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
  }

  /* ── MOBILE ── */
  .sidebar-hidden {
    display: none;
  }

  /* Extra bottom padding when the fixed BottomNav overlaps the content.
     Also account for landscape safe areas on left/right edges. */
  .content-area-mobile {
    padding: 16px calc(16px + var(--sar)) calc(72px + var(--sab)) calc(16px + var(--sal));
  }

  @media (max-width: 640px) {
    .topbar {
      padding: var(--sat) 12px 0;
      height: calc(52px + var(--sat));
      gap: 8px;
    }

    .page-title {
      font-size: 15px;
      letter-spacing: 2px;
      white-space: nowrap;
    }

    .topbar-actions {
      gap: 6px;
      flex-shrink: 0;
    }

    .new-goal-btn {
      display: none;
    }

    .update-banner {
      top: var(--sat);
    }
  }
</style>
