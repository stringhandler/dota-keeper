# Hero Item Build Timings

## Summary

On the hero detail/analysis page, show a visual breakdown of which items the user typically buys on a given hero and when — displaying purchase time ranges similar to [dota2protracker.com](https://dota2protracker.com).

## Background

dota2protracker shows per-item timing distributions across many games, visualised as horizontal bars indicating the typical window in which an item is purchased (e.g. "Power Treads bought between 5–9 min"). This gives players a clear picture of their build pacing and lets them compare against expectations.

For Dota Keeper, we can generate this from the user's own match history using OpenDota's parsed match data, which includes `purchase_log` — a per-player list of `{ key: item_name, time: seconds }` events.

## Requirements

### Data Needed
- `purchase_log` from OpenDota parsed match data for each match on the relevant hero
- Items to include: non-consumable, non-ward items (filter out `tpscroll`, `ward_observer`, `ward_sentry`, `dust`, etc.)
- Group by item name across all matches

### Visualisation
- Horizontal timeline per item showing:
  - Median purchase time (centre marker or line)
  - Min–max range (or 10th–90th percentile bar to reduce outlier noise)
  - Optional: number of games the item was purchased in (e.g. "14/20 games")
- Sort items by median purchase time (earliest at top)
- Should be scoped to a selected hero (accessible from hero stats or hero detail view)

### Filtering / UX
- Minimum game threshold before showing (e.g. at least 3 games on the hero)
- Option to filter by win/loss to compare build paths
- Show "not enough data" state gracefully if < 3 games

## Data Source

OpenDota parsed match endpoint provides `purchase_log` under the player object:

```json
{
  "purchase_log": [
    { "time": 312, "key": "power_treads" },
    { "time": 890, "key": "manta" }
  ]
}
```

This requires matches to have been parsed. If a match is unparsed, prompt the user to request parsing (same pattern as farming analysis task).

## Acceptance Criteria

- [ ] Purchase log data is retrieved and stored (or fetched on demand) per match
- [ ] Items are grouped and time statistics computed per hero across the user's matches
- [ ] Timeline visualisation renders with min/median/max (or percentile) per item
- [ ] Items sorted by median purchase time
- [ ] Consumables and wards filtered out
- [ ] Win/loss filter available
- [ ] Graceful fallback if too few games or match is unparsed
