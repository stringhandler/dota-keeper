# Bug: "Failed to delete data, the database is locked"

## Symptom

Users (and the dev) see warnings/errors like:

> Failed to delete data, the database is locked

This occurs intermittently, typically when multiple Tauri commands run concurrently (e.g. on app startup when the frontend calls several commands at once).

## Root Cause

Every Tauri command calls `init_db()` to open a **new** `rusqlite::Connection`. SQLite uses file-level write locking — only one writer can hold the lock at a time. When multiple async Tauri commands are in flight simultaneously, each with its own connection, any that try to write while another holds the lock will fail immediately with `SQLITE_BUSY` ("database is locked").

This is a structural issue: the pattern of opening a new connection per command call is inherently unsafe under concurrency.

## Fix

Replace the per-command `init_db()` call pattern with a **single shared connection** protected by a `Mutex`, stored in Tauri's managed state.

```rust
// In lib.rs
use std::sync::Mutex;
use rusqlite::Connection;

pub struct DbState(pub Mutex<Connection>);
```

Register it at app startup:

```rust
tauri::Builder::default()
    .manage(DbState(Mutex::new(database::init_db().expect("DB init failed"))))
    // ...
```

Each command receives it via `State`:

```rust
#[tauri::command]
async fn my_command(state: tauri::State<'_, DbState>) -> Result<..., String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    // use &*conn
}
```

This serializes all DB access through one connection, eliminating concurrent-write races entirely. The `Mutex` ensures only one command touches the DB at a time.

## Additional Hardening (optional, do after the fix)

- Set `PRAGMA journal_mode=WAL` after opening the connection. WAL allows concurrent readers with a single writer and dramatically reduces lock contention if we ever move to a connection pool.
- Set `PRAGMA busy_timeout=5000` as a belt-and-suspenders measure so SQLite waits up to 5 s before returning SQLITE_BUSY instead of failing immediately.

## Affected Area

`src-tauri/src/lib.rs` — all ~35 `let conn = init_db()?;` call sites.
`src-tauri/src/database.rs` — `init_db()` function.
