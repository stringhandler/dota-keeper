# Match Details: Positional Matchup (Net Worth & Last Hits vs Opponent)

## Summary

On the match details page, show a head-to-head comparison between the tracked user and their lane opponent, broken down by position. For example, Pos 1 vs enemy Pos 1, Pos 2 vs enemy Pos 2, etc. Key metrics shown are net worth and last hits.

## Requirements

### Matchup Display
- Identify each player's position (1–5) using the `lane_role` field from OpenDota match data
- Pair each Radiant player with their Dire counterpart by matching position
- Show the tracked user's matchup prominently (highlighted or placed first)

### Metrics Per Matchup
- **Net worth** — gold at a configurable time snapshot (e.g. 10 min, 20 min, end of game)
- **Last hits** — last hit count at the same time snapshot

### Layout
- Display as a side-by-side row per position:
  - Left: Radiant player (hero icon, name, net worth, last hits)
  - Centre: position label (e.g. "Pos 1")
  - Right: Dire player (hero icon, name, net worth, last hits)
- Visually indicate which side won each matchup (e.g. colour coding or an arrow)
- Highlight the tracked user's row

### Time Snapshot Filter
- Allow switching between: **10 min**, **20 min**, **end of game**
- Switching updates all matchup rows without a page reload

## Data Source

OpenDota match API provides per-player `gold_t` (net worth timeline) and `lh_t` (last hits timeline) arrays, and a `lane_role` field to determine position. All data should already be available from the stored match data or the existing OpenDota fetch.

## Acceptance Criteria

- [ ] Positional matchup section visible on match details page
- [ ] All 5 position rows shown (Pos 1–5), each pairing Radiant vs Dire by role
- [ ] Net worth and last hits shown per player per row
- [ ] Tracked user's row is visually highlighted
- [ ] Time snapshot filter (10m / 20m / end game) updates all rows
- [ ] Visual indicator showing which player leads each matchup
