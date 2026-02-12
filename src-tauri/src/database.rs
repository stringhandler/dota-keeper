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
    Level,
}

impl GoalMetric {
    fn to_string(&self) -> &'static str {
        match self {
            GoalMetric::Networth => "networth",
            GoalMetric::Kills => "kills",
            GoalMetric::LastHits => "last_hits",
            GoalMetric::Level => "level",
        }
    }

    fn from_string(s: &str) -> Option<Self> {
        match s {
            "networth" => Some(GoalMetric::Networth),
            "kills" => Some(GoalMetric::Kills),
            "last_hits" => Some(GoalMetric::LastHits),
            "level" => Some(GoalMetric::Level),
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
    pub hero_id: Option<i32>,  // None means "any hero"
    pub metric: GoalMetric,
    pub target_value: i32,
    pub target_time_minutes: i32,
    pub game_mode: GoalGameMode,
    pub created_at: i64,
}

/// Input for creating a new goal (without id and created_at)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewGoal {
    pub hero_id: Option<i32>,
    pub metric: GoalMetric,
    pub target_value: i32,
    pub target_time_minutes: i32,
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
}

impl Match {
    /// Determine if the player won this match
    pub fn is_win(&self) -> bool {
        let is_radiant = self.player_slot < 128;
        (is_radiant && self.radiant_win) || (!is_radiant && !self.radiant_win)
    }
}

/// Get the path to the database file in the AppData directory
fn get_db_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|mut path| {
        path.push("DotaKeeper");
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

    // Cleanup: Reset any "parsing" matches to "unparsed" (in case app crashed during parsing)
    conn.execute(
        "UPDATE matches SET parse_state = 'unparsed' WHERE parse_state = 'parsing'",
        [],
    ).map_err(|e| format!("Failed to cleanup parsing state: {}", e))?;

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
            parse_state TEXT NOT NULL DEFAULT 'unparsed'
        )",
        [],
    ).map_err(|e| format!("Failed to create matches table: {}", e))?;

    // Add parse_state column if it doesn't exist (for existing databases)
    let _ = conn.execute(
        "ALTER TABLE matches ADD COLUMN parse_state TEXT NOT NULL DEFAULT 'unparsed'",
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
            gold_per_min, last_hits, denies, hero_damage, tower_damage, hero_healing, parse_state
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
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
                    gold_per_min, last_hits, denies, hero_damage, tower_damage, hero_healing, parse_state
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
                    gold_per_min, last_hits, denies, hero_damage, tower_damage, hero_healing, parse_state
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
        "INSERT INTO goals (hero_id, metric, target_value, target_time_minutes, game_mode, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            goal.hero_id,
            goal.metric.to_string(),
            goal.target_value,
            goal.target_time_minutes,
            goal.game_mode.to_string(),
            now,
        ],
    ).map_err(|e| format!("Failed to insert goal: {}", e))?;

    let id = conn.last_insert_rowid();

    Ok(Goal {
        id,
        hero_id: goal.hero_id,
        metric: goal.metric.clone(),
        target_value: goal.target_value,
        target_time_minutes: goal.target_time_minutes,
        game_mode: goal.game_mode.clone(),
        created_at: now,
    })
}

/// Get all goals from the database
pub fn get_all_goals(conn: &Connection) -> Result<Vec<Goal>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, hero_id, metric, target_value, target_time_minutes, game_mode, created_at
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
         target_time_minutes = ?4, game_mode = ?5 WHERE id = ?6",
        params![
            goal.hero_id,
            goal.metric.to_string(),
            goal.target_value,
            goal.target_time_minutes,
            goal.game_mode.to_string(),
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
    // Check if goal applies to this match based on hero
    if let Some(goal_hero_id) = goal.hero_id {
        if goal_hero_id != match_data.hero_id {
            return None; // Goal doesn't apply to this hero
        }
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
            // Try to get exact CS data from match_cs table if match is parsed
            if match_data.parse_state == MatchState::Parsed {
                // For a goal at "10 minutes", we need lh_t[10] which is stored with minute=10
                // because we store the array index directly
                if let Ok(Some(cs_data)) = get_match_cs_at_minute(conn, match_data.match_id, target_minutes) {
                    cs_data.last_hits
                } else {
                    // Fall back to linear estimation if no parsed data
                    if duration_minutes <= target_minutes {
                        match_data.last_hits
                    } else {
                        ((match_data.last_hits as f32 / duration_minutes as f32) * target_minutes as f32) as i32
                    }
                }
            } else {
                // Use linear estimation for unparsed matches
                if duration_minutes <= target_minutes {
                    match_data.last_hits
                } else {
                    ((match_data.last_hits as f32 / duration_minutes as f32) * target_minutes as f32) as i32
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

    let achieved = actual_value >= goal.target_value;

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
        .prepare("SELECT id, hero_id, metric, target_value, target_time_minutes, game_mode, created_at FROM goals WHERE id = ?1")
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
        .prepare("SELECT id, hero_id, metric, target_value, target_time_minutes, game_mode, created_at FROM goals WHERE id = ?1")
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

    // Generate target: 5-10% higher than average
    let improvement_factor = 1.05 + rng.gen_range(0.0..0.05);
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
