# UC-024: View Goal Detail

## Description
The user views detailed statistics, a histogram, and recent match history for a single goal.

## Actor
Authenticated user

## Preconditions
- User is on the Goals page
- At least one goal exists with match data

## Steps

```gherkin
Feature: View Goal Detail

  Scenario: View goal detail page
    Given the user is on the Goals page
    When the user clicks on a goal
    Then the goal detail page is displayed
    And the goal description, metric, and game mode are shown
    And the achievement status indicator is shown (Too Easy / Excellent / Good / Low / Critical)
    And the following statistics are shown: total matches, achieved count, failed count, achievement rate, average value, min value, max value

  Scenario: View histogram
    Given the user is on a goal detail page
    And there is sufficient match data
    Then a histogram is displayed showing the distribution of actual values
    And each bar shows a breakdown of pass (green) vs fail (red) matches
    And the target value is marked on the histogram

  Scenario: View last 10 games
    Given the user is on a goal detail page
    Then the last 10 matches for this goal are shown in chronological order
    And each match shows: date, hero played, actual value, pass/fail indicator

  Scenario: View goal suggestion
    Given the user is on a goal detail page
    And the last 10 games average significantly exceeds the current target
    Then a suggestion is shown to raise the goal target
    And a recommended new target value is displayed

  Scenario: Filter by hero on an any-hero goal
    Given the user is on a goal detail page for an "Any Core" goal
    When the user selects a specific hero from the filter
    Then the statistics and histogram update to show only matches played on that hero
```

## Notes
- Achievement status colours match the 75% target design principle
- The suggestion threshold checks if last 10 games average exceeds the current target
