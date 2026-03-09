# Bug: Weekly Challenge Completed Shows Wrong State

## Description

When the weekly challenge is completed, the UI shows "Choose this week's challenge" instead of displaying the achieved challenge with a "refreshes in X days" countdown.

## Expected Behavior

After completing a weekly challenge:
- Show the completed challenge details (name, description, progress)
- Show a "Refreshes in X days" indicator until the next weekly reset
- Clearly indicate the challenge was achieved (e.g., checkmark, completed styling)

## Actual Behavior

- Shows "Choose this week's challenge" UI as if no challenge has been selected/completed

## Likely Cause

The condition for determining whether to show the "choose" state vs the "active/completed" state is probably only checking if a challenge is active (in progress), not if one was already completed this week.

## Steps to Reproduce

1. Select and complete a weekly challenge
2. Navigate to the weekly challenge section
3. Observe it shows "Choose this week's challenge" instead of the completed state

## Priority

**HIGH** - Incorrect feedback on goal completion is misleading to users
