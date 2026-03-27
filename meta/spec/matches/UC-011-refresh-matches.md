# UC-011: Refresh Matches from OpenDota

## Description
The user manually triggers a fetch of their latest matches from the OpenDota API.

## Actor
Authenticated user

## Preconditions
- User is logged in with a valid Steam ID
- Internet connection is available

## Steps

```gherkin
Feature: Refresh Matches

  Scenario: Successfully fetch new matches
    Given the user is on the Matches page
    When the user clicks the "Refresh" button
    Then the app calls the OpenDota API for the latest 10 matches
    And new matches not already in the database are saved
    And the match list is updated to show the latest matches

  Scenario: No new matches since last refresh
    Given the user is on the Matches page
    When the user clicks the "Refresh" button
    And OpenDota returns matches that are already in the database
    Then the match list remains unchanged
    And no duplicate matches are added

  Scenario: OpenDota API is unavailable
    Given the user is on the Matches page
    When the user clicks the "Refresh" button
    And the OpenDota API returns an error
    Then an error message is displayed
    And the existing match list is unchanged
```

## Notes
- Only the latest 10 matches are fetched per refresh
- To fetch older matches, use the "Backfill Historical Matches" setting
- Newly fetched matches initially have parse_state = Unparsed
