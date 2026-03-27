# Mental Health: Check-in Frequency Setting

## Overview

Add a user-configurable setting for how often the post-game mood check-in card appears. Currently the trigger logic is hardcoded (loss streak, 4+ game session, or ~25% random). This task replaces that random logic with an explicit user preference.

## New Setting

Add to the **Mental Wellbeing** section in Settings, below the on/off toggle.

Only visible when tracking is enabled.

**Label:** "How often to check in?"

**Options:**

| Value | Label | Behaviour |
|-------|-------|-----------|
| `every_game` | Every game | Always show for the most recent unchecked match |
| `every_3` | Every 3 games | Show after every 3rd unchecked match |
| `every_5` | Every 5 games | Show after every 5th unchecked match |
| `every_10` | Every 10 games | Show after every 10th unchecked match |
| `once_per_session` | Once per session | Show at most once per app session (current default-ish) |
| `after_loss` | After every loss | Show only when the most recent match was a loss |

**Loss streak override:** Regardless of frequency setting, always trigger after a 3+ loss streak (existing behaviour).

**Default:** `once_per_session`

## Backend Changes

- Add `checkin_frequency: String` to `Settings` struct (default `"once_per_session"`)
- Add `save_checkin_frequency(frequency: String)` Tauri command
- Update `get_pending_checkin()` to respect the frequency setting:
  - `every_game` — return pending match if unchecked
  - `every_3/5/10` — count unchecked matches since last successful check-in; trigger when count reaches threshold
  - `once_per_session` — existing session flag logic (handled frontend-side is fine)
  - `after_loss` — only trigger if the pending match was a loss

## Frontend Changes

- Add frequency selector to Settings → Mental Wellbeing (shown only when tracking is on)
- Use a `<select>` or segmented button group
- Save on change (no separate Save button needed — single field)
- Pass frequency to `get_pending_checkin` OR handle `once_per_session` in frontend as now

## Acceptance Criteria

- [ ] Frequency selector visible in Settings when tracking is enabled
- [ ] Hidden when tracking is disabled
- [ ] Default is `once_per_session`
- [ ] Each frequency option correctly controls when the check-in card appears
- [ ] Loss streak always overrides frequency (check-in shown regardless)
- [ ] Preference persists across restarts
- [ ] Works on mobile (selector is touch-friendly)
