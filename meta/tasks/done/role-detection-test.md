# Testing: Role Detection

## Steps

1. **Run the dev build** (`npm run tauri dev` or equivalent)
2. **Open the app** and go to Settings — verify it loads without errors
3. **Fetch new matches** — triggers the normal parse flow
4. **Trigger a reparse** via the UI (if a "Reparse" option exists) or wait for auto-parse
5. **Check the database** directly to confirm the `role` column is populated:
   ```sql
   SELECT match_id, role FROM matches WHERE parse_state = 'parsed' LIMIT 10;
   ```
   - Expect values of 1–5 for parsed matches (not all 0)
   - Matches where OpenDota didn't return `lane_role` will stay 0

## What was implemented

- `matches.role` column added via `ALTER TABLE` migration (existing DBs auto-migrate on startup)
- `lane_role` is read from `players[].lane_role` in the OpenDota `/matches/{id}` response
- Role is stored for all three parse paths: single parse, backfill, and reparse
- `find_lane_partner()` helper in `opendota.rs` — returns the lane partner's `DetailedPlayer` for support players (pos 4→pos 3, pos 5→pos 1)
