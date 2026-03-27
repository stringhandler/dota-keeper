# Post-Loss Self-Analysis

## Overview

After a loss, prompt the user to reflect on what they think went wrong — before showing them the actual data. The key psychological moment is the **reveal**: they commit to a belief about their own performance, then see the numbers. This creates genuine insight rather than post-hoc rationalisation. It's also a useful data source for detecting patterns like external attribution bias ("always blaming teammates") vs genuine self-awareness.

This is distinct from the mood check-in (which tracks *how you felt*). This tracks *what you believe caused the loss*.

---

## UX Flow

```
[Loss detected on refresh]
     ↓
[Self-analysis card appears on Dashboard]
     ↓
[User picks 1–3 things they think went wrong]
     ↓
[User taps "Show me the data"]
     ↓
[Card expands to show actual match stats vs their baseline]
     ↓
[User can dismiss or tap "Note this" to save the reflection]
```

---

## Step 1 — What Do You Think Went Wrong?

Multi-select (up to 3 choices). Options:

**Performance**
- 💀 Too many deaths
- 🌾 Poor farm / CS
- 🗺️ Poor map awareness / positioning
- ⚔️ Lost the laning phase
- 🎒 Bad item choices
- 🤺 Poor team fight decisions

**Mindset**
- 😤 Tilted from earlier in the session
- 🎯 Lack of focus / on autopilot
- 😰 Played too safe / too passive
- 🔥 Played too aggressively / went for greedy plays

**External**
- 👥 Teammates underperformed
- 🧱 Hard matchup / bad hero pick against them
- 🍀 Unlucky — nothing to improve

Prompt copy: *"Before we show you the stats — what do you think went wrong?"*

---

## Step 2 — The Reveal

After submitting, expand the card to show:

| Stat | This Game | Your Average | Difference |
|------|-----------|--------------|------------|
| Deaths | 8 | 4.2 | 🔴 +3.8 |
| CS @ 10 | 52 | 68 | 🔴 -16 |
| KDA | 1.1 | 3.4 | 🔴 -2.3 |
| GPM | 410 | 520 | 🔴 -110 |

Only show stats that are meaningfully different from the player's baseline (>20% deviation). If nothing stands out, show: *"Your stats weren't far off your average. This one might just have been bad luck or team factors."*

**Calibration messaging:**
- If they picked "Too many deaths" and deaths were indeed elevated → *"You were right — deaths were your weak point this game."*
- If they picked "Teammates" but their own stats were below average → *"Your stats were actually below your average too — worth looking at your own game as well."*
- If they picked "Poor farm" but CS was fine → *"Interestingly, your CS was close to average. The issue might have been elsewhere."*

This feedback should be **non-judgmental** and framed as curiosity, not blame.

---

## Data Model

```sql
CREATE TABLE self_analysis (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  match_id      INTEGER UNIQUE REFERENCES matches(match_id),
  analysed_at   INTEGER NOT NULL,
  selections    TEXT NOT NULL,  -- JSON array of selected option keys
  skipped       INTEGER NOT NULL DEFAULT 0
);
```

---

## Settings

Add to the **Mental Wellbeing** section in Settings, similar to check-in frequency.

**Label:** "Post-loss self-analysis"

**Options (when to ask):**
- Every loss
- After a loss streak (2+ losses)
- After 3+ losses in a row only
- Never

**Default:** After a loss streak (2+ losses) — less intrusive than every loss, but frequent enough to build insight.

**Loss streak override:** Same as mood check-in — always shows after 3+ losses regardless of setting.

The self-analysis prompt and the mood check-in should **not** appear in the same session for the same match — pick one. Priority: self-analysis if it's triggered, mood check-in otherwise (or show both but stacked, not simultaneously).

---

## Backend Changes

- Create `self_analysis` table in `init_db()`
- Add `self_analysis_frequency: String` to `Settings` (default `"loss_streak"`)
- New Tauri commands:
  - `get_pending_self_analysis()` → returns match_id if a qualifying loss needs analysis
  - `save_self_analysis(match_id, selections)` → stores the reflection
  - `dismiss_self_analysis(match_id)` → marks as skipped
  - `get_match_analysis_context(match_id)` → returns this match's stats vs the player's 30-game baseline for the reveal

## Frontend Changes

- New `PostLossAnalysis.svelte` component (two-phase: selection → reveal)
- Shown on Dashboard below the mood check-in card (or instead of it)
- Reveal phase uses the existing match stats structure

## Future Ideas

- Track attribution patterns over time ("you blame teammates 60% of the time — but your stats are below average in 70% of those games")
- Correlate self-analysis accuracy with actual improvement (players who correctly identify their own mistakes tend to improve faster)
- Show a "self-awareness score" over time in the analysis section

## Acceptance Criteria

- [ ] Card appears after qualifying losses (respects frequency setting)
- [ ] Multi-select up to 3 options
- [ ] "Show me the data" button submits selection and reveals stats
- [ ] Reveal shows only meaningfully deviant stats (>20% from baseline)
- [ ] Calibration message shown based on selection vs actual data
- [ ] Skipping records a skip (no selections stored)
- [ ] Self-analysis and mood check-in don't clash for the same match
- [ ] Frequency setting persists across restarts
- [ ] Works on mobile (options wrap cleanly, reveal table readable)
