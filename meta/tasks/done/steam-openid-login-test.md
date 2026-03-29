# Test: Steam OpenID Login

## Prerequisites
- Run the app fresh (or log out via Settings if already logged in)
- You need a Steam account and a browser available

## Happy path — Sign in through Steam

1. On the login screen, click **"Sign in through Steam"**
2. Your default browser should open to `steamcommunity.com/openid/login`
3. Log in (or if already logged in, click the "Sign In" button to authorise)
4. The browser tab should show: *"Steam login successful! You can close this tab and return to Dota Keeper."*
5. Switch back to Dota Keeper — it should now be logged in and show the dashboard
6. Confirm the Steam ID shown in the sidebar matches your actual Steam64 ID

## Manual input still works

7. Log out (Settings → bottom)
8. Enter your Steam ID or profile URL manually and click **Save & Continue**
9. Should work exactly as before

## Cancellation / error path

10. Log out again
11. Click **"Sign in through Steam"** — the button should show "Waiting for Steam…"
12. Close the browser tab without logging in (or click Cancel on Steam's page)
13. Dota Keeper should show an appropriate error message (login cancelled)
14. The button should return to its normal state

## Mobile (if testing on Android/iOS)

- The Steam URL should open in the device's default browser
- After authorising, returning to the app should complete the login
