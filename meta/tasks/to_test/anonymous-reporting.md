# Anonymous Analytics

## Overview

Add anonymous usage analytics to understand how the app is being used. Enabled by default with a clear opt-out in Settings. No personal data is ever collected.

## What to Track

- App version and platform (Windows/macOS/Linux)
- Page views / navigation (e.g. "opened Goals page", "opened Challenges page")
- Feature usage events (e.g. "goal created", "challenge accepted", "match parsed", "suggestion adopted")
- Error events (type and message only, no stack traces with personal paths)

## What NOT to Track

- Steam ID or any personally identifiable information
- Match IDs, hero choices, or specific game data
- Goal descriptions, targets, or any user-entered content

## Implementation

### Backend / Service
- Use [PostHog](https://posthog.com) (cloud-hosted)
- Events sent as anonymous HTTP POST with: `{ event, app_version, platform, timestamp }`

### Settings
- Add `analytics_enabled: bool` (default: `true`) to `Settings`
- Show an opt-out toggle in the Settings page under a "Privacy" section
- Brief description: "Help improve Dota Keeper by sending anonymous usage data. No personal information is collected."

### Frontend
- Small `analytics.js` helper: `trackEvent(name, props?)` — no-ops if disabled
- Call from Svelte pages on mount and on key user actions
- Check `analytics_enabled` setting before sending (read once on app start, cached)

### Rust
- Optionally expose a `track_event_cmd` Tauri command so Rust-side events can also be reported

## Acceptance Criteria
- [ ] Enabled by default; user can opt out in Settings
- [ ] Opt-out is respected immediately and persisted
- [ ] No PII is ever sent — verified by code review
- [ ] Analytics calls fail silently (never break the UI)
- [ ] Events are documented in code with clear names
