# UC-025: View and Accept Hero Goal Suggestion

## Description
The application suggests a hero-specific CS goal based on the user's recent performance. The user can accept it to create a goal automatically.

## Actor
Authenticated user

## Preconditions
- User is on the main Dashboard
- User has sufficient match history for at least one hero

## Steps

```gherkin
Feature: Hero Goal Suggestion

  Scenario: View a hero goal suggestion
    Given the user is on the Dashboard
    And enough match data exists to generate a suggestion
    Then a goal suggestion card is displayed
    And the card shows: the suggested hero, the user's current average CS at 10 minutes, the suggested target value, and the number of games analysed

  Scenario: Accept a hero goal suggestion
    Given a hero goal suggestion is displayed on the Dashboard
    When the user clicks "Accept" on the suggestion
    Then a new Last Hits goal is created for the suggested hero with the suggested target
    And the suggestion card is replaced or updated

  Scenario: Refresh the suggestion
    Given a hero goal suggestion is displayed on the Dashboard
    When the user clicks "Refresh" or "Get new suggestion"
    Then a new suggestion is generated (ignoring the cached suggestion)
    And the new suggestion is displayed

  Scenario: No suggestion available
    Given the user is on the Dashboard
    And there is insufficient match data to generate a suggestion
    Then the suggestion card either shows an empty state or is hidden
```

## Notes
- Suggestions are cached per week — the same suggestion shows until refreshed or a week passes
- The suggested target is calibrated so the user would hit it approximately 75% of the time
- The difficulty of suggestions can be configured in Settings (Easy / Medium / Hard / Custom)
