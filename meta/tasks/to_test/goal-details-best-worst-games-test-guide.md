# Test Guide: Goal Details Best & Worst Games

## Setup
Make sure you have a goal with enough parsed matches (ideally 10+) to populate the lists.

## Steps

### 1. Best & Worst game lists appear
1. Open any goal with multiple parsed matches
2. Scroll to the bottom of the goal details page
3. Verify two side-by-side sections appear: **Best Games** and **Worst Games**
4. Confirm each shows up to 10 rows

### 2. Correct sorting
- For a **LastHits / Networth / Kills / Denies** goal: Best should have the highest values, Worst the lowest
- For an **ItemTiming** goal: Best should have the lowest timing (earliest), Worst the highest

### 3. Match row click
- Click any row in either list
- Confirm it navigates to the correct match details page (`/matches/{id}`)

### 4. Hero filter (any-hero goals)
1. Open a goal that applies to **Any Hero** (no specific hero set)
2. Confirm the Hero filter dropdown is visible in the filters row
3. Select a hero — both Best and Worst lists should update to show only that hero's matches

### 5. Game mode filter (any-hero + All mode goals)
1. Open a goal with **Any Hero** AND **Game Mode = All**
2. Confirm the **All / Ranked / Turbo** game mode filter is visible
3. Switch between modes — lists should update reactively
4. Confirm the game mode filter is **not** shown for goals scoped to a specific hero

### 6. Filters reset
- Apply hero + game mode filters, then click **Reset** — all filters should clear including game mode back to "All"

### 7. No data state
- Apply a filter that results in zero matches (e.g., a hero with no games for this goal)
- Confirm the Best/Worst sections don't appear (hidden when `filteredData.length === 0`)
