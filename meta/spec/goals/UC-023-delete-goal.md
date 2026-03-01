# UC-023: Delete a Goal

## Description
The user permanently removes a goal from the application.

## Actor
Authenticated user

## Preconditions
- User is on the Goals page
- At least one goal exists

## Steps

```gherkin
Feature: Delete a Goal

  Scenario: Successfully delete a goal
    Given the user is on the Goals page
    When the user clicks the delete button on a goal
    Then a confirmation prompt is shown
    When the user confirms the deletion
    Then the goal is removed from the database
    And the goal no longer appears in the goals list

  Scenario: Cancel deletion
    Given the user is on the Goals page
    When the user clicks the delete button on a goal
    And a confirmation prompt is shown
    When the user cancels the prompt
    Then the goal is not deleted
    And it remains in the goals list
```

## Notes
- Deletion is permanent and cannot be undone
- Deleting a goal does not affect match data
