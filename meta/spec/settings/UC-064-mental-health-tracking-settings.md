# UC-064: Mental Health Tracking Settings

## Overview
Users can opt in to post-game mood check-ins via a dedicated "Mental Wellbeing" section in Settings. Tracking is **off by default**. Users can also clear all mood data at any time.

---

## Feature: Enable mental health tracking

```gherkin
Scenario: Mental health tracking is off by default
  Given I have freshly installed Dota Keeper
  When I open Settings
  Then the Mental Wellbeing toggle shows "Off"
  And no check-in cards are shown on the Dashboard

Scenario: User enables mental health tracking for the first time
  Given mental health tracking is disabled
  When I click the toggle in Mental Wellbeing settings
  Then tracking is enabled
  And a first-enable explanation modal is shown
  And the modal explains local-only data storage, skippable check-ins, and privacy mode

Scenario: First-enable modal is only shown once
  Given mental health tracking was previously enabled
  When I disable and re-enable tracking
  Then the explanation modal is NOT shown again

Scenario: User disables mental health tracking
  Given mental health tracking is enabled
  When I click the toggle to disable it
  Then the toggle shows "Off"
  And no mood check-in cards appear on the Dashboard
  And existing mood data is retained

Scenario: Tracking preference persists across restarts
  Given I enable mental health tracking and restart the app
  When I open Settings
  Then the toggle still shows "On"
```

## Feature: Clear mood data

```gherkin
Scenario: User clears all mood data
  Given some mood check-ins have been recorded
  When I click "Clear Mood Data" in Settings
  And I confirm the dialog
  Then all rows are deleted from mood_checkins
  And a success message is shown
  And match data and goals are not affected

Scenario: Clearing mood data shows a confirmation dialog
  When I click "Clear Mood Data"
  Then a confirmation dialog is shown before any data is deleted

Scenario: Cancelling the clear dialog preserves data
  When I click "Clear Mood Data" and then cancel the dialog
  Then no data is deleted
```
