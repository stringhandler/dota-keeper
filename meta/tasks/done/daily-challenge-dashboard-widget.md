# Daily Challenges – Dashboard Widget

Depends on: `daily-challenge-core`

## Goal
Show today's daily challenge progress on the dashboard with streak tracking.

## Requirements

### Widget States
- **Active**: Show description, progress bar, time until midnight ("resets in Xh Ym")
- **Completed**: Show "✅ Daily Complete!" with time completed and current streak
- **No matches yet**: Show at 0% progress with motivational copy
- **Loading/no challenge**: Skeleton/spinner

### Layout (fits within existing dashboard card grid)
```
⚡ Today's Challenge
Get 10+ kills in one game
[████░░░░░░] 0/1
Resets in 8h 23m
🔥 3 day streak
```

### Behaviour
- Load on dashboard mount alongside existing goal calendar
- Refresh after `refresh_matches` is called (re-invoke `get_daily_challenge_progress`)
- Does NOT need its own refresh button — piggybacks on existing match refresh

### Navigation
- No dedicated challenges page needed yet — widget is self-contained

## Acceptance Criteria
- [ ] Widget visible on dashboard
- [ ] Shows correct progress after fetching matches
- [ ] Countdown to midnight updates each minute (use `setInterval`)
- [ ] Streak displayed when > 0
- [ ] Completed state visually distinct from active state
