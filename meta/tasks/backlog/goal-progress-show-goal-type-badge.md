# Show Goal Type (Weekly vs Standard) in Goal Progress Panel

## Problem Statement

The Goal Progress panel displays all goals together without any visual distinction between weekly goals (time-limited, reset each week) and standard goals (ongoing, no expiry). Users can't tell at a glance which goals have a deadline and which are long-term.

## Requirements

### Visual Indicators

Each goal card/row in the Goal Progress panel should display a badge or label indicating its type:

**Weekly Goal:**
- Badge: `📅 Weekly` or `W` pill in a distinct colour (e.g. purple/blue)
- Optionally show days remaining: `📅 Weekly · 3 days left`

**Standard Goal:**
- Badge: `Standard` or no badge at all (weekly is the exception, standard is the default)
- Simpler approach: only badge weekly goals, leave standard goals unmarked

### Recommended Approach: Badge Weekly Goals Only

Since standard is the default, only weekly goals need a badge. This reduces visual noise:

```
┌─────────────────────────────────────────┐
│ [AM icon] 70 CS by 10 min       68%     │  ← standard, no badge
│ [Any    ] Win rate > 55%  [📅 Weekly]   │  ← weekly badge shown
│ [Invoker] KDA > 3.5              55%    │  ← standard, no badge
│ [Any    ] Deaths < 6      [📅 3d left]  │  ← weekly with countdown
└─────────────────────────────────────────┘
```

### Badge Styling

```css
.badge-weekly {
    font-size: 0.7rem;
    padding: 0.15rem 0.5rem;
    border-radius: 9999px;
    background: rgba(139, 92, 246, 0.15);  /* purple tint */
    color: #8b5cf6;
    border: 1px solid rgba(139, 92, 246, 0.3);
    white-space: nowrap;
}

.badge-weekly--expiring {
    /* When <= 2 days remaining */
    background: rgba(239, 68, 68, 0.15);   /* red tint */
    color: #ef4444;
    border-color: rgba(239, 68, 68, 0.3);
}
```

### Implementation

```svelte
{#if goal.is_weekly}
    <span class="badge-weekly" class:badge-weekly--expiring={daysLeft <= 2}>
        📅 {daysLeft !== null ? `${daysLeft}d left` : 'Weekly'}
    </span>
{/if}
```

## Success Criteria

- [ ] Weekly goals are clearly marked with a badge in the Goal Progress panel
- [ ] Standard goals are visually distinct (no badge, or a neutral badge)
- [ ] Badge turns red/warning colour when a weekly goal is expiring soon (≤ 2 days)
- [ ] Badge is compact and doesn't disrupt card layout

## Priority
**LOW** - Visual clarity improvement

## Estimated Complexity
**Very Low** - Conditional badge render using existing goal type data
