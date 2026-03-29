# Test Guide: Match Details — Goal Comparisons & Compare to Match

## Goal comparison chips (per goal card)

1. Open any parsed match that has applicable goals
2. Scroll to the **Goals** section
3. Each goal card should show small chips below the target/actual values:
   - **Last** — value from the most recent previous match where this goal applied, with a green `+N` / red `-N` diff vs. this match
   - **Best** — your all-time best value for this goal, with diff
4. If no previous match exists for a goal, the "Last" chip should not appear
5. For **ItemTiming** goals, values should display as M:SS (e.g. `7:30`)
6. For lower-is-better goals (ItemTiming, Deaths): green diff = lower than reference, red = higher

---

## "Compare to" button

1. Click the **⇄ Compare to** button in the page header
2. Popup opens with three options:

### Last game
- Button shows the hero name and date of your most recent match
- Click it — popup closes and a **Comparison** panel appears at the bottom of the page
- Panel shows both matches side-by-side with GPM, XPM, last hits, hero dmg, tower dmg, healing, KDA, duration
- The better value in each row should be highlighted green
- "This match" column is labelled with a gold border and hero name

### Last game with this hero
- Shows the most recent match on the **same hero**
- If no prior match with this hero, button is disabled
- Selecting it loads comparison the same way

### Specific match ID
- Type any match ID in the text field and press Enter or click **Go**
- If found in history, comparison panel loads
- If not found, shows error "Match not found in your history."

### Clear comparison
- Click **✕ Clear** in the comparison panel header to dismiss it

---

## Re-parse button (tested alongside)
- Click **↺ Re-parse** — button should change to **⟳ Parsing…** with a pulse animation
- After parse completes, stats on the page update automatically (GPM, XPM, etc.)
- Any parse errors appear below the button row
