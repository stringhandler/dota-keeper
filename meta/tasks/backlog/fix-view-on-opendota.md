# Bug: "View on OpenDota" doesn't work

## Description

The "View on OpenDota" button on the match details page (`/matches/[matchId]`) does not open the browser as expected.

## Steps to Reproduce

1. Navigate to Matches
2. Click the 🔍 icon on any match to open the details page
3. Click "View on OpenDota"

## Expected Behaviour

Opens `https://www.opendota.com/matches/{match_id}` in the system browser.

## Actual Behaviour

Nothing happens (or an error occurs).

## Notes

- The same pattern is used on the matches list page (`openInOpenDota`) and may also be broken there — worth checking both.
- Uses `@tauri-apps/plugin-opener` dynamically imported inside the click handler.
- Possible causes: plugin not configured for this page, capability missing, or async import failing silently.
