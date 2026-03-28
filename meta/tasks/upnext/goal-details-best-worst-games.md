# Goal Details: Best & Worst Games

## Description

On the goal details page, show two sections: **Best Games** and **Worst Games** ranked by performance according to that goal's metric.

## Requirements

### Game Lists
- Show the top N best and worst matches scored by the goal's metric (e.g. lowest deaths for a deaths goal, highest win rate context, etc.)
- Each match row should be clickable and open the **match details page** (same as used elsewhere in the app)

### Filters

#### Hero Filter
- If the goal is **not** hero-specific (i.e. applies to "any hero"), show a **hero filter** so the user can narrow results to a specific hero

#### Game Mode Filter
- If the goal's hero filter is set to **"any"**, show a **ranked/turbo filter** (All / Ranked / Turbo) to let the user slice results by game mode

### Sorting / Scoring
- Best games = matches where the goal metric was most achieved (e.g. fewest deaths, highest KDA, etc.)
- Worst games = matches where the goal metric was least achieved
- Use the same metric logic already used to evaluate goal pass/fail

## Acceptance Criteria
- [ ] Best games section visible on goal details page
- [ ] Worst games section visible on goal details page
- [ ] Hero filter shown when goal applies to any hero
- [ ] Game mode filter (All / Ranked / Turbo) shown when goal hero is "any"
- [ ] Clicking a match opens the match details page
- [ ] Filters update both best and worst lists reactively
