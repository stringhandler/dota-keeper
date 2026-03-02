# Mental Health: Tilt Suggestions & Interventions

## Epic
Part of: [epic-mental-health-tilt-tracking.md](epic-mental-health-tilt-tracking.md)

## Priority
**MEDIUM** (implement after tilt analysis engine)

## Overview

When the tilt analysis engine detects a pattern, a contextual suggestion card appears on the Dashboard. Suggestions are carefully framed to be helpful rather than preachy. They are based on evidence from sports psychology and esports coaching.

## Suggestion Library

Each suggestion is tied to a tilt pattern and a severity level.

### `fatigue` pattern

**Mild (score 31–55):**
> "You've played quite a bit today. Even top pros schedule regular breaks — your reaction time and decision-making genuinely improve with rest."

**Moderate–High (score 56+):**
> "5+ games in one sitting is a lot. Fatigue is one of the biggest hidden performance drains in Dota. A 15-minute break — walk, water, stretch — often makes the next game feel completely different."
>
> **Actions:** [Take a Break] [Play One More Anyway]

### `loss_spiral` pattern

**Mild:**
> "3 losses in a row. It happens to everyone. Consider whether a different hero or role might give you a mental reset."

**Moderate–High:**
> "You're in a loss streak and your deaths are climbing. This is the classic tilt spiral — each loss makes the next one more likely. The single most effective thing you can do right now is stop and come back in an hour."
>
> **Actions:** [Step Away] [Switch to Turbo for Fun] [Keep Grinding]

### `team_friction` pattern

**Mild:**
> "Rough teammates lately? It's worth remembering that in a 5-player team game, your individual impact is highest when you focus on what *you* can control."

**Moderate:**
> "When frustration comes from teammates, it often makes us play more aggressively or alone — which backfires. Try focusing purely on your own farm, positioning, and cooldown usage this next game."

### `self_doubt` pattern

**Mild:**
> "You're being hard on yourself. That self-awareness is actually a strength — most players never notice their mistakes at all."

**Moderate:**
> "You seem focused on your own errors. Replaying mistakes mentally during a game is called 'paralysis by analysis' — try to notice mistakes once, then let them go and focus on the next decision."

### `peak_performance` pattern (positive reinforcement)

> "You're playing well and feeling good about it. This is the state that produces your best Dota — notice what's working so you can recreate it."

## UI Design

Suggestion card on Dashboard, below the quick stats strip:

```
┌─────────────────────────────────────────────┐
│  🧠  Mental Note                            │
│                                             │
│  You've played 5 games today and your       │
│  deaths have been climbing. A short break   │
│  now often leads to a much better game      │
│  when you come back.                        │
│                                             │
│  [Take a Break]    [Keep Playing]   [Dismiss]│
└─────────────────────────────────────────────┘
```

- "Take a Break" → logs a break intention, hides for 60 minutes
- "Keep Playing" → logs override, hides for the session
- "Dismiss" → hides for this app session, no data recorded
- "Learn More" link → opens a brief in-app explanation of the pattern (future)

### Styling Notes
- Icon: 🧠 or a subtle brain/mind SVG
- Colour: use `--teal` accent (not gold/red — this isn't a goal or error)
- Soft, rounded card matching existing card style
- Body text at 14px (larger than usual labels — readability matters here)
- Hidden entirely in privacy mode

## Cooldown Logic

- Same suggestion pattern doesn't repeat within 24 hours
- `peak_performance` shown at most once per week (don't over-celebrate)
- Never show more than one suggestion card at a time (pick highest severity)

## Acceptance Criteria

- [ ] Correct suggestion shown for each of the 5 pattern types
- [ ] Severity level (mild vs moderate/high) changes the suggestion text
- [ ] "Take a Break" action logs intent and hides card for 60 minutes
- [ ] "Dismiss" hides for the session
- [ ] Card is hidden in privacy mode
- [ ] Suggestions don't repeat within 24 hours for the same pattern
- [ ] Card doesn't appear if mental health tracking is disabled
- [ ] Mobile layout is readable (no text truncation on 390px screens)
- [ ] All suggestion text is non-judgmental, positive in framing
