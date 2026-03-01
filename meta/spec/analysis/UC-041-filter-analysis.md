# UC-041: Filter Performance Analysis

## Description
The user adjusts filters on the Analysis page to change the time marker, sample window, game mode, or hero shown in the analysis.

## Actor
Authenticated user

## Preconditions
- User is on the Analysis page
- Match data exists

## Steps

```gherkin
Feature: Filter Performance Analysis

  Scenario: Change the time marker
    Given the user is on the Analysis page
    When the user selects "15 minutes" as the time marker
    Then the analysis recalculates average CS at 15 minutes for all heroes
    And the hero statistics update accordingly

  Scenario: Change the sample window size
    Given the user is on the Analysis page
    When the user changes the sample window to "50 games"
    Then the analysis uses the last 50 matches per hero for calculations
    And the game counts and averages update

  Scenario: Filter by game mode
    Given the user is on the Analysis page
    When the user selects "Ranked" as the game mode filter
    Then only Ranked matches are used in the analysis calculations

  Scenario: Filter by specific hero
    Given the user is on the Analysis page
    When the user selects a specific hero from the hero filter
    Then only matches played on that hero are shown
    And the trend card updates to show only that hero's data

  Scenario: Clear all filters
    Given filters are active on the Analysis page
    When the user resets the filters to defaults
    Then all heroes are shown using the default time marker and sample size
```

## Notes
- Time marker options: 10 minutes, 15 minutes, 20 minutes
- Sample window options: 20 games, 30 games, 50 games
- Game mode options: All, Ranked, Turbo, All Pick
- Filters are applied server-side via the get_last_hits_analysis_data command
