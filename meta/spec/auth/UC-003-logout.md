# UC-003: Logout

## Description
The user logs out, clearing their Steam ID from the application settings.

## Actor
Authenticated user

## Preconditions
- User is logged in (Steam ID is saved in settings)

## Steps

```gherkin
Feature: Logout

  Scenario: User logs out via Settings
    Given the user is on the Settings page
    And the user's Steam ID is displayed
    When the user clicks the "Logout" button
    Then the Steam ID is cleared from settings
    And the user is returned to the login screen

  Scenario: User returns to app after logout
    Given the user has previously logged out
    When the application is opened
    Then the login screen is displayed
    And no Steam ID is pre-filled
```

## Notes
- Logout only clears the Steam ID from settings
- Match data and goals stored in the database are NOT deleted on logout
- The user must re-enter their Steam ID to use the app again
