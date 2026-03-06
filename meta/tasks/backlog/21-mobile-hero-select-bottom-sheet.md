# Hero Select: Bottom Sheet on Mobile

## Priority
**LOW-MEDIUM** — Usability improvement for a frequently-used component

## Problem

The `HeroSelect` component ([src/lib/HeroSelect.svelte](src/lib/HeroSelect.svelte)) renders a custom dropdown that opens downward. On mobile:

1. The dropdown opens within the form area — on small screens this can be clipped or extend off-screen
2. The touch targets for each hero in the list may be too small (the list items are designed for hover/mouse interaction)
3. The search input inside the dropdown is difficult to use when the keyboard pops up, as it shifts the layout and may push the dropdown off-screen

The hero list is long (100+ heroes), making it unwieldy in a small inline dropdown.

## Solution

On mobile, render `HeroSelect` as a **bottom sheet** instead of a dropdown:

```
┌─────────────────────────────┐
│ Select Hero                 │  ← Header
│ [🔍 Search heroes...]       │  ← Sticky search
├─────────────────────────────┤
│ ★ Favorites                 │
│   Anti-Mage                 │
│   Juggernaut                │
├─────────────────────────────┤
│ Any Role                    │
│   Any Core (pos 1/2/3)      │
│   Any Carry (pos 1)         │
│   Any Support (pos 4/5)     │
├─────────────────────────────┤
│ All Heroes (A–Z)            │
│   Abaddon                   │
│   Alchemist                 │
│   ...                       │
└─────────────────────────────┘
```

**Bottom sheet behaviour:**
- Slides up from the bottom (80% screen height)
- Backdrop dims the rest of the screen
- Search input at the top (auto-focused)
- Hero items: 48px tall minimum (comfortable touch targets)
- Drag handle at top to dismiss

### Implementation

In `HeroSelect.svelte`, detect mobile via a `isMobile` prop or `matchMedia`:

```svelte
<script>
  let isMobile = typeof window !== 'undefined' && window.innerWidth < 640;
</script>

{#if isMobile}
  <!-- Trigger button -->
  <button class="hero-select-trigger" onclick={() => sheetOpen = true}>
    {selectedLabel}
  </button>

  <!-- Bottom sheet -->
  {#if sheetOpen}
    <div class="sheet-overlay" onclick={() => sheetOpen = false}>
      <div class="sheet" onclick|stopPropagation>
        <div class="sheet-handle"></div>
        <div class="sheet-header">Select Hero</div>
        <input type="search" bind:value={searchQuery} placeholder="Search..." autofocus />
        <!-- hero list -->
      </div>
    </div>
  {/if}
{:else}
  <!-- existing dropdown -->
{/if}
```

## Acceptance Criteria

- [ ] On mobile, tapping the hero selector opens a bottom sheet
- [ ] Bottom sheet has a search input that auto-focuses
- [ ] Hero items have adequate touch target height (≥44px)
- [ ] Selecting a hero closes the sheet and updates the value
- [ ] Tapping the backdrop closes the sheet
- [ ] Desktop dropdown behaviour is unchanged
- [ ] The sheet handles the keyboard appearing without breaking layout
