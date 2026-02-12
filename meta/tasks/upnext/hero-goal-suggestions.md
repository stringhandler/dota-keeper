# Hero-Specific Goal Suggestions

## Overview
Add a dashboard feature that automatically suggests personalized improvement goals for heroes the player has recently played.

## Requirements

### Hero Selection Criteria
- Analyze the most recent 20 games to identify candidate heroes
- Only consider heroes with at least 5 games stored in total history
- Randomly select one qualifying hero to display on the dashboard

### Goal Generation
- Calculate the average last hits for the selected hero from their last 5 games
- Generate a goal that is 5-10% higher than this average
- Goal should be specific and measurable (e.g., "Average 65 last hits on Anti-Mage")
- **Refresh weekly**: New hero and goal suggestion generated once per week

### Dashboard Display
- Show the selected hero with their current performance stats
- Display the suggested goal prominently
- Include context: current average vs. suggested target
- Consider showing a progress indicator or visual comparison

## Technical Considerations

### Database Queries
- Query last 20 games to find recently played heroes
- For each qualifying hero, calculate total games played
- Retrieve last 5 games for the selected hero to compute average last hits

### Calculation Logic
```
target_last_hits = avg_last_hits_last_5_games * (1 + random(0.05, 0.10))
```

### Weekly Refresh Logic
- Store the timestamp of when the current suggestion was generated
- On dashboard load, check if more than 7 days have passed since last suggestion
- If yes, generate a new hero selection and goal
- If no, display the existing suggestion
- Store: `hero_id`, `suggested_last_hits`, `created_date` in database

### Edge Cases
- Handle heroes with exactly 5 games vs. heroes with many more
- What if no heroes meet the criteria (< 20 games total, or no hero with 5+ games)?
- What if user hasn't played any games in the past week?
- First-time users with no existing suggestion stored

## User Experience

### Example Display
```
🎯 Suggested Goal for Storm Spirit
Current Average (last 5 games): 58 last hits
Suggested Target: 62 last hits (+7%)
Total Games Played: 12
```

## Future Enhancements
- Allow users to accept/decline suggested goals
- Track progress on accepted goals
- Expand to other metrics (GPM, KDA, XPM)
- Suggest different types of goals based on hero role (carry vs. support)
- Machine learning to identify which metrics matter most per hero

## Dependencies
- Existing match data in database
- Hero statistics calculation system
- Dashboard UI component

## Acceptance Criteria
- [ ] System identifies heroes from last 20 games with 5+ total games
- [ ] Random hero is selected when generating new suggestion
- [ ] Goal is calculated as 5-10% improvement over last 5 games average
- [ ] Goal is displayed with context (current vs. target)
- [ ] Suggestion persists for 7 days before refreshing
- [ ] New suggestion generated automatically after 7 days
- [ ] Suggestion timestamp stored in database
- [ ] Edge cases handled gracefully (no qualifying heroes)
