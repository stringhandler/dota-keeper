# Toast / Snackbar Notification System

## Priority
**MEDIUM** — Significant UX improvement; no feedback on most user actions

## Problem

Currently there is no feedback when users complete actions:
- Goal created → form resets silently, list refreshes
- Goal updated → same
- Goal deleted → no confirmation the deletion succeeded
- Matches refreshed → button text changes to "Refreshing..." then back, but no success message

On mobile this is especially jarring — users don't know if their action worked.

The goal delete flow also uses the browser's native `confirm()` dialog (in [src/routes/goals/+page.svelte](src/routes/goals/+page.svelte)):

```js
async function deleteGoal(goalId) {
  if (!confirm("Delete this goal?")) return;
  ...
}
```

Native `confirm()` looks out of place in a Tauri app and is especially poor on mobile.

## Solution

### 1. Create a `Toast.svelte` component

A lightweight snackbar that appears at the bottom of the screen (above the BottomNav on mobile):

```
┌─────────────────────────────┐
│ ✓  Goal created             │  ← appears for 3s then fades
└─────────────────────────────┘
```

**Variants:**
- `success` — green left border, checkmark icon
- `error` — red left border, X icon
- `info` — gold left border, info icon

**Behaviour:**
- Auto-dismiss after 3 seconds
- Swipe to dismiss on mobile
- Stack if multiple toasts (max 3 visible)
- Position: bottom-center, above BottomNav (`bottom: calc(72px + env(safe-area-inset-bottom))` on mobile)

### 2. Create a `toast` store / utility

```js
// src/lib/toast.js
import { writable } from 'svelte/store';

export const toasts = writable([]);

export function showToast(message, type = 'success', duration = 3000) {
  const id = Date.now();
  toasts.update(t => [...t, { id, message, type }]);
  setTimeout(() => {
    toasts.update(t => t.filter(toast => toast.id !== id));
  }, duration);
}
```

### 3. Replace `confirm()` for delete

Instead of `confirm("Delete this goal?")`, show an inline confirmation state on the goal row:

```
[Edit]  [Delete]
         ↓ on click
[Edit]  [Confirm delete?  Yes  No]
```

This is more mobile-friendly and consistent with the app's design system.

### 4. Add toasts to key actions

In [src/routes/goals/+page.svelte](src/routes/goals/+page.svelte):
- Goal created → `showToast("Goal created")`
- Goal updated → `showToast("Goal updated")`
- Goal deleted → `showToast("Goal deleted")`

In [src/routes/matches/+page.svelte](src/routes/matches/+page.svelte):
- Refresh complete → `showToast("Matches updated")`

## Acceptance Criteria

- [ ] Toast appears after: create goal, update goal, delete goal, successful match refresh
- [ ] Toast auto-dismisses after 3 seconds
- [ ] Toast is positioned above BottomNav on mobile (not obscured)
- [ ] Delete goal no longer uses browser `confirm()` — uses inline confirmation or modal
- [ ] Error toasts appear for failed actions (in addition to current error banner)
- [ ] Multiple toasts stack correctly
