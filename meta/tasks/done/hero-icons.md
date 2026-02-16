# Hero Icons Visual Cues

## Overview
Add hero icons when displaying heroes in lists or on pages to provide better visual identification and improve user experience.

## Requirements

- Display hero icons alongside hero names in all relevant UI components
- Icons should be used in:
  - Match history lists (showing heroes played)
  - Hero performance statistics
  - Hero selection/filtering interfaces
  - Any other location where heroes are displayed

## Technical Considerations

- Source hero icons from OpenDota API or Dota 2 assets
- Ensure icons are properly sized and optimized for different contexts
- Consider caching strategy for hero icons
- Maintain consistent styling across the application

## Implementation Notes

- OpenDota provides hero icon URLs via their API
- Format: `https://cdn.cloudflare.steamstatic.com/apps/dota2/images/dota_react/heroes/{hero_name}.png`
- Should gracefully handle missing/failed icon loads with fallback text

## Acceptance Criteria

- [ ] Hero icons display correctly in all lists and pages
- [ ] Icons are appropriately sized for their context
- [ ] Performance is not negatively impacted by icon loading
- [ ] Fallback behavior works when icons fail to load
