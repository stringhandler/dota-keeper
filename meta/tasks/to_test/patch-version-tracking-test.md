# Test Guide: Patch Version Tracking

## What was implemented
- `patches` table in SQLite to cache Dota 2 patch names + release dates
- `patch` column on `matches` table
- Two new Tauri commands: `get_patches`, `sync_patches`
- Patch badge on each match row (date column)
- Patch filter chips on the matches page (groups by major patch, e.g. "7.40")
- "Sync Patches" button in Settings > Data section

## Test Steps

### 1. Sync patches
1. Open the app and go to **Settings**
2. Find the **Sync Patch Data** section
3. Click **Sync Patches**
4. Expect a toast: "Patches synced. N matches updated." (N ≥ 0)

### 2. Verify patch badge on match rows
1. Go to **Matches**
2. Each match row should show a small purple patch badge (e.g., `7.40e`) in the date column
3. Matches played on very old dates may still show no badge if they predate the patch table

### 3. Verify patch filter chips
1. On the Matches page, the filter bar should now include patch filter chips (e.g., "7.40") after the hero chips
2. Click a patch chip (e.g., "7.40") — only matches from that major patch family should be shown
3. Matches from "7.40", "7.40b", "7.40c" etc. should all appear under the "7.40" major filter
4. Click a different patch or "All" to reset

### 4. New match refresh
1. Click **Refresh** on the Matches page to fetch new matches
2. Newly inserted matches should automatically show a patch badge (no re-sync needed) if patches are already cached

### 5. Edge cases
- If no patches are cached yet, no patch chips appear and no badges show
- After syncing, re-opening the app should still show patches (data is persisted in SQLite)
