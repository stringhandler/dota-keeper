# Task: Goal timelines (short / medium / long)

## Overview

Allow users to assign a **timeline** to each goal, giving it a defined horizon over which progress is evaluated. This adds a time-pressure dimension to the existing goal system and makes it clearer whether a goal is a quick focus or a long-term commitment.

## Timeline Options

| Label  | Duration  |
|--------|-----------|
| Short  | 1 week    |
| Medium | 1 month   |
| Long   | 6 months  |

A goal may also have no timeline (current behaviour — rolling evaluation).

## Requirements

- Add a timeline selector to the **Create Goal** and **Edit Goal** forms (optional field, defaults to none)
- Store the timeline and a `started_at` date on the goal record
- On the Goals page and Goal Detail page, show:
  - The timeline label (Short / Medium / Long)
  - Days remaining (e.g. "18 days left") or "Expired" if the deadline has passed
  - A thin progress bar showing how far through the timeline the user is
- When a goal's timeline expires:
  - Mark it as **completed** or **failed** based on whether the achievement rate met the 75% target
  - Show a summary card: "You hit this goal X% of the time over 1 month"
  - Move expired goals to an "Archive" section rather than deleting them
- Dashboard goal cards should show the timeline badge and days-remaining chip

## DB Changes

Add columns to the `goals` table:

```sql
ALTER TABLE goals ADD COLUMN timeline TEXT;        -- 'short' | 'medium' | 'long' | NULL
ALTER TABLE goals ADD COLUMN timeline_started_at TEXT;  -- ISO date, set when goal is created/timeline assigned
ALTER TABLE goals ADD COLUMN timeline_ended_at TEXT;    -- set when expired/archived
ALTER TABLE goals ADD COLUMN timeline_result TEXT;      -- 'achieved' | 'failed' | NULL
```

## Implementation Notes

- "Days remaining" computed in the frontend from `timeline_started_at` + duration
- Expiry check can run on app startup or when the Goals page loads — no background daemon needed
- Archive section on Goals page: collapsed by default, expandable
- Keep existing no-timeline goals working as before

## Acceptance Criteria

- [ ] Timeline selector in create/edit goal forms (optional)
- [ ] Timeline badge and days-remaining shown on goal cards and goal detail
- [ ] Progress bar showing time elapsed through the timeline
- [ ] Expired goals auto-marked achieved/failed based on 75% target
- [ ] Expired goals moved to Archive section on Goals page
- [ ] Summary card shown for expired goals
- [ ] Dashboard goal cards show timeline info
- [ ] No-timeline goals unaffected
