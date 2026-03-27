# Suggestion History Tracking

## Overview
Track and display the history of past hero goal suggestions, including which suggestions were accepted, declined, or ignored by the user.

## User Story
As a player, I want to see my past goal suggestions so that I can review what heroes and targets were recommended and track my progression over time.

## Requirements

### Data to Track
- All generated suggestions (accepted or not)
- Timestamp of suggestion generation
- Hero and metric suggested
- Target value
- Whether suggestion was accepted (created as goal)
- Whether suggestion was manually refreshed (replaced)
- If accepted, link to the created goal ID

### Database Schema

#### New Table: `suggestion_history`
```sql
CREATE TABLE suggestion_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    hero_id INTEGER NOT NULL,
    metric TEXT NOT NULL,
    suggested_value INTEGER NOT NULL,
    current_average REAL NOT NULL,
    created_at INTEGER NOT NULL,
    accepted BOOLEAN DEFAULT FALSE,
    goal_id INTEGER,  -- NULL if not accepted, links to goals table
    replaced_at INTEGER,  -- NULL if still current, set when refreshed
    FOREIGN KEY (goal_id) REFERENCES goals(id)
)
```

### UI Features

#### New Page: Suggestion History (`/suggestions/history`)
- Table showing all past suggestions with columns:
  - Date
  - Hero
  - Metric
  - Target Value
  - Status (Accepted, Declined, Expired, Replaced)
  - Action (if accepted: link to goal)
- Filters:
  - By hero
  - By status
  - Date range
- Sort options: Date (newest/oldest), Hero name

#### Dashboard Enhancement
- Show count of total suggestions generated
- Show acceptance rate (% of suggestions that became goals)

### Backend Changes

#### Update Existing Functions
- `save_hero_suggestion()`: Also insert into `suggestion_history`
- `generate_hero_suggestion()`: Mark previous suggestion as replaced

#### New Functions
- `get_suggestion_history()`: Retrieve paginated history
- `mark_suggestion_accepted(suggestion_id, goal_id)`: Link suggestion to created goal
- `get_suggestion_stats()`: Calculate acceptance rate and other metrics

### Frontend Changes

#### Accept Suggestion Flow
When user clicks "Create Goal":
1. Create the goal
2. Call backend to mark suggestion as accepted with goal ID
3. Update suggestion card to show "Goal Created" status

#### History Page
- Sortable, filterable table component
- Click on accepted suggestion → navigate to goal details
- Visual indicators for status (icons/badges)

## Technical Considerations

### Migration Strategy
- Existing users won't have history before this feature
- Start tracking from implementation date forward
- Consider adding "Suggestion History" as a new nav item

### Performance
- Limit history display to last 50-100 suggestions
- Add pagination for users with many suggestions
- Index on `created_at` for fast queries

### Privacy
- All data stored locally (no privacy concerns)
- Consider adding "Clear History" option in settings

## Analytics Potential

### Insights to Provide
- **Acceptance Rate**: What % of suggestions are accepted?
- **Popular Heroes**: Which heroes appear most in suggestions?
- **Popular Metrics**: Which metrics are suggested/accepted most?
- **Trend Analysis**: Is user's acceptance rate improving over time?

### Dashboard Widget Ideas
- "You've accepted 15 of your last 20 suggestions (75%)"
- "Most suggested hero: Anti-Mage (8 times)"
- "You're more likely to accept GPM goals (85%) than Last Hit goals (60%)"

## Future Enhancements
- Export history to CSV/JSON
- Compare suggested vs actual achieved values for accepted goals
- Machine learning to improve suggestion algorithm based on acceptance patterns
- Weekly summary email/notification of suggestion history

## Implementation Priority
**Low** - Interesting data tracking feature but not critical for core suggestion functionality

## Estimated Effort
Medium - 4-5 hours of development work
- Database schema and migration: 1 hour
- Backend functions: 1.5 hours
- Frontend history page: 2 hours
- Integration with accept flow: 30 minutes
- Testing: 1 hour
