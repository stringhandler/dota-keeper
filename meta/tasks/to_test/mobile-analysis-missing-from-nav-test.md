# Test: Analysis Page Reachable on Mobile

## Steps

1. Run the app and log in with a Steam ID.
2. Resize the window to < 640px wide (or use DevTools mobile emulation, e.g. iPhone 14 Pro 390px).
3. Confirm the BottomNav shows: **Home | Matches | Analysis | Goals | Settings** (Challenges is gone from the nav bar).
4. Tap **Analysis** — confirm it navigates to `/analysis` and the tab highlights gold.
5. Navigate to a hero detail page (`/analysis/[heroId]`) — confirm the Analysis tab still highlights (active prefix match).
6. Confirm **Challenges** is still reachable via:
   - The **⚡ Daily Challenge** / **⚡ Challenges** badge in the topbar.
   - The Challenges card on the Dashboard (if present).
