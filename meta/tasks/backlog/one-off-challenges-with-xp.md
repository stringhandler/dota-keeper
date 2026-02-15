# One-Off Challenges with Experience System

## Overview
An achievement-like system that automatically generates single-use challenges for players. Once a challenge is achieved, it's marked complete and awards experience points (XP). Unlike ongoing goals, these challenges are completed once and never reset.

## User Story
As a player, I want to complete automatically-generated one-time challenges and earn experience points, so that I have a sense of progression and achievement as I improve at Dota 2.

## Requirements

### Challenge Characteristics
- **One-Time Only**: Each challenge can only be completed once
- **Auto-Generated**: Challenges are created by the system based on player statistics
- **Experience Rewards**: Completing challenges awards XP
- **Progressive Difficulty**: Challenges get harder as player improves
- **Multiple Categories**: Different types of challenges (hero mastery, performance milestones, variety, etc.)

### Challenge Types

#### 1. Hero Mastery Challenges
- "Play your first game with [Hero]" - 50 XP
- "Win your first game with [Hero]" - 100 XP
- "Win 5 games with [Hero]" - 200 XP
- "Win 10 games with [Hero]" - 500 XP
- "Achieve 60+ CS at 10 min on [Hero]" - 300 XP
- "Get a Rampage with [Hero]" - 1000 XP

#### 2. Performance Milestones
- "Achieve your first double kill" - 100 XP
- "Achieve your first triple kill" - 250 XP
- "Get 10+ kills in a game" - 150 XP
- "Get 15+ kills in a game" - 300 XP
- "Get 20+ kills in a game" - 500 XP
- "Die less than 3 times in a 40+ minute game" - 200 XP
- "Achieve 600+ GPM in a game" - 400 XP
- "Achieve 800+ XPM in a game" - 400 XP

#### 3. Economy Challenges
- "Reach 100 CS in a game" - 150 XP
- "Reach 200 CS in a game" - 300 XP
- "Reach 300 CS in a game" - 500 XP
- "Achieve 70+ CS at 10 minutes" - 400 XP
- "Deny 20+ creeps in a game" - 200 XP

#### 4. Variety Challenges
- "Play 5 different heroes" - 150 XP
- "Play 10 different heroes" - 300 XP
- "Play 25 different heroes" - 750 XP
- "Play 50 different heroes" - 1500 XP
- "Win with 5 different heroes" - 400 XP
- "Win with 10 different heroes" - 800 XP

#### 5. Consistency Challenges
- "Win 3 games in a row" - 300 XP
- "Win 5 games in a row" - 700 XP
- "Play 10 games" - 100 XP
- "Play 50 games" - 500 XP
- "Play 100 games" - 1200 XP

#### 6. Comeback Challenges
- "Win a game after losing both Tier 3 towers" - 500 XP
- "Win a game lasting 60+ minutes" - 400 XP
- "Get a kill streak of 5+" - 300 XP

### Challenge Generation Engine

**Initial Challenge Set:**
- When player first uses the app, generate a starter set of ~20-30 achievable challenges
- Mix of easy, medium, and hard difficulties
- Includes basic milestones (first win, first 10 games, etc.)

**Dynamic Challenge Generation:**
- Monitors player progress and generates new challenges based on:
  - Heroes they're playing (generate hero-specific challenges)
  - Performance trends (if averaging 8 kills/game, generate "Get 12+ kills" challenge)
  - Play patterns (if playing mostly one role, suggest variety challenges)
- Generates 5-10 new challenges per week based on activity

**Difficulty Scaling:**
- **Easy (50-150 XP)**: Basic milestones, first-time achievements
- **Medium (200-400 XP)**: Solid performance, moderate skill requirements
- **Hard (500-800 XP)**: Exceptional performance, rare events
- **Epic (1000+ XP)**: Extremely difficult, prestige achievements

### Experience System

**XP Accumulation:**
- Total XP accumulates over time
- Never resets
- Used to calculate player level

**Level Calculation:**
```rust
// Example formula: Level = floor(sqrt(total_xp / 100))
// 100 XP = Level 1
// 400 XP = Level 2
// 900 XP = Level 3
// 1600 XP = Level 4
// 2500 XP = Level 5
// etc.
```

**Level Milestones:**
- Display level on dashboard
- Show progress to next level
- Unlock cosmetic rewards (future enhancement)

### Challenge States
- **Locked**: Challenge requirements not yet met (e.g., "Win 10 games with Hero X" when player has only 3 games)
- **Available**: Player can complete this challenge right now
- **In Progress**: Challenge has partial progress (e.g., 3/5 wins)
- **Completed**: Challenge achieved, XP awarded
- **Hidden**: Future challenges not yet revealed

### Completion Detection
- Automatically evaluate all active/available challenges after each match is parsed
- When challenge is completed:
  - Mark as completed with timestamp
  - Award XP to player profile
  - Recalculate player level
  - Show completion notification
  - Generate new related challenges (e.g., completing "Win 5 with Hero X" unlocks "Win 10 with Hero X")

## Database Schema

### New Table: `player_profile`
```sql
CREATE TABLE player_profile (
    id INTEGER PRIMARY KEY DEFAULT 1,
    total_xp INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    CHECK (id = 1)  -- Only one profile row
)
```

### New Table: `challenges`
```sql
CREATE TABLE challenges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,  -- "hero_mastery", "performance", "economy", "variety", "consistency", "comeback"
    difficulty TEXT NOT NULL,  -- "easy", "medium", "hard", "epic"
    xp_reward INTEGER NOT NULL,

    -- Challenge criteria
    metric TEXT NOT NULL,  -- "wins", "kills", "last_hits", "games_played", "hero_pool", etc.
    target_value INTEGER NOT NULL,
    hero_id INTEGER,  -- NULL for non-hero-specific challenges

    -- State
    status TEXT NOT NULL DEFAULT 'available',  -- "locked", "available", "in_progress", "completed", "hidden"
    progress_current INTEGER DEFAULT 0,
    completed_at INTEGER,  -- NULL until completed

    -- Unlocking logic
    requires_challenge_id INTEGER,  -- NULL or ID of challenge that must be completed first
    requires_games_played INTEGER,  -- NULL or minimum games required

    -- Metadata
    generated_at INTEGER NOT NULL,
    is_generated BOOLEAN DEFAULT 0,  -- 1 if auto-generated, 0 if predefined

    FOREIGN KEY (requires_challenge_id) REFERENCES challenges(id)
)
```

### New Table: `challenge_completions`
```sql
CREATE TABLE challenge_completions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    challenge_id INTEGER NOT NULL,
    completed_at INTEGER NOT NULL,
    xp_awarded INTEGER NOT NULL,
    match_id INTEGER,  -- The match that completed the challenge (nullable)

    FOREIGN KEY (challenge_id) REFERENCES challenges(id)
)
```

## Frontend Components

### Dashboard Widget
```
┌─────────────────────────────────────┐
│ 🏅 Level 7 Keeper                   │
│ 2,850 / 3,600 XP to Level 8        │
│ ████████████░░░░ 79%                │
│                                     │
│ Recent Challenges:                  │
│ ✅ Win 5 games with Anti-Mage       │
│    +200 XP                          │
│ ⚡ Win 10 games (8/10)               │
│ ⚡ Achieve 600+ GPM (0/1)            │
│                                     │
│ [View All Challenges]               │
└─────────────────────────────────────┘
```

### Challenges Page (`/challenges`)
```
Challenges

[All] [Available] [Completed] [In Progress]  ← Filter tabs

Level 7 Keeper
2,850 / 3,600 XP (79%) ████████████░░░░

=== Available Challenges ===

🎯 Hero Mastery
  ⚡ Win 10 games with Anti-Mage (8/10)
     +500 XP | Medium

  ⚡ Achieve 60+ CS at 10 min on Phantom Assassin (0/1)
     +300 XP | Medium

🏆 Performance Milestones
  ⚡ Get 15+ kills in a game (0/1)
     +300 XP | Medium

  ⚡ Achieve 800+ XPM in a game (0/1)
     +400 XP | Hard

🌟 Variety
  ⚡ Play 25 different heroes (18/25)
     +750 XP | Hard

=== Completed (23) ===

✅ Win your first game with Anti-Mage
   Completed Feb 5, 2026 | +100 XP

✅ Win 5 games with Anti-Mage
   Completed Feb 12, 2026 | +200 XP

✅ Achieve your first triple kill
   Completed Feb 8, 2026 | +250 XP

[Show More]
```

### Challenge Completion Notification
```
┌─────────────────────────────────────┐
│           🎉 Challenge Complete! 🎉  │
│                                     │
│  Win 10 games with Anti-Mage        │
│            +500 XP                  │
│                                     │
│  Level 7 → Progress: 3,350 XP       │
│  550 XP to Level 8                  │
│                                     │
│  [Awesome!]                         │
└─────────────────────────────────────┘
```

## Backend Implementation

### Core Functions (database.rs)

1. **`init_player_profile() -> PlayerProfile`**
   - Creates player profile if doesn't exist
   - Returns current profile data

2. **`get_player_profile() -> PlayerProfile`**
   - Returns current XP and level

3. **`award_xp(amount: i32) -> PlayerProfile`**
   - Adds XP to player profile
   - Recalculates level
   - Returns updated profile

4. **`calculate_level(xp: i32) -> i32`**
   - Formula: `floor(sqrt(xp / 100))`
   - Returns current level based on total XP

5. **`generate_initial_challenges() -> Vec<Challenge>`**
   - Called on first run
   - Creates predefined starter challenges
   - Returns challenge list

6. **`generate_dynamic_challenges() -> Vec<Challenge>`**
   - Analyzes player stats
   - Generates hero-specific challenges for recently played heroes
   - Generates performance challenges based on current skill level
   - Returns new challenges

7. **`get_all_challenges(filter: Option<String>) -> Vec<Challenge>`**
   - Returns challenges based on filter ("available", "completed", "in_progress", or None for all)

8. **`evaluate_challenges() -> Vec<ChallengeCompletion>`**
   - Called after each match is fetched/parsed
   - Evaluates all active challenges
   - Awards XP for completed challenges
   - Returns list of newly completed challenges

9. **`update_challenge_progress(challenge_id: i64) -> Challenge`**
   - Recalculates current progress for a challenge
   - Updates `progress_current` field
   - Updates `status` if completed

10. **`complete_challenge(challenge_id: i64, match_id: Option<i64>) -> ChallengeCompletion`**
    - Marks challenge as completed
    - Awards XP
    - Unlocks dependent challenges
    - Returns completion record

### Tauri Commands

```rust
#[tauri::command]
fn get_player_profile() -> Result<PlayerProfile, String>

#[tauri::command]
fn get_challenges(filter: Option<String>) -> Result<Vec<Challenge>, String>

#[tauri::command]
fn get_challenge_progress(challenge_id: i64) -> Result<Challenge, String>

#[tauri::command]
fn trigger_challenge_evaluation() -> Result<Vec<ChallengeCompletion>, String>

#[tauri::command]
fn get_recent_completions(limit: i32) -> Result<Vec<ChallengeCompletion>, String>
```

## Challenge Evaluation Logic

```rust
pub fn evaluate_challenge(conn: &Connection, challenge: &Challenge) -> Option<i32> {
    let current_value = match challenge.metric.as_str() {
        "wins" => {
            if let Some(hero_id) = challenge.hero_id {
                // Count wins with specific hero
                count_hero_wins(conn, hero_id)
            } else {
                // Count total wins
                count_total_wins(conn)
            }
        }
        "games_played" => {
            if let Some(hero_id) = challenge.hero_id {
                count_hero_games(conn, hero_id)
            } else {
                count_total_games(conn)
            }
        }
        "max_kills" => {
            // Get max kills in any single game
            get_max_kills(conn, challenge.hero_id)
        }
        "max_cs" => {
            get_max_last_hits(conn, challenge.hero_id)
        }
        "hero_pool" => {
            // Count unique heroes played
            count_unique_heroes(conn)
        }
        "win_streak" => {
            // Calculate current/max win streak
            get_max_win_streak(conn)
        }
        // ... more metrics
        _ => return None,
    };

    Some(current_value)
}
```

## Edge Cases

### 1. Challenge Completed by Backfilled Match
- **Scenario**: Player backfills old matches, one completes a challenge
- **Behavior**: Award XP normally, mark as completed, show notification

### 2. Multiple Challenges Completed in One Match
- **Scenario**: Single match completes 3 different challenges
- **Behavior**: Award XP for all, show combined notification with total XP

### 3. Challenge Already Locked When Generated
- **Scenario**: Auto-generation creates challenge player can't achieve yet
- **Behavior**: Set status to "locked", will unlock when requirements met

### 4. Hero Challenge for Unplayed Hero
- **Scenario**: Challenge generated for hero player has never played
- **Behavior**: Keep as "available" - it's an invitation to try new heroes

### 5. Level Up From Challenge Completion
- **Scenario**: Completing challenge provides enough XP to level up
- **Behavior**: Show both challenge completion AND level up notifications

## Technical Considerations

### XP to Level Formula
```rust
fn calculate_level(total_xp: i32) -> i32 {
    // Level = floor(sqrt(total_xp / 100))
    // This creates a nice curve where each level requires more XP
    ((total_xp as f64 / 100.0).sqrt()).floor() as i32
}

fn xp_for_level(level: i32) -> i32 {
    // Inverse: XP = level² * 100
    level * level * 100
}

fn xp_to_next_level(current_xp: i32) -> i32 {
    let current_level = calculate_level(current_xp);
    let next_level_xp = xp_for_level(current_level + 1);
    next_level_xp - current_xp
}
```

### Challenge Dependency Tree
```rust
// Example: Anti-Mage win progression
fn generate_hero_win_progression(hero_id: i32) -> Vec<Challenge> {
    vec![
        Challenge {
            title: "First Win with Anti-Mage",
            target_value: 1,
            xp_reward: 100,
            requires_challenge_id: None,
        },
        Challenge {
            title: "Win 5 with Anti-Mage",
            target_value: 5,
            xp_reward: 200,
            requires_challenge_id: Some(previous_challenge_id),  // Unlocks after first win
        },
        Challenge {
            title: "Win 10 with Anti-Mage",
            target_value: 10,
            xp_reward: 500,
            requires_challenge_id: Some(win_5_challenge_id),
        },
    ]
}
```

### Predefined vs Generated Challenges
- **Predefined**: Created at initialization, same for all players
  - "Play your first game"
  - "Win your first game"
  - "Play 10 games"
  - Universal milestones

- **Generated**: Created dynamically based on player activity
  - Hero-specific challenges (when player starts using a hero)
  - Performance targets (based on current skill level)
  - Variety challenges (based on hero pool)

## UX Enhancements

### Progress Indicators
- Show progress bars for in-progress challenges
- Highlight challenges close to completion (e.g., 9/10 wins)
- Badge indicators for new challenges

### Notifications
- Toast notification when challenge is completed
- Summary notification if multiple challenges completed
- Level-up celebration when gaining a level

### Sorting and Filtering
- Sort by: XP reward, progress, difficulty, category
- Filter by: Available, In Progress, Completed, Locked
- Search by challenge title/description

### Level Rewards (Future Enhancement)
- Unlock special titles at certain levels
- Unlock cosmetic features (avatar frames, badges)
- Unlock advanced features at higher levels

## Implementation Priority
**Medium-High** - Adds a progression system that gives long-term engagement beyond daily/weekly goals. Creates a sense of achievement and provides auto-generated objectives.

## Estimated Effort
**Large** - 12-16 hours of development work

### Breakdown:
- Database schema & migration: 2 hours
- Player profile & XP system: 2 hours
- Predefined challenge generation: 2 hours
- Dynamic challenge generation algorithm: 3 hours
- Challenge evaluation logic: 2 hours
- Frontend challenges page: 3 hours
- Dashboard widget: 1.5 hours
- Notification system: 1.5 hours
- Testing & edge cases: 2 hours

## Dependencies
- Existing match data system
- Parse state (challenges should only evaluate on parsed matches)
- Hero statistics
- Goal evaluation logic (similar patterns)

## Acceptance Criteria
- [ ] Player profile created with XP and level tracking
- [ ] Initial set of predefined challenges generated on first run
- [ ] Challenges displayed on dedicated page with filters
- [ ] Challenges automatically evaluated after each match
- [ ] XP awarded and level updated when challenges completed
- [ ] Challenge progress shown on dashboard widget
- [ ] Completion notifications displayed
- [ ] Challenge categories properly organized
- [ ] Locked challenges unlock when requirements met
- [ ] Dynamic challenges generated based on player activity
- [ ] Level calculation formula works correctly
- [ ] Edge cases handled (multiple completions, backfilled matches, etc.)

## Future Enhancements (Out of Scope)
1. **Leaderboards** - Compare levels/XP with friends
2. **Prestige System** - Reset level for bonus rewards
3. **Challenge Creator** - Let players create custom challenges
4. **Social Sharing** - Share challenge completions
5. **Achievement Badges** - Visual badges for completed challenges
6. **Challenge Resets** - Optional weekly/monthly challenge resets for recurring XP
7. **XP Multipliers** - Bonus XP events or streaks
8. **Challenge Hints** - Tips on how to complete difficult challenges
