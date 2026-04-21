# Task: Performance Journal tab

## Overview

Add a **Performance Journal** tab that surfaces the user's best and worst individual game performances, giving them a quick way to reflect on standout games in either direction.

## Requirements

- New route: `/journal` (or add as a tab within the Analysis page — TBD based on nav space)
- Add navigation entry in the sidebar

### Best Performances section
- Top N games ranked by a composite score or per individual metric (KDA, CS, GPM, XPM, damage, etc.)
- Allow filtering by metric so the user can see e.g. "my best CS games" or "my highest KDA games"
- Allow filtering by hero so the user can see their best/worst performances on a specific hero
- Show hero played, result (W/L), date, and the standout stat value
- Clicking a row navigates to the match detail page

### Worst Performances section
- Same layout as Best Performances but sorted ascending
- Useful for identifying recurring weak areas

### Metrics to support (at minimum)
- KDA
- Last hits at 10 min
- GPM / XPM
- Hero damage
- Deaths

### Post-game Reflection Survey

When a completed match is detected (after match sync) and it qualifies as either:
- A **best performance** — lands in the user's top 10 for any metric against their goals, or
- A **worst performance** — lands in the user's bottom 10 for any metric against their goals

…prompt the user with a short reflection survey while the game is still fresh.

**Survey questions (quick-tap format, not free-text):**

For a best game:
- What went well? *(multi-select chips: "Good draft", "Lane dominance", "Teamfight", "Farm efficiency", "Communication", "Mental focus", "Opponent was weak")*
- Would you say this was a peak performance or got lucky? *(Peak / Mixed / Lucky)*

For a worst game:
- What went wrong? *(multi-select chips: "Bad draft", "Lost lane", "Poor teamfight", "Fell behind on farm", "Tilted", "Team issues", "Outplayed")*
- What would you do differently? *(free-text, optional, max 200 chars)*

The survey should appear as a modal (similar to the existing MoodCheckin modal) triggered once per match — do not re-prompt if the user has already answered or dismissed for that match.

**Storage:** Save survey responses to a new `performance_reflections` table:
```sql
CREATE TABLE performance_reflections (
  match_id    INTEGER PRIMARY KEY,
  type        TEXT NOT NULL,  -- 'best' | 'worst'
  tags        TEXT NOT NULL,  -- JSON array of selected chips
  rating      TEXT,           -- 'peak' | 'mixed' | 'lucky' (best only)
  notes       TEXT,           -- optional free-text (worst only)
  created_at  TEXT NOT NULL
);
```

Reflection responses should be visible on the match detail page and summarised in the Performance Journal (e.g. tag frequency across best games to highlight recurring strengths).

## Implementation Notes

- Query data from the existing `matches` / `match_players` tables — no new DB schema needed
- Add a Tauri command `get_performance_journal(metric, hero_id, limit)` returning top/bottom N matches for that metric, optionally filtered to a specific hero (`hero_id = null` means all heroes)
- Default limit: 10 per section
- Only include parsed matches (where per-minute data is available) for last-hit metrics; use all matches for KDA/GPM/etc.

## Acceptance Criteria

- [ ] Performance Journal accessible from main navigation
- [ ] Best and worst sections visible on load (default metric: KDA)
- [ ] Metric selector switches both sections simultaneously
- [ ] Hero filter (uses existing HeroSelect component) narrows results to that hero; defaults to "All heroes"
- [ ] Each row links to the match detail page
- [ ] Works with data from both OpenDota and Stratz
- [ ] After a match sync, qualifying best/worst games trigger the reflection survey modal
- [ ] Survey is shown once per match; dismissed or answered matches are not re-prompted
- [ ] Responses saved to `performance_reflections` table
- [ ] Reflection tags shown on the match detail page
- [ ] Tag frequency summary shown in the Performance Journal
