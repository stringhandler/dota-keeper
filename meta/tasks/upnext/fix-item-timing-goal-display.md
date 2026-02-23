# Fix Item Timing Goal Display Bug

## Problem

Item timing goals are displaying incorrect values:
- Shows "200 achieved at 0 mins" when actual is 147 seconds
- The achievement time is showing as 0 mins instead of the actual timing
- The value being displayed appears to be the target, not the actual achievement

## Example

**Expected**: "Battle Fury achieved at 147 seconds (2:27)"
**Actual**: "Battle Fury achieved at 0 mins" or showing target value instead of actual

## Investigation Needed

1. Check where item timing goals are being evaluated/displayed
2. Verify the data being pulled from the database (item_timings table)
3. Check the goal evaluation logic for ItemTiming metric
4. Verify the display logic in the UI components

## Possible Root Causes

- Item timing data not being correctly fetched from database
- Goal evaluation logic using wrong field (target vs actual)
- Display component showing target instead of actual achievement
- Timing conversion issue (seconds vs minutes)

## Files to Check

- `src-tauri/src/database.rs` - Goal evaluation for ItemTiming goals
- Components that display item timing goal progress
- Any UI components showing item achievements
- Item timing storage/retrieval logic

## Acceptance Criteria

- [ ] Item timing goals show the correct actual achievement value
- [ ] Achievement time displays in the correct format (e.g., "2:27" or "147s")
- [ ] Target value and actual value are clearly distinguished
- [ ] Works correctly for all item types
