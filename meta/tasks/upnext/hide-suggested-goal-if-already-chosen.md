# Hide Suggested Goal of the Week if Already Chosen

## Bug Description

The "Suggested Goal for the Week" panel continues to display even after the user has already added that goal. This is redundant and wastes screen space — if the user has already acted on the suggestion, there is nothing left to do.

## Steps to Reproduce

1. View the Goals or home screen
2. See the "Suggested Goal for the Week" panel
3. Click to add/accept the suggested goal
4. Return to the Goals screen
5. **Bug**: The suggested goal panel is still visible even though the goal now exists

## Expected Behaviour

Once the user has added the suggested goal (or a goal that matches the suggestion), the panel should:
- Either be hidden entirely, OR
- Show a "✓ Already added" state and then fade/collapse

## Fix

When rendering the suggested goal panel, check whether the user already has a goal that matches the suggestion. If they do, hide the panel.

### Detection Logic

```rust
// Backend: check if suggestion is already adopted
fn is_suggestion_already_adopted(suggestion: &GoalSuggestion, goals: &[Goal]) -> bool {
    goals.iter().any(|g| {
        g.goal_type == suggestion.goal_type
        && g.hero == suggestion.hero
        // optionally check target value is similar
    })
}
```

Or on the frontend:

```svelte
<script>
    // Hide panel if user already has a matching goal
    $: isSuggestionAdopted = goals.some(g =>
        g.goal_type === suggestedGoal.goal_type &&
        g.hero === suggestedGoal.hero
    );
</script>

{#if suggestedGoal && !isSuggestionAdopted}
    <SuggestedGoalPanel goal={suggestedGoal} />
{/if}
```

## Acceptance Criteria

- [ ] Suggested goal panel is hidden if the user already has a matching goal
- [ ] Panel reappears if the matching goal is deleted
- [ ] No regression to the suggestion logic itself

## Priority
**MEDIUM** - Visible UX bug, confusing to users

## Estimated Complexity
**Very Low** - Conditional render based on existing goals list
