# Bug: "Choose this week's challenge" shown but accepting fails

## Symptom

The UI shows the "Choose this week's challenge" card/prompt, but when the user selects a challenge it returns:

> Failed to accept challenge: A challenge has already been accepted for this week

## Expected Behaviour

If a challenge has already been accepted, the UI should show the active challenge, not the selection card.

## Likely Cause

The frontend is not correctly checking whether a challenge is already active before showing the selection UI. The backend correctly rejects the duplicate, but the frontend state is stale or the active-challenge check is missing/broken.
