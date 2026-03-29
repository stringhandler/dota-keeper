# Match Details: Compare to Another Match

**Epic: Performance Journal**

## Description

On the match details page, allow the user to compare the current match side-by-side with another match. A "Compare to" button opens a selection popup with quick options and a manual entry field.

## Requirements

### "Compare to" Button (Match Details Page)
- Add a **Compare to** button on the match details page
- Opens a popup with the following options:
  - **Last game** — compare to the most recent match in history
  - **Last game with this hero** — compare to the most recent match played on the same hero
  - **Specific match ID** — text input to enter an arbitrary match ID manually

### Comparison View
- Once a comparison target is selected, show both matches side-by-side (or stacked on smaller layouts)
- Highlight differences in key stats (KDA, GPM, XPM, deaths, hero damage, etc.)
- Make it clear which match is "this match" vs "comparison match"

### Compare Buttons in Matches List (per Goal)
- In the matches list popup/modal (used on goal details or elsewhere), add a **Compare to best** button for each match row
- "Best" = the best match for that goal according to the goal's metric
- Clicking it navigates to / opens the match details page in comparison mode with that goal's best match pre-selected

## Acceptance Criteria
- [ ] "Compare to" button visible on match details page
- [ ] Popup shows: Last game, Last game with this hero, Specific match ID input
- [ ] Selecting any option loads the comparison view
- [ ] Comparison view clearly labels both matches and highlights stat differences
- [ ] Matches list popup has a "Compare to best" button per match for each goal context
- [ ] "Compare to best" opens match details in comparison mode with the goal's best match
