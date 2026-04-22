# Test: Performance Journal

## What was done
Added a Performance Journal page (`/journal`) showing per-metric performance analysis across your match history:
- Metric selector: KDA / GPM / XPM / Damage / Deaths / LH@10
- Hero filter (All Heroes or specific hero)
- Best 5 and Worst 5 games for the selected metric, with links to match detail
- Navigation added to sidebar and bottom nav

## Steps to test

1. Navigate to **Journal** (sidebar or bottom nav).
2. Page should load with KDA selected by default, showing Best 5 and Worst 5 game tables.
3. Each row shows: hero, match date, the metric value, and a link to the match detail page.
4. Switch metrics (GPM, XPM, Damage, Deaths, LH@10) — tables should update immediately.
5. Filter by a specific hero — only that hero's matches should appear in the tables.
6. Click a match row → navigates to the correct match detail page.
7. On mobile (≤640px): page should be readable with no horizontal overflow.
8. With no matches: page should show an empty state (no crash).
