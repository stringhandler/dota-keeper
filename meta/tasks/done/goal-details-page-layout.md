# Goal Details Page Layout Improvements

## Goal
Improve the layout and UX of the goal details page (`/goals/[goalId]`).

## Tasks

1. **Compact filter controls** — Make the filter controls as small as possible. Ideally just a date range/period selector, stripping out any unnecessary controls. Match the compact filter-row style used on the analysis pages.

2. **Reorder page content** — Move the Distribution graph to the top of the page (first visible content after filters). Move the Statistics section lower down the page.

## Notes
- Use the existing `form-select` + `filter-label` classes for compact filter styling.
- Follow the same card layout pattern as the updated analysis detail page.
