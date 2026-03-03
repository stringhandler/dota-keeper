# Beta Release Channel

## Summary

Introduce a separate **beta build** of Dota Keeper alongside the stable build, so new features can be tested with opt-in users without risking breakage for everyone on the stable track.

## Decision: Separate Build (not a channel toggle)

A **fully separate beta build** is the right approach rather than a runtime setting that switches the update URL. Reasons:

- A bad beta update can't accidentally land on a stable installation.
- The beta can (and should) use a separate database so beta data doesn't pollute stable saves.
- It installs side-by-side — you can run both at the same time.
- CI can publish beta releases independently without touching the stable update manifest.

A "switch update channel in settings" approach is tempting but dangerous: if the beta update itself is broken you can't roll back via the updater.

## What Needs to Change

### 1. Tauri Config (`tauri.conf.json` → new `tauri.beta.conf.json`)

```json
{
  "productName": "Dota Keeper Beta",
  "identifier": "com.volthawk.dota-keeper-beta",
  "plugins": {
    "updater": {
      "endpoints": [
        "https://raw.githubusercontent.com/stringhandler/dota-keeper/main/meta/autoupdate/latest-beta.json"
      ]
    }
  }
}
```

Key differences vs stable:
- `productName`: `"Dota Keeper Beta"` — shows "Beta" in title bar and taskbar
- `identifier`: `com.volthawk.dota-keeper-beta` — separate OS install, separate registry entry
- `endpoints`: points to `latest-beta.json` instead of `latest.json`

### 2. Database Separation (`database.rs`)

The database name is already `dota_keeper.db` for release and `dota_keeper_dev.db` for dev. Add a third variant for beta: `dota_keeper_beta.db`.

Detect via a compile-time feature flag or a `DOTA_KEEPER_BETA` env var set in the beta build script.

### 3. New Autoupdate Manifest (`meta/autoupdate/latest-beta.json`)

Same structure as `latest.json`. CI populates this on every beta release.

### 4. CI / GitHub Actions

Add a `release-beta.yml` workflow (or a conditional path in the existing `release.yml`) that mirrors the existing `release` branch workflow:
- Triggers on pushes to the `beta` branch (same pattern as `release` → stable)
- Builds using `tauri.beta.conf.json`
- Publishes artifacts as a GitHub **pre-release**
- Updates `meta/autoupdate/latest-beta.json` and commits/pushes it to `main`

Stable releases continue to trigger on pushes to `release` and update `latest.json`.

### 5. Window Title / Visual Indicator

Add a subtle "BETA" badge in the custom title bar (or window title string) when running the beta build, so it's obvious which version is open.

## Release Flow

```
main (dev work)
  │
  ├─► merge to beta branch
  │     → CI builds beta → GitHub pre-release + latest-beta.json updated
  │     → beta users auto-update
  │
  └─► (after testing) merge beta → release branch
        → CI builds stable → GitHub release + latest.json updated
        → stable users auto-update
```

This mirrors the existing `release` branch pattern — no tags needed, the branch push is the release trigger.

## Out of Scope

- In-app channel switching (too risky, decided against above)
- Android / iOS beta tracks (separate concern)
- Crash reporting / telemetry differences between tracks
