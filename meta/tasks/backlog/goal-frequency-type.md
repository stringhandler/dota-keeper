# Goal Frequency / Consistency Type

## Summary
When creating a goal, let users specify how often they want to achieve it — consistently across games, occasionally, or just once — rather than assuming all goals are consistency-based.

## Motivation
Currently all goals implicitly target consistency (e.g. hit X in ~75–80% of games). But some users may set aspirational one-off targets ("hit 90 last hits at 10 minutes at least once") or soft targets ("average this over a week"). Forcing all goals into a single mould creates confusion and inaccurate pass/fail evaluation.

## Proposed Design

### Goal Frequency Field
When creating or editing a goal, add a **"Frequency"** (or "How often?") dropdown with options:

| Option | Meaning | Evaluation |
|---|---|---|
| Just once | Achieve this at least one time ever | Pass as soon as it's hit once |
| On average | Hit this on average across all games in window | Compare rolling average to threshold |
| 50% of games | Hit this in at least half of games | % pass rate >= 50% |
| 75% of games | Hit this in most games (default) | % pass rate >= 75% |
| 90% of games | Hit this in almost every game | % pass rate >= 90% |

The default should remain **75% of games** to align with the project's target achievement rate design principle.

### UI Changes
- Add the frequency dropdown to the goal creation form
- Display the selected frequency on the goal card (e.g. "50 last hits at 10min · 75% of games")
- Goal pass/fail status and history charts must respect the frequency type when evaluating

### Evaluation Logic
- `Just once`: goal is permanently marked as achieved once the threshold is ever met; no further evaluation needed
- `On average`: compare the rolling mean of the metric to the threshold over the selected window
- Percentage options: use the existing % success rate logic with the chosen target rate

## Acceptance Criteria
- Frequency field is present on goal creation and edit forms
- Default frequency is "75% of games"
- Goal cards and detail views display the frequency
- Evaluation engine uses the correct logic per frequency type
- "Just once" goals show a permanent "Achieved" badge once met, with the date it was first achieved

## Notes
- The `On average over a week` idea from the original note is covered by combining the "On average" type with a time window filter — this can be a follow-up
- Frequency type should be stored in the `goals` table in SQLite (new column)
