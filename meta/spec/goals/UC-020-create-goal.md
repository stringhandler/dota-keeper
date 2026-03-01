# UC-020: Create a Goal

## Description
The user defines a new performance goal specifying a metric, target value, hero scope, and game mode.

## Actor
Authenticated user

## Preconditions
- User is on the Goals page
- User is logged in

## Steps

```gherkin
Feature: Create a Goal

  Scenario: Create a Last Hits goal for a specific hero
    Given the user is on the Goals page
    When the user selects a hero from the hero picker
    And the user selects "Last Hits" as the metric
    And the user sets the target time to 10 minutes
    And the user enters a target value of 70
    And the user selects "Ranked" as the game mode
    And the user clicks "Save"
    Then a new goal is created and appears in the goals list
    And the goal shows the hero, metric, target value, and game mode

  Scenario: Create a Last Hits goal for any hero in a role
    Given the user is on the Goals page
    When the user selects "Any Carry" as the hero scope
    And the user selects "Last Hits" as the metric
    And the user sets the target time to 10 minutes
    And the user enters a target value of 60
    And the user clicks "Save"
    Then a goal is created that applies to all matches played as Carry (position 1)

  Scenario: Create a Kills goal
    Given the user is on the Goals page
    When the user selects a hero or scope
    And the user selects "Kills" as the metric
    And the user sets the target time and value
    And the user clicks "Save"
    Then a kills goal is created

  Scenario: Create an Item Timing goal
    Given the user is on the Goals page
    When the user selects a hero or scope
    And the user selects "Item Timing" as the metric
    And the user selects an item from the item list
    And the user sets the target purchase time
    And the user clicks "Save"
    Then an item timing goal is created

  Scenario: Goal already exceeded - warning shown
    Given the user is on the Goals page
    And the user has recent matches showing their current average CS is 80 at 10 minutes on a hero
    When the user creates a Last Hits goal for that hero with a target of 70
    Then a warning is displayed indicating the target is already being exceeded
    And the user is prompted to set a higher target

  Scenario: Create a goal with missing required fields
    Given the user is on the Goals page
    When the user submits the goal form without filling all required fields
    Then validation errors are shown
    And the goal is not saved
```

## Notes
- Hero scope options: Specific Hero, Any Carry, Any Core, Any Support, Any Hero
- Metrics: Last Hits, Kills, Denies, Item Timing, Partner Networth
- Networth and Level goals are stored but not currently evaluated
- Game mode options: Ranked, Turbo
