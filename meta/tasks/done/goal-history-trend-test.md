# Test: Goal History & Trend Chart

## What was done
Added a "Performance Trend" section above Statistics on the goal detail page. Shows:
- Success rate chips for Last 7d / Last 30d / All time (green/amber/red by rate vs 75%)
- Line chart: per-game value as coloured dots (green = achieved, red = missed)
- 5-game rolling average line in gold
- Dashed goal threshold line

## Steps to test

1. Go to Goals → tap any goal with ≥2 matches.
2. A **Performance Trend** section should appear above Statistics.
3. Success rate chips show %, coloured appropriately (green ≥75%, amber 50–75%, red <75%).
4. Chart shows dots (green/red) for each game over time, left = oldest.
5. Gold line = 5-game rolling average, dashed gold line = goal target.
6. Hover a dot → tooltip shows value + achieved/missed.
7. With only 1 matching game, the trend section should not appear (needs ≥2).
8. Filters (hero, period, game mode) update the chart in real time.
