# Hero Icons in Dropdowns

## Overview

Show hero portrait icons next to hero names in all hero filter/select dropdowns across the app. Use the small hero icons from the Valve/Steam CDN (the same minimap-style icons already used elsewhere in the app via `HeroIcon`).

## Problem

Hero dropdowns currently show plain text names only, making it harder to quickly identify heroes visually.

## Approach

Native `<select>` / `<option>` elements don't support images. Options:

1. **Replace `<select>` with a custom dropdown component** — renders a styled list with `<img>` tags next to each name. Most work but best UX.
2. **Use a lightweight existing component** — e.g. a simple popover list that mimics a select.

The custom component is the right approach since `HeroIcon` already handles the CDN image URL logic.

## Scope

Apply to all hero dropdowns:
- Goals list page (`/goals`) — hero filter
- Goal details page (`/goals/[goalId]`) — hero filter + edit form hero select
- Analysis page (`/analysis`) — hero filter
- Goal creation flow (if any inline forms)

## Acceptance Criteria
- [ ] Hero icon shown to the left of hero name in all dropdowns
- [ ] Favorites section still appears at the top
- [ ] Selected hero shows its icon in the collapsed state
- [ ] Keyboard navigation still works (or is acceptable without it)
- [ ] Consistent sizing with the rest of the UI
