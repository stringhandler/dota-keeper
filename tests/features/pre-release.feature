Feature: Pre-Release Checklist
  These scenarios automate the checks in meta/pre-release-testing.md.
  Run against a release build before publishing.

  Background:
    Given the app is running and I am logged in

  @smoke @feedback
  Scenario: 1. Feedback form submits successfully
    When I click the "Feedback" button
    And I select the "Bug / Something broken" category
    And I fill in the feedback text with "Test submission — please ignore"
    And I select priority "Minor"
    And I click "Send Feedback"
    Then I should see a success toast "Feedback submitted — thank you!"
    And the feedback modal should be closed

  @smoke @goals
  Scenario: 2-3. Create, edit, and delete a goal
    When I navigate to the Goals page
    And I set the goal metric to "Last Hits"
    And I set the goal target to "50"
    And I click "Add Goal"
    Then I should see a toast "Goal created"
    And I should see a goal card with metric "Last Hits"
    When I click "Edit" on the first goal
    And I change the target to "60"
    And I click "Update"
    Then I should see a toast "Goal updated"
    And the first goal card should show target "60"
    When I click "Delete" on the first goal
    And I confirm deletion
    Then I should see a toast "Goal deleted"
    And no goal cards should be visible

  @smoke @matches
  Scenario: 4. Match sync loads recent matches
    When I navigate to the Matches page
    And I click "Refresh Matches"
    Then at least one match row should be displayed

  @manual
  Scenario: 5. Auto-updater notification (manual — release builds only)
    Given I have an older version of the app installed
    When I launch the app
    Then an update notification should appear within a few seconds
