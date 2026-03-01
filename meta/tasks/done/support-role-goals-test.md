# Testing: Support Role Goals

## Denies Goal

1. Go to Goals → create a new goal
2. Select metric **Denies**, set a target value (e.g. 10 denies) and a time (e.g. 10 min)
3. Save the goal
4. On the Goals list, verify the card shows e.g. "Any Hero — 10 denies by 10 min"
5. Open the goal detail page — verify the histogram renders using actual deny values from `match_cs.denies`
6. Check the database to confirm evaluation is using denies:
   ```sql
   SELECT match_id, minute, denies FROM match_cs WHERE match_id = <some_parsed_match> LIMIT 5;
   ```

## Partner Networth Goal

1. Create a new goal with metric **Partner Networth**, e.g. "Partner: 5000g by 10 min"
2. Save and verify the card shows "Any Hero — Partner: 5000g by 10 min"
3. Open goal detail — verify matches where you played support show networth data; core matches show as unevaluable (not included in histogram)
4. After reparsing a support match, check the DB:
   ```sql
   -- Partner slot was stored
   SELECT match_id, role, partner_slot FROM matches WHERE parse_state = 'parsed' LIMIT 10;

   -- Networth data was stored for all players
   SELECT match_id, player_slot, minute, networth FROM player_networth WHERE match_id = <id> ORDER BY player_slot, minute LIMIT 20;
   ```

## What was implemented

- `Denies` and `PartnerNetworth` added to `GoalMetric` enum with full evaluation
- New `player_networth` table — per-minute networth for all 10 players per match
- `partner_slot` column on `matches` — set during parsing for support players
- `net_worth` field added to `DetailedPlayer` (parsed from OpenDota)
- All three parse paths (single, backfill, reparse) now store networth + partner_slot
- Frontend: metric dropdown, labels, units, and goal descriptions updated for both new metrics
