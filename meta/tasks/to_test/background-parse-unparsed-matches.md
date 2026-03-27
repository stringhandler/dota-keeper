# Background Parse Unparsed Matches

## Summary

When the app is running, automatically parse any stored matches that are still unparsed — at a slow, throttled rate — so that over time the local database is fully parsed without triggering OpenDota's rate limits.

## Motivation

Users may have hundreds of matches in the database that were fetched but never parsed (e.g. backfilled history, or matches where parsing was skipped). Today, parsing is manual. This task makes parsing happen automatically in the background so goal evaluation and analysis are accurate without user intervention.

## Acceptance Criteria

- On app start (or shortly after), the backend checks for matches with `parsed = 0` (or `parsed IS NULL`) in the database.
- It processes one unparsed match at a time, with a configurable delay between requests (default: **10 seconds** between parse attempts).
- If OpenDota returns a rate-limit error (HTTP 429), the background parser backs off and retries after a longer delay (e.g. 60 seconds).
- The parser stops when all matches are parsed, and restarts the next time the app launches.
- Parsing runs on a background Tokio task; it does **not** block the UI.
- The user sees no mandatory notification, but a subtle indicator (e.g. status text in Settings or footer) can optionally show "Parsing X matches in background…".
- The feature can be paused/disabled in Settings if the user doesn't want background network activity.

## Implementation Notes

- Tauri command to query `SELECT id FROM matches WHERE parsed = 0 ORDER BY match_id DESC` (newest first, so recent matches are parsed sooner).
- Spawn a `tokio::spawn` background loop at app startup (in `lib.rs` setup or via a dedicated `start_background_parser` command called from the frontend on mount).
- Use `tokio::time::sleep(Duration::from_secs(10))` between parse calls.
- On 429, sleep 60 s before retrying the same match.
- Reuse the existing `parse_match` / `request_parse` OpenDota logic.
- Consider adding a `parsing_in_progress` boolean to settings/state so the frontend can show a subtle indicator.

## Rate Limit Guidance

OpenDota free tier allows roughly 60 requests/minute. Parsing a match typically requires 1–2 calls. At 10 s between matches this is well within limits. At 5 s it is still safe but leaves less headroom.

## Out of Scope

- Prioritising specific matches for immediate parse (manual parse button still handles that).
- Showing a detailed parse queue UI.
