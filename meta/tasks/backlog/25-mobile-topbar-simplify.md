# Simplify Mobile Topbar

## Priority
**MEDIUM** — UX polish, reduces clutter

## Problem

The topbar on mobile (52px tall) currently shows:

```
[Page Title]   [⚡ Daily Challenge: X/Y]  [+ New Goal]
```

On small screens (~390px) this is very cramped. The `+ New Goal` button is:
1. Always visible regardless of which page you're on (even irrelevant on the Matches page)
2. Duplicated by the Goals page itself, which also has a way to create goals
3. Taking valuable horizontal space from the page title

The challenge badge + goal button together make the right side of the topbar overflow on narrow devices.

## Solution

### Option A — Hide "+ New Goal" from topbar on mobile
The Goals page already handles goal creation. Remove the `+ New Goal` button from the topbar on mobile only. The `challenge-badge` alone is a reasonable single action.

In [src/routes/+layout.svelte](src/routes/+layout.svelte):
```css
@media (max-width: 640px) {
  .topbar-actions a[href="/goals"] {
    display: none;
  }
}
```

Or conditionally render in Svelte using the `isMobile` state that already exists.

### Option B — Replace both with a single icon button on mobile
Show just the challenge progress as an icon (⚡) on mobile, with the count shown in a small badge. This maximizes space for the page title.

### Option C — Move challenge badge below topbar
Show it as a slim banner beneath the topbar instead of inline. More visible and less cramped.

## Recommended Approach

Option A is simplest and lowest risk. The Goals tab in BottomNav is the canonical way to reach goal creation on mobile.

## Acceptance Criteria

- [ ] Topbar on mobile does not overflow or feel cramped on a 390px wide screen
- [ ] Page title is fully visible
- [ ] Challenge progress is still accessible on mobile (via badge or Dashboard)
- [ ] No regression on desktop topbar
