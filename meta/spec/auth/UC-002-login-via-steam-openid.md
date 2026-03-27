# UC-002: Login via Steam OpenID

## Description
The user authenticates by redirecting to the official Steam login page (OpenID flow), which returns their Steam ID automatically.

## Actor
Unauthenticated user

## Preconditions
- Application is open
- No Steam ID is saved in settings

## Steps

```gherkin
Feature: Login via Steam OpenID

  Scenario: Successfully authenticate through Steam
    Given the user is on the login screen
    When the user clicks "Sign in through Steam"
    Then the application opens the Steam OpenID authorization URL in a browser
    And the application waits for the Steam callback
    And the user completes authentication on Steam's website
    And the application receives the Steam ID64 from the callback
    And the Steam ID is saved to settings
    And the user is taken to the main dashboard

  Scenario: Steam authentication is cancelled or fails
    Given the user is on the login screen
    When the user clicks "Sign in through Steam"
    And the user cancels the Steam login or an error occurs
    Then the user remains on the login screen
    And an appropriate error or no-change state is shown
```

## Notes
- Steam OpenID flow opens an external browser page
- The app listens on a local callback URL for the redirect from Steam
- The Steam ID64 is extracted from the OpenID response
