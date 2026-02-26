# Test: Custom Title Bar

## What changed

- `tauri.conf.json`: `"decorations": false` added to the window — hides the native Windows title bar
- `src/lib/TitleBar.svelte`: New component with drag region, title text, and Minimize / Maximize / Close buttons
- `src/routes/+layout.svelte`: TitleBar imported and placed above the main app layout
- `src/app.css`: `#svelte` changed from `display: contents` to `display: flex; flex-direction: column` so the title bar and page content stack vertically
- `.loading-screen`, `.login-screen`, `.app-layout` changed from `height: 100vh` to `flex: 1`
- Update banner adjusted to `top: 32px` so it doesn't overlap the title bar

## Steps to test

1. Build and launch the app
2. **Title bar visible**: A 32px bar appears at the very top with "DOTA KEEPER" label and three window control buttons on the right
3. **Dragging**: Click and drag on the title bar (anywhere except the buttons) — the window should move
4. **Minimize**: Click the `–` button — window minimises to taskbar
5. **Maximize/Restore**: Click the `□` button — window toggles between maximized and its normal size
6. **Double-click to maximize**: Double-click the title bar drag area — should toggle maximize
7. **Close**: Click the `✕` button — window closes (app exits)
8. **Close button hover**: Hovering the close button should turn it red (`#c42b1c`)
9. **Content not obscured**: Page content starts below the title bar — no overlap
10. **Login screen**: Log out and verify the login screen also sits below the title bar with no overlap
11. **Update banner (if available)**: If an update is available, the banner appears below the title bar, not behind it
