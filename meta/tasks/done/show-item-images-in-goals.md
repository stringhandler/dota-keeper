# Show Item Images in Item Timing Goals

## Feature Request

When displaying item timing goals, show the actual Dota 2 item image instead of (or in addition to) just the item name text.

## Current State

Item timing goals currently display only text (e.g., "Battle Fury", "Black King Bar").

## Desired State

Display the actual Dota 2 item icon/image for each item timing goal, making it more visually appealing and easier to recognize at a glance.

## Implementation Approach

1. **Item Images Source**
   - Use OpenDota CDN for item images: `https://cdn.cloudflare.steamstatic.com/apps/dota2/images/dota_react/items/{item_name}.png`
   - Or use local assets if preferred for offline support

2. **Create ItemIcon Component** (similar to HeroIcon)
   - Create `src/lib/ItemIcon.svelte`
   - Takes `itemId` or `itemName` as prop
   - Maps to the correct image URL
   - Handles loading/error states

3. **Update Display Components**
   - Update goal display components to use ItemIcon
   - Show image alongside or instead of text name
   - Ensure consistent sizing across the UI

4. **Item Name Mapping**
   - Verify item IDs map correctly to image names
   - Handle any special cases (spaces, underscores, etc.)
   - Reference `src-tauri/src/items.rs` for item definitions

## Files to Create/Modify

- `src/lib/ItemIcon.svelte` (new component)
- Components that display item timing goals
- Dashboard hero suggestion component (if it shows items)
- Goal detail views
- Any UI showing item achievements

## Acceptance Criteria

- [ ] ItemIcon component created and working
- [ ] Item images display correctly for all trackable items
- [ ] Images are appropriately sized and styled
- [ ] Fallback handling for missing/failed image loads
- [ ] Works in all places where item timing goals are shown
- [ ] Visual consistency with existing HeroIcon component

## Reference

Similar to how `HeroIcon.svelte` displays hero avatars, create an equivalent for items.
