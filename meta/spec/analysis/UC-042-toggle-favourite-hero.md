# UC-042: Toggle Favourite Hero

## Description
The user marks or unmarks a hero as a favourite. Favourited heroes are sorted to the top in analysis and goal views.

## Actor
Authenticated user

## Preconditions
- User is on the Analysis page or Goals page
- At least one hero has match data

## Steps

```gherkin
Feature: Toggle Favourite Hero

  Scenario: Mark a hero as a favourite
    Given the user is on the Analysis page
    When the user clicks the star icon next to a hero
    Then the hero is marked as a favourite
    And the star icon becomes filled/highlighted
    And the hero moves to the top of the hero list on the Analysis page

  Scenario: Unmark a hero as a favourite
    Given a hero is already marked as a favourite
    When the user clicks the star icon next to that hero
    Then the favourite is removed
    And the star icon returns to the unfilled state
    And the hero returns to its position in the sorted list

  Scenario: Favourite heroes in goal views
    Given a hero is marked as a favourite
    When the user is on the Goals page or creating a goal
    Then the favourited hero is prioritised at the top of hero selection lists
```

## Notes
- Favourites are stored in the database and persist across sessions
- Favourites affect ordering in Analysis and Goals views
