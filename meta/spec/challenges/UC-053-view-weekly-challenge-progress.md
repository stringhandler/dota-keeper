# UC-053: View Weekly Challenge Progress

## Description
The user views the progress of their accepted weekly challenge, including current value vs target and days remaining.

## Actor
Authenticated user

## Preconditions
- User is on the Challenges page
- A weekly challenge has been accepted for the current week

## Steps

```gherkin
Feature: View Weekly Challenge Progress

  Scenario: View in-progress weekly challenge
    Given the user is on the Challenges page
    And the user has an active weekly challenge
    Then the challenge description is shown
    And a difficulty badge is displayed (Easy/Medium/Hard)
    And a progress bar shows the proportion of the target achieved
    And the current value and target value are shown
    And the number of days remaining in the week is displayed
    And the status shows "In Progress"

  Scenario: View completed weekly challenge
    Given the user's active weekly challenge target has been reached
    When the user visits the Challenges page
    Then the status shows "Completed!"
    And the progress bar is full

  Scenario: Weekly challenge with hero context
    Given the user's weekly challenge involves a specific hero
    Then the hero icon is displayed alongside the challenge description
```

## Notes
- Progress is calculated from the user's match data automatically
- Once the target is met the challenge is marked Completed regardless of days remaining
