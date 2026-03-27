# UC-060: Manage Database

## Description
The user manages the local SQLite database, including viewing its location, opening the folder, and clearing all match data.

## Actor
Authenticated user

## Preconditions
- User is on the Settings page

## Steps

```gherkin
Feature: Manage Database

  Scenario: View database location
    Given the user is on the Settings page
    Then the full path to the database file is displayed
    And the path points to the user's local AppData/DotaKeeper directory

  Scenario: Open database folder in file explorer
    Given the user is on the Settings page
    When the user clicks "Open Database Folder"
    Then the file explorer opens to the folder containing the database file

  Scenario: Clear all matches
    Given the user is on the Settings page
    When the user clicks "Clear All Matches"
    Then a confirmation prompt is shown
    When the user confirms
    Then all match data is deleted from the database
    And the match list shows an empty state

  Scenario: Cancel clear matches
    Given the user is on the Settings page
    When the user clicks "Clear All Matches"
    And a confirmation prompt is shown
    When the user cancels
    Then no data is deleted

  Scenario: Reparse pending matches
    Given the user is on the Settings page
    When the user clicks "Reparse Pending Matches"
    Then all matches with parse_state = Unparsed or Failed are requeued for parsing
    And the parsing process begins for those matches
```

## Notes
- The database is located at %LOCALAPPDATA%/DotaKeeper/ on Windows
- Clearing matches does NOT delete goals or settings
- Reparse is useful when a previous parse failed due to a temporary API error
