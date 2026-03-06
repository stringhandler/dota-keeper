# Game State Integration (GSI) — In-Game Goal Overlay

## Overview

When a Dota 2 game starts, automatically detect the hero being played and show a focused overlay screen displaying the active goals for that hero. When the game ends, return to the normal app view.

## How It Works

Valve provides **Game State Integration (GSI)** — Dota 2 will POST JSON to a local HTTP server at configurable intervals with live game state (hero, game phase, stats, etc.). No third-party tools required.

---

## Implementation

### 1. GSI Config File

The player must place a `.cfg` file in their Dota 2 game directory:

```
<Steam>/steamapps/common/dota 2 beta/game/dota/cfg/gamestate_integration/gamestate_integration_dotakeeper.cfg
```

Contents:
```
"dota2-GSI Configuration"
{
    "uri"           "http://127.0.0.1:7322/"
    "timeout"       "5.0"
    "buffer"        "0.1"
    "throttle"      "0.5"
    "heartbeat"     "30.0"
    "data"
    {
        "hero"          "1"
        "map"           "1"
    }
}
```

The app should offer to auto-install this file (it knows the Steam path) or provide copy-paste instructions in Settings.

### 2. Backend — GSI HTTP Listener (Rust)

Add a lightweight HTTP server (using `tiny_http` or `axum`) that:
- Listens on `127.0.0.1:7322`
- Receives POST payloads from Dota 2
- Emits a Tauri event to the frontend when game phase changes

Key GSI fields:
```json
{
  "map": { "game_state": "DOTA_GAMERULES_STATE_GAME_IN_PROGRESS" },
  "hero": { "name": "npc_dota_hero_crystal_maiden", "id": 5 }
}
```

Game states to watch:
- `DOTA_GAMERULES_STATE_HERO_SELECTION` — hero being picked (show hero-specific goals preview)
- `DOTA_GAMERULES_STATE_GAME_IN_PROGRESS` — game started
- `DOTA_GAMERULES_STATE_POST_GAME` — game ended, return to normal view

Tauri events to emit:
- `gsi-hero-selected` — payload: `{ hero_id: i32, hero_name: String }`
- `gsi-game-started` — payload: `{ hero_id: i32 }`
- `gsi-game-ended`

The listener should be started at app launch and run in the background.

### 3. Frontend — In-Game Goals Screen

When `gsi-game-started` is received, navigate to (or overlay) a focused "In-Game" view:

- Shows the hero portrait and name
- Lists all active goals scoped to that hero (specific hero goals + any-core / any-support / any-hero goals that match the hero's role)
- Minimal UI — large text, easy to read at a glance
- A manual "Back" button in case the event is missed

When `gsi-game-ended` fires, automatically return to the normal app.

### 4. Settings

Add a "Game Integration" section in Settings:
- Toggle to enable/disable the GSI listener
- Status indicator (listening / not listening)
- Button to auto-install the GSI config file (requires user to provide their Steam path, or auto-detect from `%PROGRAMFILES(X86)%\Steam`)
- Instructions for manual installation

---

## GSI Config Auto-Install

Auto-detect Steam path from registry:
```
HKEY_CURRENT_USER\Software\Valve\Steam → InstallPath
```

Then write the cfg file to:
`{steam_path}/steamapps/common/dota 2 beta/game/dota/cfg/gamestate_integration/`

---

## Acceptance Criteria

- [ ] GSI listener starts automatically with the app (when enabled)
- [ ] Dota 2 sends game state updates to the listener (verified with a test game)
- [ ] When a game starts, the correct hero is detected
- [ ] In-game view shows goals relevant to the current hero (hero-specific + role-group goals)
- [ ] View returns to normal when the game ends
- [ ] Settings page has enable/disable toggle and cfg install helper
- [ ] App handles the listener already being in use (port conflict) gracefully
