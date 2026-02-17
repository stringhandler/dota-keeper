# Move Logout to Settings Menu Only

## Problem Statement

The logout button is currently exposed as a standalone button (likely in the nav/header). Since users rarely need to log out, having it prominently placed wastes UI space and risks accidental clicks. Logout should be a deliberate action accessible only from the Settings menu.

## Requirements

- Remove the standalone logout button from its current location (nav/header/sidebar)
- Add a "Log Out" option inside the Settings menu/page
- Logout behavior itself does not change — only its location

## Implementation

### 1. Remove Logout Button

Find and remove the logout button from its current location in the nav or header component.

### 2. Add to Settings

In the Settings page or menu, add a logout option at the bottom (conventionally the last item, often styled in red/destructive color):

```svelte
<section class="settings-danger-zone">
    <button class="btn-destructive" on:click={handleLogout}>
        Log Out
    </button>
</section>
```

### Styling

```css
.btn-destructive {
    color: var(--color-danger, #e53e3e);
    background: transparent;
    border: 1px solid var(--color-danger, #e53e3e);
}

.btn-destructive:hover {
    background: var(--color-danger, #e53e3e);
    color: white;
}
```

### Confirmation Dialog (Optional but Recommended)

```
Are you sure you want to log out?
Your match data will remain stored locally.

[Log Out]  [Cancel]
```

## Success Criteria

- [ ] Standalone logout button is removed from nav/header
- [ ] Logout option is present in Settings
- [ ] Logout still functions correctly from its new location
- [ ] Styling indicates it is a destructive action

## Priority
**LOW** - UX cleanup, no functional change

## Estimated Complexity
**Very Low** - Move button, update location only
