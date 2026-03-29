# Test Guide: Background Auto-Parse

## Prerequisites
- Have at least a few unparsed matches in the database (state = `Fetched` or `Failed`).
  You can create them by running "Backfill Historical Matches" or "Clear All Matches" + re-fetching.
- Steam ID must be configured.

## Steps

### Basic functionality
1. Launch the app. Wait ~5 seconds (the background parser starts after a short delay).
2. Open **Settings → Match History → Background Auto-Parse**.
3. The toggle should show **Enabled**.
4. If there are unparsed matches, you should see "Parsing in background — N match(es) remaining…" below the description.
5. Navigate to the **Matches** tab. Watch for matches to flip from `Fetched` / `Failed` → `Parsing` → `Parsed` over time (~15–20 s per match including the 10 s delay).

### Disable / re-enable
6. Toggle **Background Auto-Parse** to **Disabled** while parsing is running.
7. The current match in-flight will complete, but no further matches should start.
8. Toggle back to **Enabled** — note: the parser only starts at **app launch**, so re-enabling mid-session does not restart it. The setting will take effect on the next app start.

### No unparsed matches
9. Once all matches are parsed, the "Parsing in background…" indicator should disappear.

### Rate limit backoff (hard to test manually)
10. If OpenDota returns 429, the parser waits 60 s and retries once before skipping the match.

### Settings persistence
11. Disable the setting, quit and relaunch the app.
12. Open Settings — toggle should still show **Disabled** and the background parser should NOT start.
