# UC-043 — Mental Health History & Suggestions Tab

## Overview
A dedicated "Mind" tab shows the user's mood check-in history and a tilt assessment card with contextual suggestions based on objective match performance and subjective mood data.

---

## Feature: Mental Health tab navigation

### Scenario: Mind tab appears in nav when tracking is enabled
```gherkin
Given mental health tracking is enabled
When I look at the bottom navigation bar
Then I see a "Mind" tab with a heart icon
```

### Scenario: Mind tab navigates to the mental health page
```gherkin
When I tap the "Mind" tab
Then I am taken to the /mental-health route
```

---

## Feature: Disabled state

### Scenario: Disabled state shown when tracking is off
```gherkin
Given mental health tracking is disabled
When I visit /mental-health
Then I see a message prompting me to enable tracking in Settings
And I see a link to Settings → Mental Wellbeing
```

---

## Feature: Tilt assessment card

### Scenario: Assessment shows 7-day averages when data exists
```gherkin
Given I have 3+ completed mood check-ins in the last 7 days
When I visit the Mental Health page
Then I see energy and calm progress bars with 7-day averages
```

### Scenario: Insufficient data state shown before 3 check-ins
```gherkin
Given I have fewer than 3 completed mood check-ins
When I visit the Mental Health page
Then I see "Complete more check-ins to see your mood trends"
And I see a dot indicator showing e.g. "2 of 3 needed"
```

### Scenario: Trend arrow reflects recent mood direction
```gherkin
Given my recent check-ins show improving calm scores
When I view the tilt assessment card
Then I see an "↗ Improving" trend indicator in green
```

### Scenario: Warning badge shown for high tilt
```gherkin
Given I have a tilt score above 55
When I view the tilt assessment card
Then I see a ⚠ warning badge next to the "Mental State" title
```

---

## Feature: Suggestion card

### Scenario: Suggestion shown when tilt score >= 31
```gherkin
Given my tilt score is between 31 and 55
When I view the page
Then a mild suggestion card is shown with appropriate text
```

### Scenario: Peak performance encouragement shown for low tilt
```gherkin
Given my tilt score is <= 20 and I have no recent loss streak
When I view the page
Then a positive "You're in the zone" card is shown
```

### Scenario: Loss spiral suggestion for 3+ consecutive losses with elevated deaths
```gherkin
Given I have 3+ consecutive losses and my deaths are 50%+ above my baseline
When I view the page
Then the suggestion card shows loss-spiral guidance
```

### Scenario: Fatigue suggestion for 5+ games in 24h
```gherkin
Given I have played 5 or more games in the last 24 hours
When I view the page
Then the suggestion card shows fatigue guidance
And action buttons include "Take a Break"
```

---

## Feature: Check-in history

### Scenario: History list shows completed check-ins
```gherkin
Given I have completed mood check-ins
When I view the Check-in History section
Then I see each check-in entry with date, hero icon, energy emoji, and calm emoji
```

### Scenario: Win/loss pill shown per match
```gherkin
When I view a check-in entry
Then I see a W or L pill indicating whether the match was won
```

### Scenario: Skipped check-ins shown as "Skipped" with reduced opacity
```gherkin
Given I have skipped some check-ins
When I view the history list
Then skipped entries show "Skipped" label with reduced opacity
```

### Scenario: Empty history state
```gherkin
Given I have no check-in history
When I view the Check-in History section
Then I see an empty state message about check-in prompts appearing after matches
```
