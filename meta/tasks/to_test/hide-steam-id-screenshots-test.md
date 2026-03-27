# Test Guide: Privacy Mode (Hide Steam ID)

## Steps to Test

1. **Build and launch** the app.

2. **Navigate to Settings > Privacy section** (at the top of the Privacy section, above Anonymous Analytics).

3. **Verify the toggle is visible** with label "Privacy Mode" and description explaining it masks the Steam ID.

4. **Toggle Privacy Mode ON** — the button should show "Enabled".
   - Check the sidebar: the Steam ID should now appear as the first 5 digits followed by `••••••••••` (e.g. `76561••••••••••`).

5. **Toggle Privacy Mode OFF** — the button should show "Disabled".
   - Check the sidebar: the full Steam ID should reappear.

6. **Restart the app** with Privacy Mode ON — confirm the setting persists (sidebar still shows masked ID, toggle still shows "Enabled").

7. **Restart the app** with Privacy Mode OFF — confirm the setting persists (full ID shown).

## Expected Behaviour
- Steam ID in the sidebar is masked when Privacy Mode is enabled.
- The actual Steam ID is never lost — toggling off restores the full display.
- Setting persists across app restarts (stored in `settings.json`).
