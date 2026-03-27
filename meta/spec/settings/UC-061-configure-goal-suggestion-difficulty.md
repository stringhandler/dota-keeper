# UC-061: Configure Goal Suggestion Difficulty

## Description
The user configures how ambitious the automatically suggested goals should be, choosing from preset difficulty levels or a custom improvement percentage.

## Actor
Authenticated user

## Preconditions
- User is on the Settings page

## Steps

```gherkin
Feature: Configure Goal Suggestion Difficulty

  Scenario: Select a preset difficulty
    Given the user is on the Settings page
    When the user selects "Easy" from the suggestion difficulty options
    Then future hero goal suggestions will target a 3-5% improvement over the current average
    And the setting is saved

  Scenario: Select Medium difficulty (default)
    Given the user is on the Settings page
    When the user selects "Medium"
    Then future suggestions target a 5-10% improvement

  Scenario: Select Hard difficulty
    Given the user is on the Settings page
    When the user selects "Hard"
    Then future suggestions target a 10-15% improvement

  Scenario: Enter a custom difficulty percentage
    Given the user is on the Settings page
    When the user selects "Custom"
    Then an input field appears for entering a percentage
    When the user enters a value (e.g., 8%)
    And saves the setting
    Then future suggestions target exactly that percentage improvement
```

## Notes
- The difficulty setting affects the hero goal suggestion on the Dashboard
- Default is Medium (5-10%)
- The 75% target achievement rate principle applies — difficulty changes how the target is calculated
