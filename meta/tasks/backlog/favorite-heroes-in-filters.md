# Show Favorite Heroes at Top of Filter Lists

## Problem Statement

When filtering matches or goals by hero, users currently have to scroll through an alphabetically sorted list of all 100+ Dota 2 heroes to find the ones they play most often. Since users typically play a small subset of heroes regularly, having their favorite heroes at the top of filter dropdowns would significantly improve UX.

## Requirements

### Visual Hierarchy
Filter lists should display in this order:
1. **"All Heroes"** - Default option
2. **Favorites section** - Starred/favorited heroes (if any exist)
   - Visual separator or header: "⭐ Favorites" or "Your Favorites"
   - Show 1-N favorite heroes
3. **All heroes section** - Complete alphabetical list
   - Optional header: "All Heroes" or simple divider
   - Full hero pool sorted alphabetically

### Example UI
```
┌─────────────────────────┐
│ All Heroes              │ ← Default/first option
├─────────────────────────┤
│ ⭐ Favorites            │ ← Section header
│   Anti-Mage             │
│   Invoker               │
│   Phantom Assassin      │
├─────────────────────────┤
│ All Heroes              │ ← Section header (optional)
│   Abaddon               │
│   Alchemist             │
│   Ancient Apparition    │
│   Anti-Mage             │ ← Appears again in full list
│   ...                   │
└─────────────────────────┘
```

### Interaction Behavior

#### Option 1: Favorites Only at Top (Recommended)
- Favorites appear ONLY in the favorites section
- They are removed from the main alphabetical list
- Cleaner, no duplication
- Easier to scan

#### Option 2: Favorites in Both Sections
- Favorites appear at top AND in alphabetical position
- Visual indicator (star icon) in main list
- More redundant but ensures findability both ways
- User can find hero either by memory (favorites) or alphabetically

**Recommendation**: Start with Option 1, add Option 2 if users request it

### Where to Apply

Apply this pattern to all hero filter dropdowns:
- **Match filtering**: Filter match history by hero
- **Goal filtering**: View goals for specific heroes
- **Statistics pages**: Filter stats by hero
- **Hero detail navigation**: Quick jump to favorite heroes
- **Goal creation**: Quick select hero when creating hero-specific goal

### Empty State

If user has no favorites:
- Skip the favorites section entirely
- Show only "All Heroes" + alphabetical list
- No visual clutter or empty sections

## Technical Implementation

### Data Requirements

#### Favorites Storage (Already Exists)
```sql
-- Assuming favorites are already tracked in database
SELECT hero_id FROM hero_favorites WHERE user_id = ?;
```

#### Hero Data
```rust
struct HeroFilterOption {
    id: u32,
    name: String,
    is_favorite: bool,
    icon_url: Option<String>,
}
```

### Filter Component Updates

#### Svelte Component Structure
```svelte
<script>
    export let selectedHero = null;
    export let favorites = []; // Array of favorite hero IDs
    export let allHeroes = []; // All heroes from database

    // Build structured list
    $: heroOptions = buildHeroOptions(favorites, allHeroes);

    function buildHeroOptions(favs, all) {
        const favoriteHeroes = all.filter(h => favs.includes(h.id));
        const nonFavoriteHeroes = all.filter(h => !favs.includes(h.id));

        return {
            favorites: favoriteHeroes.sort((a, b) => a.name.localeCompare(b.name)),
            all: nonFavoriteHeroes.sort((a, b) => a.name.localeCompare(b.name))
        };
    }
</script>

<select bind:value={selectedHero}>
    <option value={null}>All Heroes</option>

    {#if heroOptions.favorites.length > 0}
        <optgroup label="⭐ Favorites">
            {#each heroOptions.favorites as hero}
                <option value={hero.id}>
                    {hero.name}
                </option>
            {/each}
        </optgroup>
    {/if}

    <optgroup label="All Heroes">
        {#each heroOptions.all as hero}
            <option value={hero.id}>
                {hero.name}
            </option>
        {/each}
    </optgroup>
</select>
```

### Alternative: Custom Dropdown Component

For better visual control (beyond native `<select>`):

```svelte
<script>
    let isOpen = false;
    let searchQuery = '';

    // Filtered + sorted options based on search
    $: filteredOptions = filterHeroes(searchQuery, heroOptions);
</script>

<div class="hero-dropdown">
    <button on:click={() => isOpen = !isOpen}>
        {selectedHero?.name || 'All Heroes'} ▼
    </button>

    {#if isOpen}
        <div class="dropdown-menu">
            <input
                type="text"
                placeholder="Search heroes..."
                bind:value={searchQuery}
            />

            <div class="option" on:click={() => selectHero(null)}>
                All Heroes
            </div>

            {#if heroOptions.favorites.length > 0}
                <div class="section-header">⭐ Favorites</div>
                {#each heroOptions.favorites as hero}
                    <div class="option" on:click={() => selectHero(hero)}>
                        <img src={hero.icon_url} alt={hero.name} />
                        {hero.name}
                    </div>
                {/each}
            {/if}

            <div class="section-header">All Heroes</div>
            {#each heroOptions.all as hero}
                <div class="option" on:click={() => selectHero(hero)}>
                    <img src={hero.icon_url} alt={hero.name} />
                    {hero.name}
                </div>
            {/each}
        </div>
    {/if}
</div>
```

### Backend Changes

#### API Endpoint Update
```rust
#[tauri::command]
fn get_hero_filter_options(user_id: u32) -> Result<HeroFilterOptions, String> {
    let favorites = db::get_favorite_heroes(user_id)?;
    let all_heroes = db::get_all_heroes()?;

    Ok(HeroFilterOptions {
        favorites,
        all_heroes,
    })
}
```

#### Data Structure
```rust
#[derive(Serialize)]
struct HeroFilterOptions {
    favorites: Vec<Hero>,
    all_heroes: Vec<Hero>,
}

#[derive(Serialize)]
struct Hero {
    id: u32,
    name: String,
    icon_url: String,
    is_favorite: bool,
}
```

## User Experience Enhancements

### Quick Actions
- **Star/Unstar from filter**: Allow favoriting directly from dropdown
- **Reorder favorites**: Drag-and-drop or manual ordering
- **Favorite count badge**: Show "⭐ Favorites (5)" in header

### Search Enhancement
If using custom dropdown with search:
- Search matches both favorites and all heroes
- Maintain section separation even in search results
- Highlight search matches

### Keyboard Navigation
- Arrow keys to navigate options
- Enter to select
- Tab to move between sections
- Type-ahead search (start typing "inv" jumps to "Invoker")

## Visual Design Considerations

### Section Dividers
- **Subtle**: Light border or background color change
- **Clear**: Section headers with icons
- **Compact**: Minimize vertical space

### Favorite Indicators
- **Star icon** (⭐) before/after hero name
- **Different background color** (subtle highlight)
- **Bold text** for favorites
- **Hero icon/portrait** for visual recognition

### Responsive Behavior
- **Desktop**: Full dropdown with search
- **Mobile**: Native select for better UX on touch devices
- **Tablet**: Adaptive based on screen size

## Success Criteria

- [ ] Favorite heroes appear at top of all hero filter dropdowns
- [ ] "All Heroes" option remains at the very top
- [ ] Clear visual separation between favorites and all heroes sections
- [ ] No favorites section shown if user has no favorites
- [ ] Favorites are removed from main alphabetical list (no duplication)
- [ ] Selecting a favorite hero correctly filters the view
- [ ] Performance is acceptable even with large hero pools

## Edge Cases

### No Favorites
- Don't show empty favorites section
- Only display "All Heroes" + alphabetical list

### All Heroes Favorited
- Show all heroes in favorites section (sorted)
- Empty "All Heroes" section? Or keep them in both?
- **Solution**: Cap favorites at reasonable number (e.g., 20) or show warning

### Favorite Removed
- Immediately update all open filter dropdowns
- Move hero from favorites section to alphabetical list

### New Hero Added to Game
- Appears in alphabetical section
- User can favorite it
- Sorting updates automatically

## Testing Checklist

- [ ] Favorites appear in correct order (alphabetically within favorites section)
- [ ] Non-favorites appear alphabetically in main section
- [ ] "All Heroes" option works correctly
- [ ] Favoriting/unfavoriting a hero updates filter lists
- [ ] Search (if implemented) works across both sections
- [ ] No duplicate heroes in the list
- [ ] Empty state (no favorites) displays correctly
- [ ] Performance with 100+ heroes is acceptable
- [ ] Keyboard navigation works
- [ ] Mobile/touch interaction works

## Future Enhancements

### Advanced Features
- **Recently played heroes**: Show "Recently Played" section between favorites and all
- **Most played heroes**: Auto-suggest heroes based on match count
- **Meta heroes**: Show "Currently Strong" based on patch data
- **Custom sections**: Let users create custom hero groups
- **Quick add to favorites**: Right-click or long-press to favorite

### Analytics
- Track which heroes users filter by most
- Suggest favoriting frequently filtered heroes
- Show filter usage statistics

## Priority
**MEDIUM** - Nice UX improvement, not critical but valuable

## Dependencies
- Existing hero favorites system
- Hero data from database
- Filter components in UI

## Estimated Complexity
**Low-Medium** - Mainly UI work, data structure already exists

## Related Features
- Hero favorites system (existing)
- Match filtering (existing)
- Goal filtering (existing)
- Statistics views (existing)
