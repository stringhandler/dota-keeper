# UC-050: Select a Weekly Challenge

## Description
At the start of each week, the user is presented with three challenge options and selects one to pursue for the week.

## Actor
Authenticated user

## Preconditions
- User is on the Challenges page
- No weekly challenge has been accepted for the current week

## Steps

```gherkin
Feature: Select Weekly Challenge

  Scenario: View weekly challenge options
    Given the user is on the Challenges page
    And no weekly challenge has been selected for this week
    Then three challenge options are displayed
    And each option shows: difficulty badge (Easy/Medium/Hard), challenge description, and metric

  Scenario: Accept a weekly challenge
    Given three challenge options are displayed
    When the user clicks "Accept" on one of the options
    Then that challenge is set as the active weekly challenge
    And the challenge selection view is replaced by the active challenge progress view

  Scenario: Weekly challenge already accepted
    Given the user has already accepted a weekly challenge this week
    When the user visits the Challenges page
    Then the challenge selection screen is not shown
    And instead the active challenge progress view is shown
```

## Notes
- Only one challenge can be active per week
- Weekly challenges reset at the start of each week (Monday UTC)
