# Test: Weekly Challenge Already Accepted Bug Fix

## What changed

`get_active_weekly_challenge` in `database.rs` now queries `status IN ('active', 'completed')` instead of `status = 'active'` only. This means a completed challenge is still returned, so the frontend shows the progress view instead of the option-selection view.

## Steps to test

### Scenario A — Challenge already completed
1. Accept a weekly challenge
2. Play enough games to complete it (the status becomes `'completed'`)
3. Navigate away from the Challenges page and back
4. **Expected:** The progress view is shown with "Completed!" status, not the "Choose a challenge" selection cards

### Scenario B — No challenge accepted yet
1. Ensure no challenge has been accepted this week (or use a fresh DB)
2. Navigate to Challenges
3. **Expected:** The three option cards are still shown normally

### Scenario C — Challenge active (in progress)
1. Accept a challenge
2. Before completing it, navigate away and back
3. **Expected:** The progress view is shown with current progress

### Scenario D — Confirm no duplicate acceptance possible
1. With a completed challenge visible in the progress view, verify there is no "Accept" button shown
2. **Expected:** The accept option should not be reachable
