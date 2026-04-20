# Local Match History from Dota 2 Install Directory

## Summary

Allow the app to read match history directly from the user's local Dota 2 install directory, as an alternative (or supplement) to fetching match data from OpenDota.

## Background

OpenDota is the current sole source of match data. It requires an internet connection, is subject to rate limits, and may not have data for matches that haven't been parsed yet. Dota 2 stores recent match data locally in the install directory, which could be read directly for faster, offline-friendly, and more immediately available match history — particularly for the most recent game.

## Requirements

### Permission & Opt-in

- The feature must be opt-in — the app must explicitly ask the user for permission before accessing the local Dota 2 install directory
- A prompt should explain what data will be read and why, with a clear Accept / Decline option
- If the user declines, the app falls back to OpenDota only (existing behaviour, no change)
- Permission can be revoked in Settings at any time

### Install Path Detection

- Attempt to auto-detect the Dota 2 install path via the Steam library configuration (e.g. `libraryfolders.vdf`)
- Common default paths to check:
  - Windows: `C:\Program Files (x86)\Steam\steamapps\common\dota 2 beta`
  - macOS: `~/Library/Application Support/Steam/steamapps/common/dota 2 beta`
  - Linux: `~/.steam/steam/steamapps/common/dota 2 beta`
- If auto-detection fails, allow the user to manually set the path in Settings

### Match Data Reading

- Read locally available match data from the Dota 2 install directory (e.g. from game state or replay index files)
- Use local data as the primary source for the most recent match(es), where available
- Fall back to OpenDota for matches not available locally, or for detailed parsed stats not present in local files
- Do not replace OpenDota entirely — treat local data as a fast, low-latency supplement

### Settings

- Add a toggle in Settings (API / Data section): "Read match data from local Dota 2 install"
- Show the detected or manually entered install path, with an option to change it
- Show a "Test" button to verify the path is valid and readable

## Acceptance Criteria

- User is prompted for permission before any local file access occurs
- Auto-detection correctly identifies the Dota 2 install path on supported platforms
- Match data from the local directory is surfaced in the app (e.g. most recent match appears without an OpenDota request)
- Fallback to OpenDota works transparently when local data is absent or incomplete
- Settings page reflects current state (enabled/disabled, path, last read)
- Declining or revoking permission causes no local file access whatsoever

## Open Questions

- Which specific local files/formats does Dota 2 expose that are useful for match history? (Needs investigation — may be limited to replay files or game state integration output)
- Should local data take precedence over OpenDota data if both are available, or should they be merged?
- Is the Game State Integration (GSI) system a better approach for live/recent match data? (See backlog task `11-game-state-integration.md`)
