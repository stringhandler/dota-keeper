# Manual Refresh Suggestion Button

## Overview
Add a button to allow users to manually regenerate their weekly hero goal suggestion on demand, rather than waiting for the automatic 7-day refresh.

## User Story
As a player, I want to manually refresh my hero goal suggestion so that I can get a new suggestion if I'm not interested in the current one or if I've already achieved it.

## Requirements

### UI Changes
- Add a "Refresh Suggestion" button next to the "Create Goal" button in the suggestion card
- Button should be styled consistently with the existing UI theme
- Show loading state while regenerating suggestion

### Backend Changes
- No new database functions needed - reuse existing `generate_hero_suggestion()` and `save_hero_suggestion()`
- Create new Tauri command: `refresh_hero_goal_suggestion()` that:
  - Generates a new suggestion (ignoring the current one's age)
  - Saves it to the database
  - Returns the new suggestion

### Frontend Changes
- Add click handler for refresh button
- Update `heroSuggestion` state with new data
- Show loading/processing indicator during refresh
- Optional: Add confirmation dialog to prevent accidental refreshes

## Technical Considerations

### Edge Cases
- What if refresh generates the same hero? (Low probability but possible)
- Should we prevent rapid successive refreshes (rate limiting)?
- Should we track refresh history to avoid suggesting same hero repeatedly?

### UX Enhancements
- Consider adding a cooldown period (e.g., can only refresh once per day)
- Show a message like "New suggestion generated!" after refresh
- Consider animating the card transition when suggestion changes

## Implementation Priority
**Low-Medium** - Nice to have feature that improves user control but not critical for core functionality

## Estimated Effort
Small - 1-2 hours of development work
