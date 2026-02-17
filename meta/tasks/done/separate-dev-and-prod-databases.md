# Separate Databases for Development and Production

## Problem Statement

Both development (`yarn tauri dev`) and released builds use the same database file:

```
%LOCALAPPDATA%\DotaKeeper\dota_keeper.db
```

This means:
- Schema migrations or experimental code run during development can corrupt or pollute real user data
- Testing destructive operations (clear matches, schema changes) affects the production database
- There is no safe sandbox to develop against without risking live data

## Current Code

[src-tauri/src/database.rs:176](src-tauri/src/database.rs#L176):
```rust
fn get_db_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|mut path| {
        path.push("DotaKeeper");
        path.push("dota_keeper.db");
        path
    })
}
```

## Desired State

| Build Type | Database Path |
|------------|--------------|
| Development (`tauri dev`) | `%LOCALAPPDATA%\DotaKeeper\dota_keeper_dev.db` |
| Production (released build) | `%LOCALAPPDATA%\DotaKeeper\dota_keeper.db` |

## Implementation

Use Tauri's `#[cfg(debug_assertions)]` compile flag, which is `true` in dev builds and `false` in release builds:

```rust
fn get_db_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|mut path| {
        path.push("DotaKeeper");

        #[cfg(debug_assertions)]
        path.push("dota_keeper_dev.db");

        #[cfg(not(debug_assertions))]
        path.push("dota_keeper.db");

        path
    })
}
```

This requires **no additional configuration** — Cargo sets `debug_assertions` automatically based on the build profile:
- `cargo build` / `tauri dev` → debug profile → `debug_assertions = true`
- `cargo build --release` / `tauri build` → release profile → `debug_assertions = false`

## Files to Change

- [src-tauri/src/database.rs:176-181](src-tauri/src/database.rs#L176-L181) — update `get_db_path()`

## Acceptance Criteria

- [ ] Running `yarn tauri dev` uses `dota_keeper_dev.db`
- [ ] A released/production build uses `dota_keeper.db`
- [ ] The two databases are fully independent
- [ ] No changes needed to any other code or configuration

## Notes

- The dev database will be created fresh on first dev run if it doesn't exist — this is expected
- Existing dev data in `dota_keeper.db` will remain untouched; devs can continue using `dota_keeper_dev.db` going forward
- The dev database name makes it obvious in the filesystem which file is which

## Priority
**HIGH** - Risk of corrupting production data during development

## Estimated Complexity
**Very Low** - 2-line change in `get_db_path()`
