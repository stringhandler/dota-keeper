# UC-031: View Daily Challenge on Dashboard

## Description
The user views today's daily challenge with progress and streak information directly on the dashboard.

## Actor
Authenticated user

## Preconditions
- User is on the Dashboard
- A daily challenge has been generated for today

## Steps

```gherkin
Feature: Daily Challenge on Dashboard

  Scenario: View active daily challenge
    Given the user is on the Dashboard
    Then the daily challenge card is shown
    And it displays the challenge description
    And a progress bar shows current progress toward the target
    And the current value / target value is displayed
    And the streak counter shows consecutive days completed

  Scenario: View completed daily challenge
    Given the user has completed today's daily challenge
    When the user is on the Dashboard
    Then the daily challenge card shows a "Completed" status
    And the progress bar is full

  Scenario: Countdown timer on daily challenge
    Given the user is on the Dashboard
    Then the time until the next daily challenge resets is shown (countdown to midnight)
    And this countdown updates approximately every minute

  Scenario: Navigate to full challenges page
    Given the user is on the Dashboard
    When the user clicks "View Stats" on the daily challenge card
    Then the user is navigated to the Challenges page
```

## Notes
- Daily challenges reset at midnight UTC
- The countdown timer updates every minute
- Streak counts consecutive days where the daily challenge was completed
