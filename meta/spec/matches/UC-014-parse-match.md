# UC-014: Parse Match

## Description
The application requests OpenDota to parse a match, providing detailed per-minute data including CS progression, networth, item timings, and lane role detection.

## Actor
System (automatic) / Authenticated user

## Preconditions
- Match is stored in the database with parse_state = Unparsed or Failed
- Internet connection is available

## Steps

```gherkin
Feature: Parse Match

  Scenario: Automatic parsing of a newly fetched match
    Given a match has been fetched from OpenDota with parse_state = Unparsed
    Then the application automatically requests parsing for recent unparsed matches
    And the match parse_state changes to "Parsing"
    And the UI shows a "Parsing..." indicator on the match

  Scenario: Match parsing completes successfully
    Given a match is in the "Parsing" state
    When OpenDota finishes parsing the match
    Then the per-minute CS data is stored
    Then the per-minute networth data for all 10 players is stored
    And item purchase timings are stored
    And lane role is detected and stored
    And the match parse_state changes to "Parsed"
    And the UI updates to show the "Parsed" state

  Scenario: Match parsing fails
    Given a match has been requested for parsing
    When OpenDota returns an error or the match cannot be parsed
    Then the match parse_state changes to "Failed"
    And the UI shows a "Failed" indicator

  Scenario: Automatic retry polling
    Given one or more matches have parse_state = Unparsed
    Then the application polls every 30 seconds for recent unparsed matches
    And automatically requests parsing for each unparsed match found
```

## Notes
- Parsing is automatic and does not require manual user action
- The UI updates in real-time using a match-state-changed event
- Parsed data enables per-minute charts and item timing goals
- The poll interval is 30 seconds
