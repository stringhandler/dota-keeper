# Check for Updates Button in Settings

## Overview

Add a "Check for Updates" button to the Settings page so the user can manually trigger an update check at any time, with clear feedback on the result (up to date, update available, or error).

## Current Behaviour

The auto-updater runs silently on app launch in `+layout.svelte`. Errors are swallowed with `console.error` and never shown to the user. There is no way to manually trigger a check.

## Required Behaviour

- Button in Settings: **"Check for Updates"**
- States:
  - Idle: "Check for Updates"
  - Checking: "Checking..." (disabled)
  - Up to date: "You're on the latest version (v0.x.x)" in green
  - Update available: show version + "Install & Restart" button (reuse existing install logic)
  - Error: show the error message clearly (e.g. permission denied, network error, server unreachable)

## Implementation

### Frontend (`src/routes/settings/+page.svelte`)
- Add `checkingUpdate`, `updateResult`, `updateError` state variables
- `checkForUpdates()` function: call `check()` from `@tauri-apps/plugin-updater`
  - On success with update available: show version and install button
  - On success with no update: show "up to date" message
  - On error: show the raw error (don't swallow it)
- Reuse `installUpdate()` logic already in `+layout.svelte`

### Capability
- `updater:default` is already in `capabilities/default.json` ✓

## Acceptance Criteria
- [ ] Button visible in Settings page
- [ ] Shows loading state while checking
- [ ] Displays current version alongside the button
- [ ] "Up to date" message shown when no update available
- [ ] Update available → shows new version + install button
- [ ] Errors shown clearly to the user (not just console)
