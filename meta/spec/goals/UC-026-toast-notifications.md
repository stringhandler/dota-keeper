# UC-026 — Toast Notifications

## Overview
A lightweight snackbar/toast system provides instant feedback after user actions. Toasts appear at the bottom of the screen, auto-dismiss after 3 seconds, and are positioned above the BottomNav on mobile.

---

## Feature: Goal actions

### Scenario: Toast shown after creating a goal
```gherkin
Given I am on the Goals page
When I successfully submit the "Create Goal" form
Then a green "Goal created" toast appears at the bottom of the screen
And the toast disappears after 3 seconds
```

### Scenario: Toast shown after updating a goal
```gherkin
Given I am editing an existing goal
When I submit the updated form
Then a green "Goal updated" toast appears
```

### Scenario: Toast shown after deleting a goal
```gherkin
Given I confirm deletion of a goal
When the goal is removed
Then a green "Goal deleted" toast appears
```

### Scenario: Error toast shown when goal save fails
```gherkin
Given a goal save operation fails
When the error occurs
Then a red error toast appears with the error message
```

---

## Feature: Goal delete confirmation (inline)

### Scenario: Inline confirmation replaces native confirm dialog
```gherkin
Given I am on the Goals page
When I click "Delete" on a goal
Then the row shows "Delete? Yes / No" buttons
And no browser confirm() dialog appears
```

### Scenario: Cancelling inline delete restores the row
```gherkin
Given the goal row is showing "Delete? Yes / No"
When I click "No"
Then the row reverts to showing "Edit" and "Delete" buttons
```

---

## Feature: Match refresh

### Scenario: Toast shown after successful match refresh
```gherkin
Given I am on the Matches page
When I click Refresh and it succeeds
Then a green "Matches updated" toast appears
```

### Scenario: Error toast shown when refresh fails
```gherkin
Given the match refresh fails
When the error occurs
Then a red toast with the error message appears
```

---

## Feature: Toast behaviour

### Scenario: Toast auto-dismisses after 3 seconds
```gherkin
Given a toast has appeared
When 3 seconds have elapsed
Then the toast fades out automatically
```

### Scenario: Toast positioned above BottomNav on mobile
```gherkin
Given I am on a mobile device
When a toast appears
Then it is visible above the BottomNav and not obscured by it
```

### Scenario: Multiple toasts stack
```gherkin
Given one toast is already showing
When a second action triggers another toast
Then both toasts are visible, stacked vertically
```
