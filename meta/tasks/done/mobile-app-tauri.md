# Convert App to Mobile (iOS & Android) via Tauri

## Problem Statement

Dota Keeper is currently desktop-only (Windows, macOS, Linux). Since Tauri 2.0 supports iOS and Android natively, the app can be extended to mobile without rewriting the frontend. Mobile access would let users check their goal progress and match history away from their PC.

## Overview

Tauri 2.0 (already in use — see `tauri = { version = "2" }` in Cargo.toml) ships with built-in mobile support. The core Rust backend and Svelte frontend can be reused largely as-is, with mobile-specific adaptations for UI layout and platform APIs.

## Target Platforms

- **Android** (API level 24+, Android 7.0+)
- **iOS** (iOS 13+)

## Key Differences from Desktop

| Area | Desktop | Mobile |
|------|---------|--------|
| Screen size | 800×600+ | 375–430px wide |
| Navigation | Sidebar nav | Bottom tab bar |
| Input | Mouse + keyboard | Touch |
| Storage | `%LOCALAPPDATA%` | App sandbox directory |
| Window chrome | Native title bar | None (full-screen webview) |
| Permissions | File system, network | Network only (sandboxed) |
| Auto-update | Tauri updater plugin | App Store / Play Store |

## Requirements

### Phase 1: Build Infrastructure

1. **Add mobile targets** to the Tauri project:
   ```bash
   yarn tauri android init
   yarn tauri ios init
   ```

2. **Configure capabilities** for mobile in `src-tauri/capabilities/`:
   - Network access for OpenDota API
   - Storage access for SQLite database

3. **Update CI/CD** to build mobile releases:
   - Android: build signed `.apk` / `.aab`
   - iOS: build `.ipa` (requires macOS runner + Apple developer account)

4. **Database path**: Use Tauri's path API for mobile-compatible storage location:
   ```rust
   // Mobile uses app's sandboxed data directory
   #[cfg(target_os = "android")]
   fn get_db_path(app_handle: &AppHandle) -> PathBuf {
       app_handle.path().app_data_dir().unwrap().join("dota_keeper.db")
   }

   #[cfg(target_os = "ios")]
   fn get_db_path(app_handle: &AppHandle) -> PathBuf {
       app_handle.path().app_data_dir().unwrap().join("dota_keeper.db")
   }
   ```

### Phase 2: UI Adaptations

#### Responsive Layout

The existing Svelte components need responsive CSS:

```css
/* Mobile breakpoint */
@media (max-width: 640px) {
    /* Hide desktop sidebar */
    .sidebar { display: none; }

    /* Show bottom navigation */
    .bottom-nav { display: flex; }

    /* Stack cards vertically */
    .goal-progress-grid {
        grid-template-columns: 1fr;
    }

    /* Larger touch targets */
    button, a {
        min-height: 44px;
        padding: 0.75rem 1rem;
    }
}
```

#### Bottom Navigation Bar (Mobile)

Replace sidebar with a bottom tab bar:

```svelte
<!-- BottomNav.svelte -->
<nav class="bottom-nav">
    <a href="/" class:active={$page.url.pathname === '/'}>
        🏠 <span>Home</span>
    </a>
    <a href="/goals" class:active={$page.url.pathname === '/goals'}>
        🎯 <span>Goals</span>
    </a>
    <a href="/matches" class:active={$page.url.pathname === '/matches'}>
        ⚔️ <span>Matches</span>
    </a>
    <a href="/settings" class:active={$page.url.pathname === '/settings'}>
        ⚙️ <span>Settings</span>
    </a>
</nav>

<style>
    .bottom-nav {
        position: fixed;
        bottom: 0;
        left: 0;
        right: 0;
        display: flex;
        background: rgba(22, 33, 62, 0.95);
        border-top: 1px solid rgba(212, 175, 55, 0.3);
        padding-bottom: env(safe-area-inset-bottom); /* iPhone notch */
    }

    .bottom-nav a {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 0.5rem;
        font-size: 0.7rem;
    }
</style>
```

#### Charts

Chart.js charts need to be responsive and touch-friendly:
- Enable `responsive: true` and `maintainAspectRatio: false`
- Ensure tooltips work on tap (not just hover)
- Reduce data density for small screens

#### Tables

Long tables need horizontal scroll or card-based layout on mobile:
```css
@media (max-width: 640px) {
    .data-table-wrapper {
        overflow-x: auto;
        -webkit-overflow-scrolling: touch;
    }
}
```

### Phase 3: Mobile-specific Features

#### Pull to Refresh
Trigger match sync when user pulls down on the main screen:
```svelte
<script>
    // Use touch events to detect pull-to-refresh gesture
    // Then call the existing fetch matches command
</script>
```

#### App Icon & Splash Screen
- Generate mobile icon sizes from existing icon assets
- Add a splash/launch screen matching the app theme

#### Safe Area Insets
Handle iPhone notch and Android gesture areas:
```css
body {
    padding-top: env(safe-area-inset-top);
    padding-bottom: env(safe-area-inset-bottom);
}
```

## Known Constraints

### No OpenDota Background Polling on Mobile
- iOS aggressively kills background processes
- Background match fetching won't work reliably
- **Solution**: Fetch on app open / manual refresh only

### SQLite on Mobile
- SQLite with `rusqlite` (bundled feature) works on both Android and iOS
- Database path must use Tauri's sandboxed path API (not `dirs::data_local_dir()`)

### No Auto-update on Mobile
- Tauri's updater plugin does not support iOS/Android
- Updates are distributed through App Store / Play Store
- Remove auto-update UI elements when running on mobile

### iOS Developer Account Required
- Requires paid Apple Developer Program membership ($99/yr) to distribute
- Simulator builds can be done for free

## Development Setup Requirements

### Android
- Android Studio + Android SDK
- `ANDROID_HOME` environment variable set
- Connected device or emulator

### iOS (macOS only)
- Xcode 14+
- Apple Developer account (for device testing / distribution)
- iOS Simulator (free, included with Xcode)

## File Changes

- `src-tauri/capabilities/` — Add mobile capability files
- `src-tauri/src/database.rs` — Mobile-compatible `get_db_path()`
- `src-tauri/gen/android/` — Auto-generated by `tauri android init`
- `src-tauri/gen/apple/` — Auto-generated by `tauri ios init`
- `src/app.css` — Mobile responsive styles
- `src/routes/+layout.svelte` — Bottom nav for mobile, hide sidebar
- `src/lib/components/BottomNav.svelte` — New component
- `.github/workflows/release.yml` — Android build step

## Success Criteria

- [ ] App builds and runs on Android emulator
- [ ] App builds and runs on iOS simulator
- [ ] Navigation works via bottom tab bar on mobile
- [ ] Goal progress cards are readable on 375px wide screen
- [ ] Match list is usable on mobile
- [ ] Charts render correctly on mobile
- [ ] Database reads/writes work correctly on both platforms
- [ ] OpenDota API calls work on mobile (network access)
- [ ] App icon and splash screen display correctly

## Priority
**LOW** - Large undertaking, desktop is the primary platform. Revisit when desktop features are stable.

## Estimated Complexity
**High** — Multi-phase effort requiring platform setup, responsive UI rework, and CI changes

## References
- [Tauri Mobile Guide](https://v2.tauri.app/start/prerequisites/)
- [Tauri Android](https://v2.tauri.app/distribute/android/)
- [Tauri iOS](https://v2.tauri.app/distribute/ios/)
