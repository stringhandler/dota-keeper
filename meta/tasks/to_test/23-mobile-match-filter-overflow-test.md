# Test: Match Filter Scroll Indicator

## What was fixed
Wrapped the chip row in `.filter-chips-container` and added a right-edge fade gradient on mobile (≤640px) to indicate the filter list is horizontally scrollable.

## Steps to test

1. Open the app at ≤640px (mobile or devtools responsive mode).
2. Go to the Matches page with several heroes played (10+ different heroes ideally).
3. The filter chip row should show a subtle right-side fade gradient — indicating scrollability.
4. Scroll the chip row left/right — all chips should be accessible.
5. Active chip highlight (coloured) should still be visible even when scrolled.
6. On desktop (>640px) — no fade gradient, chips wrap normally, no regression.
