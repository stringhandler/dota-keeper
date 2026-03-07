Feature: Feedback Button
  As a user
  I want to send feedback from within the app
  So that I can report bugs, request features, or share what I like

  Background:
    Given I am logged in and on any page of the app

  Scenario: Feedback button is visible on desktop
    Then I should see a "Feedback" button in the bottom of the sidebar

  Scenario: Feedback button is visible on mobile
    Then I should see a floating feedback icon button above the bottom navigation bar

  Scenario: Opening the feedback modal
    When I click the "Feedback" button
    Then a modal should open with a category selection

  Scenario: Category changes the follow-up prompt
    When I click the "Feedback" button
    And I select "Bug / Something broken"
    Then I should see the prompt "What happened? What did you expect?"
    When I select "Missing feature or improvement"
    Then I should see the prompt "What were you trying to do that wasn't possible?"
    When I select "Works great — tell us what you like"
    Then I should see the prompt "What specifically do you like? Why is it useful?"

  Scenario: Priority field only shown for Bug and Feature categories
    When I click the "Feedback" button
    And I select "Bug / Something broken"
    Then I should see the priority selection field
    When I select "Works great — tell us what you like"
    Then I should not see the priority selection field

  Scenario: Submitting feedback successfully
    When I click the "Feedback" button
    And I select "Bug / Something broken"
    And I fill in the follow-up field with "The match sync button doesn't work"
    And I click "Send Feedback"
    Then my feedback should be submitted to Supabase
    And I should see a success toast notification
    And the modal should close
    And the form should be reset

  Scenario: Submitting feedback with priority
    When I click the "Feedback" button
    And I select "Missing feature or improvement"
    And I fill in the follow-up field with "I want to filter matches by hero"
    And I select priority "Annoying — I work around it"
    And I click "Send Feedback"
    Then my feedback should be submitted with priority "annoying"

  Scenario: Auto-captured context is sent with submission
    When I submit any feedback
    Then the submission should include the current route/page
    And the submission should include the app version
    And the submission should include the platform (windows/macos/linux)

  Scenario: Submit fails gracefully
    Given the Supabase endpoint is unavailable
    When I submit feedback
    Then I should see an error toast notification
    And the modal should remain open

  Scenario: Cannot submit without selecting a category
    When I click the "Feedback" button
    Then the "Send Feedback" button should be disabled

  Scenario: Cannot submit without filling in the follow-up
    When I click the "Feedback" button
    And I select a category
    Then the "Send Feedback" button should be disabled until I type something

  Scenario: Error shown on startup when Supabase is not configured in the build
    Given the app was built without PUBLIC_SUPABASE_URL or PUBLIC_SUPABASE_ANON_KEY env vars
    When the app starts
    Then an error toast should appear indicating feedback is not configured

  Scenario: Feedback fails gracefully when Supabase is not configured
    Given the app was built without PUBLIC_SUPABASE_URL or PUBLIC_SUPABASE_ANON_KEY env vars
    When I click the "Feedback" button
    And I select a category and fill in the follow-up
    And I click "Send Feedback"
    Then I should see an error toast "Feedback is not configured yet."
    And the modal should remain open
