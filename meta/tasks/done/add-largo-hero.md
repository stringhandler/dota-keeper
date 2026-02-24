# Add Recent Heroes (Ringmaster, Kez, Largo)

## Description
Verify and add recent Dota 2 heroes (Ringmaster, Kez, Largo) to Dota Keeper's hero database/roster if they're missing.

## Heroes to Check

### 1. Ringmaster (2024)
- **Name**: Ringmaster (Cogliostro Kettle)
- **Role**: Support/Disabler
- **Primary Attribute**: Intelligence
- **Character**: Master of control and deception
- **Added**: 2024 (with The International 2024 Compendium)
- **Playstyle**: Strategic positioning, crowd control, trap-laying

### 2. Kez (2024)
- **Name**: Kez (Bird Samurai)
- **Role**: Carry/Nuker
- **Character**: Flightless bird, head of the Kazurai order
- **Added**: 2024 (Act IV of Crownfall event)
- **Revealed**: The International 2024
- **Lore**: Featured in Crownfall comics

### 3. Largo (2026)
- **Name**: Largo
- **Role**: Support (Melee, Disabler, Durable)
- **Primary Attribute**: Strength
- **Character**: Shamanic bard frog with a lute, from Velu'Mar
- **Added**: Patch 7.40 (2026)
- **Notable**: Entered Captains Mode in patch 7.40c (fastest hero to enter CM)
- **Signature**: Ultimate ability "Amphibian Rhapsody" is a rhythm mini-game

## Implementation Steps

1. **Check Current Hero Coverage**
   - Launch Dota Keeper in dev mode
   - Check hero dropdowns in Goals, Analysis, and Filters
   - Verify if Ringmaster, Kez, and Largo are present
   - Note which heroes (if any) are missing

2. **Verify Hero Data from API**
   - Check if OpenDota API has all three heroes' data
   - Get official hero IDs from Steam/OpenDota for any missing heroes
   - Confirm all hero metadata is available (name, attribute, role, icon)

3. **Update Application (if needed)**
   - If heroes are fetched from API: Investigate why they're not appearing
   - If heroes are hardcoded: Add missing heroes to hero list with metadata
   - Ensure hero icons/portraits are available for all three
   - Update all hero selection dropdowns to include missing heroes

4. **Test Integration**
   - Verify all three heroes appear in all dropdowns (Goals, Analysis, Filters)
   - Test goal creation with each hero
   - Verify match tracking works correctly for games with these heroes
   - Check hero-specific statistics display properly
   - Test hero favorite/unfavorite functionality for each

## Files Likely Affected
- Hero data constants/configuration files (if maintained locally)
- Hero selection components
- Any hardcoded hero lists or mappings

## Technical Notes
- Most Dota apps fetch hero data dynamically from OpenDota/Steam API
- If Dota Keeper uses dynamic fetching, Largo may already be available
- If using static data, need to manually add hero with ID and metadata
- Hero icon should be fetchable from CDN (e.g., `cdn.dota2.com`)

## Priority
Medium - Recent heroes that players may want to track

## References
### Ringmaster
- Official announcement: https://www.dota2.com/newsentry/6759426325006881791
- Liquipedia: https://liquipedia.net/dota2/Ringmaster

### Kez
- Official: https://www.dota2.com/hero/kez
- Liquipedia: https://liquipedia.net/dota2/Kez

### Largo
- Official announcement: https://www.dota2.com/largo
- Liquipedia: https://liquipedia.net/dota2/Largo
