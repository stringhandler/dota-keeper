# Add Placeholder Icon for "Any Hero" Goals in Goal Progress Table

## Problem Statement

In the Goal Progress table, goals that target a specific hero display a hero icon. Goals that apply to all heroes ("any hero") have no icon, causing the text content in that row to be misaligned compared to rows that do show an icon.

## Current State (Problem)

```
┌──────────────────────────────────────────┐
│ Goal Progress                            │
├──────────────────────────────────────────┤
│ [🗡️ Anti-Mage] 70 CS by 10 min   68%   │  ← Aligned (has icon)
│              Win rate > 55%        72%   │  ← Misaligned (no icon)
│ [⚡ Invoker ] KDA > 3.5           55%   │  ← Aligned (has icon)
│              Deaths < 6            80%   │  ← Misaligned (no icon)
└──────────────────────────────────────────┘
```

## Desired State (Solution)

```
┌──────────────────────────────────────────┐
│ Goal Progress                            │
├──────────────────────────────────────────┤
│ [🗡️ Anti-Mage] 70 CS by 10 min   68%   │  ← Aligned
│ [⚔️ Any Hero ] Win rate > 55%     72%   │  ← Now aligned
│ [⚡ Invoker  ] KDA > 3.5          55%   │  ← Aligned
│ [⚔️ Any Hero ] Deaths < 6         80%   │  ← Now aligned
└──────────────────────────────────────────┘
```

## Requirements

- When a goal has no hero filter (applies to all heroes), show a generic placeholder icon in the icon column
- Icon should clearly communicate "any hero" or "all heroes"
- Same size and alignment as specific hero icons
- No tooltip required, but a simple "Any hero" tooltip on hover is a nice touch

## Suggested Icon Options

- A generic sword/shield icon from the existing icon set
- A "★" or "◈" symbol
- A silhouette/generic hero outline
- The Dota 2 logo mark
- Text fallback: `ALL` in a small badge

## Implementation

In the Goal Progress table component, where the hero icon is rendered:

```svelte
{#if goal.hero}
    <img src={heroIconUrl(goal.hero)} alt={goal.hero} class="hero-icon" />
{:else}
    <span class="hero-icon hero-icon--any" title="Any hero">
        <!-- generic icon svg or img here -->
    </span>
{/if}
```

```css
.hero-icon {
    width: 32px;   /* match existing hero icon size */
    height: 32px;
    flex-shrink: 0;
}

.hero-icon--any {
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.4;
    font-size: 1.2rem;
}
```

## Success Criteria

- [ ] "Any hero" goals show a placeholder icon in the hero icon column
- [ ] Table rows are consistently aligned regardless of whether a goal has a hero filter
- [ ] Placeholder icon is visually distinct from specific hero icons (clearly generic)
- [ ] Icon size matches hero icon size exactly

## Priority
**LOW** - Visual polish / alignment fix

## Estimated Complexity
**Very Low** - Single conditional in the table row template
