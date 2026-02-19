# Edit Goal from Goal Details Page

## Problem Statement

Currently, users can view goal details and performance on the Goal Details page, but if they want to modify the goal (change target value, description, or other parameters), they need to navigate back to the main goals list or a separate edit page. This interrupts the workflow and requires extra navigation steps.

Users should be able to edit a goal directly from its detail page, especially when:
- They see the distribution chart and realize the goal is too easy/hard
- They receive a suggestion to adjust the goal
- They want to tweak the goal based on recent performance insights

## Requirements

### UI Components

#### Edit Button
- **Location**: Top-right corner of Goal Details page, near the goal title
- **Icon**: Pencil/edit icon ✏️
- **Label**: "Edit Goal" or just icon on mobile
- **Behavior**: Opens inline edit form or modal dialog

#### Edit Form

**Two Approaches:**

**Option 1: Inline Edit (Recommended)**
- Form replaces the goal title/metadata section
- Maintains context - chart and stats remain visible below
- Save/Cancel buttons appear at bottom of form
- Smooth transition with expand/collapse animation

**Option 2: Modal Dialog**
- Overlay modal with edit form
- Takes focus away from charts but more compact
- Better for mobile devices
- Easier to implement with existing modal components

### Editable Fields

**Core Fields:**
- **Goal Name/Description**: Text input
- **Target Value**: Number input (with validation)
- **Goal Type**: Dropdown (CS, KDA, GPM, etc.) - may be read-only if data exists
- **Comparison Operator**: Dropdown (>=, <=, ==, >, <)

**Advanced Fields** (if applicable):
- **Time Window**: e.g., "by 10 minutes", "overall"
- **Hero Filter**: Which heroes this goal applies to
- **Role Filter**: Which positions this goal applies to (future feature)
- **Active/Inactive**: Toggle to disable goal without deleting

### Validation

**Required Validations:**
- Goal name must not be empty
- Target value must be a valid number
- Target value must be positive (for most goal types)
- Goal name must be unique (or show warning if duplicate)

**Helpful Validations:**
- Warn if target is drastically different from current (e.g., 50 → 150 CS)
- Suggest sensible values based on current performance
- Show preview: "If you change this to X, your current achievement rate would be Y%"

### Save Behavior

**On Save:**
1. Validate all fields
2. Update goal in database
3. Show success toast/notification
4. Refresh charts and statistics with new goal value
5. Maintain page position (don't scroll to top)
6. Exit edit mode

**On Cancel:**
1. Discard all changes
2. Restore original values
3. Exit edit mode
4. No database changes

### Permissions & Safety

**Confirmation Required If:**
- Goal has significant history (e.g., 20+ evaluated games)
- Target value changes by more than 50%
- User is changing goal type (destructive change)

**Confirmation Dialog:**
```
⚠️ This goal has 45 games evaluated against it.
   Changing the target value will re-evaluate all matches.

   Continue?
   [Yes, Update Goal] [Cancel]
```

## User Stories

### Story 1: Quick Adjustment
> "As a user viewing my CS goal, I see the distribution shows I'm hitting it 90% of the time. I want to quickly raise the target from 70 to 80 without leaving this page."

### Story 2: Suggestion Response
> "As a user, I receive a suggestion to lower my goal from 4.0 KDA to 3.2 KDA. I want to click the suggestion and have the edit form pre-filled with the suggested value."

### Story 3: Fix Typo
> "As a user, I notice I typed 'GPM > 500 on AM' but meant 'on Anti-Mage'. I want to quickly fix the description."

### Story 4: Disable Goal
> "As a user, I created a goal for a hero I no longer play. I want to disable it without deleting my historical data."

## Technical Implementation

### Frontend (Svelte)

```svelte
<script>
    export let goal; // Current goal object
    let isEditing = false;
    let editedGoal = { ...goal };

    function startEdit() {
        editedGoal = { ...goal }; // Clone current goal
        isEditing = true;
    }

    async function saveGoal() {
        // Validate
        if (!validateGoal(editedGoal)) {
            return;
        }

        // Confirm if needed
        if (needsConfirmation(goal, editedGoal)) {
            const confirmed = await confirmDialog(
                "This goal has significant history. Continue?"
            );
            if (!confirmed) return;
        }

        // Save via Tauri command
        try {
            await invoke('update_goal', { goal: editedGoal });

            // Update local state
            goal = { ...editedGoal };
            isEditing = false;

            // Refresh charts
            await refreshGoalData();

            // Show success message
            showToast('Goal updated successfully!');
        } catch (error) {
            showToast(`Error: ${error}`, 'error');
        }
    }

    function cancelEdit() {
        editedGoal = { ...goal }; // Restore original
        isEditing = false;
    }
</script>

<div class="goal-detail-page">
    <div class="goal-header">
        {#if !isEditing}
            <h1>{goal.name}</h1>
            <button on:click={startEdit} class="edit-btn">
                ✏️ Edit Goal
            </button>
        {:else}
            <div class="edit-form">
                <input
                    type="text"
                    bind:value={editedGoal.name}
                    placeholder="Goal name"
                />
                <input
                    type="number"
                    bind:value={editedGoal.target_value}
                    placeholder="Target value"
                />
                <select bind:value={editedGoal.comparison_operator}>
                    <option value=">=">>= (greater or equal)</option>
                    <option value="<="><= (less or equal)</option>
                    <option value="==">=  (exactly)</option>
                </select>

                <div class="form-actions">
                    <button on:click={saveGoal} class="btn-primary">
                        Save Changes
                    </button>
                    <button on:click={cancelEdit} class="btn-secondary">
                        Cancel
                    </button>
                </div>
            </div>
        {/if}
    </div>

    <!-- Charts and stats continue below -->
    <GoalDistributionChart {goal} />
    <GoalStatistics {goal} />
</div>
```

### Backend (Rust)

```rust
#[tauri::command]
fn update_goal(goal: Goal) -> Result<(), String> {
    // Validate goal
    if goal.name.trim().is_empty() {
        return Err("Goal name cannot be empty".into());
    }

    if goal.target_value <= 0.0 {
        return Err("Target value must be positive".into());
    }

    // Update in database
    let conn = get_db_connection()?;

    conn.execute(
        "UPDATE goals SET
            name = ?,
            target_value = ?,
            comparison_operator = ?,
            description = ?,
            updated_at = CURRENT_TIMESTAMP
         WHERE id = ?",
        params![
            goal.name,
            goal.target_value,
            goal.comparison_operator,
            goal.description,
            goal.id
        ],
    )
    .map_err(|e| format!("Failed to update goal: {}", e))?;

    // Re-evaluate all matches for this goal
    re_evaluate_goal_matches(goal.id)?;

    Ok(())
}

fn re_evaluate_goal_matches(goal_id: u32) -> Result<(), String> {
    // Get all matches that should be evaluated against this goal
    let matches = get_matches_for_goal(goal_id)?;
    let goal = get_goal(goal_id)?;

    // Re-evaluate each match
    for match_data in matches {
        evaluate_match_against_goal(&match_data, &goal)?;
    }

    Ok(())
}
```

### Database

**No schema changes needed** - using existing `goals` table:
```sql
-- Add updated_at timestamp if not exists
ALTER TABLE goals ADD COLUMN updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP;
```

## UI/UX Design

### Layout - View Mode
```
┌────────────────────────────────────────────────────┐
│ 70 CS by 10 minutes - Anti-Mage          [✏️ Edit] │
│ Target: >= 70   |   Achievement: 68%               │
└────────────────────────────────────────────────────┘
```

### Layout - Edit Mode (Inline)
```
┌────────────────────────────────────────────────────┐
│ ┌────────────────────────────────────────────────┐ │
│ │ Goal Name: [70 CS by 10 minutes - Anti-Mage  ] │ │
│ │ Target:    [70        ] ▼                       │ │
│ │ Operator:  [>= (greater or equal)  ▼]          │ │
│ │ Hero:      [Anti-Mage             ▼]           │ │
│ │                                                 │ │
│ │ [💾 Save Changes]  [Cancel]                    │ │
│ └────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────┘
```

### Styling

```css
.edit-btn {
    background: transparent;
    border: 1px solid var(--border-color);
    padding: 0.5rem 1rem;
    border-radius: 0.25rem;
    cursor: pointer;
    transition: all 0.2s;
}

.edit-btn:hover {
    background: var(--primary-color);
    color: white;
}

.edit-form {
    background: var(--surface-color);
    padding: 1.5rem;
    border-radius: 0.5rem;
    border: 2px solid var(--primary-color);
}

.form-actions {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
}
```

## Success Criteria

- [ ] Edit button is visible and accessible on Goal Details page
- [ ] Clicking edit button enters edit mode
- [ ] All goal fields are editable
- [ ] Form validation works correctly
- [ ] Saving updates the goal in database
- [ ] Charts refresh automatically after save
- [ ] Cancel button discards changes
- [ ] Confirmation dialog appears for significant changes
- [ ] Success/error messages display appropriately
- [ ] Keyboard shortcuts work (ESC to cancel, Enter to save)

## Edge Cases

### Concurrent Edits
- **Problem**: User opens goal in two tabs, edits in both
- **Solution**: Use timestamp-based conflict detection or lock mechanism

### Invalid Values
- **Problem**: User enters negative CS target or impossible values
- **Solution**: Client-side validation + helpful error messages

### Re-evaluation Performance
- **Problem**: Re-evaluating 1000+ matches takes time
- **Solution**: Run re-evaluation in background, show progress indicator

### Deleted Goal
- **Problem**: Goal deleted in another session while user is editing
- **Solution**: Handle 404 error gracefully, show message

## Testing Checklist

- [ ] Edit button appears on goal details page
- [ ] Clicking edit shows form with current values
- [ ] Can edit goal name
- [ ] Can edit target value
- [ ] Can edit comparison operator
- [ ] Can edit other fields (hero, description)
- [ ] Validation prevents empty name
- [ ] Validation prevents invalid target value
- [ ] Save updates database
- [ ] Charts refresh after save
- [ ] Cancel restores original values
- [ ] Confirmation dialog for major changes
- [ ] Success toast appears on save
- [ ] Error handling works for network/DB errors
- [ ] Keyboard shortcuts (ESC/Enter) work
- [ ] Works on mobile devices

## Priority
**HIGH** - Important usability improvement, frequently requested workflow

## Dependencies
- Goal details page (existing)
- Goal update backend (may need enhancements)
- Toast notification system
- Modal/dialog component (if using modal approach)

## Estimated Complexity
**Low-Medium** - Straightforward CRUD operation with UI work

## Related Features
- Goal creation (existing)
- Goal details page (existing)
- Goal suggestions (future - can pre-fill edit form with suggested values)
