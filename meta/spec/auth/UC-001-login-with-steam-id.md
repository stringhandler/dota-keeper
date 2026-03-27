# UC-001: Login with Steam ID

## Description
The user enters their Steam ID (or Steam profile URL) to authenticate and begin tracking their Dota 2 matches.

## Actor
Unauthenticated user

## Preconditions
- Application is open
- No Steam ID is saved in settings

## Steps

```gherkin
Feature: Login with Steam ID

  Scenario: Successfully login with a numeric Steam ID64
    Given the user is on the login screen
    When the user enters a valid Steam ID64 into the Steam ID field
    And the user submits the form
    Then the Steam ID is saved
    And the user is taken to the main dashboard

  Scenario: Successfully login by entering a Steam profile URL
    Given the user is on the login screen
    When the user enters a Steam community profile URL (e.g. https://steamcommunity.com/id/username)
    And the user submits the form
    Then the Steam ID64 is extracted from the URL
    And the user is taken to the main dashboard

  Scenario: Login fails with an invalid Steam ID
    Given the user is on the login screen
    When the user enters an invalid value (e.g. "not-a-steam-id")
    And the user submits the form
    Then an error message is displayed
    And the user remains on the login screen
```

## Notes
- Steam IDs are stored in local settings (not the database)
- The app accepts both numeric Steam ID64 and community profile URLs
- Validation happens before saving
