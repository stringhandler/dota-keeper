# Bug: Networth Goal Not Registering / No Analysis Data

## Description

A Networth goal (e.g. 6000 gold by 10 minutes, scope: Any Core) does not register
match results or show any data in the analysis view. The goal card appears but progress
is empty and no historical data is displayed.

## Steps to Reproduce

1. Navigate to the **Goals** page.
2. Create a goal:
   - Metric: **Net Worth**
   - Target: **6000**
   - By (min): **10**
   - Hero: **Any Core (pos 1/2/3)**
   - Mode: **Ranked**
3. Click **Add Goal**.
4. Navigate to the goal's detail page.
5. Observe: no progress data, no match history, no pass/fail breakdown.

## Expected Behaviour

Recent matches where the player played a core hero should be evaluated against the
6000 gold at 10 min target. Pass/fail counts and a trend chart should be visible.

## Actual Behaviour

Goal card shows no data. Analysis section is empty or shows zeros.

## Notes

- The `hero_scope` field (`any_core`) may not be handled in the analysis query.
- Check whether the Rust backend's networth analysis command filters by `hero_scope`
  correctly, or whether it only handles specific `hero_id` values.
- The `get_last_hits_analysis_data` command is used for Last Hits — confirm there is
  an equivalent for Networth that also supports `hero_scope`.
