# Test: First-Run Refresh Matches

## What was fixed
On a fresh install with no backfill, clicking "Refresh Matches" would return success but show no matches. Root cause: the OpenDota `/recentMatches` endpoint can return an empty array if the player's profile hasn't been indexed yet. The fix uses the more reliable `/matches` endpoint (same as backfill) when the local DB is empty.

## Steps to test

1. Delete (or rename) the DotaKeeper database so the app starts fresh:
   - `%LOCALAPPDATA%\DotaKeeper\dota_keeper.db`
2. Launch the app. Complete or skip onboarding **without** clicking "Backfill".
3. Navigate to the **Matches** page.
4. Click **Refresh Matches**.
5. Expected: matches appear in the list and a toast shows "X new matches found".
6. Bonus: click Refresh again — should show "Matches up to date" (no duplicates inserted).
