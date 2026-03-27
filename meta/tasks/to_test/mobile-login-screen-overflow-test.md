# Test: Login Screen Mobile Fix

## Steps

1. Log out (or clear the Steam ID from settings so the login screen shows).
2. Resize the browser/window to 390px wide (or use DevTools mobile emulation — iPhone 14 Pro).
3. Confirm **no horizontal scroll** — the login box fits within the viewport.
4. Confirm the login box has side padding/margins and is not full-bleed edge-to-edge.
5. Tap the Steam ID input field — confirm the iOS/Android keyboard does **not** auto-capitalise or auto-correct the input.
6. Confirm the **Save & Continue** button is easily tappable (full width, visually at least 44px tall).
7. Also verify the layout still looks correct at 640px+ (desktop) — box should remain 420px wide with 40px padding.
