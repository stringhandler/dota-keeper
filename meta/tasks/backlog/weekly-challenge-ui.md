# Weekly Challenges – UI (Selection Page & Dashboard Widget)

Extracted from `weekly-challenges.md`. Depends on `weekly-challenge-progress`.

## Goal
Build the `/challenges` page for viewing/selecting weekly options, and a dashboard widget showing active progress.

## Routes
- `/challenges` — main challenge page (selection or active progress view)

## Selection View (no challenge accepted yet)
- Show 3 option cards with description, type tag, difficulty badge
- Accept button on each card
- Reroll button with remaining count ("2 rerolls remaining")
- Skip This Week button

## Active Progress View (challenge accepted)
- Challenge description + progress bar (current/target)
- Days remaining in week
- List of recent relevant matches with pass/fail indicator

## Dashboard Widget
- Card showing active weekly challenge with mini progress bar
- Link to `/challenges` for details
- Shows "Choose this week's challenge" if none accepted yet

## Acceptance Criteria
- [ ] Can view 3 options and accept one
- [ ] Reroll works and shows updated count
- [ ] Active challenge shows real-time progress
- [ ] Dashboard widget visible alongside daily widget
