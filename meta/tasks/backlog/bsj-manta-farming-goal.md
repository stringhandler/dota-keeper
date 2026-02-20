# BSJ SMART Goal: Manta Farming Efficiency

## Goal Description
Add support for BSJ's SMART goal: "Use Manta to clear creep waves as I'm walking away"

## Background
This is an advanced farming efficiency goal from BSJ's SMART goal system. The technique involves using Manta Style illusions to clear a creep wave while the hero moves to the next farming location, maximizing farm speed and map presence. This demonstrates high-level farming patterns and item usage efficiency.

## Implementation Requirements

### Data Needed
- Games where Manta Style was purchased
- Manta Style usage count per game
- Farm efficiency metrics:
  - GPM (Gold Per Minute)
  - Creep score growth rate
  - Time spent farming vs moving

### Goal Parameters
- **Target**: Use Manta on creep waves X times per game (TBD based on game length)
- **Context**: Only applicable when hero has Manta Style
- **Success Criteria**: High Manta usage correlation with efficient farming patterns

### Challenges
- **Major API Limitation**: OpenDota/Steam API likely does NOT track:
  - Specific item usage events
  - What abilities/items were used on which targets
  - Farming pattern analysis

### Possible Workarounds
1. **Indirect Measurement**:
   - Track GPM improvement after Manta purchase time
   - Compare farm rate before/after Manta
   - Use as a reminder/checklist goal rather than measurable metric

2. **Manual Tracking**:
   - User self-reports adherence to this goal after each game
   - Binary "Did I focus on this?" checkbox

3. **Educational Goal**:
   - Present as a "practice focus" goal rather than automatically tracked
   - User commits to practicing this technique for X games
   - Manual review of replays to verify

### Suggested Approach
1. Research OpenDota API for any item usage data
2. If not available, implement as a "Practice Goal" category:
   - User sets intention to practice this technique
   - After game, user marks if they focused on it
   - Track commitment/awareness rather than exact execution
3. Could combine with general farming efficiency metrics (GPM when hero has Manta)

### Related Goals
- GPM targets for specific heroes
- Creep score benchmarks
- Map efficiency (farm distribution across map)
- Item timing goals

## Priority
Low-Medium - Good advanced goal but likely requires manual tracking due to API limitations

## Notes
- This aligns with high-skill farming optimization
- Could be part of "BSJ SMART Goals" preset collection
- May need a new goal category: "Practice Focus Goals" vs "Auto-Tracked Goals"
- Educational value even without perfect tracking
- Consider adding video link or explanation of the technique

## Alternative Implementation
Create a "Farming Efficiency Checklist" feature where users can note advanced techniques they're practicing:
- [ ] Manta while walking away
- [ ] Stacking camps while farming
- [ ] Farming triangle patterns
- [ ] Shoving waves before objectives

This becomes a mindfulness/review tool rather than automated tracking.
