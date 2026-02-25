# Role Detection

## Overview

Detect and store the position (role) each player occupied in a match, sourced from OpenDota's `lane_role` field. This is a focused prerequisite for support goals (lane partner identification) and lays the foundation for the broader role-tracking feature.

## Scope

This task covers **detection and storage only** — displaying role badges in the UI and role-based goal filtering are separate concerns handled by the larger `backlog/role-tracking-and-filtering.md` task.

## Data Source

OpenDota parsed match data includes `lane_role` per player:

```
1 = Safe Lane (Carry / Pos 1)
2 = Mid Lane (Mid / Pos 2)
3 = Off Lane (Offlane / Pos 3)
4 = Soft Support (Pos 4)
5 = Hard Support (Pos 5)
0 = Unknown / Roaming
```

The field is available on parsed matches via `/matches/{match_id}` → `players[].lane_role`.

## Implementation

### Backend (Rust)

#### 1. Database Migration

```sql
ALTER TABLE matches ADD COLUMN role INTEGER DEFAULT 0;
-- 0 = unknown, 1-5 = positions
```

#### 2. OpenDota Parsing (`opendota.rs`)

Extend `DetailedPlayer` to capture `lane_role`:

```rust
#[derive(Debug, Deserialize)]
pub struct DetailedPlayer {
    // existing fields...
    pub lane_role: Option<i32>,  // 1-5 or None
}
```

#### 3. Match Storage (`database.rs`)

When saving a parsed match, write `lane_role` into `matches.role`. Default to `0` if not present.

#### 4. Lane Partner Lookup

Add a helper used by goal evaluation:

```rust
fn find_lane_partner(
    all_players: &[DetailedPlayer],
    player_slot: i32,
    player_role: i32,
) -> Option<&DetailedPlayer> {
    // Find a teammate (same team, different slot) with same lane_role
    // who has a core role (1, 2, or 3) if the current player is support (4 or 5)
}
```

Returns the lane partner's `DetailedPlayer` for use in partner networth/CS goals.

### Backfill

Existing matches will have `role = 0`. A background backfill pass should:
- Re-fetch OpenDota data for already-parsed matches
- Update `role` column
- This can be done lazily when matches are next accessed, or as a one-time migration

## Acceptance Criteria

- [ ] `matches.role` column exists and is populated for newly parsed matches
- [ ] `lane_role` is read from OpenDota API and stored correctly
- [ ] `find_lane_partner()` returns the correct teammate for support players
- [ ] Unknown/roaming players default to role `0` without errors
- [ ] Backfill strategy is documented or implemented

## Related Tasks

- `upnext/support-role-goals.md` — depends on this task for partner identification
- `backlog/role-tracking-and-filtering.md` — broader role UI and filtering (next step after this)
