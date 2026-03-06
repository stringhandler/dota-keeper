# Bug: Suggested Goal Accept Button Off Screen on Mobile

## Description
On mobile, the "Create Goal" (and "Refresh") buttons in the Suggested Goal card on the Dashboard are off screen / not reachable. The suggestion card uses a horizontal flex layout that overflows on small screens.

## Steps to Reproduce
1. Open the app on mobile (or resize browser to <640px)
2. Scroll to the "Suggested Goal" section on the Dashboard
3. The action buttons (Refresh / Create Goal) are cut off or unreachable

## Expected Behaviour
All buttons in the suggestion card should be visible and tappable on mobile.

## Fix Suggestion
Stack the suggestion card vertically on mobile — wrap `.suggestion-card` into a column layout at ≤640px so the `.suggestion-actions` buttons flow below the hero info instead of beside it.
