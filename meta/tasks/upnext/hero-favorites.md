# Task: Hero Favorites Feature

## Status
- [ ] Not Started

## Overview
Add the ability to mark heroes as favorites, allowing users to prioritize and filter their analysis by the heroes they play most often or want to track more closely.

## Requirements

### Database Schema
- Add `is_favorite` boolean column to the heroes table (or hero tracking table)
- Default value: `false`
- Should persist across application sessions

### User Interface

#### Hero List Display
- Favorites should appear at the top of the hero list in analysis views
- Visual indicator for favorite heroes (e.g., star icon, highlighted badge)
- Ability to toggle favorite status directly from the hero list

#### Filtering
Add filter dropdown/toggle with the following options:
- **All Heroes** (default) - Shows all heroes
- **Favorites Only** - Shows only heroes marked as favorite

The filter should:
- Persist during the current session
- Apply to relevant analysis views
- Update the list dynamically when changed

### Functionality

#### Marking Favorites
- Click/tap interaction to toggle favorite status
- Visual feedback when toggling (animation or state change)
- Changes should save immediately to the database

#### Sorting Behavior
- When showing "All Heroes": Favorites first, then remaining heroes
- Within each group (favorites/non-favorites), maintain existing sort order (e.g., alphabetical, by games played, etc.)
- When showing "Favorites Only": Apply normal sorting

## Technical Considerations

### Database Updates
- Migration script to add `is_favorite` column if not exists
- Update SQL queries to respect favorite status in sorting
- Ensure efficient querying with proper indexing if needed

### State Management
- Track favorite status in application state
- Handle optimistic updates for responsive UI
- Sync with database on change

### UI Components
Likely files to modify/create:
- Hero list component
- Filter controls component
- Database schema/migration files
- Hero service/API layer

## Acceptance Criteria
- [ ] Heroes can be marked/unmarked as favorites
- [ ] Favorite heroes appear at the top of lists when filter is "All Heroes"
- [ ] Filter toggles between "All Heroes" and "Favorites Only"
- [ ] Favorite status persists across application restarts
- [ ] Visual distinction between favorite and non-favorite heroes
- [ ] Filter selection persists during current session
- [ ] UI provides immediate feedback when toggling favorites

## Future Enhancements (Out of Scope)
- Favorite heroes dashboard widget
- Quick access to favorite heroes stats
- Export/import favorites configuration
- Maximum number of favorites limit
