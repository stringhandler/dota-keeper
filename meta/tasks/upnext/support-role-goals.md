# Support Role Goals

## Overview

Add goal types tailored to support players, who don't focus on last hits but instead contribute through denies, assists, wards, healing, and other team-oriented metrics.

## Starting Point: Denies

The initial implementation should focus on **denies at 10 minutes**, mirroring the existing last-hits goal pattern.

### Deny Goal
- **Metric**: Denies at `target_time_minutes`
- **Goal type name**: `Denies`
- **Example**: "Achieve 15 denies by 10 minutes on Shadow Demon"
- **Data source**: `match_cs` table already stores deny data per minute (the `denies` column)

## Full Support Metrics (Future)

Once denies are working, expand to:
- **Assists** – total assists by end of game (e.g., "10+ assists per game")
- **Observer wards placed** – from OpenDota match data
- **Sentry wards placed**
- **Healing done** – from match detail stats
- **Camps stacked**
- **Roshan assist** participation

## Implementation Notes

### Backend (Rust)
- Add `Denies` variant to `GoalMetric` enum in `database.rs`
- Update `evaluate_goal_for_match()` to handle `Denies` metric using `match_cs.denies` at the target minute
- Update suggestion generation to optionally suggest deny goals for heroes with support tendencies

### Frontend (Svelte)
- Add "Denies" option to the metric dropdown in the goal creation page
- Show deny data in goal details page histogram (already stored in `match_cs`)
- Update `getMetricLabel` / `getMetricUnit` helpers in match and goal pages

## Acceptance Criteria
- [ ] Can create a goal: "Achieve X denies by N minutes"
- [ ] Goal is evaluated against parsed matches using `match_cs.denies`
- [ ] Goal appears correctly in goal details histogram
- [ ] Suggestion system can suggest a deny goal
