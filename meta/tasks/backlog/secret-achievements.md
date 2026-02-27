# Secret Achievements

## Overview

A set of hidden, humour-driven achievements that are automatically detected from match data. Players have no idea they exist until they trigger one. They appear in a dedicated panel as `???` until unlocked, at which point they reveal their name and a witty description.

This is distinct from the goal system (which is intentional and user-configured) and the one-off challenges system (which shows progress upfront). Secret achievements are pure discovery — no tracking, no hints, just a surprise when something unusual happens in a game.

## User Story

As a player, I want to occasionally be surprised by a secret achievement unlocking after a weird or exceptional game, so I get moments of delight and have a reason to look back at unusual performances.

## Design Principles

- **Hidden by default**: All achievements show as `???` until unlocked
- **No progress indicators**: Players cannot see how close they are
- **Instant unlock**: Evaluated after each match sync, triggered as soon as the condition is met
- **One-time only**: Each achievement unlocks once, permanently
- **Witty tone**: Names and descriptions should be dry or self-aware, not generic

## Achievement List

### Performance Memes

| Internal Key | Unlock Name | Description | Condition |
|---|---|---|---|
| `died_a_lot_won` | "It Was Worth It" | You died 15 times and somehow won. | deaths >= 15, win = true |
| `deathless_win` | "Ghost" | You won without dying once. Suspicious. | deaths = 0, win = true |
| `zero_kills_win` | "True Support" | Won the game with 0 kills. You are invisible. | kills = 0, win = true |
| `carry_the_team` | "Whole Squad" | More kills than your entire team combined. | kills > sum of teammate kills |
| `speed_run` | "Was That It?" | Won in under 22 minutes. | duration < 1320, win = true |
| `marathon` | "Are We Still Playing?" | Survived a game lasting over 70 minutes. | duration > 4200 |
| `gpm_god` | "Money Printer" | Hit 800+ GPM in a single game. | gold_per_min >= 800 |
| `last_hit_king` | "Denying Everything" | 700+ last hits in a game. | last_hits >= 700 |
| `mega_comeback` | "Didn't Even Break a Sweat" | Won after the enemy destroyed your barracks. | `barracks_status` bits indicate enemy destroyed radiant barracks, win = true |

### Hero Memes

| Internal Key | Unlock Name | Description | Condition |
|---|---|---|---|
| `pudge_games_10` | "This Is Fine" | Played 10 Pudge games. | hero_id = 14, game count >= 10 |
| `invoker_games_10` | "Actually Playing Invoker" | Completed 10 Invoker games without abandoning. | hero_id = 74, game count >= 10, abandon = false |
| `techies_win` | "Everyone Hates You" | Won a game as Techies. | hero_id = 105, win = true |
| `sniper_win_5` | "Stay Back" | Won 5 games as Sniper. | hero_id = 35, win count >= 5 |

### Variety & Chaos

| Internal Key | Unlock Name | Description | Condition |
|---|---|---|---|
| `hero_collector_30` | "Decision Paralysis" | Played 30 unique heroes. | distinct hero_ids >= 30 |
| `lost_five_row` | "Research Phase" | Lost 5 games in a row. | 5 consecutive losses |
| `won_five_row` | "Solved It" | Won 5 games in a row. | 5 consecutive wins |
| `no_kills_no_deaths` | "Spectator Mode" | Finished a game with 0 kills and 0 deaths. | kills = 0, deaths = 0 |
| `high_assists` | "Team Player" | Got 25+ assists in a single game. | assists >= 25 |
| `triple_roles` | "Switching It Up" | Played carry, mid, and support in the same week. | detected roles across matches in 7-day window |

## UI

### Achievements Panel (new page or tab)

```
Secret Achievements

Unlocked: 3 / ??

[Unlocked]

🏅 Ghost
   You won without dying once. Suspicious.
   Unlocked Feb 19, 2026

🏅 It Was Worth It
   You died 15 times and somehow won.
   Unlocked Feb 22, 2026

🏅 Money Printer
   Hit 800+ GPM in a single game.
   Unlocked Feb 25, 2026

[Locked]

???  ???  ???  ???  ???  ???
???  ???  ???  ???  ???  ???
```

- Locked achievements show as greyed-out `???` tiles — count visible so players know more exist, exact number shown
- No category labels for locked ones (that would be a hint)
- Unlocked achievements show icon, name, description, unlock date, and a share button

### Share Button

Each unlocked achievement has a "Share on X" button. Clicking it opens a pre-filled tweet in the browser:

```
Unlocked a secret achievement in Dota Keeper: "Ghost"
You won without dying once. Suspicious.

Match: https://www.opendota.com/matches/{match_id}
Track your Dota 2 games: https://dotakeeper.app
```

- Opens via `tauri::api::shell::open()` with a `twitter.com/intent/tweet?text=...` URL (URL-encoded)
- Only shown for achievements that have a `match_id` (single-game triggers)
- For cumulative achievements (e.g. hero count, win streak) that have no single triggering match, omit the OpenDota link:

```
Unlocked a secret achievement in Dota Keeper: "Decision Paralysis"
Played 30 unique heroes. You need help.

Track your Dota 2 games: https://dotakeeper.app
```

### Unlock Notification

When an achievement unlocks during a match sync, show a toast with a share shortcut:

```
🔓 Secret Achievement Unlocked
   Ghost — You won without dying once. Suspicious.
   [Share on X]
```

## Backend Implementation

### Database

```sql
CREATE TABLE secret_achievements (
    key TEXT PRIMARY KEY,           -- e.g. "deathless_win"
    unlocked_at INTEGER,            -- NULL until unlocked
    match_id INTEGER                -- match that triggered the unlock
);
```

Pre-populate all rows with `unlocked_at = NULL` on first run.

### Evaluation

After each match sync, run `evaluate_secret_achievements(match: &Match, conn: &Connection)`:
- Check each locked achievement's condition against the latest match (and cumulative stats where needed)
- If condition met, set `unlocked_at` and `match_id`
- Return list of newly unlocked achievements to surface in the UI notification

Cumulative conditions (win streak, hero count, etc.) query the full match history, not just the latest match.

### Tauri Commands

```rust
#[tauri::command]
fn get_secret_achievements() -> Result<Vec<SecretAchievement>, String>
// Returns all rows; frontend hides name/description where unlocked_at is NULL

#[tauri::command]
fn share_achievement_on_twitter(key: String) -> Result<(), String>
// Builds tweet text from achievement metadata + match_id, opens browser via shell::open()
```

## Relation to One-Off Challenges

These are intentionally simpler and separate from the `one-off-challenges-with-xp.md` task:
- No XP, no levels, no progression tracking
- Fully hardcoded — no dynamic generation
- The appeal is pure surprise, not a reward loop

If the XP challenge system is implemented first, secret achievements could optionally award a small flat XP bonus on unlock, but this is not required.

## Acceptance Criteria

- [ ] All achievements pre-seeded in DB as locked on first run
- [ ] Evaluation runs after each match sync
- [ ] Correct unlock conditions for each achievement
- [ ] Unlock notification appears when triggered
- [ ] Achievements panel shows unlocked count and `???` tiles for locked ones
- [ ] Unlocked achievements show name, description, and date
- [ ] No hints given for locked achievements
- [ ] Share button appears on each unlocked achievement
- [ ] Clicking share opens a pre-filled tweet in the browser
- [ ] Tweet includes achievement name, description, and OpenDota match link (where applicable)
- [ ] Tweet includes link to dotakeeper.app
- [ ] Unlock toast notification also has a "Share on X" shortcut
