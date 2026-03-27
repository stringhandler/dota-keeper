# UC-063: Check for Application Updates

## Description
The user manually checks whether a newer version of Dota Keeper is available and installs it if so.

## Actor
Authenticated user

## Preconditions
- User is on the Settings page
- Internet connection is available

## Steps

```gherkin
Feature: Check for Application Updates

  Scenario: Check for updates and none available
    Given the user is on the Settings page
    And the current version is shown
    When the user clicks "Check for Updates"
    Then the application queries the update server
    And a message is shown indicating the app is up to date

  Scenario: Update is available
    Given the user is on the Settings page
    When the user clicks "Check for Updates"
    And a newer version is available
    Then the available version number is displayed
    And a "Download and Install" button is shown

  Scenario: Download and install update
    Given an update is available
    When the user clicks "Download and Install"
    Then the update is downloaded
    And the application restarts to apply the update

  Scenario: Update check fails
    Given the user is on the Settings page
    When the user clicks "Check for Updates"
    And the update server is unreachable
    Then an error message is displayed
    And the current version is still shown
```

## Notes
- Updates are managed via Tauri's built-in updater
- The current app version is always shown in Settings, regardless of update status
