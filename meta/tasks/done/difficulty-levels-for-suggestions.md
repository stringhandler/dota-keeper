# Difficulty Levels for Goal Suggestions

## Overview
Allow users to choose the difficulty level of their goal suggestions by selecting different improvement percentages (e.g., 5%, 10%, or 15% improvement targets).

## User Story
As a player, I want to adjust the difficulty of my goal suggestions so that I can set challenging but achievable targets based on my current skill progression rate.

## Requirements

### Difficulty Options
- **Easy**: 3-5% improvement over current average
- **Medium**: 5-10% improvement (current default)
- **Hard**: 10-15% improvement
- **Custom**: User-defined percentage

### Settings Storage
- Add user preference to settings/configuration
- Persist difficulty choice across sessions
- Default to "Medium" for new users

### UI Changes

#### Settings Page
- Add difficulty selector dropdown in Settings page
- Options: Easy, Medium, Hard, Custom
- If Custom selected, show number input for percentage

#### Dashboard Suggestion Card
- Display current difficulty level (e.g., "Medium Difficulty" badge)
- Optional: Show what target would be at different difficulty levels

### Backend Changes

#### Database
- Add to `Settings` struct: `suggestion_difficulty: String` (Easy/Medium/Hard/Custom)
- Add `suggestion_custom_percentage: Option<f64>` for custom difficulty

#### Suggestion Generation
Modify `generate_hero_suggestion()` to:
```rust
let improvement_range = match settings.suggestion_difficulty {
    "Easy" => (0.03, 0.05),      // 3-5%
    "Medium" => (0.05, 0.10),    // 5-10% (default)
    "Hard" => (0.10, 0.15),      // 10-15%
    "Custom" => {
        let pct = settings.suggestion_custom_percentage.unwrap_or(0.10);
        (pct - 0.02, pct + 0.02)  // Custom ± 2%
    }
    _ => (0.05, 0.10)
};

let improvement_factor = 1.0 + rng.gen_range(improvement_range.0..improvement_range.1);
```

## Technical Considerations

### Edge Cases
- Very low skill players might find even "Easy" difficult
- Very high skill players might need > 15% to feel challenged
- Custom percentage validation (must be > 0%, reasonable upper limit like 50%)

### UX Enhancements
- Show success rate of accepted goals to recommend difficulty adjustment
- Auto-suggest difficulty based on historical goal achievement rate
- Display difficulty in suggestion card with color coding

### Validation
- Ensure custom percentage is between 1% and 50%
- Prevent negative or zero percentages
- Handle edge cases where target would exceed realistic max values

## Future Enhancements
- **Adaptive Difficulty**: Automatically adjust based on goal achievement rate
  - If user achieves 90%+ of goals → suggest increasing difficulty
  - If user achieves < 50% of goals → suggest decreasing difficulty
- **Per-Metric Difficulty**: Different difficulty for GPM vs Last Hits vs Kills
- **Skill-Based Difficulty**: Factor in MMR or rank for difficulty suggestions

## Implementation Priority
**Low-Medium** - Nice customization feature but not essential for core functionality

## Estimated Effort
Small-Medium - 2-3 hours of development work
- Settings UI: 30 minutes
- Backend difficulty logic: 1 hour
- Testing different difficulty levels: 30 minutes
- Documentation: 30 minutes
