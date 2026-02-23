# Test Guide: Move Logout to Settings

## Overview
The logout button has been moved from the sidebar to the Settings page only. It now appears as a destructive action (red styling) in the Account section.

## Changes Made
1. ✅ Removed logout button from sidebar footer in [+layout.svelte](../../src/routes/+layout.svelte)
2. ✅ Added logout functionality to [Settings page](../../src/routes/settings/+page.svelte)
3. ✅ Added "Account" section with destructive-styled logout button
4. ✅ Added confirmation dialog before logout

## Testing Steps

### 1. Verify Logout Button Removed from Sidebar
- [ ] Launch the application
- [ ] Log in with your Steam ID
- [ ] Check the sidebar footer (bottom left)
- [ ] **Expected:** Only the Rank pill should be visible, NO logout button

### 2. Verify Logout Button in Settings
- [ ] Navigate to Settings page (click Settings in sidebar)
- [ ] Scroll to the bottom of the settings page
- [ ] **Expected:** See an "Account" section with a "Log Out" button
- [ ] **Expected:** Button should have red/destructive styling (red border and text)

### 3. Test Logout Functionality
- [ ] Click the "Log Out" button in Settings
- [ ] **Expected:** A confirmation dialog appears asking "Are you sure you want to log out?"
- [ ] **Expected:** Dialog mentions "Your match data will remain stored locally"
- [ ] Click "Cancel" in the dialog
- [ ] **Expected:** Nothing happens, you stay logged in

### 4. Test Actual Logout
- [ ] Click "Log Out" button again
- [ ] Click "OK" in the confirmation dialog
- [ ] **Expected:** Application reloads and shows the login screen
- [ ] Log back in with your Steam ID
- [ ] **Expected:** Your match data should still be there (data persists)

### 5. Visual Checks
- [ ] Hover over the "Log Out" button in Settings
- [ ] **Expected:** Button should have a red glow/highlight effect on hover
- [ ] **Expected:** Background should turn red on hover
- [ ] **Expected:** Text should turn white on hover

## Success Criteria
- ✅ No logout button visible in sidebar
- ✅ Logout button present in Settings under "Account" section
- ✅ Button has destructive red styling
- ✅ Confirmation dialog appears before logout
- ✅ Logout works correctly from new location
- ✅ Data persists after logout/login cycle

## Rollback Instructions
If issues are found, the changes can be reverted by checking out the previous commit:
```bash
git checkout HEAD~1 src/routes/+layout.svelte src/routes/settings/+page.svelte
```
