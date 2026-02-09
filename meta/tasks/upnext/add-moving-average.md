# Task: Last Hits Moving Average & Trend Analysis

## Overview
Create a screen that displays moving average and trend analysis for last hits at specific time markers, enabling players to track their farming performance over time and compare against historical periods.

## Problem Statement
Players need to track their farming efficiency (last hits) at key timing benchmarks (typically 10 minutes) to identify improvement or regression in their mechanical skill. The ability to see trends per hero and game type helps identify which heroes/modes they're improving on.

## User Stories

### Primary
- As a player, I want to see my moving average of last hits at 10 minutes across my recent games
- As a player, I want to filter this data by specific heroes to see hero-specific trends
- As a player, I want to filter by game type (Ranked, Unranked, Turbo, etc.) to understand context-specific performance
- As a player, I want to compare my last 30 games vs the 30 games before that to see if I'm improving

### Secondary
- As a player, I want to customize the time marker (not just 10 minutes, but also 5, 15, 20 minutes)
- As a player, I want to customize the window size (not just 30 games, but 20, 50, 100 games)
- As a player, I want to see the trend visually (chart/graph)
- As a player, I want to see statistical indicators (improving/declining/stable)

## Requirements

### Data Requirements
- **Metric**: Last hits at specific time markers (default: 10 minutes)
- **Time Markers**: Configurable (5, 10, 15, 20 minutes)
- **Window Size**: Configurable rolling window (default: 30 games)
- **Filters**:
  - Hero-specific filtering (single or multiple heroes)
  - Game type filtering (Ranked, Unranked, Turbo, All Pick, etc.)
  - Date range filtering (optional)
- **Comparison Periods**: Previous N games vs N games before that

### Calculations
1. **Moving Average**: Calculate rolling average of last hits at X minutes over the last N games
2. **Trend Analysis**:
   - Linear regression to show upward/downward trend
   - Percentage change from period to period
   - Current period average vs previous period average
3. **Statistical Metrics**:
   - Mean, median, min, max
   - Standard deviation
   - Best/worst performances in window

### UI/UX Requirements

#### Screen Layout
```
┌─────────────────────────────────────────────────────────┐
│ Last Hits Analysis                                      │
├─────────────────────────────────────────────────────────┤
│ Filters:                                                │
│ [Time: 10 min ▼] [Window: 30 games ▼]                  │
│ [Hero: All ▼] [Game Type: All ▼]                       │
├─────────────────────────────────────────────────────────┤
│ Current Period (Last 30 games)                          │
│ ┌─────────────────────────────────────────────────────┐ │
│ │         [Line/Bar Chart showing trend]              │ │
│ │                                                     │ │
│ └─────────────────────────────────────────────────────┘ │
│ Avg: 65.3 LH | Min: 42 | Max: 89 | Trend: ↗ +5.2%     │
├─────────────────────────────────────────────────────────┤
│ Comparison: Last 30 vs Previous 30                      │
│ Current:  65.3 avg LH                                   │
│ Previous: 58.1 avg LH                                   │
│ Change:   +7.2 LH (+12.4%) ✓ IMPROVING                 │
├─────────────────────────────────────────────────────────┤
│ Per-Hero Breakdown (if "All Heroes" selected)           │
│ Anti-Mage:    72.5 avg (15 games) ↗ +8%                │
│ Juggernaut:   68.2 avg (10 games) ↗ +3%                │
│ Phantom Assassin: 61.1 avg (5 games) ↘ -2%             │
└─────────────────────────────────────────────────────────┘
```

#### Visual Elements
- **Chart**: Line chart showing moving average over time
  - X-axis: Game number or date
  - Y-axis: Last hits at X minutes
  - Trend line overlaid
  - Shaded regions for comparison periods
- **Color Coding**:
  - Green: Improving (positive trend)
  - Red: Declining (negative trend)
  - Yellow: Stable (minimal change)
- **Indicators**: Up/down arrows for quick visual feedback

### Technical Considerations

#### Database Schema
Ensure the following data is available:
- Match ID
- Hero played
- Game type/mode
- Match duration
- Last hits at various time intervals (need to calculate from match data)
- Match date/time

#### API/Data Source
- OpenDota API provides `laning_performance` which may include last hits at 10 minutes
- Alternative: Calculate from `gold_t` and `xp_t` arrays if last hit data not directly available
- Steam API may have different data structure

#### Calculations Implementation
```
Moving Average (Simple):
MA(n) = (x₁ + x₂ + ... + xₙ) / n

Trend (Linear Regression):
slope = Σ((xᵢ - x̄)(yᵢ - ȳ)) / Σ((xᵢ - x̄)²)

Period Comparison:
Change% = ((Current Avg - Previous Avg) / Previous Avg) × 100
```

#### Performance Considerations
- Cache calculated moving averages to avoid recalculation
- Index database on hero_id, game_type, match_date for fast filtering
- Consider pagination for large datasets

### Data Edge Cases
- **Insufficient Data**: Handle when player has < window size games
  - Show what's available
  - Display warning: "Only X games available (need Y for full window)"
- **No Previous Period**: If player is new and only has one period
  - Show current period only
  - Hide comparison section
- **Short Games**: Games ending before the time marker (e.g., 7-minute game when analyzing 10-minute mark)
  - Exclude from analysis, OR
  - Use final last hit count (mark with asterisk)
- **Missing Data**: Some matches may not have detailed timing data
  - Skip those matches
  - Note in UI how many matches were excluded

### Acceptance Criteria

#### Functional
- [ ] User can view moving average of last hits at configurable time marker
- [ ] User can filter by specific hero(es)
- [ ] User can filter by game type
- [ ] User can configure window size (20, 30, 50, 100 games)
- [ ] User can see comparison between current and previous period
- [ ] Chart displays accurately with trend line
- [ ] Statistics (avg, min, max, trend%) calculate correctly
- [ ] Per-hero breakdown shows when viewing all heroes

#### Non-Functional
- [ ] Screen loads in < 2 seconds for 100-game window
- [ ] Chart is responsive and interactive (hover for details)
- [ ] UI is intuitive and requires no explanation
- [ ] Data persists correctly in database
- [ ] Calculations are accurate and verifiable

#### Edge Cases
- [ ] Gracefully handles insufficient data
- [ ] Handles games shorter than time marker
- [ ] Handles missing data in some matches
- [ ] Handles zero games in filter selection

## Implementation Notes

### Phase 1: MVP
- Fixed 10-minute marker
- Fixed 30-game window
- Hero and game type filtering
- Basic comparison (current vs previous)
- Simple statistics (average only)

### Phase 2: Enhanced
- Configurable time markers
- Configurable window sizes
- Full statistics (min, max, stddev)
- Visual chart/graph
- Trend indicators

### Phase 3: Advanced
- Multiple metrics (GPM, XPM, denies, etc.)
- Multiple time markers simultaneously
- Export data
- Share comparisons
- Goal integration (compare to defined goals)

## Questions to Resolve
1. Should short games be included or excluded from analysis?
2. What should be the minimum viable dataset size before showing analysis?
3. Should we show individual game data points or only aggregated moving average?
4. Should comparison be strictly N vs previous N, or allow custom date ranges?
5. How to handle heroes played only a few times (statistical significance)?

## Dependencies
- Match data with timing information must be stored in database
- OpenDota/Steam API integration must provide last hit timing data
- Database schema may need updates to store calculated metrics

## Estimated Complexity
**Medium-High**
- Backend: API integration, database queries, statistical calculations
- Frontend: Chart library integration, responsive filtering, data visualization
- Testing: Edge cases, calculation accuracy, performance testing

## Success Metrics
- Users can identify farming improvement/decline trends
- Users can compare performance across heroes
- Feature is used regularly by active users
- Positive user feedback on feature usefulness
