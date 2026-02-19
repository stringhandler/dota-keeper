# Challenge History Page

Extracted from `weekly-challenges.md`. Depends on `weekly-challenge-progress` and `daily-challenge-core`.

## Goal
View past completed and failed challenges (both weekly and daily) in a combined history view.

## Route
`/challenges/history`

## Layout
- Filter tabs: All | Weekly | Daily
- Grouped by week
- Each entry shows: type icon, description, status (✅/✗), completion date or failure note, target achieved

## Tauri Command
```rust
fn get_challenge_history(
    challenge_type: Option<String>,  // "weekly", "daily", or None
    limit: i32
) -> Result<Vec<ChallengeHistoryItem>, String>
```

## Acceptance Criteria
- [ ] Lists past challenges from `challenge_history` table
- [ ] Filter tabs work (weekly/daily/all)
- [ ] Grouped by week with clear date headings
- [ ] Empty state handled gracefully
