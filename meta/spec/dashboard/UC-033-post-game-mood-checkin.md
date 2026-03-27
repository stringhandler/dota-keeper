# UC-033: Post-Game Mood Check-In

## Overview
After qualifying games, a dismissible check-in card appears on the Dashboard. Users answer 1–2 quick emoji-scale questions. Data is stored locally in `mood_checkins` and used for future tilt analysis.

---

## Feature: Check-in trigger logic

```gherkin
Scenario: Check-in does not appear when tracking is disabled
  Given mental health tracking is off
  When I open the Dashboard
  Then no check-in card is shown

Scenario: Check-in appears after a 3+ loss streak
  Given mental health tracking is enabled
  And my last 3 matches are all losses
  When I open the Dashboard
  Then a check-in card is shown

Scenario: Check-in appears after 4+ games in a session
  Given mental health tracking is enabled
  And I have played 4 or more matches in the last 6 hours
  When I open the Dashboard
  Then a check-in card is shown

Scenario: Check-in appears for ~25% of games (random trigger)
  Given mental health tracking is enabled
  And the most recent match_id is divisible by 4
  When I open the Dashboard
  Then a check-in card is shown

Scenario: Check-in only appears once per session (non-loss-streak)
  Given a non-loss-streak trigger fires
  When I dismiss the check-in and navigate away and back
  Then no new check-in card is shown (session flag prevents it)

Scenario: Loss streak overrides once-per-session limit
  Given a loss streak occurs after I already dismissed a check-in this session
  When I open the Dashboard
  Then a new check-in card is shown
```

## Feature: Check-in interaction

```gherkin
Scenario: User completes a check-in
  Given a check-in card is shown
  When I select energy score 4
  And I select calm score 3
  And I click Submit
  Then a mood_checkins row is saved with energy=4, calm=3, skipped=0
  And the card is dismissed

Scenario: User skips a check-in
  Given a check-in card is shown
  When I click Skip
  Then a mood_checkins row is saved with skipped=1
  And the card is dismissed
  And no energy or calm data is stored

Scenario: Q3 appears when calm score is 1 or 2
  Given a check-in card is shown
  When I select calm score 1 or 2
  Then a third question "What got under your skin?" appears
  With options: Teammates, The enemy, My own mistakes, Nothing — I was fine actually

Scenario: Q3 attribution is stored with the check-in
  Given calm score 2 is selected and Q3 is shown
  When I select "Teammates" and submit
  Then the mood_checkins row has attribution = "Teammates"

Scenario: Q3 does not appear when calm score is 3 or higher
  Given a check-in card is shown
  When I select calm score 3, 4, or 5
  Then no third question is shown

Scenario: Submit is disabled until both questions are answered
  Given a check-in card is shown
  When only energy has been selected
  Then the Submit button is disabled
```

## Feature: Check-in UI

```gherkin
Scenario: Check-in card matches app design system
  Given a check-in card is shown
  Then it uses the dark gold card style
  And emoji buttons have min 44px touch targets
  And selected scores are highlighted in gold

Scenario: Check-in card is accessible on mobile
  Given I am on a mobile viewport (390px wide)
  When a check-in card is shown
  Then all emoji buttons are tappable without overflow
  And action buttons are full-width row layout
```
