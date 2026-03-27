# Match Details: Player Performance Chart

## Summary

On the match details page, add a chart showing all 10 players' early-game stats with configurable filters.

## Requirements

### Chart Data (per player)
- Last hits at selected time
- Denies at selected time
- Net worth (gold) at selected time
- Experience at selected time

### Filters
- **Chart type**: Bar chart, line chart (progression over time), radar chart
- **Time snapshot**: 5 min, 10 min, 15 min, 20 min

### Display
- Show all 10 players grouped (Radiant vs Dire, or side-by-side)
- Player name/hero visible on chart
- Highlight the tracked user's bar/line

## Data Source

OpenDota match API already returns `benchmarks` and `laning` data per player. The `radiant_gold_adv` and `radiant_xp_adv` arrays track minute-by-minute data. Per-player `gold_t` and `xp_t` arrays provide individual player net worth and XP at each minute.

Tauri command needed: extend or add a command to return per-player `gold_t[N]`, `xp_t[N]`, `lh_t[N]`, `dn_t[N]` (last hits/denies timeline arrays) from stored match data or re-fetched from OpenDota.

## Acceptance Criteria

- [ ] Chart renders on match details page
- [ ] All 10 players visible
- [ ] Time filter buttons: 5m, 10m, 15m, 20m
- [ ] Chart type selector: Bar / Line / Radar
- [ ] Switching filters updates chart without page reload
- [ ] Current user's data visually highlighted
