# Skeleton Loaders Replace "Loading..." Text

## Priority
**LOW** — Polish; current loading states are functional but feel primitive

## Problem

Loading states across the app use plain text:

- Dashboard: implicitly blank (no loading indicator while data loads)
- Matches: `<div class="loading-state">Loading matches...</div>`
- Goals: `<div class="loading-state">Loading goals...</div>`
- Analysis: similar text

On mobile, the transition from "Loading..." to content causes a jarring layout shift. Skeleton screens give users a preview of the layout structure while data loads, which feels faster and more polished.

## Solution

Create a set of reusable skeleton elements:

### `SkeletonLine.svelte`
A shimmer line placeholder for text:
```svelte
<!-- Usage: <SkeletonLine width="60%" /> -->
<div class="skeleton-line" style="width: {width}"></div>

<style>
  .skeleton-line {
    height: 12px;
    border-radius: 4px;
    background: linear-gradient(90deg, var(--bg-elevated) 25%, var(--border) 50%, var(--bg-elevated) 75%);
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
  }
  @keyframes shimmer {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }
</style>
```

### `SkeletonCard.svelte`
A placeholder for a match row or goal row.

### Usage

In [src/routes/matches/+page.svelte](src/routes/matches/+page.svelte):
```svelte
{#if isLoading}
  {#each Array(5) as _}
    <div class="match-row skeleton-row">
      <SkeletonLine width="40px" />    <!-- hero icon placeholder -->
      <SkeletonLine width="80px" />    <!-- hero name -->
      <SkeletonLine width="40px" />    <!-- mode -->
      <SkeletonLine width="30px" />    <!-- result -->
      <SkeletonLine width="50px" />    <!-- kda -->
    </div>
  {/each}
{:else}
  ...
{/if}
```

## Pages to Update

1. Matches list (most impactful — loads 10+ rows)
2. Goals list
3. Dashboard quick stats strip (4 stat cards)
4. Analysis page (hero breakdown)

## Acceptance Criteria

- [ ] Skeleton cards match the rough layout of real content (same heights)
- [ ] Shimmer animation plays smoothly (no jank)
- [ ] Skeletons replaced by real content without large layout shift
- [ ] Works on both mobile and desktop
- [ ] No regressions on error states (skeletons should not appear on error)
