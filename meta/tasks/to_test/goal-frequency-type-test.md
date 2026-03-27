# Test: Goal Frequency / Consistency Type

## Steps

### Create
1. Go to **Goals** page
2. Create a new goal — verify the **"How often?"** dropdown is present with options: Just once / On average / 50% of games / 75% of games (default) / 90% of games
3. Create one goal of each type and save them
4. On the goals list, verify each card shows a small grey frequency badge (e.g. "75% of games")

### Edit
5. Click Edit on a goal — verify the frequency dropdown pre-selects the saved value
6. Change the frequency and save — verify the badge updates on the card

### Detail page — 75% of games (default)
7. Open a goal with match data — verify the Achievement Rate card shows "Target: ~75%" and the status label uses 75% thresholds

### Detail page — Just once
8. Open a "Just once" goal — achievement rate card should show "One-time goal" and either "Achieved!" (green) or "Not yet" (orange), no progress bar

### Detail page — On average
9. Open an "On average" goal — card should show "Average: X vs target Y" and a status of "On track", "Close", "Below", etc., no bar

### Detail page — 50% / 90%
10. Open goals with 50% and 90% frequency — verify target label changes to "Target: ~50%" / "Target: ~90%" and status thresholds shift accordingly

### Existing goals
11. Existing goals with no frequency_type should default to 75% and display correctly (no crashes)
