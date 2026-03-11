# Test Guide: Medal History (Task #19)

## Prerequisites
- App built and running with a Steam ID that has recent matches
- At least some matches synced (backfill helps to get rank variety)

## Test Steps

### 1. Sync matches and check rank data is captured
- Go to **Matches** → click **Refresh Matches**
- Open SQLite DB (`%LOCALAPPDATA%/DotaKeeper/dota_keeper_dev.db`) and run:
  ```sql
  SELECT match_id, start_time, rank_tier FROM matches WHERE rank_tier IS NOT NULL LIMIT 10;
  ```
- Expected: Some matches have a non-null `rank_tier` value (e.g. 43, 55, 71)

### 2. Sidebar rank pill
- Expected: The sidebar footer rank pill (bottom-left) shows your current medal name (e.g. "Archon 3") instead of "N/A"
- If all matches have `rank_tier = NULL`, it stays "N/A" — that's correct
- Clicking the rank pill should navigate to `/medals`

### 3. Navigate to Medal History page
- Click the **Medals** nav item in the sidebar (shield icon)
- Expected: Page loads with title "Medal History"

### 4. Stats cards
- Expected three cards: **Current Medal**, **Peak Medal** (with "All-time Best" tag), **Tracked Games**
- Medal names should be correct: Herald 1-5, Guardian 1-5, Crusader, Archon, Legend, Ancient, Divine, Immortal
- Stars should match: e.g. rank_tier=43 → "Archon 3" with 3 stars

### 5. Rank change timeline
- Expected: Timeline shows only entries where rank changed (consecutive same-rank matches are collapsed)
- Each entry shows: medal name + stars, date, match link
- Peak medal entry has a gold "Peak" chip
- Timeline is ordered most-recent first

### 6. Empty state
- If no matches have rank_tier: empty state message shown, no cards/timeline

### 7. Medal colors
- Herald = gray, Guardian = green, Crusader = teal, Archon = blue, Legend = purple, Ancient = orange, Divine = red, Immortal = gold

## Edge Cases
- rank_tier = 0 or null → should show "Unranked" or be excluded from history
- Only one distinct rank → "Your rank has not changed across tracked matches" message shown
- Immortal (rank_tier = 80) → shows "Immortal" with no stars
