# Daily Challenges – Core Backend

Extracted from `weekly-challenges.md`.

## Goal
Implement the database schema, generation logic, and Tauri commands for daily challenges.

## Requirements

### Database
New tables:
- `daily_challenges` – one row per day, auto-generated at first access
- `challenge_history` – archived completed/failed challenges (shared with future weekly challenges)

```sql
CREATE TABLE daily_challenges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    challenge_date TEXT NOT NULL,        -- "YYYY-MM-DD"
    challenge_type TEXT NOT NULL,        -- "quick_win", "performance", "efficiency", "participation"
    challenge_description TEXT NOT NULL,
    challenge_target INTEGER NOT NULL,
    challenge_target_games INTEGER DEFAULT 1,
    hero_id INTEGER,                     -- NULL for non-hero challenges
    metric TEXT,                         -- "wins", "kills", "gpm", "last_hits", "deaths", "hero_damage"
    status TEXT NOT NULL DEFAULT 'active', -- active, completed, failed
    created_at INTEGER NOT NULL,
    completed_at INTEGER,
    UNIQUE(challenge_date)
);

CREATE TABLE challenge_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    challenge_type TEXT NOT NULL,        -- 'daily' (or 'weekly' later)
    period_start_date TEXT NOT NULL,
    challenge_description TEXT NOT NULL,
    status TEXT NOT NULL,                -- completed, failed
    completed_at INTEGER,
    target_achieved INTEGER
);
```

### Challenge Types & Templates
Use a fixed template list with hero/stat filled from player history:

**Quick Win (easy):**
- "Win 1 game today"
- "Win 1 game with [Hero]" (hero from last 10 games)

**Performance (medium):**
- "Get [N]+ kills in one game" (N = avg kills + 2, min 10)
- "Achieve [N]+ GPM in one game" (N = avg GPM + 30, min 400)
- "Finish with positive KDA in one game"
- "Deal [N]+ hero damage in one game" (N calibrated to avg, min 15000)

**Efficiency (medium):**
- "Get [N]+ CS at 10 minutes" (N = avg CS@10 + 5, min 50) – requires parsed matches
- "Die [N] times or less in one game" (N = avg deaths - 1, max 3)

**Participation (easy):**
- "Play 2 games today"
- "Play a game with [Hero]" (hero not played in last 7 days)

### Generation Logic
- 60% Easy (quick_win / participation), 30% Medium (performance), 10% Hard
- Check last 7 daily challenges to avoid repeating same type consecutively
- For hero-specific challenges, pick from matches in last 10 games
- Auto-generate when first accessed for the day; cache in DB

### Daily Streak
- Count consecutive days where `status = 'completed'` going back from yesterday
- Return 0 if yesterday was not completed

### Archive Logic
- On each call to `get_current_daily_challenge`, check if any `active` challenges exist for past dates and mark them `failed`, archive to `challenge_history`

### Tauri Commands
```rust
fn get_daily_challenge() -> Result<Option<DailyChallenge>, String>
fn get_daily_challenge_progress() -> Result<Option<DailyChallengeProgress>, String>
fn get_daily_streak() -> Result<i32, String>
```

### DailyChallengeProgress struct
```rust
pub struct DailyChallengeProgress {
    pub challenge: DailyChallenge,
    pub current_value: i32,     // e.g. wins so far today
    pub target: i32,
    pub completed: bool,
    pub games_counted: i32,     // number of matches evaluated
}
```

## Acceptance Criteria
- [ ] Tables created in `init_db`
- [ ] Daily challenge auto-generated on first access each day
- [ ] Old active challenges auto-archived as failed
- [ ] Progress evaluated against today's matches (after midnight)
- [ ] Challenge auto-marked completed when target met
- [ ] Streak correctly calculated from history
- [ ] All 3 Tauri commands working
