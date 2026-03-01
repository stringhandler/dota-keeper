# Mental Health: Settings Toggle (Enable/Disable Tracking)

## Epic
Part of: [epic-mental-health-tilt-tracking.md](epic-mental-health-tilt-tracking.md)

## Priority
**HIGH** (must ship with the first mental health feature)

## Overview

Mental health tracking must be fully opt-in. Users must be able to enable or disable it at any time from Settings, and disabling it must immediately stop all check-ins and hide all tilt-related UI.

## Settings Page Changes

Add a new "Mental Wellbeing" section to [src/routes/settings/+page.svelte](src/routes/settings/+page.svelte):

```
┌─────────────────────────────────────────────┐
│  Mental Wellbeing                           │
├─────────────────────────────────────────────┤
│  Post-game mood check-ins                   │
│  Get occasional prompts to reflect on how   │
│  you felt during a session.                 │
│                                             │
│  [Off]  [On]                                │
│                                             │
│  When enabled, the app may ask 1–2 simple   │
│  questions after some games. All data is    │
│  stored locally on your device only.        │
│                                             │
│  [Clear all mood data]                      │
└─────────────────────────────────────────────┘
```

### Toggle Behaviour

- Default: **Off** (tracking is opt-in)
- Switching **On**: Shows a brief one-time explanation modal on first enable
- Switching **Off**: Immediately suppresses all check-in prompts and suggestion cards; existing data is retained (not deleted) unless "Clear all mood data" is clicked

### "Clear all mood data" Action
- Confirmation dialog: "This will permanently delete all mood check-in history. Your match data is not affected."
- On confirm: deletes all rows from `mood_checkins` table
- Shown regardless of whether tracking is on or off (allow data deletion anytime)

### First-Enable Explanation Modal
Shown once when the user first turns tracking on:

```
┌─────────────────────────────────────────┐
│  About mood check-ins                   │
├─────────────────────────────────────────┤
│  Occasionally, after a session, we'll   │
│  ask how you felt — using simple emoji  │
│  scales. This helps you spot patterns   │
│  like fatigue or frustration before     │
│  they affect your gameplay.             │
│                                         │
│  • All data stays on your device        │
│  • You can skip any check-in            │
│  • You can hide all mood data with      │
│    Privacy Mode (see below)             │
│                                         │
│  [Got it, let's go]                     │
└─────────────────────────────────────────┘
```

## Backend Changes

Add `mental_health_tracking_enabled` boolean to the settings table (default `false`). New commands:
- `get_settings` already returns settings — add the new field
- `save_settings` already handles settings — add the new field
- `clear_mood_data()` — deletes all rows from `mood_checkins`

## Acceptance Criteria

- [ ] Mental health tracking is **off** by default
- [ ] Settings section exists with a clear on/off toggle
- [ ] First-enable explanation modal shown once
- [ ] Disabling immediately hides all check-in cards and suggestion cards
- [ ] "Clear all mood data" button works with confirmation
- [ ] Clearing data does not affect match history or goals
- [ ] Settings preference persists across app restarts
