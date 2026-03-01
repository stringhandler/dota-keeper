# UC-012: Filter Matches

## Description
The user applies filters to the match list to narrow down results by game mode or hero.

## Actor
Authenticated user

## Preconditions
- User is on the Matches page
- Matches are stored in the database

## Steps

```gherkin
Feature: Filter Matches

  Scenario: Filter by game mode
    Given the user is on the Matches page
    When the user selects "Ranked" from the game mode filter
    Then only Ranked matches are displayed in the list

  Scenario: Filter by hero
    Given the user is on the Matches page
    When the user selects a hero from the hero filter
    Then only matches played on that hero are displayed

  Scenario: Clear filters
    Given the user is on the Matches page
    And a filter is active
    When the user clears the filter (selects "All")
    Then all matches are displayed again
```

## Notes
- Filters are applied client-side to stored matches
- Filters do not trigger a new API call to OpenDota
