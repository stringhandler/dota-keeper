# Test Guide: Mental Health Settings + Post-Game Check-in

## Prerequisites
- Build and run the app (`cargo tauri dev`)
- Have at least a few matches synced

---

## 1. Settings — Mental Wellbeing section

1. Open **Settings** → scroll down to **Mental Wellbeing**
2. Verify toggle defaults to **"Off"**
3. Click the toggle to enable
   - Expected: toggle shows **"On"**, first-enable modal appears
4. Read the modal (should mention local-only data, skippable check-ins)
5. Click **"Got it, let's go"**
   - Expected: modal closes
6. Disable and re-enable tracking again
   - Expected: modal does NOT appear a second time
7. Disable tracking → navigate to Dashboard
   - Expected: no check-in card shown

## 2. Clear mood data

1. Enable mental health tracking
2. Trigger and submit at least one check-in (see section 3)
3. Go to Settings → **Clear Mood Data**
   - Expected: confirmation dialog appears
4. Cancel → verify nothing deleted (re-check later data is still there)
5. Click again → confirm → verify success message
   - Expected: "All mood check-in data has been cleared."

## 3. Post-game mood check-in card on Dashboard

### Trigger via loss streak
1. Ensure you have 3+ consecutive losses in match history
2. Enable mental health tracking
3. Delete any existing `mood_checkins` rows (or use "Clear Mood Data") so the latest match is unchecked
4. Open the Dashboard
   - Expected: check-in card appears with "How was that session?"

### Fill in the check-in
5. Click emoji 1 on "How energised" → click emoji 2 on "How calm"
   - Expected: Q3 appears ("What got under your skin?")
6. Select an attribution option
7. Click **Submit**
   - Expected: card disappears, data saved

### Skip flow
8. Clear mood data, reload Dashboard (ensure trigger conditions still met)
9. Click **Skip**
   - Expected: card disappears immediately, no energy/calm data stored (skipped=1)

### Once-per-session rule
10. After dismissing/submitting, navigate away and back (but don't fully restart app)
    - Expected: check-in card does NOT reappear (unless loss streak fires again)

## 4. Q3 conditional appearance
- Select calm score 3, 4, or 5 → Q3 should NOT appear
- Select calm score 1 or 2 → Q3 SHOULD appear

## 5. Submit button disabled state
- Open check-in card
- Verify Submit is greyed out until both energy AND calm are selected
