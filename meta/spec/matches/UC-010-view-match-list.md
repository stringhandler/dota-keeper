# UC-010: View Match List

## Description
The user views a paginated list of all stored Dota 2 matches, showing key stats for each match.

## Actor
Authenticated user

## Preconditions
- User is logged in
- At least one match has been fetched and stored

## Steps

```gherkin
Feature: View Match List

  Scenario: View matches with data
    Given the user is on the Matches page
    And matches are stored in the database
    Then a list of matches is displayed ordered by date (newest first)
    And each match shows: date, hero icon, game mode badge, win/loss, duration, KDA, GPM, and CS

  Scenario: View matches with no data
    Given the user is on the Matches page
    And no matches are stored
    Then an empty state message is displayed
    And a prompt to refresh matches is shown

  Scenario: Navigate to next page of matches
    Given the user is on the Matches page
    And more than 10 matches are stored
    When the user clicks to go to the next page
    Then the next 10 matches are displayed

  Scenario: Click a match to view its detail
    Given the user is on the Matches page
    When the user clicks on a match row
    Then the user is navigated to the match detail page for that match
```

## Notes
- Matches are paginated at 10 per page
- Game mode is shown as a badge (Ranked / Turbo / Other)
- Win/loss is shown as a coloured W or L indicator
