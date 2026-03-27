# Suppress Goal Lowering Suggestions

## Problem Statement

The warning banner that suggests lowering a goal (shown on the goal detail page when pass rate is low) can become annoying for users who are aware their goal is hard and don't want to be prompted. There's currently no way to dismiss it permanently.

## Requirements

### "Don't ask me again" button
- Add a **"Don't ask me again"** button alongside the existing dismiss button on the goal suggestion/warning banner
- Clicking it opens a small confirmation dialog with two options:
  - **For this goal** — suppresses suggestions for this specific goal only
  - **For all goals** — suppresses suggestions globally across all goals

### Re-enable per goal
- On the goal detail page, add a small **"Auto-suggest improvements"** toggle or button (subtle, e.g. in the filters row or near the stats section)
- When suppressed for a goal, this toggle shows as off; clicking it re-enables suggestions for that goal
- When suppressed globally, show a note that suggestions are globally disabled, with a link to Settings to re-enable

### Re-enable globally (Settings)
- Add an **"Auto-suggest improvements"** toggle in Settings (under Goal Preferences or similar section)
- Default: on

## Storage

- Per-goal suppression: `localStorage` key `suggestion_suppressed_<goalId>` = `"true"`
- Global suppression: `localStorage` key `suggestions_suppressed_all` = `"true"`
  - Alternatively, store in the Settings object (Rust side) if persistence across devices/reinstalls matters — but localStorage is fine to start

## UI Behaviour

- If suppressed (for goal or globally): hide the suggestion banner entirely
- The "Auto-suggest improvements" toggle on the goal detail page reflects the suppressed state for that goal
- Clearing suppression (toggling back on) immediately shows the banner again if conditions are met

## Acceptance Criteria

- [ ] "Don't ask me again" button appears on the goal suggestion/warning banner
- [ ] Clicking it shows a dialog with "For this goal" / "For all goals" options
- [ ] "For this goal" suppresses the banner on that goal detail page permanently (until re-enabled)
- [ ] "For all goals" suppresses the banner on all goal detail pages
- [ ] A toggle/button on each goal detail page allows re-enabling suggestions for that goal
- [ ] A global toggle in Settings re-enables suggestions for all goals
- [ ] Suppression state survives page reloads (localStorage)

## Notes

- The existing one-time dismiss (per session, tied to match count) is separate from this — keep that behaviour
- The "For all goals" option in Settings should be a simple toggle next to an "Auto-suggest improvements" label
