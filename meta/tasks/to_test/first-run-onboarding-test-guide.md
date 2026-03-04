# Test Guide — First-Run Onboarding

## Prerequisites
- Build and run the app: `npm run tauri dev`

## Test 1 — Fresh first login

1. If you already have a Steam ID saved, log out via Settings → Logout.
2. Delete (or rename) `%LOCALAPPDATA%\DotaKeeper\settings.json` to simulate a fresh install.
3. Restart the app and log in with your Steam ID.
4. **Expected**: The onboarding overlay appears immediately after login, showing Step 1 of 4.

## Test 2 — Step 1: Goal selection

- Click one or more goal cards (they should highlight in gold with a ✓).
- Click Continue.
- **Expected**: Advances to Step 2. Progress dots update.

## Test 3 — Step 2: Hero favourites

- Click up to 5 hero chips — they should highlight gold.
- Try clicking a 6th — it should be disabled/un-clickable.
- Click a selected hero to deselect it.
- Click Continue.
- **Expected**: Advances to Step 3. Open Analysis page later and confirm those heroes appear as favourites.

## Test 4 — Step 3: Mental health

- Click "Yes, enable it".
- **Expected**: Shows "✓ Enabled" badge. Continue button appears.
- Go to Settings after onboarding completes — mental health tracking should be ON, and the intro modal should NOT appear.

## Test 5 — Step 4: Backfill

- Click "Yes, backfill now".
- **Expected**: Spinner appears. You can click "Continue in background" and the app loads.
- OR click "Skip for now" — onboarding completes, redirects to Dashboard (or Goals if selected in Step 1).

## Test 6 — Skip everything

- Fresh login → click "Skip all" on Step 1.
- **Expected**: Jumps directly to Dashboard. Onboarding does NOT reappear on next app launch.

## Test 7 — Existing user not shown onboarding

- Log out and log back in (settings.json still has `onboarding_completed: true`).
- **Expected**: No onboarding overlay. Goes straight to Dashboard.

## Test 8 — Persistence check

- Complete onboarding, close the app, reopen.
- **Expected**: Onboarding does NOT appear again.
