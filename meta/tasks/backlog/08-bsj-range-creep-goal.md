# BSJ SMART Goal: Range Creep Positioning

## Goal Description
Add support for BSJ's SMART goal: "Be in position to get 9/10 range creeps"

## Background
This is a fundamental laning mechanics goal from BSJ's SMART goal system. The range creep dies first in each creep wave and is worth more gold/XP, making it a high-priority last hit. This goal measures whether the player is properly positioned to secure these crucial last hits.

## Implementation Requirements

### Data Needed
- Range creeps secured per game (last hits on range creeps)
- Total range creeps available (based on game duration and lane phase)
- Timing: Typically measured during laning phase (0-10 minutes)

### Goal Parameters
- **Target**: 90% of range creeps (9/10)
- **Time Window**: Laning phase (first 10 minutes)
- **Success Criteria**: Player gets the last hit on 9 out of every 10 range creeps

### Challenges
- **API Limitation**: OpenDota/Steam API may not provide granular "range creep last hits" data
- May need to estimate based on:
  - Total last hits in first 10 minutes
  - Average last hits per wave
  - Lane matchup difficulty

### Suggested Approach
1. Research what data is available from OpenDota API for laning phase
2. If exact range creep data isn't available, determine best approximation method
3. Implement goal tracking with available data
4. Add to goal templates/suggestions for users

### Related Goals
- Total last hits in first 10 minutes
- Last hit percentage in lane
- Lane dominance metrics

## Priority
Medium - Good example of a specific, measurable laning goal that follows the SMART framework

## Notes
- This aligns with the 75% achievement rate principle (goal is 90%, which is challenging)
- Could be part of a larger "BSJ SMART Goals" preset collection
- Consider adding educational context about WHY range creeps matter
