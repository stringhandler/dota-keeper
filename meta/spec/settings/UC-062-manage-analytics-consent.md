# UC-062: Manage Analytics Consent

## Description
The user controls whether anonymous usage analytics are sent from the application.

## Actor
Authenticated user

## Preconditions
- User is on the Settings page or responding to the initial consent prompt

## Steps

```gherkin
Feature: Manage Analytics Consent

  Scenario: Accept analytics on first prompt
    Given the user has not yet set their analytics preference
    Then an analytics consent modal or prompt is displayed
    When the user clicks "Accept"
    Then analytics consent is saved as Accepted
    And anonymous usage events will be sent from the application

  Scenario: Decline analytics on first prompt
    Given the user has not yet set their analytics preference
    And a consent prompt is shown
    When the user clicks "Decline"
    Then analytics consent is saved as Declined
    And no analytics events will be sent

  Scenario: Change consent in Settings
    Given the user is on the Settings page
    And their current consent status is shown
    When the user clicks to change their consent preference
    Then the consent status is updated
    And future analytics behaviour matches the new preference

  Scenario: Reset analytics consent
    Given the user is on the Settings page
    When the user resets their analytics consent
    Then the consent status returns to "Not Yet"
    And the user will be prompted again at next opportunity
```

## Notes
- Analytics are anonymous — no personally identifiable information is sent
- The installation ID (UUID) is used as an anonymous identifier
- If consent is Declined, no events are tracked at all
