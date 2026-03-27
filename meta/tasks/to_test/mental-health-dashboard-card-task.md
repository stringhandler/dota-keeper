# Mental Health: Dashboard Summary Card

## Epic
Part of: [epic-mental-health-tilt-tracking.md](epic-mental-health-tilt-tracking.md)

## Priority
**MEDIUM** (implement after check-in and analysis engine)

## Overview

Once a user has several check-in data points, the Dashboard gets a compact "Mental State" summary card showing their mood trends, current tilt signal, and a link to their full wellbeing history. This replaces the ephemeral suggestion card when things are going well (or augments it when they're not).

## Card Design

### When trending well

```
┌─────────────────────────────────────────────┐
│  🧠  Mental State                           │
│                                             │
│  Energy    ████████░░  4.1 avg (7d)         │
│  Calm      ██████░░░░  3.2 avg (7d)         │
│                                             │
│  ↗  Improving over last 5 sessions          │
└─────────────────────────────────────────────┘
```

### When tilt is detected

```
┌─────────────────────────────────────────────┐
│  🧠  Mental State                      ⚠   │
│                                             │
│  Energy    ████░░░░░░  2.1 avg (7d)         │
│  Calm      ███░░░░░░░  1.8 avg (7d)         │
│                                             │
│  ↘  Declining — see suggestion below        │
└─────────────────────────────────────────────┘
```

### When insufficient data (<3 check-ins)

```
┌─────────────────────────────────────────────┐
│  🧠  Mental State                           │
│                                             │
│  Keep completing check-ins to see your      │
│  mood trends here.                          │
│                                             │
│  2 of 3 needed                              │
└─────────────────────────────────────────────┘
```

## Visual Specifications

- Same card width as quick stat cards
- Icon: 🧠 in gold (or a custom SVG)
- Progress bars: same style as goal progress bars (gradient gold)
- Trend arrows: ↗ green, → gold, ↘ red (matches existing trend indicators)
- Hidden entirely in privacy mode
- Only rendered if mental health tracking is enabled in settings

## Data Required

From `get_tilt_assessment()`:
- 7-day average energy score
- 7-day average calm score
- Trend direction (improving/stable/declining)
- Tilt score (to determine ⚠ badge)

## Placement on Dashboard

Below the quick stats strip, above goal progress. Collapsed by default on mobile (shown as a slim banner that expands on tap) to preserve vertical space.

```
Mobile collapsed:
┌─────────────────────────────┐
│ 🧠 Mental State: Stable ↗  │  ← tap to expand
└─────────────────────────────┘

Mobile expanded (same as desktop card):
┌─────────────────────────────┐
│  🧠  Mental State           │
│  Energy ████░░ 3.8          │
│  Calm   ████░░ 3.5          │
│  ↗ Improving                │
└─────────────────────────────┘
```

## Acceptance Criteria

- [ ] Card shows when mental health tracking is enabled AND ≥ 1 check-in exists
- [ ] Shows energy and calm bars with 7-day averages
- [ ] Trend direction shown with arrow + label
- [ ] Warning badge (⚠) shown when tilt score > 55
- [ ] Insufficient data state shown correctly when < 3 check-ins
- [ ] Card is hidden in privacy mode
- [ ] Mobile: collapsed by default, expands on tap
- [ ] Data loads without blocking other Dashboard data (load in parallel)
