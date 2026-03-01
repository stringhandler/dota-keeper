# Mental Health: Privacy Mode (Hide Tilt Data)

## Epic
Part of: [epic-mental-health-tilt-tracking.md](epic-mental-health-tilt-tracking.md)

## Priority
**MEDIUM**

## Overview

Sometimes users show their app to friends, teammates, or stream it. Mental health data is personal and they may not want others to see it. Privacy Mode hides all tilt-related UI with a single tap, without deleting any data.

## Privacy Mode Behaviour

When Privacy Mode is **active**:

| Element | Behaviour |
|---------|-----------|
| Post-game check-in card | Hidden (not shown) |
| Tilt suggestion card on Dashboard | Hidden |
| Mental Wellbeing section in Settings | Shows "Privacy Mode is ON" banner instead of data |
| Dashboard mental health summary card | Hidden |
| Any mood score displays | Replaced with "—" or hidden entirely |

When Privacy Mode is **inactive**:
- All mental health UI returns to normal
- No data is lost during privacy mode

## Activation

### Method 1: Settings Toggle
In the Mental Wellbeing settings section, a "Privacy Mode" toggle:

```
┌─────────────────────────────────────────────┐
│  Privacy Mode                               │
│  Hide all mood data from the app.           │
│  Your data is preserved.                    │
│                                             │
│  [Off]  [On]                               │
└─────────────────────────────────────────────┘
```

### Method 2: Quick Access (Recommended Addition)
A discreet **lock icon** somewhere accessible on mobile — e.g. in the topbar or Settings tab — that toggles privacy mode with one tap. This is useful for quick toggling before handing the phone to someone.

Suggestion: small 🔒 icon in the topbar, right-most action, only visible when mental health tracking is enabled.

```
[Page Title]   [⚡ Challenges]  [🔒]
```

Tapping 🔒 toggles privacy mode. When active, icon becomes 🔓 with a subtle gold highlight to remind the user it's on.

## Persistence

Privacy mode state is stored in settings (persists across sessions). This is intentional — if a user enables it before an in-person event, they don't want it disabling itself on restart.

## Implementation Notes

- `privacy_mode_active: bool` added to settings
- All mental health components check this flag (passed as prop or via a Svelte store)
- The flag is readable without enabling tracking (privacy mode can be pre-enabled before ever turning on tracking)

## Acceptance Criteria

- [ ] Privacy mode can be toggled from Settings
- [ ] When active, all check-in cards and suggestion cards are hidden from the UI
- [ ] Mood data is NOT deleted when privacy mode is enabled
- [ ] Disabling privacy mode restores all mental health UI
- [ ] Privacy mode state persists across app restarts
- [ ] Quick-access toggle (lock icon) is available in topbar when tracking is enabled
- [ ] Lock icon has clear visual state for active/inactive privacy mode
- [ ] Works correctly on mobile (lock icon is tappable, ≥44px touch target)
