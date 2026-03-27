# UC-065 — Check-in Frequency Setting

## Overview
User can configure how often the post-game mood check-in card appears, replacing the previous hardcoded trigger logic (long session + random).

---

## Feature: Configurable check-in frequency

### Background
```gherkin
Given the user has enabled mental health tracking
And the user is on the Settings page
```

---

### Scenario: Frequency selector is visible when tracking is enabled
```gherkin
Given mental health tracking is enabled
When I view the Mental Wellbeing section
Then I see a "How often to check in?" selector
And the selector shows the current frequency option
```

### Scenario: Frequency selector is hidden when tracking is disabled
```gherkin
Given mental health tracking is disabled
When I view the Mental Wellbeing section
Then the frequency selector is not visible
```

### Scenario: Default frequency is "once_per_session"
```gherkin
Given I have never changed the frequency setting
When I view the frequency selector
Then "Once per session" is selected
```

### Scenario: Frequency preference is saved immediately on change
```gherkin
When I select "Every 3 games" from the frequency selector
Then the preference is saved without requiring a separate Save button
And the selector still shows "Every 3 games" after navigating away and returning
```

### Scenario: every_game — check-in shown after every match
```gherkin
Given the frequency is set to "every_game"
And there is an unchecked match
When I open the dashboard
Then the check-in card is shown
```

### Scenario: every_3 — check-in shown after 3 unchecked matches
```gherkin
Given the frequency is set to "every_3"
And I have played 2 matches since my last check-in
When I open the dashboard
Then no check-in card is shown
When I play a 3rd match
And I open the dashboard
Then the check-in card is shown
```

### Scenario: every_5 — check-in shown after 5 unchecked matches
```gherkin
Given the frequency is set to "every_5"
And I have 4 unchecked matches since my last check-in
When I open the dashboard
Then no check-in card is shown
When I play a 5th match
And I open the dashboard
Then the check-in card is shown
```

### Scenario: every_10 — check-in shown after 10 unchecked matches
```gherkin
Given the frequency is set to "every_10"
And I have 9 unchecked matches since my last check-in
When I open the dashboard
Then no check-in card is shown
When I play a 10th match
And I open the dashboard
Then the check-in card is shown
```

### Scenario: once_per_session — check-in shown at most once per app session
```gherkin
Given the frequency is set to "once_per_session"
And I have unchecked matches
When I open the dashboard
Then a check-in card is shown (at most once per session)
When I dismiss the card and navigate away and back
Then no further check-in card is shown in the same session
```

### Scenario: after_loss — check-in shown only when last match was a loss
```gherkin
Given the frequency is set to "after_loss"
And my most recent unchecked match was a win
When I open the dashboard
Then no check-in card is shown
Given my most recent unchecked match was a loss
When I open the dashboard
Then the check-in card is shown
```

### Scenario: Loss streak always overrides frequency
```gherkin
Given the frequency is set to "every_10"
And I have only 1 unchecked match since my last check-in
And I have lost 3 consecutive matches
When I open the dashboard
Then the check-in card is shown (loss streak override)
```

### Scenario: Preference persists across restarts
```gherkin
Given I have set the frequency to "every_5"
When I close and reopen the application
Then the frequency selector shows "Every 5 games"
And the check-in trigger respects the "every_5" setting
```
