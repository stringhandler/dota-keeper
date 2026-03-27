# Steam OpenID Login Button

## Priority
**HIGH** — First-run UX: users shouldn't have to dig up their Steam ID manually

## Problem

The login screen only accepts a manually-entered Steam ID or profile URL.
Many users won't know their Steam64 ID off the top of their head, making
first-run setup unnecessarily fiddly.

## Solution

Add a **"Sign in through Steam"** button that uses Steam's OpenID 2.0 flow:

1. Backend starts a temporary local HTTP server on a random free port
2. Frontend opens the Steam OpenID URL in the user's default browser
3. User logs in/authorises on steamcommunity.com
4. Steam redirects to `http://127.0.0.1:{port}/callback`
5. Backend verifies the assertion with Steam (`check_authentication`)
6. Backend emits a `steam-login-complete` Tauri event with the Steam64 ID
7. Frontend handles the event and saves the Steam ID

## Implementation

### Rust (`src-tauri/src/lib.rs`)
- `start_steam_login(app)` command — binds TCP listener, returns OpenID URL, spawns callback handler
- `handle_steam_callback(listener, app)` — reads one HTTP request, verifies with Steam, emits event
- Helper fns: `percent_encode`, `percent_decode`, `write_html`
- Register `start_steam_login` in `invoke_handler!`

### Frontend (`src/routes/+layout.svelte`)
- Import `listen` from `@tauri-apps/api/event`
- Import `openUrl` from `@tauri-apps/plugin-opener`
- `saveAndLogin(id64)` helper shared by both form and Steam flow
- `handleSteamLogin()` — calls command, opens URL, listens for event
- Add "Sign in through Steam" button + divider below the manual form

## Acceptance Criteria
- [ ] Clicking "Sign in through Steam" opens the Steam login page in the system browser
- [ ] After authorising, the app logs in automatically (no copy-paste needed)
- [ ] The Steam ID is saved identically to the manual flow
- [ ] If the user cancels/closes the browser before completing, the app shows an appropriate error
- [ ] The manual Steam ID input still works unchanged
