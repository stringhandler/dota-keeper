# Analysis Page Unreachable on Mobile

## Priority
**CRITICAL** — Core feature is completely inaccessible on mobile

## Problem

The Analysis page (`/analysis`) exists in the desktop sidebar but is **not in the BottomNav**. On mobile the sidebar is hidden and BottomNav is the only navigation. Current BottomNav tabs:

```
Home | Matches | Goals | Challenges | Settings
```

Analysis is missing. Mobile users have no way to reach it.

## Solution

Replace **Challenges** in the BottomNav with **Analysis**, and move Challenges access elsewhere. Rationale: challenges are surfaced on the Dashboard already (the challenge badge in the topbar and the challenge card), so they are discoverable without a dedicated tab.

### Option A — Swap Challenges for Analysis (Recommended)
- BottomNav becomes: `Home | Matches | Analysis | Goals | Settings`
- Challenges still accessible via Dashboard card + topbar badge
- Simpler, no new UI needed

### Option B — Use a "More" overflow tab
- BottomNav: `Home | Matches | Goals | Challenges | ···`
- Tapping `···` opens a sheet with: Analysis, Settings, Challenges History
- More discoverable but more complex to build

## Implementation (Option A)

In [src/lib/BottomNav.svelte](src/lib/BottomNav.svelte):

1. Remove the Challenges nav item
2. Add an Analysis nav item between Matches and Goals:

```js
{
  href: "/analysis",
  label: "Analysis",
  exact: false,
  icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />`
}
```

3. Ensure the Challenge topbar badge + Dashboard challenge card still link to `/challenges`

## Acceptance Criteria

- [ ] `/analysis` is reachable from the mobile BottomNav
- [ ] Active state highlights correctly on Analysis and its sub-route `/analysis/[heroId]`
- [ ] `/challenges` is still reachable via Dashboard card or topbar badge on mobile
- [ ] All 5 BottomNav tabs have adequate touch targets (min 44px height, met by existing 56px)
