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
    Then the per-minute gold (gold_t) data for all 10 players is stored
    And item purchase timings are stored
    And lane role is detected and stored
    And the match parse_state changes to "Parsed"
    And the UI updates to show goal result chips for the match

  Scenario: Match parsing fails with pending status
    Given a match has been requested for parsing
    When OpenDota has not yet finished processing the match (lh_t/dn_t not available)
    Then the match parse_state changes to "Failed"
    And the Goals cell shows a grey clock icon (🕐)
    And hovering the icon shows "not yet parsed / will be picked up on next sync"

  Scenario: Match parsing fails with an unexpected error
    Given a match has been requested for parsing
    When OpenDota returns a non-recoverable error (network failure, unexpected response)
    Then the match parse_state changes to "Failed"
    And the Goals cell shows a red warning icon (⚠)
    And hovering the icon shows the specific error message

  Scenario: Failed match shows Retry button
    Given a match has parse_state = Failed
    When the user views the match list
    Then the Goals cell shows a "↺ Retry" button
    And clicking it re-attempts the full parse flow

  Scenario: Match visible as parsed on OpenDota but Unparsed locally
    Given a match exists in the local database with parse_state = Unparsed
    And the match IS fully parsed on OpenDota (lh_t, dn_t and gold_t data available)
    When the user clicks "Parse" or auto-parse runs
    Then the parse succeeds and the match transitions to parse_state = Parsed
    And goal chips are shown in the Goals cell

  Scenario: Automatic retry polling
    Given one or more matches have parse_state = Unparsed or Failed
    Then the application polls every 30 seconds for recent unparsed/failed matches
    And automatically requests parsing for each one found
```

## Notes
- Parsing is automatic and does not require manual user action
- The UI updates in real-time using a match-state-changed event
- Parsed data enables per-minute charts and item timing goals
- The poll interval is 30 seconds
- Per-match parse errors are shown inline (icon + tooltip) — no global error banner
- Per-minute gold data is stored from the OpenDota `gold_t` field (not `net_worth`, which is a scalar)
- A grey clock indicates a transient/pending state; a red warning indicates an actual error
