# Feature: Goals Can Target Ranked, Turbo, or Any Game Mode

## Summary

When creating or editing a goal, users should be able to specify which game mode the goal applies to:
- **Ranked** — only ranked matches count toward this goal
- **Turbo** — only turbo matches count toward this goal
- **Any** — all matches count toward this goal (current implicit default)

## Current State

The `GoalGameMode` enum in `database.rs` already has `Ranked`, `Turbo`, and `All` variants, but the UI may not expose all options or may not filter correctly when evaluating goal progress.

## Acceptance Criteria

- [ ] Goal creation/edit form includes a game mode selector with options: Any, Ranked, Turbo
- [ ] Goal evaluation (pass/fail, progress) only counts matches of the selected game mode
- [ ] Existing goals without a game mode set behave as "Any" (backwards compatible)
- [ ] Goal list displays the game mode filter alongside each goal
- [ ] Dashboard and analysis views respect the game mode filter when calculating goal metrics

## Notes

- Check `GoalGameMode` enum in `src-tauri/src/database.rs` — variants may already exist
- Match game mode is stored in the `matches` table; verify the field name and values used by OpenDota
- "Any" / "All" should be the default for new goals
