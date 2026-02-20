# Add Largo as a Hero

## Description
Add Largo to the hero database/roster in Dota Keeper.

## Context
Hero needs to be added to the system for tracking, goal setting, and analysis features.

## Implementation Steps

1. **Identify Hero Details**
   - Verify hero name is "Largo" (not Phantom Lancer or another hero)
   - Get official hero ID from OpenDota/Steam API
   - Confirm hero attributes (primary attribute, role, etc.)

2. **Update Hero Data**
   - Add hero to hero list/database if maintained locally
   - Ensure hero icon/image is available
   - Add to hero selection dropdowns across the app

3. **Test Integration**
   - Verify hero appears in all dropdowns (Goals, Analysis, Filters)
   - Test goal creation with this hero
   - Verify match tracking works correctly
   - Check hero-specific statistics display properly

## Files Likely Affected
- Hero data constants/configuration files
- Hero selection components
- Any hardcoded hero lists

## Notes
- Verify this is the correct hero name (not "Phantom Lancer" or another hero with a similar name)
- Check if OpenDota API already includes this hero in their data
- May need to update hero icons/assets if not automatically fetched

## Priority
Low - Cosmetic/data addition
