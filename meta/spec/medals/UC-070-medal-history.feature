Feature: Medal History
  As a Dota 2 player
  I want to see my rank medal history
  So that I can track my MMR progression and celebrate my peak

  Background:
    Given the user is logged in with a Steam ID
    And at least one synced match includes rank_tier data from OpenDota

  # UC-070: View medal history page
  Scenario: User navigates to the Medals page
    Given the user has matches with rank_tier data stored
    When the user navigates to "/medals"
    Then they see their current medal displayed prominently
    And they see their all-time peak medal with a "Peak" badge
    And they see a count of tracked games with rank data

  # UC-071: Medal history timeline
  Scenario: Rank change timeline is shown
    Given the user has matches where rank_tier changed over time
    When the user views the "/medals" page
    Then a timeline of rank changes is shown, most recent first
    And each timeline entry shows the medal name, stars, and date
    And the match that triggered the change is linked

  # UC-072: No duplicate entries in timeline
  Scenario: Consecutive matches with the same rank are collapsed
    Given the user has 10 consecutive matches all at "Archon 3"
    When the user views the medal history timeline
    Then only one entry for "Archon 3" is shown (not 10)

  # UC-073: Peak medal highlighted
  Scenario: Peak medal is clearly identified
    Given the user's highest ever rank was "Legend 4"
    And their current rank is "Archon 5"
    When the user views the "/medals" page
    Then "Legend 4" is shown in the peak medal card
    And "Archon 5" is shown in the current medal card
    And the peak medal entry in the timeline has a "Peak" chip

  # UC-074: Empty state when no rank data
  Scenario: No rank data available
    Given the user has matches but none have rank_tier data
    When the user views the "/medals" page
    Then an empty state is shown explaining that rank is captured from OpenDota
    And no timeline or stats cards are shown

  # UC-075: Sidebar rank pill
  Scenario: Current rank displayed in sidebar
    Given the user's most recent match has rank_tier = 43 (Archon 3)
    When the user is on any page
    Then the sidebar rank pill shows "Archon 3"
    And clicking the rank pill navigates to "/medals"

  # UC-076: rank_tier captured on match sync
  Scenario: rank_tier is stored when syncing new matches
    Given OpenDota returns rank_tier = 55 for a recent match
    When the user syncs their matches
    Then the match is stored with rank_tier = 55
    And the medal history reflects "Legend 5" for that match
