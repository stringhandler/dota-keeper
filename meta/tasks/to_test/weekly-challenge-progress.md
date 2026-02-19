# Weekly Challenges – Acceptance & Progress Tracking

Extracted from `weekly-challenges.md`. Depends on `weekly-challenge-generation`.

## Goal
Accept a challenge, track progress against matches played during the week, auto-complete or auto-fail.

## Requirements
- On accept: set `accepted_at` timestamp; only matches after this time and before week end count
- Progress evaluated per challenge type (wins, avg stat, hero-specific, etc.)
- Auto-complete when target reached; auto-fail on Sunday rollover
- Archive completed/failed challenges to `challenge_history`

## Tauri Commands
```rust
fn accept_weekly_challenge(challenge_id: i64) -> Result<WeeklyChallenge, String>
fn get_active_weekly_challenge() -> Result<Option<WeeklyChallenge>, String>
fn get_weekly_challenge_progress() -> Result<Option<ChallengeProgress>, String>
```

## ChallengeProgress struct
```rust
pub struct ChallengeProgress {
    pub challenge: WeeklyChallenge,
    pub current_value: i32,
    pub target: i32,
    pub games_counted: i32,
    pub days_remaining: i32,
    pub completed: bool,
}
```

## Acceptance Criteria
- [ ] Accepting locks in challenge, disables reroll
- [ ] Progress counts only post-acceptance, within the week
- [ ] Auto-completes when target met
- [ ] Sunday rollover archives incomplete as failed
- [ ] `days_remaining` calculated correctly
