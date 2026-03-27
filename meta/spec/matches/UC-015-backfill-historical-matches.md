# UC-015: Backfill Historical Matches

## Description
The user fetches up to 100 older matches from OpenDota, extending the match history beyond what the standard refresh provides.

## Actor
Authenticated user

## Preconditions
- User is on the Settings page
- User is logged in with a valid Steam ID
- At least one match already exists in the database

## Steps

```gherkin
Feature: Backfill Historical Matches

  Scenario: Successfully backfill historical matches
    Given the user is on the Settings page
    When the user clicks "Backfill Historical Matches"
    Then the app fetches up to 100 matches from OpenDota that predate the oldest stored match
    And these matches are saved to the database
    And the user is shown a success confirmation

  Scenario: No older matches available
    Given the user is on the Settings page
    When the user clicks "Backfill Historical Matches"
    And OpenDota returns no matches older than what is already stored
    Then the user is informed that no new matches were found

  Scenario: API error during backfill
    Given the user is on the Settings page
    When the user clicks "Backfill Historical Matches"
    And the OpenDota API returns an error
    Then an error message is displayed
    And no changes are made to the database
```

## Notes
- Backfill fetches matches that come before the oldest match currently in the database
- The batch size is 100 matches
- After backfilling, matches will be in Unparsed state and will be parsed automatically
