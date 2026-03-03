# Test Guide: Beta Release Channel

## Files Changed

- `src-tauri/tauri.beta.conf.json` — new overlay config
- `src-tauri/Cargo.toml` — added `[features] beta = []`
- `src-tauri/src/database.rs` — beta uses `dota_keeper_beta.db`
- `src-tauri/src/lib.rs` — added `is_beta_build()` Tauri command
- `src/lib/TitleBar.svelte` — shows BETA badge when beta feature is active
- `meta/autoupdate/latest-beta.json` — placeholder manifest
- `.github/workflows/release-beta.yml` — CI workflow for beta releases

## Manual Tests

### 1. Normal dev build — no badge
```
yarn tauri dev
```
- Title bar should show **"Dota Keeper"** only, no BETA badge.

### 2. Beta feature flag — BETA badge appears
```
yarn tauri dev -- -- --features beta
```
Or build with:
```
yarn tauri build -- --features beta --config src-tauri/tauri.beta.conf.json
```
- Title bar should show **"Dota Keeper BETA"** (gold badge next to the title).
- Window title (OS taskbar) should read **"Dota Keeper Beta"**.

### 3. Database isolation
Run stable and beta builds on the same machine and confirm:
- Stable: `%LOCALAPPDATA%/DotaKeeper/dota_keeper.db`
- Beta: `%LOCALAPPDATA%/DotaKeeper/dota_keeper_beta.db`

Each build should open/create its own separate database file.

### 4. Separate app identifier
Install both stable and beta (with `--config`). They should appear as separate
entries in Windows "Apps & Features" — one for `com.volthawk.dota-keeper`,
one for `com.volthawk.dota-keeper-beta`.

### 5. CI workflow (GitHub)
Push a commit to the `beta` branch and confirm:
- `release-beta.yml` workflow triggers (not `release.yml`).
- A GitHub **pre-release** is created tagged `v<version>-beta`.
- `meta/autoupdate/latest-beta.json` is committed back to `main` with platform
  URLs pointing to the beta release assets.
