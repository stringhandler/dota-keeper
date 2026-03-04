# First-Run Onboarding

## Overview

When a user logs in for the first time (no prior Steam ID stored), show a short onboarding flow that helps them get value from the app immediately. The flow should feel fast and optional — every step can be skipped, and it should complete in under 2 minutes.

## Trigger

Show the onboarding flow immediately after a successful first Steam login. Do **not** show it on subsequent logins or if Steam ID is already stored.

Detection: `settings.steam_id` was `None` before login completed, or a dedicated `onboarding_completed: bool` flag in settings (preferred — more explicit).

## Flow Steps

### Step 1 — Welcome & Goal
**"What do you want to get out of Dota Keeper?"**

Multi-select cards (choose all that apply):
- 🎯 **Track my goals** — Set targets and see if I'm hitting them
- 📊 **Analyse my performance** — Understand trends in my CS, KDA, win rate
- 🧠 **Monitor my mental game** — Spot tilt patterns and avoid burnout
- 🏆 **Daily/weekly challenges** — Stay motivated with structured targets

This tells us which features to highlight in the rest of onboarding and could optionally pre-navigate the user to the relevant section when they finish.

### Step 2 — Favourite Heroes *(skippable)*
**"Which heroes do you play most?"**

Show the hero picker (reuse `HeroSelect.svelte`). The user can pick up to 5 favourites. These are saved via the existing `toggle_favorite_hero` command.

Skip button is prominent. Copy: *"You can always set favourites later in the Analysis page."*

### Step 3 — Mental Wellbeing *(skippable)*
**"Want to track your mental game?"**

Short explanation: *"After some sessions, we'll ask 2 quick questions — like how calm you felt. It takes 5 seconds and helps you spot burnout before it hits your rank."*

- **Yes, enable it** → calls `save_mental_health_enabled(true)`, marks intro as shown
- **Maybe later** → skips, tracking stays off

Do **not** show the first-enable modal again (mark `mental_health_intro_shown = true` regardless of choice here, since this step serves the same purpose).

### Step 4 — Match History *(skippable)*
**"Want to pull in your last 100 matches?"**

Copy: *"Backfilling gives Dota Keeper historical data to build better goal suggestions and trend analysis. It takes a few minutes."*

- **Yes, backfill now** → triggers `backfill_historical_matches`, shows progress/spinner, user can wait or continue
- **Skip for now** → continues without backfilling

## UI Design

- Full-screen overlay / dedicated route (e.g. `/onboarding`) layered above the main app
- Progress indicator: step dots or "Step 2 of 4"
- Each step has: title, short description, action(s), and a "Skip" link
- Consistent with the dark gold design system — not a jarring white wizard
- On mobile: single-column layout, large tap targets

## Completion

After step 4 (or after all skips), mark `onboarding_completed: true` in settings and redirect to the Dashboard.

If the user selected "Track my goals" in step 1, consider deep-linking to the Goals page instead.

## Implementation Notes

- Add `onboarding_completed: bool` (default `false`) to the `Settings` struct
- Add `complete_onboarding()` Tauri command (sets flag)
- New SvelteKit route: `src/routes/onboarding/+page.svelte`
- Check for `!settings.onboarding_completed` in the layout guard after login, redirect to `/onboarding`
- Backfill in step 4 runs async — show a progress indicator but don't block navigation

## Acceptance Criteria

- [ ] Onboarding shown once, on first login only
- [ ] Skipping any step still completes onboarding correctly
- [ ] Hero favourites saved when selected
- [ ] Mental health tracking enabled when user opts in (intro modal suppressed)
- [ ] Backfill starts when user opts in (non-blocking)
- [ ] `onboarding_completed` persists across restarts
- [ ] Flow works on mobile (full-width, touch-friendly)
- [ ] Existing users (already have Steam ID) never see the onboarding flow
