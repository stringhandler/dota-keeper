# Steam Login

## Feature Request

Allow the user to log in with their Steam account instead of manually entering a Steam ID.

## Current State

Users must manually enter their Steam ID (a 17-digit number) in the Settings page to connect their account.

## Desired State

Provide a "Login with Steam" button that authenticates via Steam's OpenID and automatically retrieves the user's Steam ID, name, and avatar.

## Implementation Approach

1. **Steam OpenID Authentication**
   - Steam uses OpenID 2.0 for login
   - The login flow: open browser → user authenticates on Steam → Steam redirects back with identity URL containing Steam ID
   - Identity URL format: `https://steamcommunity.com/openid/id/{steamId64}`

2. **Tauri Integration**
   - Use Tauri's `shell` plugin (or `opener`) to open the Steam login URL in the system browser
   - Start a local HTTP server (e.g., tiny_http or axum) on a localhost port to receive the OAuth callback
   - Parse the Steam ID from the returned identity URL
   - Store the Steam ID in the app's settings/database

3. **UI Changes**
   - Add "Login with Steam" button on the Settings page (or an onboarding screen)
   - Show logged-in state: Steam avatar, display name, Steam ID
   - Allow logout / switch account

4. **Steam API (optional enhancement)**
   - After obtaining the Steam ID, optionally call Steam API to fetch the user's display name and avatar
   - Requires a Steam API key (user-provided or bundled) for some endpoints
   - Display name and avatar may be available via OpenDota as well

## Files to Create/Modify

- `src-tauri/src/steam_auth.rs` (new) — handle OpenID callback server and Steam ID extraction
- `src-tauri/src/lib.rs` — register new Tauri commands
- `src/routes/settings/+page.svelte` — add Steam login button and display

## Acceptance Criteria

- [ ] "Login with Steam" button opens Steam login page in browser
- [ ] After successful login, Steam ID is automatically populated and saved
- [ ] User's Steam display name (and optionally avatar) is shown in settings
- [ ] Works on Windows without requiring any extra setup from the user
- [ ] Handles errors gracefully (login cancelled, timeout, network error)
