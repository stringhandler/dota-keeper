# Mental Health: Tilt Analysis Engine

## Epic
Part of: [epic-mental-health-tilt-tracking.md](epic-mental-health-tilt-tracking.md)

## Priority
**HIGH** (implement after post-game check-in)

## Overview

The tilt analysis engine runs in the background and correlates mood check-in data with match performance statistics to detect tilt patterns. It produces a **tilt score** and a **pattern classification** that power the suggestion system.

## Tilt Signals

The engine looks at two layers of data:

### Layer 1: Objective Performance Signals (always available)
These can be computed even without any check-in data:

| Signal | Tilt Indicator |
|--------|----------------|
| Deaths in last 3 games vs. 30-game baseline | Deaths ≥ 50% above baseline |
| KDA in last 3 games vs. 30-game baseline | KDA ≤ 60% of baseline |
| Loss streak in current session | 3+ consecutive losses |
| Win rate in current session vs. 7d average | Session WR ≤ 50% of 7d WR |
| Games played today | ≥ 5 games in a day |
| Match duration distribution | Many short games (early GG culture) |

### Layer 2: Subjective Mood Signals (from check-ins)
These enrich the picture when check-in data exists:

| Signal | Tilt Indicator |
|--------|----------------|
| Calm score trend | Declining over last 3 check-ins |
| Calm score absolute | ≤ 2 in last check-in |
| Energy score | ≤ 2 (fatigue signal) |
| Attribution: "Teammates" repeated | Team-blaming pattern |

### Tilt Score Calculation

A composite score 0–100:

```
tilt_score = (
  objective_signals_score * 0.6 +
  subjective_signals_score * 0.4
)
```

If no check-in data, weight objective signals 100%.

**Thresholds:**
- 0–30: Healthy — no intervention needed
- 31–55: Mild tilt — monitor, show gentle note
- 56–75: Moderate tilt — show suggestion card
- 76–100: High tilt — show stronger suggestion, recommend break

## Pattern Classification

Beyond the score, classify the pattern type for targeted suggestions:

| Pattern | Signs |
|---------|-------|
| `fatigue` | Energy ≤ 2, 5+ games today, late-session decline |
| `loss_spiral` | 3+ losses, deaths rising, rapid re-queuing |
| `team_friction` | Attribution = "Teammates" ≥ 2x, KDA fine but WR dropping |
| `self_doubt` | Attribution = "My own mistakes", calm dropping |
| `peak_performance` | All signals positive — reinforce this state |

## Backend Implementation

### New Rust command: `get_tilt_assessment()`

Returns:
```json
{
  "tilt_score": 62,
  "pattern": "loss_spiral",
  "signals": [
    { "type": "loss_streak", "value": 4, "threshold": 3 },
    { "type": "deaths_elevated", "value": 1.8, "threshold": 1.5 }
  ],
  "last_checkin": { "energy": 2, "calm": 2 },
  "has_sufficient_data": true
}
```

Called by Dashboard on load (if mental health tracking is enabled).

### Data Requirements

- Needs access to recent matches (already available)
- Needs mood_checkins table (from post-game check-in task)
- Pure read — no side effects

## Frontend Usage

- Dashboard card reads tilt assessment and chooses which suggestion to show
- Analysis page could optionally show a tilt trend chart over time
- Settings page shows "Mental Health" section only if enabled

## Acceptance Criteria

- [ ] Tilt score calculated correctly from objective signals alone (before any check-ins exist)
- [ ] Tilt score updates when new check-in data is added
- [ ] Pattern classification is one of the defined types
- [ ] Assessment is only computed when mental health tracking is enabled
- [ ] Edge case: fewer than 3 games played — returns low confidence, no suggestions shown
- [ ] Performance: runs in < 50ms (SQLite queries, no heavy computation)
