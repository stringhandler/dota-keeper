# Fix Inconsistent Box Sizes on Goal Progress Cards

## Problem Statement

The goal progress cards/boxes on the Goal Progress section are rendering at different sizes. This creates an uneven, misaligned grid that looks unpolished. All cards should be uniform in height and/or width depending on the layout.

## Current State (Problem)

```
┌──────────────┐  ┌──────────────────────┐  ┌────────┐
│ 70 CS by 10m │  │ Win rate > 55%        │  │ KDA    │
│              │  │ on carry heroes       │  │ > 3.5  │
│ 68%   ████░  │  │                       │  │        │
│              │  │ 72%   ████████░       │  │ 55%    │
└──────────────┘  │                       │  │ ████░  │
                  └──────────────────────┘  └────────┘
```

Cards are different heights due to:
- Varying goal name lengths causing text wrapping
- Different amounts of content (hero filter, role, etc.)
- No enforced minimum or fixed height

## Desired State (Solution)

```
┌──────────────┐  ┌──────────────────────┐  ┌────────────────┐
│ 70 CS by 10m │  │ Win rate > 55%        │  │ KDA > 3.5      │
│              │  │ on carry heroes       │  │                │
│ 68%   ████░  │  │ 72%   ████████░       │  │ 55%    ████░   │
│              │  │                       │  │                │
└──────────────┘  └──────────────────────┘  └────────────────┘
```

All cards in the same row share the same height.

## Requirements

- All goal progress cards must be the same height within a row (or globally if single-column)
- Content within cards should be vertically consistent (e.g., stats always at the bottom)
- Cards should not expand indefinitely — long goal names should truncate or wrap within a fixed height
- Fix should work for both grid and list layouts

## Implementation

### CSS Grid Approach (Recommended)

Use CSS Grid's `align-items: stretch` on the container so all cards in a row match the tallest card:

```css
.goal-progress-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
    gap: 1rem;
    align-items: stretch; /* ensures all cards in a row are same height */
}

.goal-card {
    display: flex;
    flex-direction: column;
    min-height: 120px; /* enforce minimum height */
}

.goal-card__footer {
    margin-top: auto; /* push stats to bottom of card */
}
```

### Goal Name Handling

Prevent very long goal names from exploding card height:

```css
.goal-card__title {
    display: -webkit-box;
    -webkit-line-clamp: 2;       /* max 2 lines */
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
}
```

### Flexbox Alternative (List Layout)

If using a list/table rather than grid:

```css
.goal-progress-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.goal-row {
    display: flex;
    align-items: center; /* vertically center all cells */
    min-height: 64px;
}
```

## Success Criteria

- [ ] All goal progress cards are consistent heights within each row
- [ ] Long goal names are clamped/truncated rather than expanding card height
- [ ] Stats/progress bar is vertically aligned consistently across all cards
- [ ] Fix works at different screen widths (responsive)
- [ ] No content is hidden or cut off unintentionally

## Priority
**LOW** - Visual polish

## Estimated Complexity
**Very Low** - CSS-only fix
