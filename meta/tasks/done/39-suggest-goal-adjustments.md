# Suggest Goal Adjustments Based on Performance

## Problem Statement

Users may set goals that are too ambitious or don't reflect their current skill level. If a user consistently fails to hit a goal over multiple games, the application should suggest lowering the goal to a more achievable target to maintain motivation and provide a sense of progress.

## Requirements

### Detection Logic
- Track goal achievement rate over a sliding window of recent games
- Trigger suggestion when:
  - Goal has been missed for X consecutive games (e.g., 10-15 games)
  - Goal achievement rate is below Y% over last Z games (e.g., <20% over 20 games)
  - Goal has existed for at least N games to avoid premature suggestions

### Suggestion Mechanism
- **Notification/UI Element**: Show a suggestion banner or modal
- **Smart Calculation**: Calculate a recommended new goal value based on:
  - Recent performance average
  - Slight stretch (e.g., 10-15% above current average)
  - Statistical trend (improving vs declining performance)

### User Options
When suggestion is shown, user can:
1. **Accept**: Lower the goal to suggested value
2. **Dismiss**: Keep current goal and suppress suggestion for X games
3. **Custom**: Manually adjust to a different value
4. **Delete Goal**: Remove goal entirely if no longer relevant

### Example Scenarios

**Scenario 1: CS Goal**
- Goal: 80 CS by 10 minutes
- Last 15 games average: 52 CS by 10 minutes
- Suggestion: "You've missed this goal in the last 15 games. Consider lowering to 60 CS by 10 minutes (15% above your recent average)."

**Scenario 2: Win Rate Goal**
- Goal: 60% win rate on Invoker
- Current: 45% win rate over last 20 Invoker games
- Suggestion: "Your Invoker win rate is trending at 45%. Consider adjusting to 50% to maintain achievable progress."

**Scenario 3: KDA Goal**
- Goal: KDA > 4.0
- Last 12 games average KDA: 2.3
- Suggestion: "Your recent KDA average is 2.3. Try setting a goal of 2.8 to build momentum."

## Technical Implementation

### Database Changes
```sql
-- Track goal suggestion history
CREATE TABLE goal_suggestions (
    id INTEGER PRIMARY KEY,
    goal_id INTEGER NOT NULL,
    suggested_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    original_value REAL NOT NULL,
    suggested_value REAL NOT NULL,
    action TEXT, -- 'accepted', 'dismissed', 'custom', 'deleted'
    dismissed_until INTEGER, -- game count to suppress
    FOREIGN KEY (goal_id) REFERENCES goals(id)
);
```

### Analysis Engine
- Add function to calculate goal achievement rate
- Implement sliding window analysis for recent games
- Calculate suggested goal value based on percentile approach
- Track dismissal state to avoid suggestion spam

### UI Components
- Suggestion banner component for goal list page
- Modal dialog for detailed suggestion with chart showing recent performance
- Option to view suggestion history

## Success Criteria

- [ ] System accurately detects when goals are consistently missed
- [ ] Suggestions are helpful and not annoying (good timing, suppressible)
- [ ] Suggested values are realistic based on recent performance
- [ ] Users can easily accept, dismiss, or customize suggestions
- [ ] Suggestion history is tracked to prevent repeated suggestions
- [ ] Visual feedback shows performance trend leading to suggestion

## Edge Cases

- **New goals**: Don't suggest changes until sufficient data (minimum 10-15 games)
- **Volatile performance**: Consider variance when making suggestions
- **Already declining goals**: Don't suggest lowering if user recently lowered manually
- **Seasonal players**: Consider time gaps in match history

## Future Enhancements

- Suggest raising goals when consistently exceeding them
- ML-based goal recommendations based on rank, hero, role
- Peer comparison: "Players at your rank typically achieve X"
- Adaptive suggestions based on user's response history

## Priority
**MEDIUM** - Nice-to-have feature that improves user experience and motivation

## Related Features
- Goal system (existing)
- Goal evaluation (existing)
- Performance analytics (existing)
- Notifications system (future)

## Estimated Complexity
**Medium** - Requires analytics logic, UI work, and database changes
