# Test Guide — Toast Notifications & Inline Goal Delete

## Setup
- Build and run the app
- Log in with a valid Steam ID

## Steps

### 1. Goal created toast
- Go to Goals page
- Fill in the goal form and click Save
- Confirm a green "Goal created" toast appears at the bottom of the screen
- Confirm it auto-dismisses after ~3 seconds

### 2. Goal updated toast
- Click Edit on an existing goal
- Make a change and save
- Confirm a green "Goal updated" toast appears

### 3. Goal deleted — inline confirm (no browser dialog!)
- Click "Delete" on a goal
- Confirm the row shows "Delete? Yes / No" inline buttons (no browser confirm popup)
- Click "No" — confirm the row reverts to Edit / Delete buttons
- Click "Delete" again, then "Yes"
- Confirm a green "Goal deleted" toast appears
- Confirm the goal is removed from the list

### 4. Match refresh toast
- Go to Matches page
- Click the Refresh button
- On success: confirm a green "Matches updated" toast appears
- On failure (e.g. disconnect network): confirm a red error toast with a friendly message

### 5. Toast positioning
- On mobile: confirm the toast appears above the BottomNav, not behind it
- On desktop: confirm the toast is visible in the main content area (not behind the sidebar)

### 6. Multiple toasts
- Trigger two actions in quick succession (e.g. refresh then navigate away quickly)
- Confirm multiple toasts stack vertically
