# Test: BenchmarkTable (Last Hitting Rank) on Mobile

## What was fixed
On mobile (≤640px), the benchmark table had fixed-width grid columns (~465px) that overflowed
without scrolling. Added `overflow-x: auto` + `min-width: 480px` to the table on mobile,
and `flex-wrap: wrap` to the medal-badge-row so the tier badge and stats don't get clipped.

## Steps to test

1. Open the app at ≤640px width.
2. Go to a match detail page for a parsed match.
3. Scroll to "Last Hitting Rank for this game".
4. Expected: tier badge (medal icon + tier name e.g. "Legend") is visible and not clipped.
5. The table below should be horizontally scrollable, showing all columns (Mean, SD, z-score, etc).
6. Also check the hero Analysis page — "Last Hitting Rank" compact badges (Ranked / Turbo) should show tier name.
