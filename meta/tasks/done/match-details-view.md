# Match Details View

## Goal
Create a detailed view for a single match that displays comprehensive match information and performance graphs.

## Requirements

### Core Features
- View detailed information for a single match
- Display match-specific graph showing last hits by minute
- Show match metadata (date, hero, result, duration, etc.)
- Display key performance metrics (KDA, GPM, XPM, etc.)

### Last Hits Graph
- X-axis: Game time in minutes
- Y-axis: Total last hits
- Should show progression throughout the match
- Consider showing comparison to average/expected last hits for that hero/role

### UI/UX
- Accessible from match history/list view
- Clean, readable layout for match statistics
- Graph should be interactive/zoomable if possible

## Technical Considerations
- Need to fetch or store per-minute last hit data from OpenDota API
- May need to update database schema to store time-series data
- Consider which charting library to use (already added in recent commit)
- Ensure data is available from the API (OpenDota provides player performance timeseries)

## API Data
OpenDota API provides:
- `/matches/{match_id}` - Detailed match data
- Player timeseries data including `lh_t` (last hits over time)

## Status
- [ ] Design match details page layout
- [ ] Fetch and store per-minute last hit data
- [ ] Implement graph component for last hits visualization
- [ ] Add navigation from match list to details view
- [ ] Style and polish the UI
