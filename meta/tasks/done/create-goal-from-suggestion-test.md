# Test: Create Goal from Suggested Goal

## What was fixed
`acceptSuggestion` in `src/routes/+page.svelte` was missing `frequency_type` and `hero_scope`
fields required by the `NewGoal` Rust struct, causing "Failed to create goal: invalid args,
missing frequency_type" on mobile (and desktop).

## Steps to test

1. Go to the Dashboard.
2. Scroll to the "Suggested Goal" card (requires match history with a tracked hero).
3. Click "Create Goal" on the suggestion.
4. Expected: goal is created successfully, toast shows confirmation.
5. Go to Goals page — the new LastHits/Ranked goal for that hero should appear.
