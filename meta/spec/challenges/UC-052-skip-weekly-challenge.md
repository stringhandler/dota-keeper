# UC-052: Skip Weekly Challenge

## Description
The user opts out of the weekly challenge for the current week. They cannot accept a challenge after skipping until next week.

## Actor
Authenticated user

## Preconditions
- User is on the Challenges page
- No weekly challenge has been accepted yet this week

## Steps

```gherkin
Feature: Skip Weekly Challenge

  Scenario: Skip the weekly challenge
    Given three challenge options are displayed
    When the user clicks the "Skip" button
    Then the challenge options are hidden
    And a "Skipped" state is shown for the current week
    And the user cannot select a challenge until the following week

  Scenario: Skipped week does not affect history
    Given the user has skipped a week
    When the user views the challenge history
    Then the skipped week does not appear as a completed challenge
```

## Notes
- Skipping is irreversible for that week
- After skipping, the user must wait until the next week to engage with weekly challenges again
