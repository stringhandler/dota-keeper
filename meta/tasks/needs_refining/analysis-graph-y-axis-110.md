# Task: Analysis details graph — extend Y-axis to 110

## Description

The graph on the analysis details page currently auto-scales its Y-axis. Set the Y-axis maximum to **110** so that values near 100 (e.g. last-hit counts, percentages) are never clipped and there is consistent headroom across sessions.

## Where

- Page: Analysis → hero/metric detail (the chart showing per-game performance over time)
- Likely file: `src/routes/analysis/[heroId]/+page.svelte` or the relevant chart component

## Change

Set `scales.y.max = 110` (or `suggestedMax: 110`) in the Chart.js config for this graph.

Use `suggestedMax` if the data can occasionally exceed 110 (so the axis can still expand), or `max` if the cap should be hard.

## Acceptance Criteria

- [ ] Y-axis on the analysis details graph reaches up to 110
- [ ] Values are not clipped
- [ ] Change does not affect other charts
