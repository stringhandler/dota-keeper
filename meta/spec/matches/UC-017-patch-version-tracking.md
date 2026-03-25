# UC-017: Patch Version Tracking

## Overview
Store the Dota 2 patch version (e.g., "7.40", "7.40e") on each match and expose it as a filter on the matches page.

## Actors
- Player

## Preconditions
- At least one match is stored in the database

---

## Scenarios

### UC-017-1: Sync patch data
**Given** the player is on the Settings page
**When** they click "Sync Patches"
**Then** patch data is fetched from OpenDota constants API
**And** each stored match without a patch value is assigned the appropriate patch based on its start time
**And** a toast shows "Patches synced. N matches updated."

### UC-017-2: Patch displayed on match row
**Given** patch data has been synced
**And** the matches page is open
**When** the player views the match list
**Then** each match row shows a small patch badge (e.g., "7.40e") in the date column

### UC-017-3: Filter matches by major patch
**Given** patch data has been synced and multiple major patches are represented
**When** the player clicks a major patch filter chip (e.g., "7.40")
**Then** only matches played during any 7.40.x sub-version are shown (7.40, 7.40b, 7.40c, 7.40d, 7.40e, etc.)

### UC-017-4: Matches without patch data
**Given** patch data has not been synced
**When** the player views the match list
**Then** no patch badge is shown on match rows
**And** no patch filter chips are displayed

---

## Notes
- Patch versions use the naming convention: base release "7.40", minor patches "7.40b"–"7.40e"
- The major patch filter uses prefix matching: "7.40" matches "7.40" and "7.40[a-z]"
- Patches are cached in a local `patches` SQLite table and populated via `sync_patches` command
- New matches inserted after a sync automatically get their patch assigned on insert
