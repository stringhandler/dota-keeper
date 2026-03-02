# Better OpenDota Error Messages

## Problem

When OpenDota API calls fail, the user sees raw technical errors like:

> "failed to parse response body"

This is meaningless to a non-developer. The user doesn't know if it's their fault, OpenDota's fault, or if they should do something about it.

## Goal

Replace raw error strings with friendly, actionable messages that:
- Explain what went wrong in plain English
- Reassure the user the app will retry automatically
- Only surface "do something" advice when the user actually needs to act

## Error Categories & Desired Messages

| Situation | Current message (example) | Desired message |
|---|---|---|
| OpenDota API down / 5xx | `failed to parse response body` | "OpenDota is unavailable right now. Your matches will sync automatically when it comes back." |
| Rate limited (429) | `request failed: 429` | "Too many requests to OpenDota. Will try again shortly." |
| Network offline | `error sending request` | "Couldn't reach OpenDota — check your internet connection. Will retry when you're back online." |
| Match not parsed yet | `no player data found` | "This match hasn't been parsed by OpenDota yet. It will be picked up on the next sync." |
| Invalid Steam ID | `empty match list` or 400 | "No matches found for this Steam ID. Double-check the ID in Settings." |
| Parse job failed | `parse request failed` | "OpenDota couldn't parse this match. It will be skipped and retried later." |

## Implementation Notes

- The error classification should live in `src-tauri/src/opendota.rs`, close to where HTTP calls are made.
- Add a helper that maps `reqwest::Error` / HTTP status codes to a `UserFacingError` enum or just a friendly `String`.
- Avoid changing the `Result<_, String>` return type of Tauri commands — just improve the string content.
- The "will try again" phrasing should only appear for transient errors (network, rate limit, 5xx), not permanent ones (bad Steam ID).

## UI Considerations

- The existing error banner / toast infrastructure is fine — just improve the message text.
- Consider keeping the raw error in a collapsible "details" section for debugging, but showing the friendly message prominently. (Optional, low priority.)

## Acceptance Criteria

- [ ] OpenDota 5xx / timeout surfaces a friendly "unavailable, will retry" message
- [ ] 429 rate limit shows a "too many requests, retrying soon" message
- [ ] Network offline shows a "check connection" message
- [ ] Invalid/unknown Steam ID shows a "check your ID in Settings" message
- [ ] No raw Rust/HTTP error strings shown to the user in normal error flows
