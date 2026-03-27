# UC-040: View Performance Analysis

## Description
The user views a breakdown of their CS (creep score) performance grouped by hero, with trend information and comparison to personal goals.

## Actor
Authenticated user

## Preconditions
- User is on the Analysis page
- Match data exists in the database

## Steps

```gherkin
Feature: View Performance Analysis

  Scenario: View per-hero CS analysis
    Given the user is on the Analysis page
    And match data exists
    Then a list of heroes is shown sorted by favourites first, then by average CS descending
    And for each hero the following is shown: hero icon, current average CS, game count, horizontal bar chart of value, trend sparkline

  Scenario: View trend card
    Given the user is on the Analysis page
    Then a trend card is shown highlighting the top-performing hero
    And it shows the current average CS and the change percentage from the previous period

  Scenario: No match data available
    Given the user is on the Analysis page
    And no matches are stored
    Then an empty state is displayed
    And a prompt to refresh matches is shown

  Scenario: View insight messages
    Given the user is on the Analysis page
    And the user has a Last Hits goal for a hero
    And the user's current average exceeds that goal's target
    Then an insight message is shown: "Your [Hero] CS avg ([value]) already beats your [target] CS goal. Consider raising the target."
    And if the average is below the goal target, a message reads: "Focus [Hero] farming — you're [diff] CS below your [target] goal on avg."
```

## Notes
- Analysis is based on CS (last hits) at a configurable time marker (10, 15, or 20 minutes)
- The default time marker is 10 minutes
- Trend compares the current sample period to the previous equivalent period
