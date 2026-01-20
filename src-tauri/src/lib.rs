mod database;
mod opendota;
mod settings;

use database::{
    delete_goal, evaluate_match_goals, get_all_goals, get_all_matches, get_goal_by_id,
    get_goal_match_data, get_goals_with_daily_progress, get_matches_with_goals, init_db,
    insert_goal, insert_goal_progress, insert_match, insert_match_cs_data, match_exists,
    update_goal, update_match_state, Goal, GoalEvaluation, GoalProgress, GoalWithDailyProgress,
    MatchDataPoint, MatchState, MatchWithGoals, NewGoal,
};
use settings::Settings;

/// Get the current settings
#[tauri::command]
fn get_settings() -> Settings {
    Settings::load()
}

/// Save the Steam ID to settings
#[tauri::command]
fn save_steam_id(steam_id: String) -> Result<Settings, String> {
    let mut settings = Settings::load();
    settings.steam_id = Some(steam_id);
    settings.save()?;
    Ok(settings)
}

/// Clear the Steam ID (logout)
#[tauri::command]
fn logout() -> Result<Settings, String> {
    let mut settings = Settings::load();
    settings.steam_id = None;
    settings.save()?;
    Ok(settings)
}

/// Refresh matches from OpenDota API
#[tauri::command]
async fn refresh_matches() -> Result<Vec<MatchWithGoals>, String> {
    let settings = Settings::load();
    let steam_id = settings
        .steam_id
        .ok_or_else(|| "No Steam ID configured".to_string())?;

    // Fetch recent matches from OpenDota
    let matches = opendota::fetch_recent_matches(&steam_id, 10).await?;

    // Initialize database
    let conn = init_db()?;

    // Insert matches that don't already exist
    let mut new_matches = Vec::new();
    for m in matches {
        if !match_exists(&conn, m.match_id)? {
            insert_match(&conn, &m)?;
            new_matches.push(m);
        }
    }

    // Return all matches from database
    get_matches_with_goals(&conn)
}

/// Get all stored matches
#[tauri::command]
fn get_matches() -> Result<Vec<MatchWithGoals>, String> {
    let conn = init_db()?;
    get_matches_with_goals(&conn)
}

/// Create a new goal
#[tauri::command]
fn create_goal(goal: NewGoal) -> Result<Goal, String> {
    let conn = init_db()?;
    insert_goal(&conn, &goal)
}

/// Get all goals
#[tauri::command]
fn get_goals() -> Result<Vec<Goal>, String> {
    let conn = init_db()?;
    get_all_goals(&conn)
}

/// Update an existing goal
#[tauri::command]
fn save_goal(goal: Goal) -> Result<(), String> {
    let conn = init_db()?;
    update_goal(&conn, &goal)
}

/// Delete a goal
#[tauri::command]
fn remove_goal(goal_id: i64) -> Result<(), String> {
    let conn = init_db()?;
    delete_goal(&conn, goal_id)
}

/// Evaluate goals for a specific match
#[tauri::command]
fn evaluate_goals_for_match(match_id: i64) -> Result<Vec<GoalEvaluation>, String> {
    let conn = init_db()?;

    // Get the match
    let matches = get_all_matches(&conn)?;
    let match_data = matches
        .iter()
        .find(|m| m.match_id == match_id)
        .ok_or_else(|| "Match not found".to_string())?;

    // Evaluate goals for this match
    evaluate_match_goals(&conn, match_data)
}

/// Parse a match and extract goal progress data
#[tauri::command]
async fn parse_match(match_id: i64, steam_id: String) -> Result<(), String> {
    let conn = init_db()?;

    // Update match state to parsing
    update_match_state(&conn, match_id, MatchState::Parsing)?;

    // Request OpenDota to parse the match
    if let Err(e) = opendota::request_match_parse(match_id).await {
        update_match_state(&conn, match_id, MatchState::Failed)?;
        return Err(format!("Failed to request parse: {}", e));
    }

    // Wait a bit for the parse to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Fetch detailed match data
    let detailed_match = match opendota::fetch_match_details(match_id).await {
        Ok(m) => m,
        Err(e) => {
            update_match_state(&conn, match_id, MatchState::Failed)?;
            return Err(format!("Failed to fetch match details: {}", e));
        }
    };

    // Convert Steam ID to account ID for matching
    let account_id = steam_id64_to_id32(&steam_id)?;

    // Find the player's data
    let player_data = detailed_match
        .players
        .iter()
        .find(|p| p.account_id == Some(account_id))
        .ok_or_else(|| "Player not found in match".to_string())?;

    // Store all per-minute CS data
    if let (Some(lh_t), Some(dn_t)) = (&player_data.lh_t, &player_data.dn_t) {
        insert_match_cs_data(&conn, match_id, lh_t, dn_t)?;
    }

    // Update match state to parsed
    update_match_state(&conn, match_id, MatchState::Parsed)?;

    Ok(())
}

/// Get goals with daily progress for the last N days
#[tauri::command]
fn get_goals_calendar(days: i32) -> Result<Vec<GoalWithDailyProgress>, String> {
    let conn = init_db()?;
    get_goals_with_daily_progress(&conn, days)
}

/// Get a specific goal by ID
#[tauri::command]
fn get_goal(goal_id: i64) -> Result<Goal, String> {
    let conn = init_db()?;
    get_goal_by_id(&conn, goal_id)
}

/// Get match data for a specific goal (for histogram visualization)
#[tauri::command]
fn get_goal_histogram_data(goal_id: i64) -> Result<Vec<MatchDataPoint>, String> {
    let conn = init_db()?;
    get_goal_match_data(&conn, goal_id)
}

/// Get the path to the database folder
#[tauri::command]
fn get_database_folder_path() -> Result<String, String> {
    dirs::data_local_dir()
        .map(|mut path| {
            path.push("DotaKeeper");
            path.to_string_lossy().to_string()
        })
        .ok_or_else(|| "Could not determine database directory".to_string())
}

/// Open the database folder in the system file explorer
#[tauri::command]
async fn open_database_folder() -> Result<(), String> {
    let folder_path = get_database_folder_path()?;

    // Ensure the folder exists
    std::fs::create_dir_all(&folder_path)
        .map_err(|e| format!("Failed to create database directory: {}", e))?;

    // Open the folder using tauri opener plugin
    tauri_plugin_opener::open_path(folder_path, None::<&str>)
        .map_err(|e| format!("Failed to open folder: {}", e))?;

    Ok(())
}

/// Convert Steam ID64 to Steam ID32 (account ID)
fn steam_id64_to_id32(steam_id64: &str) -> Result<u32, String> {
    let id64: u64 = steam_id64
        .parse()
        .map_err(|_| "Invalid Steam ID format".to_string())?;

    const STEAM_ID64_BASE: u64 = 76561197960265728;

    if id64 < STEAM_ID64_BASE {
        return Ok(id64 as u32);
    }

    Ok((id64 - STEAM_ID64_BASE) as u32)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_steam_id,
            logout,
            refresh_matches,
            get_matches,
            create_goal,
            get_goals,
            get_goals_calendar,
            get_goal,
            get_goal_histogram_data,
            save_goal,
            remove_goal,
            evaluate_goals_for_match,
            parse_match,
            get_database_folder_path,
            open_database_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
