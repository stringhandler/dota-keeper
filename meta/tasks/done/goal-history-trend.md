# Goal History & Trend Tracking

## Summary
Show how a user's performance against a goal has evolved over time, so they can see improvement (or regression) across their match history.

## Motivation
Goals are meant to drive gradual improvement. Without a history view, users can't see their progress arc — only their current standing. A trend view motivates continued effort by making improvement visible.

## Acceptance Criteria

### Goal History View
- Each goal has a history section or dedicated view showing performance per game (or per session) over time
- Each data point shows: date, metric value achieved, and whether the goal was met that game
- The view is ordered chronologically (oldest to newest)

### Trend Visualisation
- A chart or graph shows the metric value over time (e.g. last hits at 10min per game)
- A rolling average line is overlaid to smooth noise and show the trend direction
- A horizontal reference line marks the goal threshold (e.g. 50 last hits)
- The success rate (% of games where goal was met) is shown for configurable time windows: last 7 days, last 30 days, all time

### Improvement Indicators
- Indicate clearly whether the user is trending up, down, or flat relative to the goal
- Highlight milestone moments (e.g. "First time you hit this goal", "10-game streak hitting this goal")

## Notes
- This feature supports the core design principle of the 75% target achievement rate — users should be able to see when they're comfortably above or below that threshold
- Consider showing the rolling success rate on the goal card in the goals list as a summary
- Data should come from the existing match + goal evaluation records already stored in SQLite
