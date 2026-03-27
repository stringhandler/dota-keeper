# UC-021: View Goals List

## Description
The user views all their active goals with achievement rates and trend indicators.

## Actor
Authenticated user

## Preconditions
- User is logged in
- User is on the Goals page

## Steps

```gherkin
Feature: View Goals List

  Scenario: View goals when goals exist
    Given the user is on the Goals page
    And the user has created at least one goal
    Then a list of goals is displayed
    And each goal shows: hero icon or scope badge, metric description, game mode, achievement rate percentage, trend indicator, and a progress bar

  Scenario: View goals when no goals exist
    Given the user is on the Goals page
    And no goals have been created
    Then an empty state message is displayed
    And a prompt to create the first goal is shown

  Scenario: Achievement rate colour coding
    Given the user is on the Goals page
    And a goal has a >85% achievement rate
    Then the progress bar is shown in a "Too Easy" colour
    And a goal with 70-80% achievement shows an "Optimal" colour
    And a goal with <50% achievement shows a "Way Too Hard" colour

  Scenario: Trend indicator
    Given the user is on the Goals page
    And a goal has been improving over recent matches
    Then the trend indicator shows "Improving"
    And a goal that has been declining shows "Declining"
    And a stable goal shows "Stable"
```

## Notes
- Achievement rate thresholds: Too Easy >85%, Optimal 70-80%, Below Target 60-70%, Too Hard <60%, Way Too Hard <50%
- Trend is calculated from recent match performance vs earlier matches
