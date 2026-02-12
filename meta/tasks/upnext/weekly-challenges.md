# Weekly and Daily Challenges

## Overview
A dual-challenge system that provides both weekly and daily challenges:
- **Weekly Challenges**: Players choose from 3 different challenge options each Sunday. Challenges run for 7 days (Sunday-Saturday) and can be rerolled before acceptance.
- **Daily Challenges**: A single quick challenge refreshes every day at midnight. Simpler tasks designed to be completed in 1-3 games.

Both challenge types track progress only on games played during their respective active periods.

## User Story
As a player, I want to choose from multiple weekly challenges and complete quick daily challenges so that I have both long-term and short-term goals to keep me engaged and motivated every day.

## Requirements

### Challenge Selection
- Generate 3 different challenge options each week
- Challenges become available every Sunday at 00:00 (start of week)
- Player can:
  - **Accept** one of the three challenges
  - **Reroll** to get 3 new challenge options (limit: 2 rerolls per week)
  - **Skip** the week entirely
- Once a challenge is accepted, it becomes active for that week
- Cannot change challenge after accepting (until next week)

### Challenge Period
- **Start**: Sunday 00:00
- **End**: Saturday 23:59 (following Saturday)
- **Duration**: 7 days
- Only matches played during the active challenge period count toward progress
- Matches played before accepting the challenge do NOT count
- If player accepts mid-week (e.g., Wednesday), only matches from Wednesday onward count

### Challenge Types

#### 1. Hero-Specific Challenges
- "Win 5 games with [Hero]"
- "Average 60+ CS at 10 min on [Hero] (3 games minimum)"
- "Maintain 3.0+ KDA on [Hero] (5 games minimum)"
- "Deal 20,000+ hero damage per game on [Hero] (3 games minimum)"

#### 2. Role-Based Challenges
- "Win 7 games as Carry"
- "Average 450+ GPM across 5 games"
- "Play 3 different Support heroes and win"

#### 3. Performance Challenges
- "Win 5 out of 7 games this week"
- "Achieve 10+ kills in 3 games"
- "Die less than 5 times in 5 consecutive games"
- "Get a Rampage (or achieve 15+ kills in one game)"

#### 4. Variety Challenges
- "Play 5 different heroes this week"
- "Win with 3 heroes you haven't played in 30 days"
- "Try a new role (play 3 games in a role with <10% of your total games)"

### Challenge Generation Logic

**Hero Selection for Challenges:**
- Use heroes from player's last 30 games
- Prioritize heroes with 10+ total games (familiar heroes)
- Occasionally suggest heroes with 3-9 games (expanding hero pool)
- Never suggest heroes with 0 games

**Difficulty Balancing:**
- Mix easy, medium, and hard challenges
- Easy: ~70-80% achievable based on recent stats
- Medium: ~50-60% achievable (slight stretch)
- Hard: ~30-40% achievable (significant improvement needed)

**Variety:**
- Ensure 3 challenges are diverse (not all hero-specific)
- Avoid duplicate challenge types in same set
- Rotate challenge types week-to-week

### Reroll System
- **Reroll Limit**: 2 rerolls per week
- **Reroll Behavior**:
  - Generates completely new set of 3 challenges
  - Reroll count resets every Sunday
  - Cannot reroll after accepting a challenge
- **UI Indication**: Show "X rerolls remaining" on challenge screen

### Progress Tracking

**Active Challenge Display:**
- Show on dashboard when challenge is active
- Real-time progress updates as matches are played
- Visual progress bar or counter
- Time remaining in challenge period

**Challenge States:**
- **Not Started**: Sunday arrived, no challenge selected yet
- **Active**: Challenge accepted, tracking in progress
- **Completed**: Challenge achieved within the week
- **Failed**: Week ended without completing challenge
- **Skipped**: Player chose not to participate this week

**Completion Detection:**
- Automatically check progress after each new match is fetched
- Notify player when challenge is completed
- Award completion status (for future badge/streak system)

## Daily Challenges

### Overview
Daily challenges are simpler, faster-paced challenges that reset every 24 hours. Unlike weekly challenges, there's only ONE daily challenge - no selection process.

### Daily Challenge Characteristics
- **Duration**: 24 hours (midnight to midnight)
- **Quantity**: 1 challenge per day (not 3 options)
- **No Selection**: Automatically generated, no reroll option
- **Auto-Accept**: Challenge is active immediately upon generation
- **Simpler Goals**: Designed to be completable in 1-3 games
- **Higher Completion Rate**: ~60-70% achievable targets

### Daily Challenge Types

#### 1. Quick Win Challenges
- "Win 1 game today"
- "Win 1 game with [Hero]" (a hero played in last 10 games)
- "Win 1 game as [Role]"

#### 2. Performance Milestones
- "Get 10+ kills in one game"
- "Achieve 400+ GPM in one game"
- "Get a triple kill or higher"
- "Finish with positive KDA (K+A > D)"

#### 3. Efficiency Challenges
- "Get 50+ CS at 10 minutes in one game"
- "Die 3 times or less in one game"
- "Deal 15,000+ hero damage in one game"

#### 4. Participation Challenges
- "Play 2 games today"
- "Play a game with [specific hero]" (hero not played in last 7 days)
- "Complete 1 ranked match"

### Daily Challenge Generation Logic

**Difficulty Distribution:**
- 60% Easy challenges (1 game, achievable stats)
- 30% Medium challenges (1-2 games, moderate skill)
- 10% Hard challenges (skill expression, rare achievements)

**Hero Selection for Daily:**
- Prefer heroes played in last 10 games (higher familiarity)
- Occasionally suggest heroes with 5+ total games
- Never suggest completely new heroes

**Variety Over Time:**
- Track last 7 daily challenges
- Avoid repeating same challenge type 2 days in a row
- Rotate between win-based, performance-based, and participation challenges

### Daily Challenge Lifecycle

**Refresh Timing:**
```
00:00:00 (midnight) → New daily challenge generated
23:59:59 (end of day) → Current challenge archived
                        (Status: completed or failed)
```

**Progress Tracking:**
- Only games played after midnight count
- Challenge evaluates immediately after each match
- Auto-complete when target met
- Auto-fail at midnight if incomplete

**No Reroll:**
- Daily challenges cannot be rerolled
- Accept what you get or skip for the day
- Keeps daily challenges simple and quick

### Daily vs Weekly Comparison

| Feature | Daily Challenge | Weekly Challenge |
|---------|----------------|------------------|
| **Duration** | 24 hours | 7 days |
| **Quantity** | 1 challenge | Choose from 3 options |
| **Selection** | Auto-assigned | Player chooses |
| **Reroll** | No reroll | 2 rerolls per week |
| **Difficulty** | Easy-Medium | Easy-Hard mix |
| **Completion Target** | 1-3 games | 3-7 games |
| **Acceptance** | Auto-active | Manual acceptance |

## Database Schema

### New Table: `weekly_challenges`
```sql
CREATE TABLE weekly_challenges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    week_start_date TEXT NOT NULL,  -- e.g., "2026-02-09" (Sunday)
    challenge_type TEXT NOT NULL,   -- "hero_wins", "cs_average", "kda", etc.
    challenge_description TEXT NOT NULL,
    challenge_target INTEGER NOT NULL,
    challenge_target_games INTEGER,  -- Minimum games required (nullable)
    hero_id INTEGER,  -- NULL for non-hero challenges
    metric TEXT,  -- "wins", "last_hits", "kills", etc.
    status TEXT NOT NULL DEFAULT 'active',  -- active, completed, failed, skipped
    accepted_at INTEGER,  -- NULL until player accepts
    completed_at INTEGER,  -- NULL until completed
    reroll_count INTEGER DEFAULT 0,
    UNIQUE(week_start_date)  -- One active challenge per week
)
```

### New Table: `challenge_options`
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
    option_index INTEGER NOT NULL,  -- 1, 2, or 3
    reroll_generation INTEGER DEFAULT 0,  -- Tracks which reroll generated this
    created_at INTEGER NOT NULL
)
```

### New Table: `challenge_history`
```sql
CREATE TABLE challenge_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    challenge_type TEXT NOT NULL,  -- 'weekly' or 'daily'
    period_start_date TEXT NOT NULL,  -- Week start or day date
    challenge_description TEXT NOT NULL,
    status TEXT NOT NULL,  -- completed, failed, skipped
    completed_at INTEGER,
    matches_counted INTEGER,
    target_achieved INTEGER  -- The actual value achieved
)
```

### New Table: `daily_challenges`
```sql
CREATE TABLE daily_challenges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    challenge_date TEXT NOT NULL,  -- e.g., "2026-02-12"
    challenge_type TEXT NOT NULL,  -- "quick_win", "performance", "efficiency", etc.
    challenge_description TEXT NOT NULL,
    challenge_target INTEGER NOT NULL,
    challenge_target_games INTEGER DEFAULT 1,
    hero_id INTEGER,  -- NULL for non-hero challenges
    metric TEXT,  -- "wins", "kills", "gpm", etc.
    status TEXT NOT NULL DEFAULT 'active',  -- active, completed, failed
    created_at INTEGER NOT NULL,
    completed_at INTEGER,  -- NULL until completed
    UNIQUE(challenge_date)  -- One challenge per day
)
```

## Frontend Components

### Challenge Selection Screen (`/challenges`)

**Layout:**
```
Weekly Challenge - Feb 9 - Feb 15

Choose one challenge for this week:

┌─────────────────────────────────────┐
│ Challenge 1                    [✓]  │
│ Win 5 games with Storm Spirit      │
│ Difficulty: Medium                  │
│ Reward: 🏆 Weekly Winner            │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│ Challenge 2                    [✓]  │
│ Average 450+ GPM (5 games)          │
│ Difficulty: Hard                    │
│ Reward: 🏆 Weekly Winner            │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│ Challenge 3                    [✓]  │
│ Play 5 different heroes             │
│ Difficulty: Easy                    │
│ Reward: 🏆 Weekly Winner            │
└─────────────────────────────────────┘

[🎲 Reroll] (1 remaining)    [Skip This Week]
```

**After Accepting:**
```
Active Challenge - 4 days remaining

Win 5 games with Storm Spirit
Progress: 3/5 wins ████░░ 60%

Recent matches:
✓ Feb 10 - Victory (3/2/8)
✓ Feb 11 - Victory (8/4/12)
✗ Feb 12 - Defeat (2/7/5)
✓ Feb 13 - Victory (10/1/9)

[View Full Match History]
```

### Dashboard Widget

**When Challenge Active:**
```
🏆 This Week's Challenge
Win 5 games with Storm Spirit
Progress: 3/5 ████░░ 60%
4 days remaining
[View Details]
```

**When Challenge Completed:**
```
✅ Challenge Complete!
Win 5 games with Storm Spirit
Completed on Feb 14
[View Next Challenge]
```

### Daily Challenge Widget (Dashboard)

**Active Daily Challenge:**
```
⚡ Today's Challenge
Get 10+ kills in one game
Progress: 0/1 ░░░░░ 0%
Resets in 8 hours
```

**Completed Daily Challenge:**
```
✅ Daily Complete!
Get 10+ kills in one game
Completed at 3:42 PM
+1 Daily Streak 🔥
```

**Combined Dashboard View:**
```
┌─────────────────────────────────────┐
│ ⚡ Today's Challenge                 │
│ Get 10+ kills in one game           │
│ Progress: 0/1 ░░░░░                 │
│ Resets in 8 hours                   │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│ 🏆 This Week's Challenge             │
│ Win 5 games with Storm Spirit      │
│ Progress: 3/5 ████░░ 60%            │
│ 4 days remaining                    │
│ [View Details]                      │
└─────────────────────────────────────┘
```

### Challenge History Page (`/challenges/history`)
```
Challenge History

[Weekly] [Daily] [All]  ← Filter tabs

=== This Week ===
Feb 12 (Today)
⚡ ✅ Daily: Get 10+ kills in one game
    Completed at 3:42 PM

Feb 11 (Yesterday)
⚡ ✗ Daily: Win 1 game with Anti-Mage
    Failed (0/1 wins)

=== Week of Feb 9 ===
🏆 ✅ Weekly: Win 5 games with Storm Spirit
    Achieved: 5/5 wins (100%)
    Completed on Feb 14

⚡ Feb 10: Get 50+ CS at 10 min - Completed
⚡ Feb 9: Play 2 games - Completed

=== Week of Feb 2 ===
🏆 ✗ Weekly: Average 60+ CS at 10 min
    Achieved: 54 avg CS (90%)

⚡ 5 daily challenges completed
⚡ 2 daily challenges failed

=== Week of Jan 26 ===
🏆 ⊝ Weekly: Skipped

[Show More]
```

## Backend Implementation

### Core Functions (database.rs)

1. **`generate_challenge_options(week_start: &str, reroll: i32) -> Vec<ChallengeOption>`**
   - Generates 3 diverse challenges based on player stats
   - Uses similar logic to hero goal suggestions
   - Balances difficulty across options

2. **`get_current_week_challenges() -> Vec<ChallengeOption>`**
   - Returns 3 challenge options for current week
   - Generates new ones if none exist
   - Returns existing options if already generated

3. **`accept_challenge(challenge_id: i64) -> Result<WeeklyChallenge>`**
   - Marks challenge as accepted
   - Sets `accepted_at` timestamp
   - Only counts matches from this point forward

4. **`reroll_challenges() -> Result<Vec<ChallengeOption>>`**
   - Checks reroll limit (max 2 per week)
   - Deletes old options
   - Generates new set of 3 challenges
   - Increments reroll count

5. **`evaluate_challenge_progress(challenge_id: i64) -> ChallengeProgress`**
   - Queries matches played since challenge acceptance
   - Filters by week period (Sunday - Saturday)
   - Calculates current progress vs target
   - Returns progress data structure

6. **`check_challenge_completion() -> Option<WeeklyChallenge>`**
   - Auto-checks after new matches are fetched
   - Updates challenge status to "completed" if target met
   - Sets `completed_at` timestamp

7. **`archive_expired_challenges()`**
   - Runs on Sunday rollover
   - Moves incomplete challenges to `challenge_history` with status "failed"
   - Clears old `challenge_options`

### Daily Challenge Functions (database.rs)

1. **`generate_daily_challenge(date: &str) -> DailyChallenge`**
   - Generates a single challenge for the specified date
   - Balances difficulty (60% easy, 30% medium, 10% hard)
   - Ensures variety by checking last 7 daily challenges
   - Auto-selects hero from recent games for hero-specific challenges

2. **`get_current_daily_challenge() -> Option<DailyChallenge>`**
   - Returns today's challenge
   - Generates new one if none exists for today
   - Returns None if database issue

3. **`evaluate_daily_progress(challenge_id: i64) -> DailyChallengeProgress`**
   - Queries matches played today (after midnight)
   - Calculates current progress vs target
   - Returns progress data structure

4. **`check_daily_completion() -> Option<DailyChallenge>`**
   - Auto-checks after new matches are fetched
   - Updates status to "completed" if target met
   - Sets `completed_at` timestamp

5. **`archive_expired_daily_challenges()`**
   - Runs at midnight rollover
   - Moves incomplete daily challenges to `challenge_history` with status "failed"
   - Generates new daily challenge for new day

6. **`get_daily_streak() -> i32`**
   - Counts consecutive days with completed daily challenges
   - Used for streak tracking and rewards

### Tauri Commands

**Weekly Challenges:**
```rust
#[tauri::command]
fn get_weekly_challenges() -> Result<Vec<ChallengeOption>, String>

#[tauri::command]
fn accept_weekly_challenge(challenge_id: i64) -> Result<WeeklyChallenge, String>

#[tauri::command]
fn reroll_weekly_challenges() -> Result<Vec<ChallengeOption>, String>

#[tauri::command]
fn get_active_weekly_challenge() -> Result<Option<WeeklyChallenge>, String>

#[tauri::command]
fn get_weekly_challenge_progress() -> Result<Option<ChallengeProgress>, String>

#[tauri::command]
fn skip_weekly_challenge() -> Result<(), String>
```

**Daily Challenges:**
```rust
#[tauri::command]
fn get_daily_challenge() -> Result<Option<DailyChallenge>, String>

#[tauri::command]
fn get_daily_challenge_progress() -> Result<Option<DailyChallengeProgress>, String>

#[tauri::command]
fn get_daily_streak() -> Result<i32, String>
```

**Combined:**
```rust
#[tauri::command]
fn get_challenge_history(
    challenge_type: Option<String>,  // "weekly", "daily", or None for all
    limit: i32
) -> Result<Vec<ChallengeHistoryItem>, String>
```

## Edge Cases

### 1. Player Accepts Challenge Mid-Week
- **Scenario**: Player accepts challenge on Thursday
- **Behavior**: Only Thursday-Saturday matches count
- **UI**: Show "2 days remaining" and clearly state acceptance date

### 2. Week Rollover While Challenge Active
- **Scenario**: Sunday arrives, player has active incomplete challenge
- **Behavior**:
  - Mark current challenge as "failed"
  - Archive to history
  - Generate new challenge options for new week

### 3. No New Matches During Challenge Week
- **Scenario**: Player accepts challenge but doesn't play any games
- **Behavior**: Challenge remains at 0% progress, fails on Sunday

### 4. Reroll Limit Reached
- **Scenario**: Player used both rerolls
- **UI**: Disable reroll button, show "No rerolls remaining"
- **Behavior**: Player must choose from current 3 options or skip

### 5. All 3 Challenge Options Involve Same Hero
- **Scenario**: Very small hero pool, algorithm generates duplicates
- **Behavior**: Force diversity by restricting max 1 hero-specific challenge per set

### 6. Challenge Impossible Due to Hero Not Played Recently
- **Scenario**: Challenge suggests hero player hasn't touched in months
- **Behavior**: Algorithm should avoid this, but if happens, allow reroll

### 7. Multiple Matches in Single Fetch
- **Scenario**: Player fetches 10 new matches at once, 7 contribute to challenge
- **Behavior**: Evaluate all 7 matches at once, update progress, check completion

## Technical Considerations

### Week Calculation
```rust
fn get_current_week_start() -> String {
    let now = chrono::Local::now();
    let days_since_sunday = now.weekday().num_days_from_sunday();
    let sunday = now - chrono::Duration::days(days_since_sunday as i64);
    sunday.format("%Y-%m-%d").to_string()
}
```

### Match Filtering
```sql
SELECT * FROM matches
WHERE start_time >= ? -- accepted_at timestamp
  AND start_time < ?  -- week_end timestamp (next Sunday 00:00)
  AND [challenge-specific criteria]
```

### Challenge Diversity Algorithm
```rust
fn ensure_challenge_diversity(challenges: &mut Vec<ChallengeOption>) {
    let types: HashSet<_> = challenges.iter().map(|c| &c.challenge_type).collect();

    // If all 3 are same type, regenerate 2 of them
    if types.len() == 1 {
        challenges[1] = generate_different_challenge(&challenges[0].challenge_type);
        challenges[2] = generate_different_challenge(&challenges[0].challenge_type);
    }

    // If 2 use same hero, regenerate 1
    let hero_ids: Vec<_> = challenges.iter().filter_map(|c| c.hero_id).collect();
    if hero_ids.len() > 1 && hero_ids[0] == hero_ids[1] {
        challenges[2] = generate_non_hero_challenge();
    }
}
```

### Daily Challenge Time Calculations
```rust
fn get_today_date() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn get_seconds_until_midnight() -> i64 {
    let now = chrono::Local::now();
    let tomorrow = (now + chrono::Duration::days(1)).date_naive();
    let midnight = tomorrow.and_hms_opt(0, 0, 0).unwrap();
    let midnight_time = chrono::Local.from_local_datetime(&midnight).unwrap();
    (midnight_time.timestamp() - now.timestamp())
}
```

### Daily Challenge Edge Cases

**8. No Matches Played Today**
- **Scenario**: Daily challenge active but player hasn't played
- **Behavior**: Challenge remains at 0% progress, auto-fails at midnight

**9. Challenge Completed in First Match**
- **Scenario**: Player achieves daily challenge in first game of the day
- **Behavior**: Immediately mark as completed, show celebration UI
- **UI**: Show "✅ Daily Complete!" with time completed

**10. Multiple Matches Fetched at Once**
- **Scenario**: Player fetches 5 matches, 2nd match completes daily challenge
- **Behavior**: Evaluate in chronological order, mark complete on first qualifying match

**11. Same Challenge Type Multiple Days**
- **Scenario**: Random generation creates similar challenge 2 days in a row
- **Behavior**: Algorithm should check last 7 days and avoid repeating same type consecutively

**12. Daily Challenge at Midnight**
- **Scenario**: Player is actively playing when midnight hits
- **Behavior**: Current challenge fails, new one generates. Matches after midnight count toward new challenge.

## UX Enhancements

### Notifications

**Weekly Challenge Notifications:**
- Push notification when weekly challenge is completed
- Reminder on Saturday evening if weekly challenge not yet accepted
- Alert when new weekly challenges are available (Sunday)

**Daily Challenge Notifications:**
- Push notification when daily challenge is completed
- Notification at 8 PM if daily challenge not yet completed (gentle reminder)
- Alert at midnight: "New daily challenge available!"

### Streaks

**Daily Streak Tracking:**
- Track consecutive days with completed daily challenges
- Display "🔥 7 day streak!" on dashboard
- Show streak on daily challenge card
- Reset streak if day is missed (failed or no matches)
- Special milestones: 3, 7, 14, 30, 60, 100 days

**Weekly Streak Tracking (Future Enhancement):**
- Track consecutive weeks with completed weekly challenges
- Display "🏆 5 week streak!" badge
- Bonus rewards for long weekly streaks

### Social Features (Future Enhancement)
- Share completed challenges on Discord/social media
- Leaderboard for challenge completion rate among friends
- Compare challenge progress with other players

### Rewards (Future Enhancement)
- XP/Points system for completed challenges
- Unlock special badges/titles
- In-app cosmetic rewards (avatar frames, etc.)

## Implementation Priority
**High** - Highly engaging feature that adds both short-term (daily) and long-term (weekly) goals. Provides variety and daily engagement beyond the goal system. More complex due to dual challenge types, selection, rerolls, and time-based tracking.

## Estimated Effort
**Very Large** - 18-24 hours of development work

### Breakdown:
- Database schema & migration: 2 hours (weekly + daily tables)
- Weekly challenge generation algorithm: 3 hours
- Daily challenge generation algorithm: 2 hours
- Week/day period calculations & filtering: 2.5 hours
- Reroll system (weekly only): 1.5 hours
- Progress tracking & evaluation (both types): 3 hours
- Frontend challenge selection UI (weekly): 3 hours
- Dashboard widgets (weekly + daily): 2 hours
- Challenge history page (combined view): 2 hours
- Streak tracking system: 1.5 hours
- Midnight/Sunday rollover logic: 1.5 hours
- Testing & edge cases: 3 hours

## Dependencies
- Existing match data system
- Hero statistics (similar to goal suggestions)
- Week/date calculation utilities
- Goal evaluation logic (can be adapted)

## Acceptance Criteria

### Weekly Challenges
- [ ] System generates 3 diverse challenge options each Sunday
- [ ] Player can view all 3 options before choosing
- [ ] Player can reroll up to 2 times per week
- [ ] Player can accept one challenge, which becomes active immediately
- [ ] Only matches after acceptance time count toward weekly progress
- [ ] Only matches within the week period (Sunday-Saturday) count
- [ ] Weekly progress updates in real-time as matches are fetched
- [ ] Weekly challenge auto-completes when target is reached
- [ ] Incomplete weekly challenges auto-fail on Sunday rollover
- [ ] Player can skip the week without penalty
- [ ] Dashboard widget shows active weekly challenge progress

### Daily Challenges
- [ ] System generates 1 daily challenge automatically at midnight
- [ ] Daily challenge is auto-active (no acceptance needed)
- [ ] Only matches played after midnight count toward daily progress
- [ ] Daily progress updates in real-time as matches are fetched
- [ ] Daily challenge auto-completes when target is reached
- [ ] Incomplete daily challenges auto-fail at midnight rollover
- [ ] Daily streak counter tracks consecutive completed days
- [ ] Dashboard widget shows active daily challenge progress
- [ ] Daily challenges avoid repeating same type consecutively

### Combined Features
- [ ] Challenge history displays both weekly and daily results
- [ ] History page has filter tabs (Weekly/Daily/All)
- [ ] Edge cases handled gracefully (no matches, rollover timing, etc.)
- [ ] Both challenge types can be active simultaneously
- [ ] Completing challenges shows celebration UI

## Future Enhancements (Out of Scope)

1. **Challenge Difficulty Selection** - Let players choose Easy/Medium/Hard difficulty before seeing weekly options
2. **Custom Challenges** - Allow players to create their own custom weekly challenges
3. **Team Challenges** - Multiplayer challenges where party members work toward shared goal
4. **Seasonal Events** - Special limited-time challenges during Dota 2 events/patches
5. **Achievement System** - Meta-achievements for challenge completion streaks, variety, etc.
6. **Challenge Rewards Shop** - Spend points earned from challenges on cosmetic rewards
7. **Daily Challenge Reroll** - Allow 1 reroll per day for daily challenges (if too restrictive)
8. **Challenge Notifications** - In-app and system notifications for challenge events
