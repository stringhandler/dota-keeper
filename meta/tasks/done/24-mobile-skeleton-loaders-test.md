# Test: Skeleton Loaders

## What was done
Replaced "Loading..." text with shimmer skeleton screens on:
- Matches page: 6 fake match rows (hero circle + name + meta chips + KDA)
- Goals page: 3 fake goal cards (title + progress bar + label)
- Dashboard: 4 stat cards + 1 section with 3 row items

## Steps to test

1. Throttle network/CPU (devtools → Performance → CPU 4x) to make loading visible.
2. Navigate to **Matches**, **Goals**, and **Dashboard** — each should show animated shimmer skeletons instead of "Loading...".
3. Once data loads, skeletons replace cleanly with real content (no layout jump).
4. Check on both mobile (≤640px) and desktop — dashboard stat row collapses to 2 columns on mobile.
5. Error states should not show skeletons (error banner shown instead).
