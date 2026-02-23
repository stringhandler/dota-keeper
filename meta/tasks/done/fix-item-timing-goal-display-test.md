# Test Steps: Fix Item Timing Goal Display

## What Was Fixed

In `src/routes/matches/[matchId]/+page.svelte`, the goal display section didn't handle `ItemTiming` goals correctly:

- Target showed raw seconds with "by 0m" (e.g. "Target: 200  by 0m")
- Actual showed raw seconds with no formatting (e.g. "Actual: 147")
- Title showed "ItemTiming" (the raw enum string) instead of the item name
- Items data wasn't being loaded, so item names couldn't be resolved

## Test Steps

1. Make sure you have at least one **Item Timing** goal set up (e.g., Battle Fury by 20:00 on a carry hero)
2. Navigate to a match where that hero was played and the item was purchased
3. On the Match Detail page, scroll to the **Goals** section
4. Verify the goal card shows:
   - **Title**: `<HeroName> — <ItemName> (Item Timing)` e.g. "Anti-Mage — Battle Fury (Item Timing)"
   - **Target**: `Target: by M:SS` e.g. "Target: by 20:00"
   - **Actual**: `Actual: M:SS` e.g. "Actual: 2:27" (not "Actual: 147")
5. Check both an **achieved** (green border) and **not-achieved** (red border) case if possible
6. Verify non-ItemTiming goals (e.g. LastHits) still display correctly with their original format
