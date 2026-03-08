# What's New Dialog on Upgrade

## Problem Statement

Users have no way of knowing what changed when the app auto-updates. A "What's New" dialog shown once after each upgrade improves discoverability and lets users appreciate new features.

## Requirements

### Trigger
- Show the dialog **once** after the app is first launched following an upgrade
- Never show again for the same version
- Detect by comparing the last-seen version (stored in localStorage or the DB) against the current app version

### Content
- Pull release notes from `CHANGELOG.md` for the current version's section
- OR embed the notes in the frontend at build time (simpler, no file I/O required)
- Show only the current version's section, not full history

### Design
- Modal overlay (dark background)
- Title: "What's New in v{version}"
- Bullet list of changes (parsed from CHANGELOG)
- Single "Got it" button to dismiss
- Optional: small version badge in the corner

### Implementation Notes

**Version tracking:**
```js
const CURRENT_VERSION = '__APP_VERSION__'; // injected at build via vite.config
const lastSeen = localStorage.getItem('last_seen_version');
if (lastSeen !== CURRENT_VERSION) {
  showWhatsNew = true;
  localStorage.setItem('last_seen_version', CURRENT_VERSION);
}
```

**Content options (pick one):**
1. **Hardcoded per-version** – maintain a `src/lib/whatsNew.js` object keyed by version string; simple but requires manual update each release
2. **Parsed from CHANGELOG at build time** – Vite plugin reads CHANGELOG.md and injects the current version's section as a string constant; automatic but more setup

Option 1 is recommended to start.

**`src/lib/whatsNew.js` structure:**
```js
export const releaseNotes = {
  '0.2.4': [
    'Goal distribution chart now shows your average performance line',
    'Achievement rate card with colour-coded status (Too Easy → Critical)',
    'Warning banner when recent pass rate drops below 75%, with one-click goal adjustment',
    'Last N games scatter chart with hover tooltips and click-to-match navigation',
  ],
  // add future versions here
};
```

**Modal placement:**
- Rendered in `+layout.svelte` so it works across all routes
- `onMount`: check version, set `showWhatsNew` state

## Acceptance Criteria

- [ ] Dialog appears exactly once per version on first launch after upgrade
- [ ] Dialog shows the correct release notes for the installed version
- [ ] Dismissing stores the version in localStorage
- [ ] Does not appear on first-ever install (no previous version to compare)
- [ ] Works correctly after auto-update

## Priority

**LOW-MEDIUM** – Nice quality-of-life feature, straightforward to implement

## Dependencies
- `CHANGELOG.md` (existing)
- `+layout.svelte` for modal placement
- App version accessible at runtime (via `@tauri-apps/api/app` or Vite env)
