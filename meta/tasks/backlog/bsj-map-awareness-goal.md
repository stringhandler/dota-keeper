# BSJ SMART Goal: Map Awareness - Don't Lane Without Vision

## Goal Description
Add support for BSJ's SMART goal: "Don't be in lane when no enemy heroes are showing"

## Background
This is a fundamental safety and map awareness goal from BSJ's SMART goal system. The principle: if enemy heroes aren't visible on the map, they could be coming to gank you. Pushing lanes without vision of enemies is one of the most common mistakes that leads to unnecessary deaths.

**Best Practice**: When enemies are missing, farm safer areas (jungle, your side of the map) or shove the wave quickly and retreat.

## Implementation Requirements

### Data Needed
- Deaths per game
- Death locations (lane vs jungle)
- Number of enemy heroes visible at time of death
- Vision data (wards placed, map coverage)
- Farm distribution (lane vs jungle)

### Goal Parameters
- **Target**: Zero deaths in lane when 3+ enemy heroes are missing
- **Alternative**: Reduce deaths in dangerous farming positions by X%
- **Success Criteria**: Player dies less frequently due to ganks in overextended lane positions

### Challenges
- **Critical API Limitation**: OpenDota/Steam API does NOT provide:
  - Enemy hero visibility state at any given time
  - Player position moment-by-moment
  - Fog of war information
  - Exact circumstances of each death

### Possible Workarounds

#### 1. Death Pattern Analysis
- Track death locations (available in API)
- Categorize deaths by map position:
  - Deep in enemy territory
  - Farming lanes alone
  - Safe farming areas
- Goal: Reduce deaths in "dangerous lane farming" category

#### 2. Death Timing Analysis
- Analyze game time of deaths
- Cross-reference with typical gank timings
- Look for patterns of dying while farming

#### 3. Proxy Metrics
- **Total deaths per game** - General safety metric
- **Deaths while farming** vs deaths in teamfights
- **Farm distribution** - More jungle farm = safer farming patterns
- **Vision score** - More wards = better map awareness

#### 4. Manual Review Goal
- User watches their deaths in replay
- Manually categorizes: "Avoidable gank death due to poor positioning"
- Self-assessment after each game
- Track trend over time

### Suggested Approach

**Phase 1 - Simple Implementation**:
- Track total deaths per game
- Set goal: "Average < X deaths per game"
- Encourage user to focus on map awareness as the method

**Phase 2 - Enhanced (if API supports)**:
- Death location heatmap
- Categorize deaths by map zone (safe/dangerous)
- Goal: "Reduce deaths in dangerous farming zones"

**Phase 3 - Manual Enhancement**:
- Post-game death review checklist
- User marks deaths as:
  - [ ] Unavoidable teamfight death
  - [ ] Overextended farming death (GOAL VIOLATION)
  - [ ] Caught by smoke gank
  - [ ] Dove under tower
- Track "overextended farming deaths" metric

### Related Goals
- Total deaths per game < X
- Vision score (wards placed)
- Farm safety (jungle farm %)
- KDA ratio improvement
- "Don't die alone" goal

## Priority
Medium-High - Death reduction is a core improvement area, even if we can't track the exact scenario

## Notes
- This aligns with the 75% achievement rate principle
- Even without perfect tracking, focusing on this improves general map awareness
- Could combine with educational content about safe farming patterns
- Part of "BSJ SMART Goals" preset collection
- Consider adding a "Death Review" feature to the app where users can categorize their deaths after watching replay

## Implementation Recommendation
Start with simple death tracking and gradually add sophistication:
1. **v1**: "Keep deaths under X per game" with reminder about map awareness
2. **v2**: Death location tracking if API supports it
3. **v3**: Manual death categorization feature for serious players

## Educational Content Ideas
- Map showing safe/dangerous farming zones by game time
- Explanation of when to lane vs jungle farm
- "If you can't see them, they can see you" principle
- Risk/reward calculator for farming positions
