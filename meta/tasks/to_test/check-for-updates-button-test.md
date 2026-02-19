# Test: Check for Updates Button

## Steps

1. Launch the app and navigate to **Settings**
2. Scroll to the **Updates** section at the bottom of the page
3. Verify the current version is shown (e.g. "Current version: v0.1.10")

### Check: Up to date
4. Click **Check for Updates**
5. Button should change to "Checking..." and be disabled while checking
6. After the check: expect "You're on the latest version (v0.x.x)" in green (assuming no new release)

### Check: Update available (if a release is available)
7. If an update is available, expect:
   - Yellow text showing the new version number
   - A green **Install & Restart** button appears below the check button
8. Click **Install & Restart** — app should download the update and relaunch

### Check: Error handling
9. To test error state, disconnect from the internet and click **Check for Updates**
10. Expect a red error message showing the actual error (network error / unreachable), not just a silent failure

## Acceptance Criteria
- [ ] Updates section visible in Settings
- [ ] Current version shown next to button
- [ ] "Checking..." disabled state while in-flight
- [ ] "Up to date" green message when no update
- [ ] Update available → new version text + Install & Restart button
- [ ] Network/other errors shown clearly in red
