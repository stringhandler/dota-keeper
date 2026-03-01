# UC-022: Edit a Goal

## Description
The user modifies an existing goal's settings, such as changing the target value or game mode.

## Actor
Authenticated user

## Preconditions
- User is on the Goals page
- At least one goal exists

## Steps

```gherkin
Feature: Edit a Goal

  Scenario: Edit goal target value
    Given the user is on the Goals page
    When the user clicks the edit button on a goal
    Then the goal edit form opens pre-filled with the current goal settings
    When the user changes the target value
    And the user clicks "Save"
    Then the goal is updated with the new target value
    And the goals list reflects the change

  Scenario: Edit goal metric
    Given the user is editing a goal
    When the user changes the metric type
    Then the form updates to show the relevant fields for the new metric
    And the user can save the updated goal

  Scenario: Cancel editing
    Given the user has opened the edit form for a goal
    When the user closes or cancels the form without saving
    Then the goal remains unchanged
```

## Notes
- The edit form uses the same fields as the create form
- Changing a goal's hero or metric may affect historical achievement calculations
