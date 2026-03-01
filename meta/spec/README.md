# Dota Keeper — Use Case Specifications

This directory contains individual use case specifications for all currently implemented features in Dota Keeper. They are written in a Gherkin-compatible format suitable for future conversion into automated Cucumber tests.

## Structure

```
spec/
  auth/
    UC-001  Login with Steam ID
    UC-002  Login via Steam OpenID
    UC-003  Logout
  matches/
    UC-010  View Match List
    UC-011  Refresh Matches from OpenDota
    UC-012  Filter Matches
    UC-013  View Match Detail
    UC-014  Parse Match (automatic)
    UC-015  Backfill Historical Matches
  goals/
    UC-020  Create a Goal
    UC-021  View Goals List
    UC-022  Edit a Goal
    UC-023  Delete a Goal
    UC-024  View Goal Detail (histogram + history)
    UC-025  View and Accept Hero Goal Suggestion
  dashboard/
    UC-030  View Dashboard (quick stats + calendar)
    UC-031  View Daily Challenge on Dashboard
    UC-032  View Weekly Challenge on Dashboard
  analysis/
    UC-040  View Performance Analysis
    UC-041  Filter Performance Analysis
    UC-042  Toggle Favourite Hero
  challenges/
    UC-050  Select Weekly Challenge
    UC-051  Reroll Weekly Challenges
    UC-052  Skip Weekly Challenge
    UC-053  View Weekly Challenge Progress
    UC-054  View Daily Challenge Progress
    UC-055  View Challenge History
  settings/
    UC-060  Manage Database
    UC-061  Configure Goal Suggestion Difficulty
    UC-062  Manage Analytics Consent
    UC-063  Check for Application Updates
```

## File Format

Each file follows this structure:
- **Description** — what the use case does
- **Actor** — who performs the action
- **Preconditions** — what must be true before the use case begins
- **Steps** — Gherkin `Feature` / `Scenario` / `Given`/`When`/`Then` blocks
- **Notes** — implementation context and edge cases

## Numbering Convention

- UC-00x: Authentication
- UC-01x: Matches
- UC-02x: Goals
- UC-03x: Dashboard
- UC-04x: Analysis
- UC-05x: Challenges
- UC-06x: Settings
