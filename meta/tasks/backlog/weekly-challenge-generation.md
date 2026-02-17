# Weekly Challenges – Generation & Reroll

Extracted from `weekly-challenges.md`. Depends on `daily-challenge-core` (shares `challenge_history` table).

## Goal
Generate 3 diverse weekly challenge options each Sunday. Allow 2 rerolls before acceptance.

## Database
```sql
CREATE TABLE challenge_options (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    week_start_date TEXT NOT NULL,
    challenge_type TEXT NOT NULL,
    challenge_description TEXT NOT NULL,
    challenge_target INTEGER NOT NULL,
    challenge_target_games INTEGER,
    hero_id INTEGER,
    metric TEXT,
    option_index INTEGER NOT NULL,  -- 1, 2, 3
    reroll_generation INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL
);

CREATE TABLE weekly_challenges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    week_start_date TEXT NOT NULL,
    challenge_type TEXT NOT NULL,
    challenge_description TEXT NOT NULL,
    challenge_target INTEGER NOT NULL,
    challenge_target_games INTEGER,
    hero_id INTEGER,
    metric TEXT,
    status TEXT NOT NULL DEFAULT 'active',  -- active, completed, failed, skipped
    accepted_at INTEGER,
    completed_at INTEGER,
    reroll_count INTEGER DEFAULT 0,
    UNIQUE(week_start_date)
);
```

## Generation Logic
- 3 options, diverse types: max 1 hero-specific, max 1 performance, mix variety
- Hero selection from last 30 games, prefer heroes with 10+ games
- Difficulty: mix of easy/medium/hard across the 3 options
- Reroll: delete old options, regenerate, increment reroll_count; max 2

## Tauri Commands
```rust
fn get_weekly_challenge_options() -> Result<Vec<ChallengeOption>, String>
fn reroll_weekly_challenges() -> Result<Vec<ChallengeOption>, String>
fn skip_weekly_challenge() -> Result<(), String>
```

## Acceptance Criteria
- [ ] 3 diverse options generated each Sunday
- [ ] Reroll generates fresh set, decrements limit
- [ ] Reroll disabled after 2 uses
- [ ] Options persist until new week or accepted
