# Integrate with Dota Cheater DB

## Problem Statement

Dota 2 has a significant problem with smurfs, boosters, and cheaters. Integrating with an external cheater/smurf database would allow Dota Keeper to flag suspicious players in match history, helping the user understand whether a loss was influenced by cheating opponents or teammates.

## Requirements

- Identify which cheater/smurf database(s) to integrate with (e.g. DotaCheaterDB, community-maintained lists)
- For each match, check whether any player (enemy or ally) appears in the database
- Display a warning or flag on the match card / match details page when a known cheater/smurf was present
- Cache lookups to avoid excessive API calls

## Open Questions

- Which database/API to use? (needs research)
- Is the data reliable enough to display without false-positive risk?
- Should flagging be per-player or per-match?
- Opt-in or always-on?

## Priority

**LOW** — Nice-to-have context enrichment
