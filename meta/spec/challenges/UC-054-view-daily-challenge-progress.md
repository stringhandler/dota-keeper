# UC-054: View Daily Challenge Progress

## Description
The user views today's daily challenge and their progress toward completing it, along with their daily streak count.

## Actor
Authenticated user

## Preconditions
- User is on the Challenges page or Dashboard
- A daily challenge exists for today

## Steps

```gherkin
Feature: View Daily Challenge Progress

  Scenario: View today's daily challenge
    Given the user is on the Challenges page
    Then today's daily challenge is shown
    And the challenge description is displayed (with hero icon if applicable)
    And a progress bar shows current progress toward the target
    And the current value / target value is shown

  Scenario: View completed daily challenge
    Given the user has completed today's daily challenge
    Then a "Completed" status is shown
    And the progress bar is full

  Scenario: View daily streak
    Given the user has completed daily challenges on consecutive days
    Then the current streak count is displayed (e.g., "3 day streak")
    And breaking the streak (missing a day) resets the count to 0

  Scenario: Daily challenge resets at midnight
    Given the user has completed today's daily challenge
    When midnight UTC passes
    Then a new daily challenge is shown for the new day
    And the streak count increments
```

## Notes
- Daily challenges are unique per UTC date
- A new challenge is generated automatically for each day
- Streaks track consecutive day completions
