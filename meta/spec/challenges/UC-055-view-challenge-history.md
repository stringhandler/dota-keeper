# UC-055: View Challenge History

## Description
The user reviews all previously completed daily and weekly challenges.

## Actor
Authenticated user

## Preconditions
- User navigates to the Challenge History page
- At least one challenge has been completed previously

## Steps

```gherkin
Feature: View Challenge History

  Scenario: View completed challenges
    Given the user is on the Challenge History page
    And challenges have been completed in the past
    Then a list of completed challenges is shown
    And each entry shows: completion date, challenge type (Daily/Weekly), description, and target achieved

  Scenario: No history yet
    Given the user is on the Challenge History page
    And no challenges have been completed
    Then an empty state is shown indicating no history yet

  Scenario: History includes both daily and weekly challenges
    Given the user has completed both daily and weekly challenges
    When viewing challenge history
    Then both types appear in the list
    And they are distinguishable by type label
```

## Notes
- Only completed challenges appear in history; in-progress or skipped entries do not
- The history page is accessible from the Challenges page
