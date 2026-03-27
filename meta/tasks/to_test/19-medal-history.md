# Medal History

## Summary
Display the user's Dota 2 rank medal history, highlighting the highest medal ever achieved.

## Requirements

### Data
- Fetch and store medal/rank data per match (rank_tier field from OpenDota match data)
- Track medal over time as matches are recorded
- Store peak (highest) medal achieved

### UI
- Medal history timeline: show how rank has changed over time
- Highlight the highest medal achieved (badge/callout)
- Current medal displayed prominently
- Show medal at the time of each match in the match history view (optional, lower priority)

### Medal Representation
- Map rank_tier values to medal names and star levels (Herald 1–5, Guardian 1–5, ... Immortal)
- Display medal icons/badges if assets are available

## Acceptance Criteria
- User can see their current medal and how it has changed over time
- Peak medal is clearly identified
- Medal data is stored and persists across sessions
