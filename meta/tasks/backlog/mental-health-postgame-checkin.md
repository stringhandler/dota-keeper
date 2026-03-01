# Mental Health: Post-Game Mood Check-In

## Epic
Part of: [epic-mental-health-tilt-tracking.md](epic-mental-health-tilt-tracking.md)

## Priority
**HIGH** (first task to implement in the epic)

## Overview

A lightweight, non-intrusive check-in that appears occasionally after a game is detected. It uses indirect, emoji-scale questions rather than clinical language — the goal is to gather mood signals without the user feeling surveilled or judged.

## When Check-Ins Appear

Not after every game (that would be exhausting). Trigger logic:

- **Always** after a 3+ loss streak (most important time to check in)
- **Randomly** after ~1 in 4 games in a session (25% chance, seeded per session)
- **After a long session** (4+ games played without quitting the app)
- **Never more than once per session** unless triggered by a loss streak

The check-in is shown on the **Dashboard** the next time the user opens the app after a qualifying match, or immediately if the app is open when matches are refreshed.

## UI Design

The check-in appears as a **dismissible card** at the top of the Dashboard (below the stats strip). It is non-blocking — the user can dismiss it and continue using the app.

```
┌─────────────────────────────────────────────┐
│  How was that session?                      │
│                                             │
│  How energised did you feel?                │
│  😴  😐  🙂  😄  ⚡                         │
│   1   2   3   4   5                        │
│                                             │
│  How calm were you?                         │
│  😤  😠  😐  😊  🧘                         │
│   1   2   3   4   5                        │
│                                             │
│  [Skip]                  [Submit]           │
└─────────────────────────────────────────────┘
```

### Questions (Indirect Framing)

**Q1: Energy** — "How energised did you feel?" (😴→⚡)
- Detects fatigue / burnout signal
- Not "are you tired?" — softer, about the *gaming experience*

**Q2: Calm** — "How calm were you?" (😤→🧘)
- Detects frustration / tilt signal
- Not "were you angry?" — focuses on felt calmness, easier to self-report

These two questions take ~5 seconds. No typing required.

### Optional Third Question (shown only if Q2 ≤ 2)
**Q3: Attribution** — "What got under your skin?"
- Options: Teammates · Enemy · My own mistakes · Nothing, I was fine actually
- Multiple choice, helps distinguish external vs internal tilt sources

## Implementation

### Frontend

1. Add a `MoodCheckin.svelte` component
2. Shown on Dashboard when `pendingCheckin` state is truthy
3. Answers submitted via Tauri invoke: `save_mood_checkin`
4. On submit or skip: dismiss card, update state so it doesn't show again this session

### Backend (Rust)

1. Add `mood_checkins` table to SQLite schema (see epic for DDL)
2. `save_mood_checkin(match_id, energy, frustration)` command
3. `get_pending_checkin()` command — returns the most recent match that hasn't had a check-in, if check-in conditions are met
4. `dismiss_checkin(match_id)` command — marks match as skipped

### Settings Gate

Only show check-ins if mental health tracking is enabled in Settings (see `mental-health-settings.md`).

## Acceptance Criteria

- [ ] Check-in card appears on Dashboard after qualifying games
- [ ] Check-in never appears if mental health tracking is disabled in settings
- [ ] Check-in never appears more than once per session (unless loss streak triggers)
- [ ] "Skip" immediately dismisses and records a skip (no data stored beyond "skipped")
- [ ] "Submit" stores energy + calm scores linked to the most recent match
- [ ] Optional Q3 appears only when calm score is 1 or 2
- [ ] Card is hidden in privacy mode (see `mental-health-privacy-mode.md`)
- [ ] Check-in looks consistent with the app's dark gold design system
- [ ] Works on mobile (emoji scale is touch-friendly, min 44px tap targets)
