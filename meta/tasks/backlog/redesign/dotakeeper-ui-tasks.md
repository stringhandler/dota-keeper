# DotaKeeper UI Redesign Tasks

> Reference design: `dota-keeper-redesign.html`

---

## Dashboard

- [ ] Add a **quick stats strip** at the top with 4 cards: Win Rate (7d), Avg CS @ 10min, Goals Hit (7d), Active Goals count
- [ ] Replace the date-grid goal table with **goal cards** — each card shows: hero avatar, goal name, a progress bar, and 7 colored hit/miss dots (✓ = green, ✗ = grey, empty = dashed)
- [ ] Add a trend label to each goal card (`Improving`, `Declining`, `Needs Attention`) based on recent hit rate
- [ ] Move the **Today's Challenge** section above the goal list, styled as a highlighted banner card with a progress bar and XP reward label
- [ ] Add a `Manage Goals` shortcut button in the section header that links to the Goals page
- [ ] Remove the `VIEW DETAILS` buttons from the goal rows — make the entire row clickable to open a goal detail modal instead

---

## Goal Detail Modal

- [ ] Create a **modal component** triggered when a goal row is clicked (on Dashboard or Goals page)
- [ ] Modal header: hero name + goal name, goal type tag (CS / Item / Win), creation date
- [ ] Add a **stats row** inside the modal: Hit Rate %, Times Attempted, Current Streak
- [ ] Add a **recent attempts list** showing: date, match ID, result value (e.g. actual CS or item timing), HIT/MISS status
- [ ] Add a **contextual tip box** at the bottom — e.g. "You hit this goal more in Turbo games"
- [ ] Add two action buttons: `Edit Goal` and `View Full Analysis` (links to Analysis page filtered by that hero)

---

## Matches

- [ ] Add **filter chips** row above the table: All, Wins, Losses, Ranked, Turbo, and one chip per tracked hero
- [ ] Make filter chips toggle-active on click and filter the match rows accordingly
- [ ] Replace the `RE-PARSE` button column with a **Goals chip** — grey if 0 goals hit, gold-highlighted if any goals were hit (e.g. `2/3 ⚡`)
- [ ] Replace the plain text game mode with a **styled mode tag**: teal pill for Turbo, gold pill for Ranked
- [ ] Make each **match row clickable** to open a match detail view or modal
- [ ] Move the `REFRESH MATCHES` button into the filter bar row (top-right) and style it as a primary button

---

## Analysis

- [ ] Replace the single-column layout with a **2-column card grid**
- [ ] Add a **per-hero bar chart** card showing average LH @ 10min for each hero the player has played
- [ ] Add a **performance range card** showing Min / Avg / Max with a visual range bar and a marker at the player's average
- [ ] Add an **insight callout box** in the hero breakdown card — automatically generated text highlighting the most actionable finding (e.g. hero already above goal target, or hero far below average)
- [ ] Add a **trend chart card** for the currently selected hero with a sparkline and a Stable / Improving / Declining label
- [ ] Move all filter dropdowns (Time Marker, Window Size, Hero, Game Mode) into a single horizontal filters row at the top of the page
- [ ] Remove the `SHOW` filter dropdown — replace its functionality with the hero-specific trend card

---

## Goals Management

- [ ] Add an **inline Create Goal form** at the top of the page with fields: Hero (dropdown), Goal Type (dropdown), Target (text input), and an `Add Goal` button — no need to navigate away
- [ ] Display all active goals as the same **goal card** component used on the Dashboard
- [ ] Add `Edit` and `Delete` buttons inside each goal card (inline, not in a separate settings page)
- [ ] Show a **contextual warning** on goals where the player's average already exceeds the target (e.g. "Your avg Viper CS is 48 — consider raising this goal")
- [ ] Add an `Archive All` button in the section header for bulk-archiving completed or stale goals

---

## Sidebar & Global Nav

- [ ] Add a **rank display** at the bottom of the sidebar showing current rank name (e.g. Archon III) with a star icon
- [ ] Add a **Daily Challenge badge** in the top bar showing current challenge progress (e.g. `⚡ Daily Challenge: 0/1`) — clickable to jump to the challenge detail
- [ ] Add a `+ New Goal` primary button to the top bar globally, so goal creation is always one click away
- [ ] Add a left-border **active indicator line** on the current nav item (gold, 2px)
- [ ] Ensure nav items show a subtle gold background tint when active, not just a colour change
