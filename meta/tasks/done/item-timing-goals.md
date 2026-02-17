# Item Timing Goals

## Goal
Implement item timing tracking and goal creation based on when specific items are purchased/completed on specific heroes.

## Overview
Allow users to create goals that track how quickly they purchase key items on specific heroes. For example, "Armlet on Chaos Knight by 9 minutes" or "Blink Dagger on Axe by 15 minutes".

## Requirements

### Core Features
- Track item purchase/completion times from match data
- Create goals based on item timings for specific heroes
- Only evaluate goal if the hero actually purchased the item in a match
- Display item timing performance and goal progress

### Goal Creation Constraints
- When goal type is "item timing", hero selection must be **required** (not "all heroes")
- User must select:
  - Specific hero (required, no "all heroes" option)
  - Specific item (from Dota 2 item list)
  - Target time (in minutes)
  - Comparison operator (e.g., "before", "by", "within")

### Goal Evaluation Logic
- Only count matches where:
  1. The selected hero was played
  2. The item was actually purchased in that match
- If item was never bought, don't penalize the goal (skip the match)
- If item was bought, check timing against goal
- Track success rate: matches with item bought within time / total matches with item bought

### Data Tracking
- Store item purchase times per match
- Link to specific heroes
- Track multiple item timings per match (for future expansion)

## Examples

### Example Goals
- "Armlet on Chaos Knight by 9 minutes"
- "Blink Dagger on Axe by 15 minutes"
- "Battle Fury on Anti-Mage before 20 minutes"
- "Black King Bar on any carry by 25 minutes"

### Goal Display
```
Goal: Armlet on Chaos Knight by 9 minutes
Progress: 7/10 matches (70%)
Last 5 timings: 8:45, 9:30, 8:15, 10:20, 8:50
Average timing: 9:06
```

## Technical Considerations

### Database Schema
- Add item timing tracking table:
  ```sql
  CREATE TABLE item_timings (
    id INTEGER PRIMARY KEY,
    match_id INTEGER,
    hero_id INTEGER,
    item_id INTEGER,
    timing_seconds INTEGER,
    FOREIGN KEY (match_id) REFERENCES matches(id)
  );
  ```

### API Data
- OpenDota API provides purchase log data
- Endpoint: `/matches/{match_id}` includes `purchase_log` with item purchases and game time
- Parse purchase data to extract timing for specific items

### UI/UX Considerations
- Item selection dropdown/autocomplete
- Time input (minutes:seconds or just minutes)
- Clear indication that hero selection is required for this goal type
- Visual feedback showing item icons
- Timing chart showing purchase times across matches

### Item Data
- Need Dota 2 item database/list
- Item IDs and names mapping
- Item icons for display
- Consider limiting to "core items" vs consumables

## Implementation Steps

### Phase 1: Data Collection
- [ ] Add item_timings table to database schema
- [ ] Fetch purchase_log data from OpenDota API
- [ ] Parse and store item purchase times for matches
- [ ] Create item reference data (IDs, names, icons)

### Phase 2: Goal System Integration
- [ ] Add "item_timing" goal type to goal creation
- [ ] Implement hero-required validation for item timing goals
- [ ] Add item selection UI component
- [ ] Add time input component (target timing)

### Phase 3: Evaluation Engine
- [ ] Implement item timing goal evaluation logic
- [ ] Handle cases where item wasn't purchased
- [ ] Calculate success rate (matches with item bought within target)
- [ ] Store item timing goal results

### Phase 4: Display & Insights
- [ ] Show item timing progress on goal cards
- [ ] Display timing trends (chart of purchase times)
- [ ] Show average timing vs target timing
- [ ] Add item timing insights to match history

### Phase 5: Polish
- [ ] Add item icons/images
- [ ] Improve timing input UX
- [ ] Add validation and error handling
- [ ] Add tooltips and help text
- [ ] Test with various items and heroes

## Future Enhancements
- Compare your timings to average player timings
- Suggest optimal item timings based on skill bracket
- Track multiple items per match (item progression)
- Support item timing sequences (e.g., "Treads → Armlet → BKB")
- Alert when falling behind on item timings during live match

## Status
- [ ] Database schema updates
- [ ] API integration for purchase data
- [ ] Goal creation UI modifications
- [ ] Evaluation engine implementation
- [ ] Display and visualization
