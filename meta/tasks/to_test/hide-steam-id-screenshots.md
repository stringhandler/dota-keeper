# Hide Steam ID for Screenshots

## Summary
Add a way to obscure/hide the user's Steam ID in the UI so it's safe to take and share screenshots without exposing personal account information.

## Motivation
Steam IDs are personally identifiable and can be used to look up a player's full profile and match history. Users should be able to share screenshots of the app without revealing their Steam ID.

## Acceptance Criteria
- A toggle in Settings to enable "Privacy Mode" (or similar name)
- When enabled, the Steam ID is masked in the UI (e.g. shown as `7656119########` or `••••••••••`)
- The masking applies everywhere the Steam ID is displayed (settings page, profile, any debug info, etc.)
- The actual Steam ID remains stored and functional — only the display is affected
- The setting persists across sessions

## Notes
- Consider whether other PII (display name, avatar) should also be maskable under the same toggle
- The toggle itself should be accessible without needing to expose the Steam ID to find it
