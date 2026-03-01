# UC-013: View Match Detail

## Description
The user views detailed statistics for a single match, including goal evaluations and parsed data if available.

## Actor
Authenticated user

## Preconditions
- User is on the Matches page
- At least one match is stored

## Steps

```gherkin
Feature: View Match Detail

  Scenario: View unparsed match detail
    Given the user clicks on a match that has not been parsed
    Then the match detail page is displayed
    And the following stats are shown: hero, game mode, duration, date, KDA, GPM, XPM, networth, hero damage, tower damage, healing
    And the win/loss result is shown
    And the goal evaluations section shows which goals applied and whether they were achieved
    And a "Not yet parsed" state is shown for per-minute data

  Scenario: View parsed match detail
    Given the user clicks on a match that has been parsed
    Then the match detail page is displayed
    And a per-minute CS progression chart is shown
    And per-minute last hits and denies data is available

  Scenario: View goal evaluations on match detail
    Given the user is on a match detail page
    And the user has goals that apply to this match
    Then each applicable goal is listed
    And each goal shows the actual value achieved and the target value
    And each goal shows a pass (tick) or fail (cross) indicator

  Scenario: Open match on OpenDota
    Given the user is on a match detail page
    When the user clicks the "View on OpenDota" button
    Then the match page opens in the default browser at the OpenDota URL
```

## Notes
- Goal evaluations are based on the match's hero, game mode, and lane role
- Per-minute charts require the match to be parsed
- Parse state is one of: Unparsed, Parsing, Parsed, Failed
