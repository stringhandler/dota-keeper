# Match Details: Player Farming Pattern Analysis

## Summary

On the match details page, visualise each player's farming pattern by analysing the source breakdown of gold received throughout the game.

## Background

OpenDota's standard match endpoint does not provide gold-source granularity. This feature requires **full replay parsing** to access per-event gold income data, specifically:

- Creep kills (lane + jungle)
- Building kills
- Hero kills / assists
- Passive gold income
- Bounty runes

This likely means using the **OpenDota parse endpoint** (`POST /api/request/{match_id}`) and then pulling data from the parsed match's `gold_reasons` or raw event log — or integrating an alternative parser (e.g. Clarity, Manta).

## Requirements

### Data Needed
- Per-player gold income broken down by source over time
- Sufficient granularity to identify patterns (e.g. 1-minute or 5-minute buckets)

### Visualisation
- Show farming pattern per player (e.g. stacked area/bar chart by gold source)
- Identify pattern types:
  - **Lane farmer** — majority from lane creeps
  - **Jungler** — majority from neutral creeps
  - **Skirmisher** — significant portion from hero kills
  - **Split pusher** — significant building gold
- Optionally cluster players by pattern similarity

### Integration Consideration
- Evaluate whether OpenDota's parsed data endpoint provides enough detail
- If not, investigate embedding a lightweight replay parser (Clarity/Manta via JVM, or a Rust-native alternative)
- This is a heavier feature — parser integration may warrant its own sub-task

## Acceptance Criteria

- [ ] Gold source breakdown available per player for a parsed match
- [ ] Farming pattern chart visible on match details page
- [ ] Pattern label assigned per player (Lane / Jungle / Skirmish / Push / Mixed)
- [ ] Graceful fallback if match has not been parsed yet (prompt user to request parse)
