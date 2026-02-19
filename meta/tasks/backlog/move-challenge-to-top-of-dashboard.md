# Move Today's Challenge to Top of Dashboard UI

## Problem Statement

The daily challenge widget is currently positioned below other dashboard content. Since it's a time-sensitive, action-oriented feature (must be completed today), it should be the first thing the user sees when opening the app.

## Requirements

- Move the "Today's Challenge" card to the top of the dashboard (`/` route, `+page.svelte`)
- It should appear above the goal calendar, hero suggestion, and any other widgets
- Keep all existing functionality intact (progress bar, streak, countdown timer)

## Acceptance Criteria
- [ ] Daily challenge widget appears at the top of the dashboard
- [ ] All other widgets remain below it in their existing order
- [ ] No visual regressions
