# Multi-Metric Hero Goal Suggestions

## Overview
Expand the hero goal suggestion system beyond last hits to include other performance metrics like GPM (Gold Per Minute), kills, XPM (Experience Per Minute), and other relevant stats.

## User Story
As a player, I want goal suggestions for different metrics (not just last hits) so that I can improve various aspects of my gameplay based on my hero role and playstyle.

## Requirements

### New Metrics to Support
1. **GPM (Gold Per Minute)** - Economic efficiency
2. **Kills** - Combat effectiveness (especially for carry/mid heroes)
3. **XPM (Experience Per Minute)** - Lane efficiency
4. **Hero Damage** - Teamfight contribution
5. **Tower Damage** - Objective focus
6. **Hero Healing** - Support effectiveness

### Metric Selection Logic
- **Automatic selection** based on hero role:
  - Carry heroes → GPM, Last Hits
  - Support heroes → Assists, Hero Healing, Wards
  - Mid heroes → Kills, XPM
  - Offlane heroes → Hero Damage, Tower Damage
- **Manual override** - Let users choose preferred metric

### Database Changes
- Modify `hero_goal_suggestions` table to include `metric` field
- Update suggestion generation algorithm to:
  - Determine optimal metric for selected hero
  - Calculate average for that metric
  - Generate 5-10% improvement target

### UI Changes
- Display metric name in suggestion card (e.g., "GPM", "Kills", "Last Hits")
- Show appropriate units for each metric
- Consider icon or color coding per metric type

## Technical Considerations

### Data Availability
- All metrics are already stored in `matches` table
- No additional API calls needed
- Calculation logic similar to last hits

### Hero Role Classification
- Need to add hero role metadata (carry, support, mid, offlane, roaming)
- Could use OpenDota meta or manual classification
- Create new `hero_roles` table or static mapping in code

### Algorithm Enhancement
```rust
fn determine_best_metric_for_hero(hero_id: i32) -> GoalMetric {
    match get_hero_role(hero_id) {
        HeroRole::Carry => GoalMetric::GoldPerMin,
        HeroRole::Support => GoalMetric::Assists,
        HeroRole::Mid => GoalMetric::Kills,
        HeroRole::Offlane => GoalMetric::HeroDamage,
        _ => GoalMetric::LastHits
    }
}
```

## Future Enhancements
- Multiple simultaneous suggestions (one per metric category)
- User preference learning (track which metrics users accept most often)
- Dynamic metric selection based on recent performance trends

## Implementation Priority
**Medium** - Significant value add that makes suggestions more comprehensive and role-appropriate

## Estimated Effort
Medium - 4-6 hours of development work
- Hero role classification: 1-2 hours
- Multi-metric support in backend: 2-3 hours
- Frontend updates: 1 hour
