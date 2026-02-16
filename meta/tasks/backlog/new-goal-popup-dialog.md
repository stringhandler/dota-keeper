# Convert New Goal Form to Popup Dialog

## Problem Statement

Currently, the "Create New Goal" form is always visible on the Goals screen, taking up valuable vertical space even when the user isn't creating a goal. This reduces the number of existing goals visible at once and creates visual clutter.

The form should be hidden by default and appear as a popup/modal dialog when the user actively wants to create a new goal.

## Current State (Problem)

```
┌─────────────────────────────────────────┐
│ Goals Page                              │
├─────────────────────────────────────────┤
│ ┌─ CREATE NEW GOAL ──────────────────┐ │ ← Always visible
│ │ Goal Name: [____________]           │ │
│ │ Target:    [____]                   │ │
│ │ Operator:  [>=  ▼]                  │ │
│ │ [Create Goal]                       │ │
│ └─────────────────────────────────────┘ │
├─────────────────────────────────────────┤
│ Existing Goals:                         │
│ • 70 CS by 10 minutes                   │
│ • KDA > 3.5                             │
│ ...                                     │
└─────────────────────────────────────────┘
```

**Issues:**
- Form takes up ~30% of screen height
- Always present even when not needed (99% of the time)
- Reduces space for viewing existing goals
- Feels cluttered and busy

## Desired State (Solution)

```
┌─────────────────────────────────────────┐
│ Goals                    [+ New Goal]   │ ← Button only
├─────────────────────────────────────────┤
│ Your Goals:                             │
│                                          │
│ • 70 CS by 10 minutes          [Edit]   │
│ • KDA > 3.5                    [Edit]   │
│ • GPM > 450 on AM              [Edit]   │
│ • Win rate > 55% on supports   [Edit]   │
│ • Deaths < 6                   [Edit]   │
│ • ...                                   │
│                                          │
└─────────────────────────────────────────┘
```

**When user clicks "+ New Goal":**

```
┌────────────────────────────────────────┐
│                                         │
│   ┌─────────────────────────────────┐  │
│   │  Create New Goal            [✕] │  │
│   ├─────────────────────────────────┤  │
│   │                                 │  │
│   │  Goal Name:                     │  │
│   │  [_______________________]      │  │
│   │                                 │  │
│   │  Goal Type:                     │  │
│   │  [Last Hits ▼]                  │  │
│   │                                 │  │
│   │  Target Value:                  │  │
│   │  [70]                           │  │
│   │                                 │  │
│   │  Comparison:                    │  │
│   │  [>= (at least) ▼]              │  │
│   │                                 │  │
│   │  Time Window: (optional)        │  │
│   │  [by 10 minutes]                │  │
│   │                                 │  │
│   │  Hero Filter: (optional)        │  │
│   │  [All Heroes ▼]                 │  │
│   │                                 │  │
│   │  [Create Goal]  [Cancel]        │  │
│   │                                 │  │
│   └─────────────────────────────────┘  │
│                                         │
└────────────────────────────────────────┘
```

## Requirements

### Trigger Button

**Location:** Top-right of Goals page (or as FAB on mobile)
**Options:**
- **Desktop**: Button in header: `[+ New Goal]` or `[+ Create Goal]`
- **Mobile**: Floating Action Button (FAB) in bottom-right corner: `[+]`

**Styling:**
- Primary color (prominent, clear call-to-action)
- Icon + text on desktop, icon only on mobile FAB
- Accessible keyboard shortcut: `Ctrl/Cmd + N`

### Modal Dialog

#### Appearance
- **Background Overlay**: Semi-transparent dark overlay (dim background)
- **Modal Position**: Centered on screen
- **Modal Size**:
  - Desktop: Medium width (~500-600px), auto height
  - Mobile: Full-screen or bottom sheet
- **Animation**: Smooth fade-in and slide-up entrance

#### Interaction
- **Open**: Click "+ New Goal" button
- **Close**:
  - Click [X] button in modal header
  - Click outside modal (on overlay)
  - Press `ESC` key
  - Click "Cancel" button
  - Successfully create goal (auto-close after success)

#### Content
- Same form fields as current implementation
- Clear title: "Create New Goal"
- Primary action button: "Create Goal"
- Secondary action button: "Cancel"

### Form Fields (No Changes to Logic)

- Goal name/description
- Goal type (CS, KDA, GPM, etc.)
- Target value
- Comparison operator (>=, <=, ==, >, <)
- Time window (optional)
- Hero filter (optional)
- Role filter (optional, future)

### Behavior

**On Open:**
1. Modal appears with fade-in animation
2. Background is dimmed (overlay)
3. Focus automatically moves to first input field
4. Page scroll is disabled (modal has focus)

**On Cancel:**
1. If form has data, show confirmation: "Discard unsaved changes?"
2. Close modal with fade-out animation
3. Clear form data
4. Re-enable page scroll

**On Create:**
1. Validate form
2. If valid: Create goal, show success toast, close modal
3. If invalid: Show inline errors, keep modal open
4. Refresh goals list to show new goal

**On Background Click:**
- Same as "Cancel" - confirm if data exists, then close

## Technical Implementation

### Component Structure

```svelte
<!-- GoalsPage.svelte -->
<script>
    import NewGoalModal from './NewGoalModal.svelte';

    let showNewGoalModal = false;

    function openNewGoalModal() {
        showNewGoalModal = true;
    }

    function closeNewGoalModal() {
        showNewGoalModal = false;
    }

    function handleGoalCreated(event) {
        const newGoal = event.detail;
        // Refresh goals list
        loadGoals();
        // Close modal
        closeNewGoalModal();
        // Show success message
        showToast(`Goal "${newGoal.name}" created!`);
    }
</script>

<div class="goals-page">
    <header>
        <h1>Goals</h1>
        <button on:click={openNewGoalModal} class="btn-primary">
            <span class="icon">+</span> New Goal
        </button>
    </header>

    <div class="goals-list">
        {#each goals as goal}
            <GoalCard {goal} />
        {/each}
    </div>
</div>

{#if showNewGoalModal}
    <NewGoalModal
        on:close={closeNewGoalModal}
        on:created={handleGoalCreated}
    />
{/if}
```

### Modal Component

```svelte
<!-- NewGoalModal.svelte -->
<script>
    import { createEventDispatcher, onMount } from 'svelte';

    const dispatch = createEventDispatcher();

    let formData = {
        name: '',
        type: 'last_hits',
        targetValue: 70,
        operator: '>=',
        timeWindow: '',
        heroFilter: null
    };

    let hasUnsavedChanges = false;

    $: hasUnsavedChanges = formData.name !== '' ||
                            formData.targetValue !== 70;

    function closeModal() {
        if (hasUnsavedChanges) {
            const confirmed = confirm('Discard unsaved changes?');
            if (!confirmed) return;
        }
        dispatch('close');
    }

    function handleBackdropClick(event) {
        if (event.target === event.currentTarget) {
            closeModal();
        }
    }

    function handleKeydown(event) {
        if (event.key === 'Escape') {
            closeModal();
        }
    }

    async function handleSubmit() {
        // Validate
        if (!validateForm(formData)) {
            return;
        }

        // Create goal
        try {
            const newGoal = await invoke('create_goal', { goal: formData });
            dispatch('created', newGoal);
        } catch (error) {
            showToast(`Error: ${error}`, 'error');
        }
    }

    onMount(() => {
        // Focus first input
        document.querySelector('.modal input')?.focus();

        // Prevent body scroll
        document.body.style.overflow = 'hidden';

        return () => {
            // Re-enable body scroll
            document.body.style.overflow = '';
        };
    });
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="modal-backdrop" on:click={handleBackdropClick}>
    <div class="modal">
        <div class="modal-header">
            <h2>Create New Goal</h2>
            <button on:click={closeModal} class="close-btn">✕</button>
        </div>

        <div class="modal-body">
            <form on:submit|preventDefault={handleSubmit}>
                <div class="form-group">
                    <label for="goal-name">Goal Name</label>
                    <input
                        id="goal-name"
                        type="text"
                        bind:value={formData.name}
                        placeholder="e.g., 70 CS by 10 minutes"
                        required
                    />
                </div>

                <div class="form-group">
                    <label for="goal-type">Goal Type</label>
                    <select id="goal-type" bind:value={formData.type}>
                        <option value="last_hits">Last Hits (CS)</option>
                        <option value="kda">KDA</option>
                        <option value="gpm">GPM</option>
                        <option value="xpm">XPM</option>
                        <option value="deaths">Deaths</option>
                    </select>
                </div>

                <div class="form-row">
                    <div class="form-group">
                        <label for="target-value">Target Value</label>
                        <input
                            id="target-value"
                            type="number"
                            bind:value={formData.targetValue}
                            required
                        />
                    </div>

                    <div class="form-group">
                        <label for="operator">Comparison</label>
                        <select id="operator" bind:value={formData.operator}>
                            <option value=">=">>= (at least)</option>
                            <option value="<="><= (at most)</option>
                            <option value="==">=  (exactly)</option>
                        </select>
                    </div>
                </div>

                <!-- Additional fields... -->

                <div class="modal-footer">
                    <button type="button" on:click={closeModal} class="btn-secondary">
                        Cancel
                    </button>
                    <button type="submit" class="btn-primary">
                        Create Goal
                    </button>
                </div>
            </form>
        </div>
    </div>
</div>

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        animation: fadeIn 0.2s ease-out;
    }

    .modal {
        background: var(--surface-color);
        border-radius: 0.5rem;
        box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
        max-width: 600px;
        width: 90%;
        max-height: 90vh;
        overflow: auto;
        animation: slideUp 0.3s ease-out;
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem;
        border-bottom: 1px solid var(--border-color);
    }

    .close-btn {
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        color: var(--text-secondary);
    }

    .close-btn:hover {
        color: var(--text-primary);
    }

    .modal-body {
        padding: 1.5rem;
    }

    .modal-footer {
        display: flex;
        justify-content: flex-end;
        gap: 0.5rem;
        margin-top: 1.5rem;
    }

    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }

    @keyframes slideUp {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    /* Mobile styles */
    @media (max-width: 640px) {
        .modal {
            width: 100%;
            max-width: none;
            border-radius: 0;
            max-height: 100vh;
        }
    }
</style>
```

### Floating Action Button (Mobile)

```svelte
<!-- FAB for mobile -->
<style>
    .fab {
        position: fixed;
        bottom: 1.5rem;
        right: 1.5rem;
        width: 56px;
        height: 56px;
        border-radius: 50%;
        background: var(--primary-color);
        color: white;
        border: none;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
        font-size: 1.5rem;
        cursor: pointer;
        z-index: 100;
    }

    .fab:hover {
        box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4);
        transform: scale(1.05);
    }

    @media (min-width: 641px) {
        .fab {
            display: none;
        }
    }
</style>

<button class="fab" on:click={openNewGoalModal}>
    +
</button>
```

## UI/UX Design

### Visual Hierarchy
1. **Goals list**: Primary focus, occupies full screen height
2. **New Goal button**: Clear but not intrusive
3. **Modal**: Takes focus when active, dims background

### Accessibility
- Modal is keyboard navigable (Tab, Shift+Tab)
- Focus trap within modal (can't tab out)
- ESC key closes modal
- ARIA labels for screen readers
- First input receives focus on open

### Animation
- Smooth 200-300ms transitions
- Background fade-in
- Modal slide-up from bottom
- Reverse animation on close

## Success Criteria

- [ ] New goal form is hidden by default
- [ ] "+ New Goal" button is visible and clear
- [ ] Clicking button opens modal dialog
- [ ] Modal appears centered with overlay
- [ ] Form fields are same as current implementation
- [ ] Creating a goal works correctly
- [ ] Clicking outside modal closes it (with confirmation if data entered)
- [ ] ESC key closes modal
- [ ] Cancel button closes modal
- [ ] Success toast appears after creating goal
- [ ] Goals list refreshes to show new goal
- [ ] Body scroll is disabled when modal is open
- [ ] Focus moves to first input when modal opens
- [ ] Mobile FAB appears on small screens

## Edge Cases

### Unsaved Changes
- **Problem**: User fills form then accidentally clicks outside
- **Solution**: Show confirmation dialog: "Discard unsaved changes?"

### Keyboard Navigation
- **Problem**: User tabs out of modal to background content
- **Solution**: Implement focus trap to keep focus within modal

### Multiple Modals
- **Problem**: Future feature might open modal while this is open
- **Solution**: Close new goal modal or stack modals (z-index management)

### Form Validation Errors
- **Problem**: User submits invalid form
- **Solution**: Show inline errors, keep modal open until valid

## Testing Checklist

- [ ] Button appears in header (desktop)
- [ ] FAB appears in bottom-right (mobile)
- [ ] Clicking button opens modal
- [ ] Modal appears centered with overlay
- [ ] Background is dimmed when modal is open
- [ ] Body scroll is disabled when modal is open
- [ ] Clicking outside modal prompts confirmation if data exists
- [ ] ESC key closes modal (with confirmation)
- [ ] Close button (X) closes modal
- [ ] Cancel button closes modal
- [ ] Creating goal closes modal and refreshes list
- [ ] Success toast appears after creation
- [ ] Form validation works
- [ ] Keyboard navigation works (Tab, Enter, ESC)
- [ ] Focus moves to first input on open
- [ ] Works on desktop
- [ ] Works on mobile/tablet
- [ ] Animation is smooth

## Priority
**MEDIUM** - UX improvement, makes page cleaner and more focused

## Dependencies
- Modal component (create new or use existing)
- Toast notification system (for success message)
- Current goal creation logic (reuse, no changes needed)

## Estimated Complexity
**Low** - Mainly UI refactoring, business logic unchanged

## Related Features
- Goal creation (existing logic, just moving to modal)
- Goal list (existing)
- Edit goal from details page (similar pattern)
