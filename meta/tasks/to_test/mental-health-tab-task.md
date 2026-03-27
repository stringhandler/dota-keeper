# EPIC: Mental Health & Tilt Tracking

## Overview

Dota 2 is well-documented as one of the most psychologically demanding competitive games. Tilt — the degraded mental state that follows frustrating losses or toxic interactions — is one of the leading causes of poor performance spirals and game burnout. This epic adds a non-intrusive mood and tilt monitoring system to help players stay self-aware, build healthier play habits, and get data-driven suggestions when their mental state is affecting their game.

## Guiding Principles (Esports Psychology)

1. **Non-confrontational measurement** — Never ask "are you tilted?" directly. Use indirect, low-friction check-ins that feel like reflection rather than diagnosis.
2. **Autonomy** — The user fully controls whether tracking is on, and can hide all tilt data at any time.
3. **Positive framing** — Suggestions are never "you're playing bad, stop." They are "here's a pattern we noticed, here's something to try."
4. **Evidence-based suggestions** — Grounded in sports psychology: taking breaks, breathing resets, playing lower-pressure modes, switching roles.
5. **Privacy by design** — All data is local. Nothing is sent anywhere.

## Scope / Sub-Tasks

This epic is broken into the following tasks (see individual files):

| Task | File | Priority |
|------|------|----------|
| Post-game mood check-in (data collection) | `mental-health-postgame-checkin.md` | High |
| Tilt analysis engine | `mental-health-tilt-analysis.md` | High |
| Tilt suggestions & interventions | `mental-health-suggestions.md` | Medium |
| Settings: enable/disable tracking | `mental-health-settings.md` | High |
| Privacy mode: hide tilt data | `mental-health-privacy-mode.md` | Medium |
| Dashboard tilt summary card | `mental-health-dashboard-card.md` | Medium |

## What "Tilt" Looks Like in Data

The system should detect tilt patterns by correlating mood check-in data with match statistics:

- **Performance degradation**: KDA drops >30% in a session compared to baseline
- **Death spike**: Deaths increase significantly in consecutive games
- **Session length signals**: 4+ games played in a row, especially with losses
- **Loss streak**: 3+ losses without a win
- **Win rate cliff**: Session win rate drops well below 7-day average
- **Rapid re-queuing**: Patterns suggesting "one more game" loop (very short gaps between matches)

## Data Model

```sql
CREATE TABLE mood_checkins (
  id          INTEGER PRIMARY KEY,
  match_id    INTEGER REFERENCES matches(match_id),
  checked_at  INTEGER NOT NULL,  -- unix timestamp
  energy      INTEGER,  -- 1-5 (how energised did you feel?)
  frustration INTEGER,  -- 1-5 (how frustrated were you?)
  focus       INTEGER,  -- 1-5 (how focused/in-the-zone were you?)
  notes       TEXT,     -- optional free-text (future)
  hidden      INTEGER DEFAULT 0  -- 1 = hidden in privacy mode
);
```

## User Experience Flow

```
[Match ends]
     ↓
[Dota Keeper detects new match via refresh]
     ↓ (probabilistic — not every game)
[Subtle check-in card appears]
     ↓
[User answers 1-2 quick questions]
     ↓
[Data stored locally]
     ↓
[After pattern detected across sessions]
     ↓
[Suggestion card appears on Dashboard]
```

## Success Metrics

- Users who engage with check-ins ≥3 times have a measurable data set for analysis
- Suggestions are shown only when statistically meaningful patterns are detected
- Feature does not feel intrusive — opt-in and easily dismissible
