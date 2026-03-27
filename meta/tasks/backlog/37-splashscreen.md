# Splash Screen

## Overview

Add a splash screen that displays while the app is initialising (database setup, settings load, etc.) to avoid a blank white flash on startup.

## Implementation

Tauri v2 supports splash screens natively via a hidden main window + a separate splash window.

### Approach
1. Add a `splashscreen` window in `tauri.conf.json` that shows immediately
2. Hide the main window on startup (`visible: false`)
3. Once the app is ready (DB initialised, settings loaded), close the splash and show main from Rust via `app.get_webview_window("main").unwrap().show()`

### Splash content
- Dark background matching the app theme
- "Dota Keeper" title with the gold styling
- Subtle loading indicator (spinner or pulsing text)
- App version number

## References
- [Tauri v2 Splashscreen guide](https://v2.tauri.app/learn/splashscreen/)

## Acceptance Criteria
- [ ] Splash appears immediately on launch with no blank flash
- [ ] Main window only shown once app is fully initialised
- [ ] Splash matches app visual theme
- [ ] Works on Windows (primary platform)
