use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use rand::Rng;

/// Goal metric types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GoalMetric {
    Networth,
    Kills,
    LastHits,
    Denies,
    Level,
    ItemTiming,
    PartnerNetworth,
}

impl GoalMetric {
    fn to_string(&self) -> &'static str {
        match self {
            GoalMetric::Networth => "networth",
            GoalMetric::Kills => "kills",
            GoalMetric::LastHits => "last_hits",
            GoalMetric::Denies => "denies",
            GoalMetric::Level => "level",
            GoalMetric::ItemTiming => "item_timing",
            GoalMetric::PartnerNetworth => "partner_networth",
        }
    }

    fn from_string(s: &str) -> Option<Self> {
        match s {
            "networth" => Some(GoalMetric::Networth),
            "kills" => Some(GoalMetric::Kills),
            "last_hits" => Some(GoalMetric::LastHits),
            "denies" => Some(GoalMetric::Denies),
            "level" => Some(GoalMetric::Level),
            "item_timing" => Some(GoalMetric::ItemTiming),
            "partner_networth" => Some(GoalMetric::PartnerNetworth),
            _ => None,
        }
    }
}

/// Game mode for goals (Ranked or Turbo)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GoalGameMode {
    Ranked,
    Turbo,
}

impl GoalGameMode {
    fn to_string(&self) -> &'static str {
        match self {
            GoalGameMode::Ranked => "ranked",
            GoalGameMode::Turbo => "turbo",
        }
    }

    fn from_string(s: &str) -> Option<Self> {
        match s {
            "ranked" => Some(GoalGameMode::Ranked),
            "turbo" => Some(GoalGameMode::Turbo),
            _ => None,
        }
    }
}

/// Represents a performance goal
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Goal {
    pub id: i64,
    pub hero_id: Option<i32>,  // None when hero_scope is set or "any hero"
    pub hero_scope: Option<String>,  // "any_core", "any_carry", "any_support", or None
    pub metric: GoalMetric,
    pub target_value: i32,  // For ItemTiming: target time in seconds
    pub target_time_minutes: i32,  // Not used for ItemTiming goals
    pub item_id: Option<i32>,  // Only used for ItemTiming goals
    pub game_mode: GoalGameMode,
    pub created_at: i64,
}

/// Input for creating a new goal (without id and created_at)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewGoal {
    pub hero_id: Option<i32>,
    pub hero_scope: Option<String>,  // "any_core", "any_carry", "any_support", or None
    pub metric: GoalMetric,
    pub target_value: i32,
    pub target_time_minutes: i32,
    pub item_id: Option<i32>,  // Only used for ItemTiming goals
    pub game_mode: GoalGameMode,
}

/// Hero goal suggestion (weekly personalized goal)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeroGoalSuggestion {
    pub hero_id: i32,
    pub suggested_last_hits: i32,
    pub current_average: f64,
    pub created_at: i64,
    pub games_analyzed: i32,
}

/// Item timing data for a match
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemTiming {
    pub id: i64,
    pub match_id: i64,
    pub item_id: i32,
    pub timing_seconds: i32,
}

/// Input for inserting item timing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewItemTiming {
    pub match_id: i64,
    pub item_id: i32,
    pub timing_seconds: i32,
}

/// Match processing state
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MatchState {
    Unparsed,
    Parsing,
    Parsed,
    Failed,
}

impl MatchState {
    fn to_string(&self) -> &'static str {
        match self {
            MatchState::Unparsed => "unparsed",
            MatchState::Parsing => "parsing",
            MatchState::Parsed => "parsed",
            MatchState::Failed => "failed",
        }
    }

    fn from_string(s: &str) -> Self {
        match s {
            "parsing" => MatchState::Parsing,
            "parsed" => MatchState::Parsed,
            "failed" => MatchState::Failed,
            _ => MatchState::Unparsed,
        }
    }
}

/// Represents a Dota 2 match from OpenDota API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Match {
    pub match_id: i64,
    pub hero_id: i32,
    pub start_time: i64,
    pub duration: i32,
    pub game_mode: i32,
    pub lobby_type: i32,
    pub radiant_win: bool,
    pub player_slot: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub xp_per_min: i32,
    pub gold_per_min: i32,
    pub last_hits: i32,
    pub denies: i32,
    pub hero_damage: i32,
    pub tower_damage: i32,
    pub hero_healing: i32,
    pub parse_state: MatchState,
    pub role: i32,  // 0=unknown, 1=carry, 2=mid, 3=offlane, 4=soft support, 5=hard support
}

impl Match {
    /// Determine if the player won this match
    pub fn is_win(&self) -> bool {
        let is_radiant = self.player_slot < 128;
        (is_radiant && self.radiant_win) || (!is_radiant && !self.radiant_win)
    }
}

/// Get the path to the database file in the AppData directory.
/// Dev builds use `dota_keeper_dev.db`; release builds use `dota_keeper.db`.
fn get_db_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|mut path| {
        path.push("DotaKeeper");

        #[cfg(debug_assertions)]
        path.push("dota_keeper_dev.db");

        #[cfg(not(debug_assertions))]
        path.push("dota_keeper.db");

        path
    })
}

/// Initialize the database, creating tables if they don't exist
pub fn init_db() -> Result<Connection, String> {
    let path = get_db_path().ok_or_else(|| "Could not determine database directory".to_string())?;

    // Create the directory if it doesn't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create database directory: {}", e))?;
    }

    let conn = Connection::open(&path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Create the matches table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS matches (
            match_id INTEGER PRIMARY KEY,
            hero_id INTEGER NOT NULL,
            start_time INTEGER NOT NULL,
            duration INTEGER NOT NULL,
            game_mode INTEGER NOT NULL,
            lobby_type INTEGER NOT NULL,
            radiant_win INTEGER NOT NULL,
            player_slot INTEGER NOT NULL,
            kills INTEGER NOT NULL,
            deaths INTEGER NOT NULL,
            assists INTEGER NOT NULL,
            xp_per_min INTEGER NOT NULL,
            gold_per_min INTEGER NOT NULL,
            last_hits INTEGER NOT NULL,
            denies INTEGER NOT NULL,
            hero_damage INTEGER NOT NULL,
            tower_damage INTEGER NOT NULL,
            hero_healing INTEGER NOT NULL,
            parse_state TEXT NOT NULL DEFAULT 'unparsed',
            role INTEGER NOT NULL DEFAULT 0
        )",
        [],
    ).map_err(|e| format!("Failed to create matches table: {}", e))?;

    // Cleanup: Reset any "parsing" matches to "unparsed" (in case app crashed during parsing)
    conn.execute(
        "UPDATE matches SET parse_state = 'unparsed' WHERE parse_state = 'parsing'",
        [],
    ).map_err(|e| format!("Failed to cleanup parsing state: {}", e))?;

    // Add parse_state column if it doesn't exist (for existing databases)
    let _ = conn.execute(
        "ALTER TABLE matches ADD COLUMN parse_state TEXT NOT NULL DEFAULT 'unparsed'",
        [],
    );

    // Add role column if it doesn't exist (for existing databases)
    let _ = conn.execute(
        "ALTER TABLE matches ADD COLUMN role INTEGER NOT NULL DEFAULT 0",
        [],
    );

    // Create the goal_progress table for storing time-based metrics
    conn.execute(
        "CREATE TABLE IF NOT EXISTS goal_progress (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            match_id INTEGER NOT NULL,
            time_minutes INTEGER NOT NULL,
            last_hits INTEGER NOT NULL,
            denies INTEGER NOT NULL,
            FOREIGN KEY (match_id) REFERENCES matches(match_id),
            UNIQUE(match_id, time_minutes)
        )",
        [],
    ).map_err(|e| format!("Failed to create goal_progress table: {}", e))?;

    // Create the match_cs table for storing per-minute CS data
    conn.execute(
        "CREATE TABLE IF NOT EXISTS match_cs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            match_id INTEGER NOT NULL,
            minute INTEGER NOT NULL,
            last_hits INTEGER NOT NULL,
            denies INTEGER NOT NULL,
            FOREIGN KEY (match_id) REFERENCES matches(match_id),
            UNIQUE(match_id, minute)
        )",
        [],
    ).map_err(|e| format!("Failed to create match_cs table: {}", e))?;

    // Create the goals table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS goals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            hero_id INTEGER,
            metric TEXT NOT NULL,
            target_value INTEGER NOT NULL,
            target_time_minutes INTEGER NOT NULL,
            game_mode TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )",
        [],
    ).map_err(|e| format!("Failed to create goals table: {}", e))?;

    // Add item_id column if it doesn't exist (for item timing goals)
    let _ = conn.execute(
        "ALTER TABLE goals ADD COLUMN item_id INTEGER",
        [],
    );

    // Add hero_scope column if it doesn't exist (for role-group goals)
    let _ = conn.execute(
        "ALTER TABLE goals ADD COLUMN hero_scope TEXT",
        [],
    );

    // Create the hero_favorites table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS hero_favorites (
            hero_id INTEGER PRIMARY KEY
        )",
        [],
    ).map_err(|e| format!("Failed to create hero_favorites table: {}", e))?;

    // Create the hero_goal_suggestions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS hero_goal_suggestions (
            id INTEGER PRIMARY KEY,
            hero_id INTEGER NOT NULL,
            suggested_last_hits INTEGER NOT NULL,
            current_average REAL NOT NULL,
            created_at INTEGER NOT NULL,
            games_analyzed INTEGER NOT NULL
        )",
        [],
    ).map_err(|e| format!("Failed to create hero_goal_suggestions table: {}", e))?;

    // Create the player_networth table (per-minute networth for all players, used for PartnerNetworth goals)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS player_networth (
            match_id INTEGER NOT NULL,
            player_slot INTEGER NOT NULL,
            minute INTEGER NOT NULL,
            networth INTEGER NOT NULL,
            PRIMARY KEY (match_id, player_slot, minute),
            FOREIGN KEY (match_id) REFERENCES matches(match_id)
        )",
        [],
    ).map_err(|e| format!("Failed to create player_networth table: {}", e))?;

    // Add partner_slot column if it doesn't exist (set during parsing for support players)
    let _ = conn.execute(
        "ALTER TABLE matches ADD COLUMN partner_slot INTEGER",
        [],
    );

    // Create the item_timings table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_timings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            match_id INTEGER NOT NULL,
            item_id INTEGER NOT NULL,
            timing_seconds INTEGER NOT NULL,
            FOREIGN KEY (match_id) REFERENCES matches(match_id),
            UNIQUE(match_id, item_id)
        )",
        [],
    ).map_err(|e| format!("Failed to create item_timings table: {}", e))?;

    // Create the daily_challenges table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS daily_challenges (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            challenge_date TEXT NOT NULL,
            challenge_type TEXT NOT NULL,
            challenge_description TEXT NOT NULL,
            challenge_target INTEGER NOT NULL,
            challenge_target_games INTEGER NOT NULL DEFAULT 1,
            hero_id INTEGER,
            metric TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'active',
            created_at INTEGER NOT NULL,
            completed_at INTEGER,
            UNIQUE(challenge_date)
        )",
        [],
    ).map_err(|e| format!("Failed to create daily_challenges table: {}", e))?;

    // Create the challenge_history table (shared between daily and weekly)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS challenge_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            challenge_type TEXT NOT NULL,
            period_start_date TEXT NOT NULL,
            challenge_description TEXT NOT NULL,
            status TEXT NOT NULL,
            completed_at INTEGER,
            target_achieved INTEGER
        )",
        [],
    ).map_err(|e| format!("Failed to create challenge_history table: {}", e))?;

    // Create the weekly challenge options table (3 cards shown to user)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS challenge_options (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            week_start_date TEXT NOT NULL,
            challenge_type TEXT NOT NULL,
            challenge_description TEXT NOT NULL,
            challenge_target INTEGER NOT NULL,
            challenge_target_games INTEGER,
            hero_id INTEGER,
            metric TEXT NOT NULL,
            option_index INTEGER NOT NULL,
            reroll_generation INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL
        )",
        [],
    ).map_err(|e| format!("Failed to create challenge_options table: {}", e))?;

    // Create the accepted weekly challenges table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS weekly_challenges (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            week_start_date TEXT NOT NULL UNIQUE,
            challenge_type TEXT NOT NULL,
            challenge_description TEXT NOT NULL,
            challenge_target INTEGER NOT NULL,
            challenge_target_games INTEGER,
            hero_id INTEGER,
            metric TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'active',
            accepted_at INTEGER,
            completed_at INTEGER,
            reroll_count INTEGER NOT NULL DEFAULT 0
        )",
        [],
    ).map_err(|e| format!("Failed to create weekly_challenges table: {}", e))?;

    Ok(conn)
}

/// Check if a match already exists in the database
pub fn match_exists(conn: &Connection, match_id: i64) -> Result<bool, String> {
    let count: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM matches WHERE match_id = ?1",
            params![match_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to check if match exists: {}", e))?;

    Ok(count > 0)
}

/// Insert a match into the database
pub fn insert_match(conn: &Connection, m: &Match) -> Result<(), String> {
    conn.execute(
        "INSERT OR IGNORE INTO matches (
            match_id, hero_id, start_time, duration, game_mode, lobby_type,
            radiant_win, player_slot, kills, deaths, assists, xp_per_min,
            gold_per_min, last_hits, denies, hero_damage, tower_damage, hero_healing, parse_state, role
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
        params![
            m.match_id,
            m.hero_id,
            m.start_time,
            m.duration,
            m.game_mode,
            m.lobby_type,
            m.radiant_win as i32,
            m.player_slot,
            m.kills,
            m.deaths,
            m.assists,
            m.xp_per_min,
            m.gold_per_min,
            m.last_hits,
            m.denies,
            m.hero_damage,
            m.tower_damage,
            m.hero_healing,
            m.parse_state.to_string(),
            m.role,
        ],
    ).map_err(|e| format!("Failed to insert match: {}", e))?;

    Ok(())
}

/// Get the oldest match timestamp from the database
pub fn get_oldest_match_timestamp(conn: &Connection) -> Result<Option<i64>, String> {
    let result = conn.query_row(
        "SELECT MIN(start_time) FROM matches",
        [],
        |row| row.get::<_, Option<i64>>(0),
    );

    match result {
        Ok(timestamp) => Ok(timestamp),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("Failed to get oldest match timestamp: {}", e)),
    }
}

/// Get all unparsed or failed matches from the database
pub fn get_unparsed_matches(conn: &Connection) -> Result<Vec<Match>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT match_id, hero_id, start_time, duration, game_mode, lobby_type,
                    radiant_win, player_slot, kills, deaths, assists, xp_per_min,
                    gold_per_min, last_hits, denies, hero_damage, tower_damage, hero_healing, parse_state, role
             FROM matches
             WHERE parse_state = 'unparsed' OR parse_state = 'failed'
             ORDER BY start_time DESC",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let matches = stmt
        .query_map([], |row| {
            let parse_state_str: String = row.get(18).unwrap_or_else(|_| "unparsed".to_string());
            Ok(Match {
                match_id: row.get(0)?,
                hero_id: row.get(1)?,
                start_time: row.get(2)?,
                duration: row.get(3)?,
                game_mode: row.get(4)?,
                lobby_type: row.get(5)?,
                radiant_win: row.get::<_, i32>(6)? != 0,
                player_slot: row.get(7)?,
                kills: row.get(8)?,
                deaths: row.get(9)?,
                assists: row.get(10)?,
                xp_per_min: row.get(11)?,
                gold_per_min: row.get(12)?,
                last_hits: row.get(13)?,
                denies: row.get(14)?,
                hero_damage: row.get(15)?,
                tower_damage: row.get(16)?,
                hero_healing: row.get(17)?,
                parse_state: MatchState::from_string(&parse_state_str),
                role: row.get(19).unwrap_or(0),
            })
        })
        .map_err(|e| format!("Failed to query matches: {}", e))?;

    let mut result = Vec::new();
    for m in matches {
        result.push(m.map_err(|e| format!("Failed to read match: {}", e))?);
    }

    Ok(result)
}

/// Clear all matches and related data from the database
pub fn clear_all_matches(conn: &Connection) -> Result<(), String> {
    // Delete all match CS data first (foreign key constraint)
    conn.execute("DELETE FROM match_cs", [])
        .map_err(|e| format!("Failed to delete match CS data: {}", e))?;

    // Delete all matches
    conn.execute("DELETE FROM matches", [])
        .map_err(|e| format!("Failed to delete matches: {}", e))?;

    Ok(())
}

/// Get all matches from the database, ordered by start_time descending
pub fn get_all_matches(conn: &Connection) -> Result<Vec<Match>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT match_id, hero_id, start_time, duration, game_mode, lobby_type,
                    radiant_win, player_slot, kills, deaths, assists, xp_per_min,
                    gold_per_min, last_hits, denies, hero_damage, tower_damage, hero_healing, parse_state, role
             FROM matches ORDER BY start_time DESC",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let matches = stmt
        .query_map([], |row| {
            let parse_state_str: String = row.get(18).unwrap_or_else(|_| "unparsed".to_string());
            Ok(Match {
                match_id: row.get(0)?,
                hero_id: row.get(1)?,
                start_time: row.get(2)?,
                duration: row.get(3)?,
                game_mode: row.get(4)?,
                lobby_type: row.get(5)?,
                radiant_win: row.get::<_, i32>(6)? != 0,
                player_slot: row.get(7)?,
                kills: row.get(8)?,
                deaths: row.get(9)?,
                assists: row.get(10)?,
                xp_per_min: row.get(11)?,
                gold_per_min: row.get(12)?,
                last_hits: row.get(13)?,
                denies: row.get(14)?,
                hero_damage: row.get(15)?,
                tower_damage: row.get(16)?,
                hero_healing: row.get(17)?,
                parse_state: MatchState::from_string(&parse_state_str),
                role: row.get(19).unwrap_or(0),
            })
        })
        .map_err(|e| format!("Failed to query matches: {}", e))?;

    let mut result = Vec::new();
    for m in matches {
        result.push(m.map_err(|e| format!("Failed to read match: {}", e))?);
    }

    Ok(result)
}

/// Insert a new goal into the database
pub fn insert_goal(conn: &Connection, goal: &NewGoal) -> Result<Goal, String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Failed to get current time: {}", e))?
        .as_secs() as i64;

    conn.execute(
        "INSERT INTO goals (hero_id, metric, target_value, target_time_minutes, game_mode, item_id, created_at, hero_scope)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            goal.hero_id,
            goal.metric.to_string(),
            goal.target_value,
            goal.target_time_minutes,
            goal.game_mode.to_string(),
            goal.item_id,
            now,
            goal.hero_scope,
        ],
    ).map_err(|e| format!("Failed to insert goal: {}", e))?;

    let id = conn.last_insert_rowid();

    Ok(Goal {
        id,
        hero_id: goal.hero_id,
        hero_scope: goal.hero_scope.clone(),
        metric: goal.metric.clone(),
        target_value: goal.target_value,
        target_time_minutes: goal.target_time_minutes,
        item_id: goal.item_id,
        game_mode: goal.game_mode.clone(),
        created_at: now,
    })
}

/// Get all goals from the database
pub fn get_all_goals(conn: &Connection) -> Result<Vec<Goal>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, hero_id, metric, target_value, target_time_minutes, game_mode, created_at, item_id, hero_scope
             FROM goals ORDER BY created_at DESC",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let goals = stmt
        .query_map([], |row| {
            let metric_str: String = row.get(2)?;
            let game_mode_str: String = row.get(5)?;
            Ok(Goal {
                id: row.get(0)?,
                hero_id: row.get(1)?,
                metric: GoalMetric::from_string(&metric_str).unwrap_or(GoalMetric::Networth),
                target_value: row.get(3)?,
                target_time_minutes: row.get(4)?,
                game_mode: GoalGameMode::from_string(&game_mode_str).unwrap_or(GoalGameMode::Ranked),
                created_at: row.get(6)?,
                item_id: row.get(7)?,
                hero_scope: row.get(8).unwrap_or(None),
            })
        })
        .map_err(|e| format!("Failed to query goals: {}", e))?;

    let mut result = Vec::new();
    for g in goals {
        result.push(g.map_err(|e| format!("Failed to read goal: {}", e))?);
    }

    Ok(result)
}

/// Update an existing goal
pub fn update_goal(conn: &Connection, goal: &Goal) -> Result<(), String> {
    conn.execute(
        "UPDATE goals SET hero_id = ?1, metric = ?2, target_value = ?3,
         target_time_minutes = ?4, game_mode = ?5, item_id = ?6, hero_scope = ?7 WHERE id = ?8",
        params![
            goal.hero_id,
            goal.metric.to_string(),
            goal.target_value,
            goal.target_time_minutes,
            goal.game_mode.to_string(),
            goal.item_id,
            goal.hero_scope,
            goal.id,
        ],
    ).map_err(|e| format!("Failed to update goal: {}", e))?;

    Ok(())
}

/// Delete a goal by ID
pub fn delete_goal(conn: &Connection, goal_id: i64) -> Result<(), String> {
    conn.execute(
        "DELETE FROM goals WHERE id = ?1",
        params![goal_id],
    ).map_err(|e| format!("Failed to delete goal: {}", e))?;

    Ok(())
}

/// Result of evaluating a goal against a match
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoalEvaluation {
    pub goal: Goal,
    pub achieved: bool,
    pub actual_value: i32,
}

/// Match with goal evaluation summary
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchWithGoals {
    #[serde(flatten)]
    pub match_data: Match,
    pub goals_achieved: i32,
    pub goals_applicable: i32,
}

/// Evaluate a single goal against a match
pub fn evaluate_goal(conn: &Connection, goal: &Goal, match_data: &Match) -> Option<GoalEvaluation> {
    // Skip goal evaluation for unparsed matches - data might be incomplete
    if match_data.parse_state == MatchState::Unparsed {
        return None;
    }

    // Check if goal applies to this match based on hero / hero scope
    let hero_matches = match goal.hero_scope.as_deref() {
        Some("any_carry")  => match_data.role == 1,
        Some("any_core")   => matches!(match_data.role, 1 | 2 | 3),
        Some("any_support")=> matches!(match_data.role, 4 | 5),
        _ => match goal.hero_id {
            Some(id) => id == match_data.hero_id,
            None => true, // any hero
        },
    };
    if !hero_matches {
        return None;
    }

    // Check if goal applies based on game mode
    // Game mode values: https://github.com/odota/dotaconstants/blob/master/build/game_mode.json
    // 22 = All Pick Ranked, 23 = Turbo
    let is_ranked = match_data.game_mode == 22;
    let is_turbo = match_data.game_mode == 23;

    let mode_matches = match goal.game_mode {
        GoalGameMode::Ranked => is_ranked,
        GoalGameMode::Turbo => is_turbo,
    };

    if !mode_matches {
        return None; // Goal doesn't apply to this game mode
    }

    // Calculate actual value at target time
    let duration_minutes = match_data.duration / 60;
    let target_minutes = goal.target_time_minutes;

    let actual_value = match &goal.metric {
        GoalMetric::Kills => {
            // For kills, we assume linear progression
            if duration_minutes <= target_minutes {
                match_data.kills
            } else {
                // Estimate kills at target time
                ((match_data.kills as f32 / duration_minutes as f32) * target_minutes as f32) as i32
            }
        }
        GoalMetric::LastHits => {
            // ONLY use exact per-minute CS data from OpenDota - never estimate
            // Linear estimation (total_cs / game_time * target_time) is completely inaccurate
            // because CS progression is not linear (it's faster early game)

            // Get exact CS data at the target minute from match_cs table
            if let Ok(Some(cs_data)) = get_match_cs_at_minute(conn, match_data.match_id, target_minutes) {
                cs_data.last_hits
            } else {
                // No parsed per-minute data available - cannot evaluate this goal
                // This goal won't be counted for this match
                return None;
            }
        }
        GoalMetric::Denies => {
            if let Ok(Some(cs_data)) = get_match_cs_at_minute(conn, match_data.match_id, target_minutes) {
                cs_data.denies
            } else {
                return None;
            }
        }
        GoalMetric::PartnerNetworth => {
            let partner_slot = match get_partner_slot(conn, match_data.match_id) {
                Ok(Some(s)) => s,
                _ => return None,
            };
            match get_partner_networth_at_minute(conn, match_data.match_id, partner_slot, target_minutes) {
                Ok(Some(nw)) => nw,
                _ => return None,
            }
        }
        GoalMetric::ItemTiming => {
            // For item timing goals, check when the item was purchased
            // goal.target_value contains the target time in seconds
            // goal.item_id contains the item ID to check

            let item_id = goal.item_id?; // If no item_id, can't evaluate

            // Get the actual purchase time for this item in this match
            match get_item_timing(conn, match_data.match_id, item_id) {
                Ok(Some(timing_seconds)) => timing_seconds,
                Ok(None) => {
                    // Item was never purchased in this match
                    // Don't penalize - skip this match for this goal
                    return None;
                }
                Err(_) => {
                    // Error querying item timing
                    return None;
                }
            }
        }
        GoalMetric::Networth | GoalMetric::Level => {
            // These require per-minute data which we don't have in the current schema
            // For now, we'll need to skip these or mark them as not evaluable
            // Return None to indicate we can't evaluate this goal with current data
            return None;
        }
    };

    let achieved = match &goal.metric {
        GoalMetric::ItemTiming => {
            // For item timing, actual_value is purchase time in seconds
            // target_value is target time in seconds
            // Achieved if purchased before or at target time
            actual_value <= goal.target_value
        }
        _ => {
            // For other metrics, achieved if actual >= target
            actual_value >= goal.target_value
        }
    };

    Some(GoalEvaluation {
        goal: goal.clone(),
        achieved,
        actual_value,
    })
}

/// Evaluate all goals against a match
pub fn evaluate_match_goals(conn: &Connection, match_data: &Match) -> Result<Vec<GoalEvaluation>, String> {
    let goals = get_all_goals(conn)?;

    let evaluations: Vec<GoalEvaluation> = goals
        .iter()
        .filter_map(|goal| evaluate_goal(conn, goal, match_data))
        .collect();

    Ok(evaluations)
}

/// Get all matches with goal evaluation summaries
pub fn get_matches_with_goals(conn: &Connection) -> Result<Vec<MatchWithGoals>, String> {
    let matches = get_all_matches(conn)?;
    let goals = get_all_goals(conn)?;

    let matches_with_goals: Vec<MatchWithGoals> = matches
        .into_iter()
        .map(|match_data| {
            let evaluations: Vec<GoalEvaluation> = goals
                .iter()
                .filter_map(|goal| evaluate_goal(conn, goal, &match_data))
                .collect();

            let goals_applicable = evaluations.len() as i32;
            let goals_achieved = evaluations.iter().filter(|e| e.achieved).count() as i32;

            MatchWithGoals {
                match_data,
                goals_achieved,
                goals_applicable,
            }
        })
        .collect();

    Ok(matches_with_goals)
}

/// Goal progress at a specific time
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoalProgress {
    pub match_id: i64,
    pub time_minutes: i32,
    pub last_hits: i32,
    pub denies: i32,
}

/// Insert goal progress data for a match
pub fn insert_goal_progress(conn: &Connection, progress: &GoalProgress) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO goal_progress (match_id, time_minutes, last_hits, denies)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            progress.match_id,
            progress.time_minutes,
            progress.last_hits,
            progress.denies,
        ],
    ).map_err(|e| format!("Failed to insert goal progress: {}", e))?;

    Ok(())
}

/// Get goal progress for a specific match
pub fn get_goal_progress(conn: &Connection, match_id: i64) -> Result<Vec<GoalProgress>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT match_id, time_minutes, last_hits, denies
             FROM goal_progress
             WHERE match_id = ?1
             ORDER BY time_minutes ASC",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let progress_items = stmt
        .query_map(params![match_id], |row| {
            Ok(GoalProgress {
                match_id: row.get(0)?,
                time_minutes: row.get(1)?,
                last_hits: row.get(2)?,
                denies: row.get(3)?,
            })
        })
        .map_err(|e| format!("Failed to query goal progress: {}", e))?;

    let mut result = Vec::new();
    for item in progress_items {
        result.push(item.map_err(|e| format!("Failed to read goal progress: {}", e))?);
    }

    Ok(result)
}

/// Update match parse state
pub fn update_match_state(conn: &Connection, match_id: i64, state: MatchState) -> Result<(), String> {
    conn.execute(
        "UPDATE matches SET parse_state = ?1 WHERE match_id = ?2",
        params![state.to_string(), match_id],
    ).map_err(|e| format!("Failed to update match state: {}", e))?;

    Ok(())
}

/// Update match role (lane position 1-5, 0 = unknown)
pub fn update_match_role(conn: &Connection, match_id: i64, role: i32) -> Result<(), String> {
    conn.execute(
        "UPDATE matches SET role = ?1 WHERE match_id = ?2",
        params![role, match_id],
    ).map_err(|e| format!("Failed to update match role: {}", e))?;

    Ok(())
}

/// Store the lane partner's player_slot for a match (None if no partner / not a support)
pub fn update_match_partner_slot(conn: &Connection, match_id: i64, partner_slot: Option<i32>) -> Result<(), String> {
    conn.execute(
        "UPDATE matches SET partner_slot = ?1 WHERE match_id = ?2",
        params![partner_slot, match_id],
    ).map_err(|e| format!("Failed to update partner slot: {}", e))?;

    Ok(())
}

/// Get the stored lane partner player_slot for a match
pub fn get_partner_slot(conn: &Connection, match_id: i64) -> Result<Option<i32>, String> {
    conn.query_row(
        "SELECT partner_slot FROM matches WHERE match_id = ?1",
        params![match_id],
        |row| row.get(0),
    ).map_err(|e| format!("Failed to query partner slot: {}", e))
}

/// Bulk insert per-minute networth for a single player in a match
pub fn insert_player_networth(conn: &Connection, match_id: i64, player_slot: i32, nw_t: &[i32]) -> Result<(), String> {
    // Delete existing data for this player in this match first
    conn.execute(
        "DELETE FROM player_networth WHERE match_id = ?1 AND player_slot = ?2",
        params![match_id, player_slot],
    ).map_err(|e| format!("Failed to delete existing networth data: {}", e))?;

    for (minute, &networth) in nw_t.iter().enumerate() {
        conn.execute(
            "INSERT INTO player_networth (match_id, player_slot, minute, networth) VALUES (?1, ?2, ?3, ?4)",
            params![match_id, player_slot, minute as i32, networth],
        ).map_err(|e| format!("Failed to insert player networth: {}", e))?;
    }

    Ok(())
}

/// Get a player's networth at a specific minute
pub fn get_partner_networth_at_minute(conn: &Connection, match_id: i64, player_slot: i32, minute: i32) -> Result<Option<i32>, String> {
    match conn.query_row(
        "SELECT networth FROM player_networth WHERE match_id = ?1 AND player_slot = ?2 AND minute = ?3",
        params![match_id, player_slot, minute],
        |row| row.get(0),
    ) {
        Ok(v) => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("Failed to query partner networth: {}", e)),
    }
}

/// Match CS data at a specific minute
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchCS {
    pub match_id: i64,
    pub minute: i32,
    pub last_hits: i32,
    pub denies: i32,
}

/// Insert CS data for all minutes in a match
pub fn insert_match_cs_data(conn: &Connection, match_id: i64, lh_t: &[i32], dn_t: &[i32]) -> Result<(), String> {
    // Delete existing data for this match first
    conn.execute(
        "DELETE FROM match_cs WHERE match_id = ?1",
        params![match_id],
    ).map_err(|e| format!("Failed to delete existing CS data: {}", e))?;

    // Insert data for each minute
    // OpenDota's lh_t array uses 0-based indexing:
    // - lh_t[10] contains the CS at the 10-minute mark
    // - We store the array index directly as the minute value in the database
    // - So for a "10 minute" goal, we store minute=10 which maps to lh_t[10]
    let max_len = lh_t.len().min(dn_t.len());

    for i in 0..max_len {
        let minute = i as i32; // Store array index as minute (lh_t[10] -> minute=10)
        conn.execute(
            "INSERT INTO match_cs (match_id, minute, last_hits, denies) VALUES (?1, ?2, ?3, ?4)",
            params![match_id, minute, lh_t[i], dn_t[i]],
        ).map_err(|e| format!("Failed to insert CS data: {}", e))?;
    }

    Ok(())
}

/// Get CS data for a match at a specific minute
pub fn get_match_cs_at_minute(conn: &Connection, match_id: i64, minute: i32) -> Result<Option<MatchCS>, String> {
    let result = conn.query_row(
        "SELECT match_id, minute, last_hits, denies FROM match_cs WHERE match_id = ?1 AND minute = ?2",
        params![match_id, minute],
        |row| {
            Ok(MatchCS {
                match_id: row.get(0)?,
                minute: row.get(1)?,
                last_hits: row.get(2)?,
                denies: row.get(3)?,
            })
        },
    );

    match result {
        Ok(cs) => Ok(Some(cs)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("Failed to query CS data: {}", e)),
    }
}

/// Get all CS data for a match ordered by minute
pub fn get_match_cs_data(conn: &Connection, match_id: i64) -> Result<Vec<MatchCS>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT match_id, minute, last_hits, denies FROM match_cs WHERE match_id = ?1 ORDER BY minute ASC",
        )
        .map_err(|e| format!("Failed to prepare CS data query: {}", e))?;

    let rows = stmt
        .query_map(params![match_id], |row| {
            Ok(MatchCS {
                match_id: row.get(0)?,
                minute: row.get(1)?,
                last_hits: row.get(2)?,
                denies: row.get(3)?,
            })
        })
        .map_err(|e| format!("Failed to query CS data: {}", e))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect CS data: {}", e))
}

/// Daily goal progress data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DayGoalProgress {
    pub date: String, // YYYY-MM-DD format
    pub achieved: i32,
    pub total: i32,
}

/// Goal with daily progress
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoalWithDailyProgress {
    pub goal: Goal,
    pub daily_progress: Vec<DayGoalProgress>,
}

/// Get goal progress by day for the last N days
pub fn get_goals_with_daily_progress(conn: &Connection, days: i32) -> Result<Vec<GoalWithDailyProgress>, String> {
    let goals = get_all_goals(conn)?;
    let matches = get_all_matches(conn)?;

    // Get current timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Calculate the start of today (midnight UTC)
    let today_start = (now / 86400) * 86400;

    let mut result = Vec::new();

    for goal in goals {
        let mut daily_progress = Vec::new();

        // Generate data for each of the last N days
        for day_offset in (0..days).rev() {
            let day_start = today_start - (day_offset as i64 * 86400);
            let day_end = day_start + 86400;

            // Format date as YYYY-MM-DD
            let date = format_unix_timestamp_as_date(day_start);

            // Filter matches for this day
            let day_matches: Vec<&Match> = matches.iter()
                .filter(|m| m.start_time >= day_start && m.start_time < day_end)
                .collect();

            // Count applicable matches and achievements
            let mut total = 0;
            let mut achieved = 0;

            for match_data in day_matches {
                if let Some(evaluation) = evaluate_goal(conn, &goal, match_data) {
                    total += 1;
                    if evaluation.achieved {
                        achieved += 1;
                    }
                }
            }

            daily_progress.push(DayGoalProgress {
                date,
                achieved,
                total,
            });
        }

        result.push(GoalWithDailyProgress {
            goal,
            daily_progress,
        });
    }

    Ok(result)
}

/// Format Unix timestamp as YYYY-MM-DD
fn format_unix_timestamp_as_date(timestamp: i64) -> String {
    use chrono::{DateTime, Utc, Datelike};
    let dt: DateTime<Utc> = DateTime::from_timestamp(timestamp, 0).unwrap();
    format!("{:04}-{:02}-{:02}", dt.year(), dt.month(), dt.day())
}

/// Match data point for histogram
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchDataPoint {
    pub match_id: i64,
    pub hero_id: i32,
    pub start_time: i64,
    pub value: i32,
    pub achieved: bool,
}

/// Get match data for a specific goal (for histogram visualization)
pub fn get_goal_match_data(conn: &Connection, goal_id: i64) -> Result<Vec<MatchDataPoint>, String> {
    // Get the goal
    let mut stmt = conn
        .prepare("SELECT id, hero_id, metric, target_value, target_time_minutes, game_mode, created_at, item_id, hero_scope FROM goals WHERE id = ?1")
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let goal = stmt
        .query_row(params![goal_id], |row| {
            let metric_str: String = row.get(2)?;
            let game_mode_str: String = row.get(5)?;
            Ok(Goal {
                id: row.get(0)?,
                hero_id: row.get(1)?,
                metric: GoalMetric::from_string(&metric_str).unwrap_or(GoalMetric::Networth),
                target_value: row.get(3)?,
                target_time_minutes: row.get(4)?,
                game_mode: GoalGameMode::from_string(&game_mode_str).unwrap_or(GoalGameMode::Ranked),
                created_at: row.get(6)?,
                item_id: row.get(7)?,
                hero_scope: row.get(8).unwrap_or(None),
            })
        })
        .map_err(|e| format!("Failed to get goal: {}", e))?;

    // Get all matches
    let matches = get_all_matches(conn)?;

    // Evaluate goal against each match and collect data points
    let mut data_points = Vec::new();

    for match_data in matches {
        if let Some(evaluation) = evaluate_goal(conn, &goal, &match_data) {
            data_points.push(MatchDataPoint {
                match_id: match_data.match_id,
                hero_id: match_data.hero_id,
                start_time: match_data.start_time,
                value: evaluation.actual_value,
                achieved: evaluation.achieved,
            });
        }
    }

    Ok(data_points)
}

/// Get a single goal by ID
pub fn get_goal_by_id(conn: &Connection, goal_id: i64) -> Result<Goal, String> {
    let mut stmt = conn
        .prepare("SELECT id, hero_id, metric, target_value, target_time_minutes, game_mode, created_at, item_id, hero_scope FROM goals WHERE id = ?1")
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    stmt.query_row(params![goal_id], |row| {
        let metric_str: String = row.get(2)?;
        let game_mode_str: String = row.get(5)?;
        Ok(Goal {
            id: row.get(0)?,
            hero_id: row.get(1)?,
            metric: GoalMetric::from_string(&metric_str).unwrap_or(GoalMetric::Networth),
            target_value: row.get(3)?,
            target_time_minutes: row.get(4)?,
            game_mode: GoalGameMode::from_string(&game_mode_str).unwrap_or(GoalGameMode::Ranked),
            created_at: row.get(6)?,
            item_id: row.get(7)?,
            hero_scope: row.get(8).unwrap_or(None),
        })
    })
    .map_err(|e| format!("Failed to get goal: {}", e))
}

/// Last hits analysis data point
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastHitsDataPoint {
    pub match_id: i64,
    pub hero_id: i32,
    pub start_time: i64,
    pub last_hits: i32,
    pub game_mode: i32,
}

/// Last hits analysis result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastHitsAnalysis {
    pub current_period: LastHitsPeriodStats,
    pub previous_period: Option<LastHitsPeriodStats>,
    pub per_hero_stats: Vec<HeroLastHitsStats>,
}

/// Statistics for a period of games
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastHitsPeriodStats {
    pub average: f64,
    pub min: i32,
    pub max: i32,
    pub count: usize,
    pub data_points: Vec<LastHitsDataPoint>,
}

/// Per-hero last hits statistics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeroLastHitsStats {
    pub hero_id: i32,
    pub average: f64,
    pub count: usize,
    pub trend_percentage: f64, // Positive = improving, Negative = declining
}

/// Get last hits analysis with filtering
pub fn get_last_hits_analysis(
    conn: &Connection,
    time_minutes: i32,
    window_size: usize,
    hero_id_filter: Option<i32>,
    game_mode_filter: Option<i32>,
) -> Result<LastHitsAnalysis, String> {
    // Build the query with filters
    let mut query = String::from(
        "SELECT m.match_id, m.hero_id, m.start_time, m.game_mode, mc.last_hits
         FROM matches m
         LEFT JOIN match_cs mc ON m.match_id = mc.match_id AND mc.minute = ?1
         WHERE m.parse_state = 'parsed' AND mc.last_hits IS NOT NULL"
    );

    let mut param_count = 1;
    if hero_id_filter.is_some() {
        param_count += 1;
        query.push_str(&format!(" AND m.hero_id = ?{}", param_count));
    }

    if game_mode_filter.is_some() {
        param_count += 1;
        query.push_str(&format!(" AND m.game_mode = ?{}", param_count));
    }

    query.push_str(" ORDER BY m.start_time DESC");

    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    // Build params vector
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(time_minutes)];
    if let Some(hero) = hero_id_filter {
        params_vec.push(Box::new(hero));
    }
    if let Some(mode) = game_mode_filter {
        params_vec.push(Box::new(mode));
    }

    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

    let rows = stmt
        .query_map(&params_refs[..], |row| {
            Ok(LastHitsDataPoint {
                match_id: row.get(0)?,
                hero_id: row.get(1)?,
                start_time: row.get(2)?,
                game_mode: row.get(3)?,
                last_hits: row.get(4)?,
            })
        })
        .map_err(|e| format!("Failed to query matches: {}", e))?;

    let mut data_points = Vec::new();
    for row in rows {
        data_points.push(row.map_err(|e| format!("Failed to read row: {}", e))?);
    }

    // Calculate current period stats (last N games)
    let current_data: Vec<_> = data_points.iter().take(window_size).cloned().collect();
    let current_period = calculate_period_stats(&current_data);

    // Calculate previous period stats (next N games) if we have enough data
    let previous_period = if data_points.len() >= window_size * 2 {
        let previous_data: Vec<_> = data_points
            .iter()
            .skip(window_size)
            .take(window_size)
            .cloned()
            .collect();
        Some(calculate_period_stats(&previous_data))
    } else {
        None
    };

    // Calculate per-hero stats (only for current period, and only if no hero filter)
    let per_hero_stats = if hero_id_filter.is_none() && !current_data.is_empty() {
        calculate_per_hero_stats(&current_data, &data_points, window_size)
    } else {
        Vec::new()
    };

    Ok(LastHitsAnalysis {
        current_period,
        previous_period,
        per_hero_stats,
    })
}

/// Calculate statistics for a period of games
fn calculate_period_stats(data_points: &[LastHitsDataPoint]) -> LastHitsPeriodStats {
    if data_points.is_empty() {
        return LastHitsPeriodStats {
            average: 0.0,
            min: 0,
            max: 0,
            count: 0,
            data_points: Vec::new(),
        };
    }

    let last_hits: Vec<i32> = data_points.iter().map(|d| d.last_hits).collect();
    let sum: i32 = last_hits.iter().sum();
    let average = sum as f64 / last_hits.len() as f64;
    let min = *last_hits.iter().min().unwrap_or(&0);
    let max = *last_hits.iter().max().unwrap_or(&0);

    // Reverse data points so charts display oldest to newest (left to right)
    let mut reversed_points = data_points.to_vec();
    reversed_points.reverse();

    LastHitsPeriodStats {
        average,
        min,
        max,
        count: data_points.len(),
        data_points: reversed_points,
    }
}

/// Calculate per-hero statistics
fn calculate_per_hero_stats(
    _current_data: &[LastHitsDataPoint],
    all_data: &[LastHitsDataPoint],
    window_size: usize,
) -> Vec<HeroLastHitsStats> {
    use std::collections::HashMap;

    // Group ALL data by hero (not just current period)
    let mut hero_data: HashMap<i32, Vec<&LastHitsDataPoint>> = HashMap::new();
    for point in all_data {
        hero_data
            .entry(point.hero_id)
            .or_insert_with(Vec::new)
            .push(point);
    }

    // Calculate stats for each hero using their own independent window
    let mut stats: Vec<HeroLastHitsStats> = hero_data
        .into_iter()
        .filter_map(|(hero_id, hero_points)| {
            // For this hero, take the last N games (window_size)
            let current_games: Vec<i32> = hero_points
                .iter()
                .take(window_size)
                .map(|p| p.last_hits)
                .collect();

            if current_games.is_empty() {
                return None;
            }

            let sum: i32 = current_games.iter().sum();
            let average = sum as f64 / current_games.len() as f64;
            let count = current_games.len();

            // Calculate trend by comparing to previous N games for THIS hero
            let previous_games: Vec<i32> = hero_points
                .iter()
                .skip(window_size)
                .take(window_size)
                .map(|p| p.last_hits)
                .collect();

            let trend_percentage = if !previous_games.is_empty() {
                let current_avg = average;
                let previous_avg = previous_games.iter().sum::<i32>() as f64 / previous_games.len() as f64;

                if previous_avg == 0.0 {
                    0.0
                } else {
                    ((current_avg - previous_avg) / previous_avg) * 100.0
                }
            } else {
                0.0
            };

            Some(HeroLastHitsStats {
                hero_id,
                average,
                count,
                trend_percentage,
            })
        })
        .collect();

    // Sort by average (descending)
    stats.sort_by(|a, b| b.average.partial_cmp(&a.average).unwrap_or(std::cmp::Ordering::Equal));

    stats
}

/// Calculate trend percentage for a specific hero
fn calculate_hero_trend(hero_id: i32, all_data: &[LastHitsDataPoint], window_size: usize) -> f64 {
    // Get current period data for this hero
    let current: Vec<i32> = all_data
        .iter()
        .take(window_size)
        .filter(|d| d.hero_id == hero_id)
        .map(|d| d.last_hits)
        .collect();

    // Get previous period data for this hero
    let previous: Vec<i32> = all_data
        .iter()
        .skip(window_size)
        .take(window_size)
        .filter(|d| d.hero_id == hero_id)
        .map(|d| d.last_hits)
        .collect();

    if current.is_empty() || previous.is_empty() {
        return 0.0;
    }

    let current_avg = current.iter().sum::<i32>() as f64 / current.len() as f64;
    let previous_avg = previous.iter().sum::<i32>() as f64 / previous.len() as f64;

    if previous_avg == 0.0 {
        return 0.0;
    }

    ((current_avg - previous_avg) / previous_avg) * 100.0
}

/// Toggle hero favorite status
pub fn toggle_hero_favorite(conn: &Connection, hero_id: i32) -> Result<bool, String> {
    // Check if hero is currently favorited
    let is_favorite = is_hero_favorite(conn, hero_id)?;

    if is_favorite {
        // Remove from favorites
        conn.execute(
            "DELETE FROM hero_favorites WHERE hero_id = ?1",
            params![hero_id],
        ).map_err(|e| format!("Failed to remove hero from favorites: {}", e))?;
        Ok(false)
    } else {
        // Add to favorites
        conn.execute(
            "INSERT INTO hero_favorites (hero_id) VALUES (?1)",
            params![hero_id],
        ).map_err(|e| format!("Failed to add hero to favorites: {}", e))?;
        Ok(true)
    }
}

/// Check if a hero is favorited
pub fn is_hero_favorite(conn: &Connection, hero_id: i32) -> Result<bool, String> {
    let count: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM hero_favorites WHERE hero_id = ?1",
            params![hero_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to check if hero is favorite: {}", e))?;

    Ok(count > 0)
}

/// Get all favorite hero IDs
pub fn get_favorite_hero_ids(conn: &Connection) -> Result<Vec<i32>, String> {
    let mut stmt = conn
        .prepare("SELECT hero_id FROM hero_favorites")
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let hero_ids = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| format!("Failed to query favorite heroes: {}", e))?;

    let mut result = Vec::new();
    for id in hero_ids {
        result.push(id.map_err(|e| format!("Failed to read hero ID: {}", e))?);
    }

    Ok(result)
}

/// Get current hero goal suggestion if it exists and is less than 7 days old
pub fn get_current_hero_suggestion(conn: &Connection) -> Result<Option<HeroGoalSuggestion>, String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let result = conn.query_row(
        "SELECT hero_id, suggested_last_hits, current_average, created_at, games_analyzed
         FROM hero_goal_suggestions
         WHERE id = 1",
        [],
        |row| {
            Ok(HeroGoalSuggestion {
                hero_id: row.get(0)?,
                suggested_last_hits: row.get(1)?,
                current_average: row.get(2)?,
                created_at: row.get(3)?,
                games_analyzed: row.get(4)?,
            })
        },
    );

    match result {
        Ok(suggestion) => {
            // Check if suggestion is less than 7 days old
            let age_seconds = now - suggestion.created_at;
            let seven_days_seconds = 7 * 24 * 60 * 60;

            if age_seconds < seven_days_seconds {
                Ok(Some(suggestion))
            } else {
                Ok(None)
            }
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("Failed to query hero suggestion: {}", e)),
    }
}

/// Generate a new hero goal suggestion based on recent gameplay
pub fn generate_hero_suggestion(conn: &Connection) -> Result<Option<HeroGoalSuggestion>, String> {
    // Get last 20 matches ordered by start_time
    let mut stmt = conn
        .prepare("SELECT hero_id FROM matches ORDER BY start_time DESC LIMIT 20")
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let hero_ids: Vec<i32> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| format!("Failed to query recent matches: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect hero IDs: {}", e))?;

    if hero_ids.is_empty() {
        return Ok(None);
    }

    // Get unique heroes and count total games for each
    let mut hero_game_counts: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for &hero_id in &hero_ids {
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM matches WHERE hero_id = ?1",
                params![hero_id],
                |row| row.get(0),
            )
            .map_err(|e| format!("Failed to count games for hero {}: {}", hero_id, e))?;

        hero_game_counts.insert(hero_id, count);
    }

    // Filter to heroes with >= 5 total games
    let qualifying_heroes: Vec<i32> = hero_game_counts
        .iter()
        .filter(|(_, &count)| count >= 5)
        .map(|(&hero_id, _)| hero_id)
        .collect();

    if qualifying_heroes.is_empty() {
        return Ok(None);
    }

    // Randomly select one qualifying hero
    let mut rng = rand::thread_rng();
    let selected_hero = qualifying_heroes[rng.gen_range(0..qualifying_heroes.len())];

    // Get last 5 games for selected hero with last hits at 10 minutes
    // First try to get from match_cs table (parsed matches), fall back to estimating from total last hits
    let mut stmt = conn
        .prepare(
            "SELECT m.match_id, m.duration,
                    COALESCE(
                        (SELECT last_hits FROM match_cs WHERE match_id = m.match_id AND minute = 10),
                        CAST(m.last_hits * 10.0 / (m.duration / 60.0) AS INTEGER)
                    ) as lh_at_10
             FROM matches m
             WHERE m.hero_id = ?1
             ORDER BY m.start_time DESC
             LIMIT 5"
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let last_hits_values: Vec<i32> = stmt
        .query_map(params![selected_hero], |row| row.get::<_, i32>(2))
        .map_err(|e| format!("Failed to query last hits: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect last hits: {}", e))?;

    if last_hits_values.is_empty() {
        return Ok(None);
    }

    // Filter out unrealistic values (0 or negative, which can happen with short games)
    let valid_values: Vec<i32> = last_hits_values.into_iter().filter(|&v| v > 0).collect();

    if valid_values.is_empty() {
        return Ok(None);
    }

    // Calculate average last hits at 10 minutes
    let sum: i32 = valid_values.iter().sum();
    let average = sum as f64 / valid_values.len() as f64;

    // Generate target based on difficulty setting
    let settings = crate::settings::Settings::load();
    let (range_min, range_max) = settings.improvement_range();
    let improvement_factor = 1.0 + range_min + rng.gen_range(0.0..(range_max - range_min).max(0.001));
    let target_last_hits = (average * improvement_factor).round() as i32;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    Ok(Some(HeroGoalSuggestion {
        hero_id: selected_hero,
        suggested_last_hits: target_last_hits,
        current_average: average,
        created_at: now,
        games_analyzed: valid_values.len() as i32,
    }))
}

/// Save a hero goal suggestion to the database
pub fn save_hero_suggestion(conn: &Connection, suggestion: &HeroGoalSuggestion) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO hero_goal_suggestions
         (id, hero_id, suggested_last_hits, current_average, created_at, games_analyzed)
         VALUES (1, ?1, ?2, ?3, ?4, ?5)",
        params![
            suggestion.hero_id,
            suggestion.suggested_last_hits,
            suggestion.current_average,
            suggestion.created_at,
            suggestion.games_analyzed,
        ],
    )
    .map_err(|e| format!("Failed to save hero suggestion: {}", e))?;

    Ok(())
}

/// Get or generate a hero goal suggestion
pub fn get_or_generate_hero_suggestion(conn: &Connection) -> Result<Option<HeroGoalSuggestion>, String> {
    // Check if we have a current valid suggestion
    if let Some(suggestion) = get_current_hero_suggestion(conn)? {
        return Ok(Some(suggestion));
    }

    // Generate a new suggestion
    if let Some(suggestion) = generate_hero_suggestion(conn)? {
        save_hero_suggestion(conn, &suggestion)?;
        return Ok(Some(suggestion));
    }

    Ok(None)
}

/// Force regenerate a new hero goal suggestion (ignores current suggestion age)
pub fn regenerate_hero_suggestion(conn: &Connection) -> Result<Option<HeroGoalSuggestion>, String> {
    // Generate a new suggestion
    if let Some(suggestion) = generate_hero_suggestion(conn)? {
        save_hero_suggestion(conn, &suggestion)?;
        return Ok(Some(suggestion));
    }

    Ok(None)
}

/// Insert item timing data for a match
pub fn insert_item_timing(conn: &Connection, timing: &NewItemTiming) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO item_timings (match_id, item_id, timing_seconds)
         VALUES (?1, ?2, ?3)",
        params![
            timing.match_id,
            timing.item_id,
            timing.timing_seconds,
        ],
    ).map_err(|e| format!("Failed to insert item timing: {}", e))?;

    Ok(())
}

/// Get item timings for a specific match
pub fn get_item_timings_for_match(conn: &Connection, match_id: i64) -> Result<Vec<ItemTiming>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, match_id, item_id, timing_seconds
             FROM item_timings
             WHERE match_id = ?1
             ORDER BY timing_seconds ASC",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let timings = stmt
        .query_map(params![match_id], |row| {
            Ok(ItemTiming {
                id: row.get(0)?,
                match_id: row.get(1)?,
                item_id: row.get(2)?,
                timing_seconds: row.get(3)?,
            })
        })
        .map_err(|e| format!("Failed to query item timings: {}", e))?;

    let mut result = Vec::new();
    for timing in timings {
        result.push(timing.map_err(|e| format!("Failed to read item timing: {}", e))?);
    }

    Ok(result)
}

/// Get timing for a specific item in a match (returns None if item wasn't purchased)
pub fn get_item_timing(conn: &Connection, match_id: i64, item_id: i32) -> Result<Option<i32>, String> {
    let result = conn.query_row(
        "SELECT timing_seconds FROM item_timings WHERE match_id = ?1 AND item_id = ?2",
        params![match_id, item_id],
        |row| row.get::<_, i32>(0),
    );

    match result {
        Ok(timing) => Ok(Some(timing)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("Failed to query item timing: {}", e)),
    }
}

// ===== Daily Challenges =====

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyChallenge {
    pub id: i64,
    pub challenge_date: String,
    pub challenge_type: String,
    pub challenge_description: String,
    pub challenge_target: i32,
    pub challenge_target_games: i32,
    pub hero_id: Option<i32>,
    pub metric: String,
    pub status: String,
    pub created_at: i64,
    pub completed_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyChallengeProgress {
    pub challenge: DailyChallenge,
    pub current_value: i32,
    pub target: i32,
    pub completed: bool,
    pub games_counted: i32,
}

fn get_today_date_string() -> String {
    use chrono::Local;
    Local::now().format("%Y-%m-%d").to_string()
}

fn get_today_midnight_timestamp() -> i64 {
    use chrono::{Local, NaiveTime, TimeZone};
    let today = Local::now().date_naive();
    let midnight = today.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    Local.from_local_datetime(&midnight).unwrap().timestamp()
}

fn match_is_win(m: &Match) -> bool {
    let is_radiant = m.player_slot < 128;
    (is_radiant && m.radiant_win) || (!is_radiant && !m.radiant_win)
}

/// Map a DB row to a Match struct (used in multiple queries)
fn row_to_match(row: &rusqlite::Row) -> rusqlite::Result<Match> {
    Ok(Match {
        match_id: row.get(0)?,
        hero_id: row.get(1)?,
        start_time: row.get(2)?,
        duration: row.get(3)?,
        game_mode: row.get(4)?,
        lobby_type: row.get(5)?,
        radiant_win: row.get::<_, i32>(6)? != 0,
        player_slot: row.get(7)?,
        kills: row.get(8)?,
        deaths: row.get(9)?,
        assists: row.get(10)?,
        xp_per_min: row.get(11)?,
        gold_per_min: row.get(12)?,
        last_hits: row.get(13)?,
        denies: row.get(14)?,
        hero_damage: row.get(15)?,
        tower_damage: row.get(16)?,
        hero_healing: row.get(17)?,
        parse_state: MatchState::from_string(&row.get::<_, String>(18)?),
        role: row.get(19).unwrap_or(0),
    })
}

/// Get all matches since a given timestamp
fn get_matches_since(conn: &Connection, since_timestamp: i64) -> Result<Vec<Match>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT match_id, hero_id, start_time, duration, game_mode, lobby_type,
                    radiant_win, player_slot, kills, deaths, assists, xp_per_min,
                    gold_per_min, last_hits, denies, hero_damage, tower_damage, hero_healing, parse_state, role
             FROM matches WHERE start_time >= ?1 ORDER BY start_time DESC",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let result = stmt.query_map(params![since_timestamp], row_to_match)
        .map_err(|e| format!("Failed to query matches: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to read match: {}", e));
    result
}

/// Map a DB row to a DailyChallenge struct
fn row_to_daily_challenge(row: &rusqlite::Row) -> rusqlite::Result<DailyChallenge> {
    Ok(DailyChallenge {
        id: row.get(0)?,
        challenge_date: row.get(1)?,
        challenge_type: row.get(2)?,
        challenge_description: row.get(3)?,
        challenge_target: row.get(4)?,
        challenge_target_games: row.get(5)?,
        hero_id: row.get(6)?,
        metric: row.get(7)?,
        status: row.get(8)?,
        created_at: row.get(9)?,
        completed_at: row.get(10)?,
    })
}

/// Get the last N challenge metrics to avoid repeating the same type
fn get_recent_challenge_metrics(conn: &Connection, limit: i64) -> Vec<String> {
    let mut stmt = match conn.prepare(
        "SELECT metric FROM daily_challenges ORDER BY challenge_date DESC LIMIT ?1",
    ) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let result = match stmt.query_map(params![limit], |row| row.get::<_, String>(0)) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(_) => vec![],
    };
    result
}

/// Get average stats from last N matches
fn get_recent_avg_stats(conn: &Connection) -> (f64, f64, f64, f64, Option<f64>) {
    // (avg_kills, avg_gpm, avg_deaths, avg_hero_damage, avg_cs_at_10)
    let row = conn.query_row(
        "SELECT AVG(kills), AVG(gold_per_min), AVG(deaths), AVG(hero_damage)
         FROM (SELECT kills, gold_per_min, deaths, hero_damage
               FROM matches ORDER BY start_time DESC LIMIT 20)",
        [],
        |row| {
            Ok((
                row.get::<_, Option<f64>>(0)?.unwrap_or(10.0),
                row.get::<_, Option<f64>>(1)?.unwrap_or(400.0),
                row.get::<_, Option<f64>>(2)?.unwrap_or(5.0),
                row.get::<_, Option<f64>>(3)?.unwrap_or(15000.0),
            ))
        },
    );

    let (avg_kills, avg_gpm, avg_deaths, avg_dmg) = row.unwrap_or((10.0, 400.0, 5.0, 15000.0));

    let avg_cs10 = conn.query_row(
        "SELECT AVG(mc.last_hits)
         FROM (SELECT match_id FROM matches WHERE parse_state = 'parsed'
               ORDER BY start_time DESC LIMIT 10) m
         JOIN match_cs mc ON m.match_id = mc.match_id AND mc.minute = 10",
        [],
        |row| row.get::<_, Option<f64>>(0),
    ).unwrap_or(None);

    (avg_kills, avg_gpm, avg_deaths, avg_dmg, avg_cs10)
}

/// Pick a random hero from the last N matches
fn pick_hero_from_recent(conn: &Connection, limit: i64) -> Option<i32> {
    let mut stmt = conn
        .prepare("SELECT hero_id FROM matches ORDER BY start_time DESC LIMIT ?1")
        .ok()?;
    let heroes: Vec<i32> = stmt
        .query_map(params![limit], |row| row.get(0))
        .ok()?
        .filter_map(|r| r.ok())
        .collect();
    if heroes.is_empty() {
        return None;
    }
    let mut rng = rand::thread_rng();
    Some(heroes[rng.gen_range(0..heroes.len())])
}

/// Pick a hero NOT played in the last 7 days but played overall
fn pick_unfamiliar_hero(conn: &Connection) -> Option<i32> {
    use chrono::Local;
    let cutoff = (Local::now().timestamp() - 7 * 24 * 3600) as i64;

    let mut stmt = conn
        .prepare(
            "SELECT DISTINCT hero_id FROM matches
             WHERE hero_id NOT IN (
                 SELECT DISTINCT hero_id FROM matches WHERE start_time >= ?1
             )
             ORDER BY RANDOM() LIMIT 1",
        )
        .ok()?;

    stmt.query_row(params![cutoff], |row| row.get::<_, i32>(0))
        .ok()
}

/// Archive any active daily challenges from past days as failed
pub fn archive_expired_daily_challenges(conn: &Connection) -> Result<(), String> {
    let today = get_today_date_string();

    // Find active challenges before today
    let mut stmt = conn
        .prepare(
            "SELECT id, challenge_date, challenge_description
             FROM daily_challenges
             WHERE status = 'active' AND challenge_date < ?1",
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let expired: Vec<(i64, String, String)> = stmt
        .query_map(params![today], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
        })
        .map_err(|e| format!("Failed to query expired challenges: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect expired challenges: {}", e))?;

    for (id, date, description) in &expired {
        conn.execute(
            "UPDATE daily_challenges SET status = 'failed' WHERE id = ?1",
            params![id],
        )
        .map_err(|e| format!("Failed to update challenge status: {}", e))?;

        conn.execute(
            "INSERT INTO challenge_history (challenge_type, period_start_date, challenge_description, status, completed_at, target_achieved)
             VALUES ('daily', ?1, ?2, 'failed', NULL, NULL)",
            params![date, description],
        )
        .map_err(|e| format!("Failed to archive challenge: {}", e))?;
    }

    Ok(())
}

/// Generate a new daily challenge for the given date
fn generate_daily_challenge_for_date(
    conn: &Connection,
    date: &str,
) -> Result<Option<DailyChallenge>, String> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let (avg_kills, avg_gpm, avg_deaths, avg_dmg, avg_cs10) = get_recent_avg_stats(conn);

    // Avoid repeating same metric as yesterday/recent
    let recent_metrics = get_recent_challenge_metrics(conn, 3);
    let last_metric = recent_metrics.first().cloned().unwrap_or_default();

    // All possible challenges, paired (metric, difficulty, build_fn)
    // We'll pick one that isn't a repeat of last_metric
    let recent_hero = pick_hero_from_recent(conn, 10);
    let unfamiliar_hero = pick_unfamiliar_hero(conn);

    // Difficulty roll: 60% easy, 30% medium, 10% hard
    let roll: f64 = rng.gen();
    let difficulty = if roll < 0.60 {
        "easy"
    } else if roll < 0.90 {
        "medium"
    } else {
        "hard"
    };

    // Build candidate challenges for each difficulty
    struct Candidate {
        metric: &'static str,
        difficulty: &'static str,
        description: String,
        target: i32,
        hero_id: Option<i32>,
    }

    let mut candidates: Vec<Candidate> = vec![];

    // Easy candidates
    candidates.push(Candidate {
        metric: "wins",
        difficulty: "easy",
        description: "Win 1 game today".to_string(),
        target: 1,
        hero_id: None,
    });

    candidates.push(Candidate {
        metric: "games_played",
        difficulty: "easy",
        description: "Play 2 games today".to_string(),
        target: 2,
        hero_id: None,
    });

    candidates.push(Candidate {
        metric: "positive_kda",
        difficulty: "easy",
        description: "Finish with positive KDA in one game (K+A > Deaths)".to_string(),
        target: 1,
        hero_id: None,
    });

    if let Some(hero_id) = recent_hero {
        candidates.push(Candidate {
            metric: "wins",
            difficulty: "easy",
            description: "Win 1 game with your hero".to_string(),
            target: 1,
            hero_id: Some(hero_id),
        });
    }

    if let Some(hero_id) = unfamiliar_hero {
        candidates.push(Candidate {
            metric: "games_played",
            difficulty: "easy",
            description: "Play a game with a hero you haven't used in 7 days".to_string(),
            target: 1,
            hero_id: Some(hero_id),
        });
    }

    // Medium candidates
    let kills_target = (avg_kills as i32 + 2).max(10);
    candidates.push(Candidate {
        metric: "kills",
        difficulty: "medium",
        description: format!("Get {}+ kills in one game", kills_target),
        target: kills_target,
        hero_id: None,
    });

    let gpm_target = (avg_gpm as i32 + 30).max(400);
    candidates.push(Candidate {
        metric: "gpm",
        difficulty: "medium",
        description: format!("Achieve {}+ GPM in one game", gpm_target),
        target: gpm_target,
        hero_id: None,
    });

    let deaths_target = ((avg_deaths as i32) - 1).max(1).min(4);
    candidates.push(Candidate {
        metric: "low_deaths",
        difficulty: "medium",
        description: format!("Die {} times or less in one game", deaths_target),
        target: deaths_target,
        hero_id: None,
    });

    // Hard candidates
    let dmg_target = (avg_dmg as i32 + 2000).max(15000);
    candidates.push(Candidate {
        metric: "hero_damage",
        difficulty: "hard",
        description: format!("Deal {}+ hero damage in one game", dmg_target),
        target: dmg_target,
        hero_id: None,
    });

    if let Some(avg) = avg_cs10 {
        let cs_target = (avg as i32 + 5).max(50);
        candidates.push(Candidate {
            metric: "cs_at_10",
            difficulty: "hard",
            description: format!("Get {}+ CS at 10 minutes", cs_target),
            target: cs_target,
            hero_id: None,
        });
    }

    // Filter by target difficulty, avoiding the last used metric
    let matching: Vec<&Candidate> = candidates
        .iter()
        .filter(|c| c.difficulty == difficulty && c.metric != last_metric.as_str())
        .collect();

    // Fall back to any difficulty if no match
    let pool: Vec<&Candidate> = if matching.is_empty() {
        candidates.iter().filter(|c| c.difficulty == difficulty).collect()
    } else {
        matching
    };

    // Fall back to any candidate if still empty
    let pool: Vec<&Candidate> = if pool.is_empty() {
        candidates.iter().collect()
    } else {
        pool
    };

    if pool.is_empty() {
        return Ok(None);
    }

    let chosen = &pool[rng.gen_range(0..pool.len())];

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    conn.execute(
        "INSERT OR IGNORE INTO daily_challenges
         (challenge_date, challenge_type, challenge_description, challenge_target,
          challenge_target_games, hero_id, metric, status, created_at)
         VALUES (?1, ?2, ?3, ?4, 1, ?5, ?6, 'active', ?7)",
        params![
            date,
            chosen.difficulty,
            chosen.description,
            chosen.target,
            chosen.hero_id,
            chosen.metric,
            now,
        ],
    )
    .map_err(|e| format!("Failed to insert daily challenge: {}", e))?;

    // Read back to get id
    conn.query_row(
        "SELECT id, challenge_date, challenge_type, challenge_description, challenge_target,
                challenge_target_games, hero_id, metric, status, created_at, completed_at
         FROM daily_challenges WHERE challenge_date = ?1",
        params![date],
        row_to_daily_challenge,
    )
    .map(Some)
    .map_err(|e| format!("Failed to read back daily challenge: {}", e))
}

/// Get or generate the daily challenge for today
pub fn get_or_generate_daily_challenge(conn: &Connection) -> Result<Option<DailyChallenge>, String> {
    let today = get_today_date_string();

    // Archive any expired active challenges
    archive_expired_daily_challenges(conn)?;

    // Check if we already have one for today
    let existing = conn.query_row(
        "SELECT id, challenge_date, challenge_type, challenge_description, challenge_target,
                challenge_target_games, hero_id, metric, status, created_at, completed_at
         FROM daily_challenges WHERE challenge_date = ?1",
        params![today],
        row_to_daily_challenge,
    );

    match existing {
        Ok(challenge) => Ok(Some(challenge)),
        Err(rusqlite::Error::QueryReturnedNoRows) => generate_daily_challenge_for_date(conn, &today),
        Err(e) => Err(format!("Failed to query daily challenge: {}", e)),
    }
}

/// Evaluate progress for a daily challenge against today's matches
pub fn evaluate_daily_challenge_progress(
    conn: &Connection,
    challenge: &DailyChallenge,
) -> Result<DailyChallengeProgress, String> {
    // If already completed, return complete state
    if challenge.status == "completed" {
        return Ok(DailyChallengeProgress {
            challenge: challenge.clone(),
            current_value: challenge.challenge_target,
            target: challenge.challenge_target,
            completed: true,
            games_counted: 0,
        });
    }

    let midnight = get_today_midnight_timestamp();
    let mut today_matches = get_matches_since(conn, midnight)?;

    // Filter by hero if challenge is hero-specific
    if let Some(hero_id) = challenge.hero_id {
        today_matches.retain(|m| m.hero_id == hero_id);
    }

    let games_counted = today_matches.len() as i32;

    let (current_value, target) = match challenge.metric.as_str() {
        "wins" => {
            let wins = today_matches.iter().filter(|m| match_is_win(m)).count() as i32;
            (wins, challenge.challenge_target)
        }
        "games_played" => (games_counted, challenge.challenge_target),
        "kills" => {
            let max = today_matches.iter().map(|m| m.kills).max().unwrap_or(0);
            (max, challenge.challenge_target)
        }
        "gpm" => {
            let max = today_matches.iter().map(|m| m.gold_per_min).max().unwrap_or(0);
            (max, challenge.challenge_target)
        }
        "hero_damage" => {
            let max = today_matches.iter().map(|m| m.hero_damage).max().unwrap_or(0);
            (max, challenge.challenge_target)
        }
        "positive_kda" => {
            let achieved = today_matches
                .iter()
                .any(|m| (m.kills + m.assists) > m.deaths);
            (if achieved { 1 } else { 0 }, 1)
        }
        "low_deaths" => {
            let achieved = today_matches
                .iter()
                .any(|m| m.deaths <= challenge.challenge_target);
            (if achieved { 1 } else { 0 }, 1)
        }
        "cs_at_10" => {
            let mut max_cs = 0i32;
            for m in today_matches.iter().filter(|m| m.parse_state == MatchState::Parsed) {
                if let Ok(Some(cs)) = get_match_cs_at_minute(conn, m.match_id, 10) {
                    if cs.last_hits > max_cs {
                        max_cs = cs.last_hits;
                    }
                }
            }
            (max_cs, challenge.challenge_target)
        }
        _ => (0, challenge.challenge_target),
    };

    let completed = current_value >= target;

    // Auto-mark as completed if achieved
    if completed && challenge.status == "active" {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        conn.execute(
            "UPDATE daily_challenges SET status = 'completed', completed_at = ?1 WHERE id = ?2",
            params![now, challenge.id],
        )
        .map_err(|e| format!("Failed to mark challenge complete: {}", e))?;

        conn.execute(
            "INSERT INTO challenge_history
             (challenge_type, period_start_date, challenge_description, status, completed_at, target_achieved)
             VALUES ('daily', ?1, ?2, 'completed', ?3, ?4)",
            params![
                challenge.challenge_date,
                challenge.challenge_description,
                now,
                current_value,
            ],
        )
        .map_err(|e| format!("Failed to archive completed challenge: {}", e))?;
    }

    Ok(DailyChallengeProgress {
        challenge: challenge.clone(),
        current_value,
        target,
        completed,
        games_counted,
    })
}

/// Get or generate today's challenge and evaluate its progress
pub fn get_daily_challenge_progress(
    conn: &Connection,
) -> Result<Option<DailyChallengeProgress>, String> {
    match get_or_generate_daily_challenge(conn)? {
        Some(challenge) => Ok(Some(evaluate_daily_challenge_progress(conn, &challenge)?)),
        None => Ok(None),
    }
}

/// Count consecutive days (ending yesterday) where the daily challenge was completed
pub fn get_daily_streak(conn: &Connection) -> Result<i32, String> {
    use chrono::{Duration, Local};

    let mut streak = 0i32;
    let mut check = Local::now().date_naive() - Duration::days(1);

    loop {
        let date_str = check.format("%Y-%m-%d").to_string();

        let status: Result<String, _> = conn.query_row(
            "SELECT status FROM daily_challenges WHERE challenge_date = ?1",
            params![date_str],
            |row| row.get(0),
        );

        match status {
            Ok(s) if s == "completed" => {
                streak += 1;
                check -= Duration::days(1);
            }
            _ => break,
        }
    }

    Ok(streak)
}

// ===== Weekly Challenges =====

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChallengeOption {
    pub id: i64,
    pub week_start_date: String,
    pub challenge_type: String,
    pub challenge_description: String,
    pub challenge_target: i32,
    pub challenge_target_games: Option<i32>,
    pub hero_id: Option<i32>,
    pub metric: String,
    pub option_index: i32,
    pub reroll_generation: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeeklyChallenge {
    pub id: i64,
    pub week_start_date: String,
    pub challenge_type: String,
    pub challenge_description: String,
    pub challenge_target: i32,
    pub challenge_target_games: Option<i32>,
    pub hero_id: Option<i32>,
    pub metric: String,
    pub status: String,
    pub accepted_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub reroll_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeeklyChallengeProgress {
    pub challenge: WeeklyChallenge,
    pub current_value: i32,
    pub target: i32,
    pub games_counted: i32,
    pub days_remaining: i32,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChallengeHistoryItem {
    pub id: i64,
    pub challenge_type: String,
    pub period_start_date: String,
    pub challenge_description: String,
    pub status: String,
    pub completed_at: Option<i64>,
    pub target_achieved: Option<i32>,
}

fn get_week_start_date() -> String {
    use chrono::{Datelike, Duration, Local};
    let today = Local::now().date_naive();
    let days_since_sunday = today.weekday().num_days_from_sunday();
    let week_start = today - Duration::days(days_since_sunday as i64);
    week_start.format("%Y-%m-%d").to_string()
}

fn get_week_end_timestamp(week_start: &str) -> i64 {
    use chrono::{Duration, Local, NaiveDate, NaiveTime, TimeZone};
    let start = NaiveDate::parse_from_str(week_start, "%Y-%m-%d").unwrap_or_else(|_| Local::now().date_naive());
    let end_day = start + Duration::days(7); // next Sunday = start of next week
    let midnight = end_day.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    Local.from_local_datetime(&midnight).unwrap().timestamp()
}

fn days_remaining_in_week() -> i32 {
    use chrono::{Datelike, Local};
    let today = Local::now().date_naive();
    let days_since_sunday = today.weekday().num_days_from_sunday();
    // days remaining = 7 - days_since_sunday - 1 (today is already partly gone)
    (6i32 - days_since_sunday as i32).max(0)
}

fn row_to_challenge_option(row: &rusqlite::Row) -> rusqlite::Result<ChallengeOption> {
    Ok(ChallengeOption {
        id: row.get(0)?,
        week_start_date: row.get(1)?,
        challenge_type: row.get(2)?,
        challenge_description: row.get(3)?,
        challenge_target: row.get(4)?,
        challenge_target_games: row.get(5)?,
        hero_id: row.get(6)?,
        metric: row.get(7)?,
        option_index: row.get(8)?,
        reroll_generation: row.get(9)?,
    })
}

fn row_to_weekly_challenge(row: &rusqlite::Row) -> rusqlite::Result<WeeklyChallenge> {
    Ok(WeeklyChallenge {
        id: row.get(0)?,
        week_start_date: row.get(1)?,
        challenge_type: row.get(2)?,
        challenge_description: row.get(3)?,
        challenge_target: row.get(4)?,
        challenge_target_games: row.get(5)?,
        hero_id: row.get(6)?,
        metric: row.get(7)?,
        status: row.get(8)?,
        accepted_at: row.get(9)?,
        completed_at: row.get(10)?,
        reroll_count: row.get(11)?,
    })
}

fn row_to_history_item(row: &rusqlite::Row) -> rusqlite::Result<ChallengeHistoryItem> {
    Ok(ChallengeHistoryItem {
        id: row.get(0)?,
        challenge_type: row.get(1)?,
        period_start_date: row.get(2)?,
        challenge_description: row.get(3)?,
        status: row.get(4)?,
        completed_at: row.get(5)?,
        target_achieved: row.get(6)?,
    })
}

struct WeeklyCandidate {
    metric: String,
    challenge_type: String,  // "easy", "medium", "hard"
    description: String,
    target: i32,
    target_games: Option<i32>,
    hero_id: Option<i32>,
}

fn generate_weekly_candidates(conn: &Connection) -> Vec<WeeklyCandidate> {
    let mut candidates = vec![];

    let (avg_kills, avg_gpm, avg_deaths, avg_dmg, avg_cs10) = get_recent_avg_stats(conn);
    let recent_hero = pick_hero_from_recent(conn, 20);

    // Easy: win/play targets over a week
    candidates.push(WeeklyCandidate {
        metric: "wins".to_string(),
        challenge_type: "easy".to_string(),
        description: "Win 3 games this week".to_string(),
        target: 3,
        target_games: None,
        hero_id: None,
    });

    candidates.push(WeeklyCandidate {
        metric: "games_played".to_string(),
        challenge_type: "easy".to_string(),
        description: "Play 5 games this week".to_string(),
        target: 5,
        target_games: None,
        hero_id: None,
    });

    candidates.push(WeeklyCandidate {
        metric: "positive_kda_games".to_string(),
        challenge_type: "easy".to_string(),
        description: "Finish with positive KDA (K+A > Deaths) in 3 games".to_string(),
        target: 3,
        target_games: None,
        hero_id: None,
    });

    // Easy hero-specific (max 1)
    if let Some(hero_id) = recent_hero {
        candidates.push(WeeklyCandidate {
            metric: "wins".to_string(),
            challenge_type: "easy".to_string(),
            description: "Win 2 games with your favourite hero".to_string(),
            target: 2,
            target_games: None,
            hero_id: Some(hero_id),
        });
    }

    // Medium
    let kills_total = (avg_kills as i32 * 4).max(20);
    candidates.push(WeeklyCandidate {
        metric: "kills_total".to_string(),
        challenge_type: "medium".to_string(),
        description: format!("Get {}+ total kills this week", kills_total),
        target: kills_total,
        target_games: None,
        hero_id: None,
    });

    let gpm_target = (avg_gpm as i32 + 50).max(450);
    candidates.push(WeeklyCandidate {
        metric: "avg_gpm".to_string(),
        challenge_type: "medium".to_string(),
        description: format!("Average {}+ GPM across 5 games", gpm_target),
        target: gpm_target,
        target_games: Some(5),
        hero_id: None,
    });

    candidates.push(WeeklyCandidate {
        metric: "wins".to_string(),
        challenge_type: "medium".to_string(),
        description: "Win 5 games this week".to_string(),
        target: 5,
        target_games: None,
        hero_id: None,
    });

    let deaths_target = ((avg_deaths as i32) - 1).max(2);
    candidates.push(WeeklyCandidate {
        metric: "low_deaths_games".to_string(),
        challenge_type: "medium".to_string(),
        description: format!("Die {} or fewer times in 4 games", deaths_target),
        target: deaths_target,
        target_games: Some(4),
        hero_id: None,
    });

    // Hard
    let dmg_target = (avg_dmg as i32 + 3000).max(18000);
    candidates.push(WeeklyCandidate {
        metric: "hero_damage_total".to_string(),
        challenge_type: "hard".to_string(),
        description: format!("Deal {}+ total hero damage this week", dmg_target * 5),
        target: dmg_target * 5,
        target_games: None,
        hero_id: None,
    });

    if avg_cs10.is_some() {
        let cs_target = (avg_cs10.unwrap() as i32 + 8).max(55);
        candidates.push(WeeklyCandidate {
            metric: "cs_at_10_avg".to_string(),
            challenge_type: "hard".to_string(),
            description: format!("Average {}+ CS at 10 minutes across 5 games", cs_target),
            target: cs_target,
            target_games: Some(5),
            hero_id: None,
        });
    }

    candidates.push(WeeklyCandidate {
        metric: "wins".to_string(),
        challenge_type: "hard".to_string(),
        description: "Win 8 games this week".to_string(),
        target: 8,
        target_games: None,
        hero_id: None,
    });

    candidates
}

fn pick_3_diverse_options(candidates: Vec<WeeklyCandidate>, _reroll_gen: i32) -> Vec<WeeklyCandidate> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();

    let mut easy_pool: Vec<WeeklyCandidate> = vec![];
    let mut medium_pool: Vec<WeeklyCandidate> = vec![];
    let mut hard_pool: Vec<WeeklyCandidate> = vec![];

    for c in candidates {
        match c.challenge_type.as_str() {
            "easy" => easy_pool.push(c),
            "medium" => medium_pool.push(c),
            _ => hard_pool.push(c),
        }
    }

    easy_pool.shuffle(&mut rng);
    medium_pool.shuffle(&mut rng);
    hard_pool.shuffle(&mut rng);

    let mut result = vec![];
    if let Some(e) = easy_pool.into_iter().next() { result.push(e); }
    if let Some(m) = medium_pool.into_iter().next() { result.push(m); }
    if let Some(h) = hard_pool.into_iter().next() { result.push(h); }

    // If fewer than 3, pick any remaining
    result
}

/// Get or generate the 3 weekly challenge options for the current week
pub fn get_weekly_challenge_options(conn: &Connection) -> Result<Vec<ChallengeOption>, String> {
    let week_start = get_week_start_date();

    // Check existing options for this week
    let mut stmt = conn.prepare(
        "SELECT id, week_start_date, challenge_type, challenge_description, challenge_target,
                challenge_target_games, hero_id, metric, option_index, reroll_generation
         FROM challenge_options WHERE week_start_date = ?1 ORDER BY option_index",
    ).map_err(|e| format!("Failed to prepare query: {}", e))?;

    let existing: Vec<ChallengeOption> = stmt
        .query_map(params![week_start], row_to_challenge_option)
        .map_err(|e| format!("Failed to query options: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect options: {}", e))?;

    if !existing.is_empty() {
        return Ok(existing);
    }

    // Generate new options
    generate_and_save_weekly_options(conn, &week_start, 0)
}

fn generate_and_save_weekly_options(conn: &Connection, week_start: &str, reroll_gen: i32) -> Result<Vec<ChallengeOption>, String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let candidates = generate_weekly_candidates(conn);
    let chosen = pick_3_diverse_options(candidates, reroll_gen);

    // Delete existing options for this week before inserting
    conn.execute(
        "DELETE FROM challenge_options WHERE week_start_date = ?1",
        params![week_start],
    ).map_err(|e| format!("Failed to delete old options: {}", e))?;

    for (i, c) in chosen.iter().enumerate() {
        conn.execute(
            "INSERT INTO challenge_options
             (week_start_date, challenge_type, challenge_description, challenge_target,
              challenge_target_games, hero_id, metric, option_index, reroll_generation, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                week_start,
                c.challenge_type,
                c.description,
                c.target,
                c.target_games,
                c.hero_id,
                c.metric,
                (i + 1) as i32,
                reroll_gen,
                now,
            ],
        ).map_err(|e| format!("Failed to insert option: {}", e))?;
    }

    // Re-read from DB to get proper IDs
    let mut stmt = conn.prepare(
        "SELECT id, week_start_date, challenge_type, challenge_description, challenge_target,
                challenge_target_games, hero_id, metric, option_index, reroll_generation
         FROM challenge_options WHERE week_start_date = ?1 ORDER BY option_index",
    ).map_err(|e| format!("Failed to prepare query: {}", e))?;

    let result = stmt.query_map(params![week_start], row_to_challenge_option)
        .map_err(|e| format!("Failed to query options: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect options: {}", e));
    result
}

/// Reroll weekly challenge options (max 2 rerolls per week)
pub fn reroll_weekly_challenges(conn: &Connection) -> Result<Vec<ChallengeOption>, String> {
    let week_start = get_week_start_date();

    // Check if there's already an accepted challenge for this week
    let accepted: Result<i64, _> = conn.query_row(
        "SELECT id FROM weekly_challenges WHERE week_start_date = ?1 AND accepted_at IS NOT NULL",
        params![week_start],
        |row| row.get(0),
    );
    if accepted.is_ok() {
        return Err("Cannot reroll after accepting a challenge".to_string());
    }

    // Check current reroll count
    let current_gen: i32 = conn.query_row(
        "SELECT COALESCE(MAX(reroll_generation), 0) FROM challenge_options WHERE week_start_date = ?1",
        params![week_start],
        |row| row.get(0),
    ).unwrap_or(0);

    if current_gen >= 2 {
        return Err("Maximum rerolls (2) used for this week".to_string());
    }

    generate_and_save_weekly_options(conn, &week_start, current_gen + 1)
}

/// Skip the weekly challenge (mark week as skipped, no options)
pub fn skip_weekly_challenge(conn: &Connection) -> Result<(), String> {
    let week_start = get_week_start_date();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    conn.execute(
        "INSERT OR IGNORE INTO weekly_challenges
         (week_start_date, challenge_type, challenge_description, challenge_target,
          metric, status, accepted_at)
         VALUES (?1, 'skipped', 'Skipped this week', 0, 'skipped', 'skipped', ?2)",
        params![week_start, now],
    ).map_err(|e| format!("Failed to skip challenge: {}", e))?;

    Ok(())
}

/// Accept a weekly challenge option
pub fn accept_weekly_challenge(conn: &Connection, option_id: i64) -> Result<WeeklyChallenge, String> {
    let week_start = get_week_start_date();

    // Check no challenge accepted yet
    let existing: Result<i64, _> = conn.query_row(
        "SELECT id FROM weekly_challenges WHERE week_start_date = ?1 AND status != 'skipped'",
        params![week_start],
        |row| row.get(0),
    );
    if existing.is_ok() {
        return Err("A challenge has already been accepted for this week".to_string());
    }

    // Get the option
    let option = conn.query_row(
        "SELECT id, week_start_date, challenge_type, challenge_description, challenge_target,
                challenge_target_games, hero_id, metric, option_index, reroll_generation
         FROM challenge_options WHERE id = ?1",
        params![option_id],
        row_to_challenge_option,
    ).map_err(|e| format!("Option not found: {}", e))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Get current reroll count
    let reroll_count: i32 = conn.query_row(
        "SELECT COALESCE(MAX(reroll_generation), 0) FROM challenge_options WHERE week_start_date = ?1",
        params![week_start],
        |row| row.get(0),
    ).unwrap_or(0);

    conn.execute(
        "INSERT INTO weekly_challenges
         (week_start_date, challenge_type, challenge_description, challenge_target,
          challenge_target_games, hero_id, metric, status, accepted_at, reroll_count)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'active', ?8, ?9)",
        params![
            week_start,
            option.challenge_type,
            option.challenge_description,
            option.challenge_target,
            option.challenge_target_games,
            option.hero_id,
            option.metric,
            now,
            reroll_count,
        ],
    ).map_err(|e| format!("Failed to accept challenge: {}", e))?;

    conn.query_row(
        "SELECT id, week_start_date, challenge_type, challenge_description, challenge_target,
                challenge_target_games, hero_id, metric, status, accepted_at, completed_at, reroll_count
         FROM weekly_challenges WHERE week_start_date = ?1 AND status = 'active'",
        params![week_start],
        row_to_weekly_challenge,
    ).map_err(|e| format!("Failed to read back weekly challenge: {}", e))
}

/// Get the active weekly challenge for the current week (if accepted)
pub fn get_active_weekly_challenge(conn: &Connection) -> Result<Option<WeeklyChallenge>, String> {
    let week_start = get_week_start_date();

    // Archive expired weekly challenges from previous weeks
    archive_expired_weekly_challenges(conn)?;

    match conn.query_row(
        "SELECT id, week_start_date, challenge_type, challenge_description, challenge_target,
                challenge_target_games, hero_id, metric, status, accepted_at, completed_at, reroll_count
         FROM weekly_challenges WHERE week_start_date = ?1 AND status IN ('active', 'completed')",
        params![week_start],
        row_to_weekly_challenge,
    ) {
        Ok(c) => Ok(Some(c)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("Failed to query weekly challenge: {}", e)),
    }
}

/// Evaluate progress of the active weekly challenge
pub fn get_weekly_challenge_progress(conn: &Connection) -> Result<Option<WeeklyChallengeProgress>, String> {
    let challenge = match get_active_weekly_challenge(conn)? {
        Some(c) => c,
        None => return Ok(None),
    };

    // If already completed
    if challenge.status == "completed" {
        return Ok(Some(WeeklyChallengeProgress {
            current_value: challenge.challenge_target,
            target: challenge.challenge_target,
            games_counted: 0,
            days_remaining: days_remaining_in_week(),
            completed: true,
            challenge,
        }));
    }

    let since = challenge.accepted_at.unwrap_or_else(|| {
        // Fall back to week start midnight
        let week_start = &challenge.week_start_date;
        use chrono::{NaiveDate, NaiveTime, Local, TimeZone};
        let date = NaiveDate::parse_from_str(week_start, "%Y-%m-%d").unwrap_or_else(|_| Local::now().date_naive());
        let midnight = date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        Local.from_local_datetime(&midnight).unwrap().timestamp()
    });

    let mut matches = get_matches_since(conn, since)?;

    // Filter by hero if hero-specific
    if let Some(hero_id) = challenge.hero_id {
        matches.retain(|m| m.hero_id == hero_id);
    }

    let games_counted = matches.len() as i32;

    let (current_value, target) = match challenge.metric.as_str() {
        "wins" => {
            let wins = matches.iter().filter(|m| match_is_win(m)).count() as i32;
            (wins, challenge.challenge_target)
        }
        "games_played" => (games_counted, challenge.challenge_target),
        "positive_kda_games" => {
            let count = matches.iter().filter(|m| (m.kills + m.assists) > m.deaths).count() as i32;
            (count, challenge.challenge_target)
        }
        "kills_total" => {
            let total: i32 = matches.iter().map(|m| m.kills).sum();
            (total, challenge.challenge_target)
        }
        "avg_gpm" => {
            let required = challenge.challenge_target_games.unwrap_or(5);
            if games_counted < required {
                // Show current avg even if not enough games yet
                let avg = if games_counted > 0 {
                    (matches.iter().map(|m| m.gold_per_min as i64).sum::<i64>() / games_counted as i64) as i32
                } else { 0 };
                (avg, challenge.challenge_target)
            } else {
                let take = required as usize;
                let avg = (matches.iter().take(take).map(|m| m.gold_per_min as i64).sum::<i64>() / take as i64) as i32;
                (avg, challenge.challenge_target)
            }
        }
        "hero_damage_total" => {
            let total: i32 = matches.iter().map(|m| m.hero_damage).sum();
            (total, challenge.challenge_target)
        }
        "low_deaths_games" => {
            let threshold = challenge.challenge_target;
            let count = matches.iter().filter(|m| m.deaths <= threshold).count() as i32;
            let required = challenge.challenge_target_games.unwrap_or(4);
            (count, required)
        }
        "cs_at_10_avg" => {
            // Evaluate parsed matches only
            let mut cs_values = vec![];
            for m in matches.iter().filter(|m| m.parse_state == MatchState::Parsed) {
                if let Ok(Some(cs)) = get_match_cs_at_minute(conn, m.match_id, 10) {
                    cs_values.push(cs.last_hits);
                }
            }
            let avg = if cs_values.is_empty() { 0 } else {
                cs_values.iter().sum::<i32>() / cs_values.len() as i32
            };
            (avg, challenge.challenge_target)
        }
        _ => (0, challenge.challenge_target),
    };

    let completed = current_value >= target;

    // Auto-complete
    if completed && challenge.status == "active" {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        conn.execute(
            "UPDATE weekly_challenges SET status = 'completed', completed_at = ?1 WHERE id = ?2",
            params![now, challenge.id],
        ).map_err(|e| format!("Failed to mark weekly complete: {}", e))?;

        conn.execute(
            "INSERT OR IGNORE INTO challenge_history
             (challenge_type, period_start_date, challenge_description, status, completed_at, target_achieved)
             VALUES ('weekly', ?1, ?2, 'completed', ?3, ?4)",
            params![challenge.week_start_date, challenge.challenge_description, now, current_value],
        ).map_err(|e| format!("Failed to archive weekly challenge: {}", e))?;
    }

    Ok(Some(WeeklyChallengeProgress {
        challenge,
        current_value,
        target,
        games_counted,
        days_remaining: days_remaining_in_week(),
        completed,
    }))
}

/// Archive any active weekly challenges from past weeks as failed
fn archive_expired_weekly_challenges(conn: &Connection) -> Result<(), String> {
    let week_start = get_week_start_date();

    let mut stmt = conn.prepare(
        "SELECT id, week_start_date, challenge_description FROM weekly_challenges
         WHERE status = 'active' AND week_start_date < ?1",
    ).map_err(|e| format!("Failed to prepare query: {}", e))?;

    let expired: Vec<(i64, String, String)> = stmt
        .query_map(params![week_start], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
        })
        .map_err(|e| format!("Failed to query expired weekly challenges: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect expired: {}", e))?;

    for (id, wstart, desc) in &expired {
        conn.execute(
            "UPDATE weekly_challenges SET status = 'failed' WHERE id = ?1",
            params![id],
        ).map_err(|e| format!("Failed to update weekly status: {}", e))?;

        conn.execute(
            "INSERT OR IGNORE INTO challenge_history
             (challenge_type, period_start_date, challenge_description, status, completed_at, target_achieved)
             VALUES ('weekly', ?1, ?2, 'failed', NULL, NULL)",
            params![wstart, desc],
        ).map_err(|e| format!("Failed to archive weekly challenge: {}", e))?;
    }

    Ok(())
}

/// Get challenge history (both daily and weekly)
pub fn get_challenge_history(
    conn: &Connection,
    challenge_type_filter: Option<String>,
    limit: i32,
) -> Result<Vec<ChallengeHistoryItem>, String> {
    let query = match challenge_type_filter.as_deref() {
        Some("weekly") => {
            "SELECT id, challenge_type, period_start_date, challenge_description, status, completed_at, target_achieved
             FROM challenge_history WHERE challenge_type = 'weekly'
             ORDER BY period_start_date DESC LIMIT ?1"
        }
        Some("daily") => {
            "SELECT id, challenge_type, period_start_date, challenge_description, status, completed_at, target_achieved
             FROM challenge_history WHERE challenge_type = 'daily'
             ORDER BY period_start_date DESC LIMIT ?1"
        }
        _ => {
            "SELECT id, challenge_type, period_start_date, challenge_description, status, completed_at, target_achieved
             FROM challenge_history
             ORDER BY period_start_date DESC LIMIT ?1"
        }
    };

    let mut stmt = conn.prepare(query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let result = stmt.query_map(params![limit], row_to_history_item)
        .map_err(|e| format!("Failed to query history: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect history: {}", e));
    result
}
