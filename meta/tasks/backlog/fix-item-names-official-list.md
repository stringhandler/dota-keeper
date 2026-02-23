# Fix Item Names - Use Official Dota 2 Item List

## Problem

Item names displayed in the app are incorrect. The current item list used by `getItemName()` does not match the official Dota 2 item names.

## Goal

Replace the current item data with the correct, up-to-date official Dota 2 item list so that item names display accurately throughout the app (goals, match details, etc.).

## Investigation Needed

1. Find where the current item list is sourced/stored
2. Identify the correct official source for Dota 2 item data:
   - OpenDota API: `https://api.opendota.com/api/constants/items`
   - Steam/Valve game files
   - Dota 2 Wiki data
3. Determine whether items should be fetched at runtime or bundled as static data

## Files to Check

- Wherever `get_all_items` is defined (Tauri backend)
- Any static item data files in the project
- `src-tauri/src/database.rs` or `src-tauri/src/lib.rs` for item storage/retrieval

## Acceptance Criteria

- [ ] Item names match official Dota 2 names (e.g. "Battle Fury", "Black King Bar", "Manta Style")
- [ ] Item list is complete and covers all purchasable items
- [ ] Item IDs match what OpenDota API returns in match data
- [ ] Item timing goals display the correct item name in match detail and goal pages
