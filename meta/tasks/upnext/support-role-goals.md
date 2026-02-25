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

## Lane Partner Networth Goal

Track how much gold your lane partner accumulates by a target time — a strong proxy for how well you enabled them during the laning phase.

- **Metric**: Lane partner's networth at `target_time_minutes`
- **Goal type name**: `PartnerNetworth`
- **Example**: "Ensure lane partner has 5000 gold by 10 minutes"
- **Data source**: Requires per-minute networth data for all players from OpenDota parsed matches (`players[].net_worth` array, similar to `lh_t`/`dn_t`)
- **Lane partner detection**: Identify the partner by proximity/lane assignment (see role detection dependency below)

### Implementation Notes

#### Backend (Rust)
- Add `PartnerNetworth` variant to `GoalMetric` enum
- Store per-minute networth for all players in a new or extended `player_laning_data` table
- `evaluate_goal_for_match()`: identify lane partner slot, look up their networth at `target_time_minutes`, compare to `target_value`
- Returns `None` (unevaluable) if lane partner cannot be determined or match is unparsed

#### Frontend (Svelte)
- Add "Partner Networth" option to the metric dropdown
- Display as: "Lane partner: {target_value}g by {target_time_minutes} min"
- Show a supporting icon/label distinguishing it as a support-role metric

## Full Support Metrics (Future)

Once denies and partner networth are working, expand to:
- **Assists** – total assists by end of game (e.g., "10+ assists per game")
- **Observer wards placed** – from OpenDota match data
- **Sentry wards placed**
- **Healing done** – from match detail stats
- **Camps stacked**
- **Roshan assist** participation
- **Partner CS** – lane partner's last hits at target time
- **Partner Level** – lane partner's level at target time

## Implementation Notes

### Backend (Rust)
- Add `Denies` variant to `GoalMetric` enum in `database.rs`
- Update `evaluate_goal_for_match()` to handle `Denies` metric using `match_cs.denies` at the target minute
- Update suggestion generation to optionally suggest deny goals for heroes with support tendencies

### Frontend (Svelte)
- Add "Denies" option to the metric dropdown in the goal creation page
- Show deny data in goal details page histogram (already stored in `match_cs`)
- Update `getMetricLabel` / `getMetricUnit` helpers in match and goal pages

## Dependencies

### Role Detection (Required)

Lane partner identification for `PartnerNetworth` (and future partner goals) requires knowing which position/lane each player was in. This task depends on **role detection** being implemented first:

- Each match must have `role` stored (position 1–5) sourced from OpenDota's `lane_role` field
- Lane partner is the teammate sharing the same lane (determined from `lane_role` or proximity)
- Without role/lane data, partner goals fall back to "unevaluable"

See task: [`upnext/role-detection.md`](../upnext/role-detection.md)

## Acceptance Criteria
- [ ] Can create a goal: "Achieve X denies by N minutes"
- [ ] Goal is evaluated against parsed matches using `match_cs.denies`
- [ ] Goal appears correctly in goal details histogram
- [ ] Suggestion system can suggest a deny goal
- [ ] Can create a goal: "Lane partner has X gold by N minutes"
- [ ] Lane partner is correctly identified using role/lane data from OpenDota
- [ ] Partner networth goal is evaluated and displayed correctly
- [ ] Goal is shown as unevaluable when lane partner cannot be determined
