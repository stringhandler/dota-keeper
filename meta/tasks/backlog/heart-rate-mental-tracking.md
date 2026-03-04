# Heart Rate — Optional Mental Health Check-in Field

## Overview

Allow users to optionally enter their heart rate (BPM) as part of the post-game mood check-in. This gives a more objective physiological signal alongside the subjective energy/calm ratings, and can be correlated with tilt patterns over time.

## Use Case

After a game, the existing check-in modal asks for energy (1–5) and calm (1–5). Add an optional "Heart rate" field:

- **Input**: BPM number field (e.g. 72)
- **Label**: "Heart rate (optional)" with a small note: "e.g. from a smartwatch or fitness tracker"
- **Validation**: Accept 30–220 BPM. Outside range shows inline warning but still allows save.
- **Skip**: Leaving it blank is fine — the field is never required.

## Where to Show It

- In the post-game check-in prompt (wherever `PendingCheckin` / `save_mood_checkin` is invoked in the UI)
- In the mental health history view, show BPM alongside energy/calm if recorded
- In the tilt assessment, flag sessions where BPM was notably high (e.g. > 100) as potential stress indicators

## Backend Changes

1. Add `heart_rate_bpm: Option<i32>` column to the `mood_checkins` table (migration: `ALTER TABLE mood_checkins ADD COLUMN heart_rate_bpm INTEGER`).
2. Update `save_mood_checkin` Tauri command signature to accept `heart_rate_bpm: Option<i32>`.
3. Update `get_checkin_history` response struct to include `heart_rate_bpm`.
4. Optionally expose average BPM in the `TiltAssessment` struct.

## Frontend Changes

- Add optional BPM number input to the mood check-in UI (after energy/calm sliders).
- Display BPM in check-in history rows (only if recorded).
- In the tilt chart/cards, optionally annotate high-BPM sessions.

## Acceptance Criteria

- [ ] Heart rate field shown in check-in UI, clearly marked optional
- [ ] BPM saved to DB when provided, null when skipped
- [ ] BPM displayed in check-in history if recorded
- [ ] Check-ins without BPM remain fully functional (no regressions)
- [ ] Input validation: warn outside 30–220 range but don't block save
