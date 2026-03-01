# UC-032: View Weekly Challenge on Dashboard

## Description
The user views the current weekly challenge progress summary on the dashboard.

## Actor
Authenticated user

## Preconditions
- User is on the Dashboard
- A weekly challenge has been accepted for the current week

## Steps

```gherkin
Feature: Weekly Challenge on Dashboard

  Scenario: View active weekly challenge
    Given the user has accepted a weekly challenge
    And the user is on the Dashboard
    Then the weekly challenge card is shown
    And it displays the challenge description
    And a progress bar shows current / target progress
    And the number of days remaining in the week is shown

  Scenario: Weekly challenge not yet selected
    Given the user has not yet selected a weekly challenge this week
    When the user is on the Dashboard
    Then the weekly challenge card prompts the user to select a challenge
    And a link navigates to the Challenges page

  Scenario: Navigate to challenges page from dashboard
    Given a weekly challenge is displayed on the Dashboard
    When the user clicks on the weekly challenge card
    Then the user is navigated to the Challenges page
```

## Notes
- The weekly challenge card on the dashboard is a summary only
- Full challenge management (selection, rerolling) is on the Challenges page
