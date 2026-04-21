# Match Filter Bar: Too Many Items on Mobile

## Priority
**LOW-MEDIUM** — Usability issue when user has many heroes

## Problem

The match filter bar in [src/routes/matches/+page.svelte](src/routes/matches/+page.svelte) shows one chip per hero the user has played. On mobile this scrolls horizontally, but:

1. There's no visual affordance that the list scrolls (no scroll indicator, shadow, or fade)
2. When a user has played 20+ different heroes, the chip list becomes very long to scroll through
3. The "Refresh Matches" button stacks below the chips and takes full width — if a user taps it accidentally, it triggers a network call

## Solution

### Short-term: Add scroll shadow indicators

Add a subtle right-edge fade gradient to indicate scrollability:

```css
.match-filters-wrap {
  position: relative;
}
.match-filters-wrap::after {
  content: '';
  position: absolute;
  right: 0; top: 0; bottom: 0;
  width: 32px;
  background: linear-gradient(to right, transparent, var(--bg-base));
  pointer-events: none;
}
```

### Medium-term: Filter sheet on mobile

Replace the chip row with a single "Filter" button that opens a bottom sheet with all filter options — including hero search. This is especially helpful when the hero list is long.

```
[Filter: All ▼]  [↻ Refresh]
```

Tapping "Filter" opens:
```
┌─────────────────────────────┐
│ Filter Matches       [Done] │
├─────────────────────────────┤
│ Result                      │
│ ○ All  ○ Wins  ○ Losses     │
├─────────────────────────────┤
│ Mode                        │
│ ○ All  ○ Ranked  ○ Turbo    │
├─────────────────────────────┤
│ Hero                        │
│ [🔍 Search hero...]         │
│ ○ Any Hero                  │
│ ○ Anti-Mage                 │
│ ...                         │
└─────────────────────────────┘
```

## Acceptance Criteria

- [ ] At minimum: visual indicator that the chip list scrolls horizontally (fade/shadow)
- [ ] Preferred: filter sheet on mobile when hero count > 8
- [ ] Active filter is clearly indicated (chip highlight or button label update)
- [ ] Dismissing the filter sheet without changing applies no changes
