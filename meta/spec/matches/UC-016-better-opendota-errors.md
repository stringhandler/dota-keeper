# UC-016 — Better OpenDota Error Messages

## Overview
Raw technical error strings from the OpenDota API are replaced with user-friendly, actionable messages that explain what went wrong and whether the user needs to act.

---

## Feature: User-friendly API errors

### Scenario: Network offline shows "check connection" message
```gherkin
Given the device has no internet connection
When I refresh my matches
Then I see "Couldn't reach OpenDota — check your internet connection. Matches will sync automatically when you're back online."
And no raw error code or Rust error string is shown
```

### Scenario: OpenDota 5xx shows "unavailable, will retry" message
```gherkin
Given OpenDota returns a 5xx server error
When I refresh my matches
Then I see "OpenDota is unavailable right now. Your matches will sync automatically when it comes back."
```

### Scenario: 429 rate limit shows friendly retry message
```gherkin
Given OpenDota returns HTTP 429 (Too Many Requests)
When I refresh my matches
Then I see "Too many requests to OpenDota. Will try again shortly."
```

### Scenario: Invalid Steam ID shows settings guidance
```gherkin
Given the user has entered an incorrect Steam ID
When I refresh my matches
Then I see "No matches found for this Steam ID. Double-check the ID in Settings."
```

### Scenario: Unparsed match shows retry message
```gherkin
Given a match hasn't been parsed by OpenDota yet
When the app tries to fetch detailed match data
Then I see "This match hasn't been parsed by OpenDota yet. It will be picked up on the next sync."
```

### Scenario: Error toast shown on refresh failure
```gherkin
Given the match refresh fails for any reason
When the error occurs
Then a red toast notification appears with the friendly error message
```
