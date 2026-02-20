# Support Goals — Lane Partner Performance Tracking

## Overview

Add a new goal type for support players that tracks their effectiveness based on their lane partner's CS (creep score) and experience, rather than their own. This reflects the reality that support players enable their carries rather than farming themselves.

**Important**: Support goals should be based on the **partnership of the lane** — measuring how well you and your lane partner perform together during the laning phase (typically 0-10 minutes). The goal is to track your contribution to your core's success, not individual performance.

## Motivation

Current goal system focuses on individual stats (CS, kills, net worth, item timings), which are carry/core-centric metrics. Support players need goals that measure their impact on their lane partner's success.

## Goal Types

### 1. Lane Partner CS Goal
**Format**: "Ensure lane partner has X CS by Y minutes"

**Example**:
- "Ensure lane partner has 50 CS by 10 minutes (Ranked)"
- "Ensure lane partner has 35 CS by 10 minutes (Turbo)"

**How it works**:
- Identify the lane partner (player with closest proximity during laning phase 0-10min)
- Track their CS at the specified time
- Goal achieved if partner's CS >= target

### 2. Lane Partner Experience Goal
**Format**: "Ensure lane partner reaches Level X by Y minutes"

**Example**:
- "Ensure lane partner reaches Level 6 by 10 minutes"
- "Ensure lane partner reaches Level 10 by 15 minutes"

**How it works**:
- Track lane partner's level at specified time
- Goal achieved if partner's level >= target

### 3. Combined Lane Dominance Goal
**Format**: "Lane partner has X CS and Level Y by Z minutes"

**Example**:
- "Lane partner has 40 CS and Level 6 by 10 minutes"

## Implementation

### Backend (Rust)

#### 1. Database Schema Changes

Add new goal metric types:
```sql
-- In goals table, add new metric types:
-- "PartnerCS" - track lane partner's CS
-- "PartnerLevel" - track lane partner's level
-- "PartnerCombined" - track both CS and level
```

#### 2. Lane Partner Detection

Create a new function to identify the lane partner:
```rust
fn identify_lane_partner(
    match_id: i64,
    player_slot: u8,
    all_players: Vec<PlayerData>
) -> Option<u8> {
    // Algorithm:
    // 1. Determine player's lane (based on position in first 10 min)
    // 2. Find teammate with closest proximity during 0-10 min
    // 3. Prefer player with "core" role (position 1, 2, 3)
    // Return player_slot of identified partner
}
```

This requires per-minute position data from OpenDota's parsed matches.

#### 3. New Goal Evaluation

```rust
fn evaluate_partner_goal(
    goal: &Goal,
    match_data: &MatchData,
    partner_data: &PlayerData
) -> bool {
    match goal.metric.as_str() {
        "PartnerCS" => {
            let partner_cs = get_cs_at_time(partner_data, goal.target_time_minutes);
            partner_cs >= goal.target_value
        }
        "PartnerLevel" => {
            let partner_level = get_level_at_time(partner_data, goal.target_time_minutes);
            partner_level >= goal.target_value
        }
        "PartnerCombined" => {
            // target_value = CS threshold
            // item_id = level threshold (repurpose this field)
            let cs = get_cs_at_time(partner_data, goal.target_time_minutes);
            let level = get_level_at_time(partner_data, goal.target_time_minutes);
            cs >= goal.target_value && level >= (goal.item_id.unwrap_or(0) as i32)
        }
        _ => false
    }
}
```

#### 4. Data Fetching

Extend OpenDota parsing to fetch:
- Per-minute player positions (`players[].lane_pos`)
- Per-minute CS data for all players
- Per-minute level/XP data for all players

Store in new table:
```sql
CREATE TABLE player_laning_data (
    match_id INTEGER,
    player_slot INTEGER,
    minute INTEGER,
    cs INTEGER,
    level INTEGER,
    position_x REAL,
    position_y REAL,
    PRIMARY KEY (match_id, player_slot, minute)
);
```

### Frontend (Svelte)

#### 1. Goals Form Update

Add new metric options in goals create/edit form:
```svelte
<select bind:value={formMetric}>
  <option value="LastHits">Last Hits</option>
  <option value="Networth">Net Worth</option>
  <option value="Kills">Kills</option>
  <option value="Level">Level</option>
  <option value="ItemTiming">Item Timing</option>
  <option value="PartnerCS">Partner CS (Support)</option>
  <option value="PartnerLevel">Partner Level (Support)</option>
  <option value="PartnerCombined">Partner CS + Level (Support)</option>
</select>
```

When `PartnerCS` or `PartnerLevel` or `PartnerCombined` is selected:
- Show helper text: "⚠️ Support goals track your lane partner's stats, not yours"
- For `PartnerCombined`, show two fields: CS threshold + Level threshold

#### 2. Goal Display

Update goal cards to show partner-based goals differently:
```svelte
{#if goal.metric === "PartnerCS"}
  <span class="goal-icon">🤝</span>
  Lane partner: {goal.target_value} CS by {goal.target_time_minutes} min
{:else if goal.metric === "PartnerLevel"}
  <span class="goal-icon">🤝</span>
  Lane partner: Level {goal.target_value} by {goal.target_time_minutes} min
{/if}
```

#### 3. Match Detail View

Show lane partner identification:
```svelte
{#if goal.metric.startsWith("Partner")}
  <div class="lane-partner-info">
    <strong>Lane Partner:</strong>
    <HeroIcon heroId={partnerHeroId} />
    {partnerHeroName}
    (CS: {partnerCS}, Level: {partnerLevel})
  </div>
{/if}
```

## Edge Cases

1. **No clear lane partner detected** (e.g. jungling, roaming)
   - Goal evaluation returns N/A or "Unable to determine lane partner"
   - Don't count as achieved or failed

2. **Partner disconnects/abandons**
   - If partner has < 5 minutes played, goal is N/A
   - Otherwise, evaluate based on actual stats at time of abandon

3. **Trilane scenarios**
   - Prioritize the core player (position 1/2 likely)
   - If multiple cores, choose the one with most proximity time

4. **Dual support lanes (2 supports + 1 core)**
   - Both supports track the same core partner

## Acceptance Criteria

- [ ] New goal types appear in goal creation form
- [ ] Support goals are correctly identified in UI (icon, description)
- [ ] Lane partner is accurately detected from match data
- [ ] Partner's CS and level are correctly extracted at target time
- [ ] Goals are evaluated correctly (achieved/failed)
- [ ] Match detail view shows identified lane partner
- [ ] Goal suggestions system can suggest partner-based goals for support players
- [ ] Analytics events track "partner_goal_created" separately

## Future Enhancements

- **Deny Goals**: "Deny X creeps by 10 minutes"
- **Stacking Goals**: "Stack camps Y times in first 10 minutes"
- **Vision Goals**: "Place X wards in first 10 minutes"
- **Pull Goals**: "Pull lane creeps Z times in first 10 minutes"

## Dependencies

- Requires OpenDota API to return per-minute data for all players in match
- Requires position data to determine laning partners
- May need enhanced parsing for older matches

## Design Philosophy: Lane Partnership

Support goals are fundamentally about **lane partnership** — the synergy between support and core during the laning phase. Key principles:

1. **Shared Success**: The support's performance is measured by how well their lane partner performs, not their own stats
2. **Laning Phase Focus**: Goals target the critical 0-10 minute window when support impact is highest
3. **Enabling Metrics**: Track CS and levels that the support helped their core achieve through zoning, pulling, stacking, and harassing
4. **Role Recognition**: Acknowledges that position 4/5 players win games by making their cores strong, not by farming or getting kills themselves

A successful support player ensures their carry has good CS and levels, even if the support ends the laning phase with 0 CS and level 3.

## Notes

This feature is particularly valuable for position 4/5 support players who want to track their enabling effectiveness rather than individual performance metrics. It shifts the focus from "what did I achieve?" to "how well did I enable my lane partner?"
