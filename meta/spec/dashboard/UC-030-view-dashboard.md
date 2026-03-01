# UC-030: View Dashboard

## Description
The user views the main dashboard, which provides an at-a-glance overview of recent performance, goal progress, and active challenges.

## Actor
Authenticated user

## Preconditions
- User is logged in
- User is on the Dashboard (home page)

## Steps

```gherkin
Feature: View Dashboard

  Scenario: View quick stats strip
    Given the user is on the Dashboard
    And match data exists for the past 7 days
    Then four stat cards are shown:
      | Stat              | Content                                     |
      | Win Rate (7 days) | Win percentage with total game count        |
      | Avg CS @ 10min    | Average last hits at 10 minutes with sample |
      | Goals Hit (7 days)| Percentage of goals achieved this week      |
      | Active Goals      | Count of currently active goals             |

  Scenario: View dashboard with no data
    Given the user is on the Dashboard
    And no matches have been fetched yet
    Then the stat cards show zero or empty states
    And a prompt to refresh matches is displayed

  Scenario: View 7-day goal progress calendar
    Given the user is on the Dashboard
    And goals exist with match data in the past 7 days
    Then each goal is shown with a row of 7 daily dots
    And each dot is filled (success) or empty (miss or no data)
    And the goal's overall 7-day achievement rate is shown

  Scenario: Navigate to a goal from the dashboard
    Given the user is on the Dashboard
    And a goal is shown in the calendar
    When the user clicks on the goal row
    Then the user is navigated to the goal detail page
```

## Notes
- The dashboard is the home/index page of the app
- Stats are calculated over the last 7 days of matches
- The CS @ 10 min stat uses a 30-game sample
