# UC-051: Reroll Weekly Challenges

## Description
The user discards the current three challenge options and generates a new set. The user is limited to 2 rerolls per week.

## Actor
Authenticated user

## Preconditions
- User is on the Challenges page
- Challenge options are displayed (no challenge accepted yet)
- At least 1 reroll remaining this week

## Steps

```gherkin
Feature: Reroll Weekly Challenges

  Scenario: Reroll challenges with rerolls remaining
    Given three weekly challenge options are shown
    And the user has rerolls remaining (shown as a counter)
    When the user clicks the "Reroll" button
    Then three new challenge options are generated and displayed
    And the reroll counter decrements by 1

  Scenario: No rerolls remaining
    Given three weekly challenge options are shown
    And the user has used all 2 rerolls for the week
    Then the "Reroll" button is disabled or hidden
    And the user must choose from the current options or skip

  Scenario: Reroll counter display
    Given the user is viewing challenge options
    Then the number of remaining rerolls is displayed (e.g., "2 rerolls left")
```

## Notes
- Maximum of 2 rerolls per week
- The reroll count resets with the weekly challenge reset
