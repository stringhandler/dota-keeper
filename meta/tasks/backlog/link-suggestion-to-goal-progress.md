# Link Suggestion to Goal and Show Progress

## Overview
When a user accepts a hero goal suggestion and creates a goal from it, maintain the link between the suggestion and the goal to show real-time progress toward the suggested target directly in the suggestion card.

## User Story
As a player, after I accept a goal suggestion, I want to see my progress toward that goal displayed in the suggestion card so I can track how close I am to achieving it without navigating to the goals page.

## Requirements

### Linking Suggestions to Goals

#### Database Changes
- Add `goal_id` field to `hero_goal_suggestions` table (nullable)
- When user accepts suggestion, update the suggestion record with the created goal ID
- Maintain this link until suggestion expires or is replaced

#### Acceptance Flow
```
User clicks "Create Goal"
  ↓
1. Create goal in database
2. Update suggestion record with goal_id
3. Reload suggestion with goal progress data
4. Update UI to show progress view
```

### Progress Display

#### UI States

**State 1: Suggestion Not Accepted (default)**
```
🎯 Suggested Goal This Week
Storm Spirit
Current Average: 58 CS
Suggested Target: 62 CS (+7%)

[Create Goal]
```

**State 2: Suggestion Accepted - In Progress**
```
🎯 Active Weekly Goal
Storm Spirit - Target: 62 CS

Progress: 3/5 games achieved (60%)
Latest: 65 CS ✓ | 61 CS ✓ | 58 CS ✗ | 64 CS ✓ | 57 CS ✗

[View Goal Details]
```

**State 3: Suggestion Accepted - Goal Achieved**
```
✅ Weekly Goal Complete!
Storm Spirit - Target: 62 CS

Achievement: 4/5 games (80% success rate)
Congratulations! You've improved your CS average.

[View Goal Details] [Get New Suggestion]
```

### Backend Changes

#### New Tauri Command
```rust
#[tauri::command]
fn get_hero_suggestion_with_progress() -> Result<Option<HeroSuggestionWithProgress>, String>
```

#### New Data Structure
```rust
pub struct HeroSuggestionWithProgress {
    pub hero_id: i32,
    pub suggested_last_hits: i32,
    pub current_average: f64,
    pub created_at: i64,
    pub games_analyzed: i32,
    pub goal_id: Option<i64>,
    pub goal_progress: Option<GoalProgressSummary>
}

pub struct GoalProgressSummary {
    pub games_played: i32,
    pub games_achieved: i32,
    pub recent_values: Vec<i32>,
    pub recent_achieved: Vec<bool>,
}
```

#### Progress Calculation
- Query matches for the linked goal since goal creation
- Evaluate each match against the goal criteria
- Return summary of performance

### Frontend Changes

#### Conditional Rendering
```svelte
{#if heroSuggestion}
  {#if heroSuggestion.goal_id}
    <!-- Show progress view -->
    <GoalProgressCard suggestion={heroSuggestion} />
  {:else}
    <!-- Show suggestion view -->
    <SuggestionCard suggestion={heroSuggestion} />
  {/if}
{/if}
```

#### Progress Visualization
- Progress bar showing % of games achieved
- Recent game results with checkmarks/X marks
- Color coding: green for achieved, red for not achieved
- Optional: Sparkline chart showing performance trend

## Technical Considerations

### Edge Cases
- User deletes the linked goal → revert to suggestion view
- User manually edits the goal parameters → show warning or break link?
- Suggestion expires while goal is still active → keep showing progress
- User creates multiple goals from same suggestion → link to most recent

### Performance
- Cache goal progress data to avoid recalculating on every dashboard load
- Update progress when new matches are fetched
- Consider using Tauri events to push updates when matches change

### UX Enhancements
- Celebrate when goal is achieved (animation, sound, special styling)
- Show comparison: suggested average vs actual achieved average
- Provide insights: "You're averaging 64 CS, 3% above target!"
- Option to extend goal or create new harder goal when achieved

## Integration with Existing Features

### Goal Calendar
- Clicking "View Goal Details" navigates to goal detail page
- Goal calendar shows progress for the linked goal
- Suggestion progress card and goal calendar stay in sync

### Weekly Refresh
- When suggestion expires (7 days), what happens to linked goal?
  - **Option A**: Unlink and generate new suggestion (goal continues independently)
  - **Option B**: Keep showing linked goal until completed, delay new suggestion
  - **Recommended**: Option A for simplicity

## Future Enhancements
- **Multi-Week Tracking**: Show performance across multiple weeks
- **Achievement Badges**: Award badges for consistent goal completion
- **Streak Tracking**: "5 weeks in a row of completed suggestions"
- **Difficulty Adjustment**: Auto-increase difficulty if consistently achieving goals

## Implementation Priority
**Medium** - Significantly enhances the suggestion feature by adding feedback loop and motivation

## Estimated Effort
Medium - 4-6 hours of development work
- Database schema update: 30 minutes
- Backend progress calculation: 2 hours
- Frontend progress UI component: 2 hours
- Integration and testing: 1.5 hours
