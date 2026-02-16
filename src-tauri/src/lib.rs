mod database;
mod opendota;
mod settings;
mod items;

use database::{
    clear_all_matches, delete_goal, evaluate_match_goals, get_all_goals, get_all_matches,
    get_goal_by_id, get_goal_match_data, get_goals_with_daily_progress, get_last_hits_analysis,
    get_matches_with_goals, get_oldest_match_timestamp, get_unparsed_matches, init_db, insert_goal,
    insert_match, insert_match_cs_data, match_exists, update_goal, update_match_state, Goal,
    GoalEvaluation, GoalWithDailyProgress, LastHitsAnalysis, MatchDataPoint, MatchState,
    MatchWithGoals, NewGoal, toggle_hero_favorite, get_favorite_hero_ids,
    get_or_generate_hero_suggestion, regenerate_hero_suggestion, HeroGoalSuggestion,
    insert_item_timing, get_item_timings_for_match, NewItemTiming,
};
use serde_json;
use settings::Settings;
use tauri::Emitter;
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

/// Get or generate weekly hero goal suggestion
#[tauri::command]
fn get_hero_goal_suggestion() -> Result<Option<HeroGoalSuggestion>, String> {
    let conn = init_db()?;
    get_or_generate_hero_suggestion(&conn)
}

/// Force regenerate hero goal suggestion (ignores cache)
#[tauri::command]
fn refresh_hero_goal_suggestion() -> Result<Option<HeroGoalSuggestion>, String> {
    let conn = init_db()?;
    regenerate_hero_suggestion(&conn)
}

/// Parse a match and extract goal progress data
#[tauri::command]
async fn parse_match(app: tauri::AppHandle, match_id: i64, steam_id: String) -> Result<(), String> {
    let conn = init_db()?;

    // Update match state to parsing
    update_match_state(&conn, match_id, MatchState::Parsing)?;
    let _ = app.emit(
        "match-state-changed",
        serde_json::json!({
            "match_id": match_id,
            "state": "Parsing"
        }),
    );

    // Request OpenDota to parse the match
    if let Err(e) = opendota::request_match_parse(match_id).await {
        update_match_state(&conn, match_id, MatchState::Failed)?;
        let _ = app.emit(
            "match-state-changed",
            serde_json::json!({
                "match_id": match_id,
                "state": "Failed"
            }),
        );
        return Err(format!("Failed to request parse: {}", e));
    }

    // Wait a bit for the parse to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Fetch detailed match data
    let detailed_match = match opendota::fetch_match_details(match_id).await {
        Ok(m) => m,
        Err(e) => {
            update_match_state(&conn, match_id, MatchState::Failed)?;
            let _ = app.emit(
                "match-state-changed",
                serde_json::json!({
                    "match_id": match_id,
                    "state": "Failed"
                }),
            );
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

    // Store all per-minute CS data - this is REQUIRED for parsing to be successful
    // Without per-minute data, we can't evaluate last-hit goals accurately
    if let (Some(lh_t), Some(dn_t)) = (&player_data.lh_t, &player_data.dn_t) {
        // Verify data is not empty
        if lh_t.is_empty() || dn_t.is_empty() {
            update_match_state(&conn, match_id, MatchState::Failed)?;
            let _ = app.emit(
                "match-state-changed",
                serde_json::json!({
                    "match_id": match_id,
                    "state": "Failed"
                }),
            );
            return Err("OpenDota returned empty per-minute data. The match may not be fully parsed yet. Try again in a few minutes.".to_string());
        }

        insert_match_cs_data(&conn, match_id, lh_t, dn_t)?;

        // Store item purchase timings if available
        if let Some(purchase_log) = &player_data.purchase_log {
            for purchase in purchase_log {
                if let Some(item_id) = items::get_item_id(&purchase.key) {
                    let timing = NewItemTiming {
                        match_id,
                        item_id,
                        timing_seconds: purchase.time,
                    };
                    // Ignore errors for individual item inserts
                    let _ = insert_item_timing(&conn, &timing);
                }
            }
        }

        // Only mark as Parsed if we successfully stored the data
        update_match_state(&conn, match_id, MatchState::Parsed)?;
        let _ = app.emit(
            "match-state-changed",
            serde_json::json!({
                "match_id": match_id,
                "state": "Parsed"
            }),
        );

        Ok(())
    } else {
        // Per-minute data not available yet
        update_match_state(&conn, match_id, MatchState::Failed)?;
        let _ = app.emit(
            "match-state-changed",
            serde_json::json!({
                "match_id": match_id,
                "state": "Failed"
            }),
        );
        Err("OpenDota has not finished parsing this match yet. The per-minute data is not available. Try again in a few minutes.".to_string())
    }
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

/// Get last hits analysis with filtering
#[tauri::command]
fn get_last_hits_analysis_data(
    time_minutes: i32,
    window_size: usize,
    hero_id: Option<i32>,
    game_mode: Option<i32>,
) -> Result<LastHitsAnalysis, String> {
    let conn = init_db()?;
    get_last_hits_analysis(&conn, time_minutes, window_size, hero_id, game_mode)
}

/// Backfill and parse historical matches (100 games before the oldest match)
#[tauri::command]
async fn backfill_historical_matches(
    app: tauri::AppHandle,
    steam_id: String,
) -> Result<String, String> {
    let conn = init_db()?;

    // Get the oldest match timestamp
    let oldest_timestamp = get_oldest_match_timestamp(&conn)?;

    // If no matches exist, fetch from current time
    let before_timestamp = oldest_timestamp.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    });

    // Fetch 100 matches before the oldest timestamp
    let matches = opendota::fetch_matches_before(&steam_id, before_timestamp, 100).await?;

    if matches.is_empty() {
        return Ok("No new matches found to backfill.".to_string());
    }

    // Insert matches that don't already exist
    let mut new_count = 0;
    let mut parsed_count = 0;

    for m in &matches {
        if !match_exists(&conn, m.match_id)? {
            insert_match(&conn, m)?;
            new_count += 1;
        }
    }

    // Convert Steam ID for parsing
    let account_id = steam_id64_to_id32(&steam_id)?;

    // Parse matches (with a small delay between each to avoid rate limiting)
    for m in &matches {
        // Request parse
        if let Err(e) = opendota::request_match_parse(m.match_id).await {
            eprintln!("Failed to request parse for match {}: {}", m.match_id, e);
            continue;
        }

        update_match_state(&conn, m.match_id, MatchState::Parsing)?;
        let _ = app.emit(
            "match-state-changed",
            serde_json::json!({
                "match_id": m.match_id,
                "state": "Parsing"
            }),
        );

        // Wait a bit for the parse to complete
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        // Fetch detailed match data
        let detailed_match = match opendota::fetch_match_details(m.match_id).await {
            Ok(dm) => dm,
            Err(e) => {
                eprintln!("Failed to fetch match details for {}: {}", m.match_id, e);
                update_match_state(&conn, m.match_id, MatchState::Failed)?;
                let _ = app.emit(
                    "match-state-changed",
                    serde_json::json!({
                        "match_id": m.match_id,
                        "state": "Failed"
                    }),
                );
                continue;
            }
        };

        // Find the player's data
        let player_data = match detailed_match
            .players
            .iter()
            .find(|p| p.account_id == Some(account_id))
        {
            Some(p) => p,
            None => {
                eprintln!("Player not found in match {}", m.match_id);
                update_match_state(&conn, m.match_id, MatchState::Failed)?;
                let _ = app.emit(
                    "match-state-changed",
                    serde_json::json!({
                        "match_id": m.match_id,
                        "state": "Failed"
                    }),
                );
                continue;
            }
        };

        // Store CS data if available
        if let (Some(lh_t), Some(dn_t)) = (&player_data.lh_t, &player_data.dn_t) {
            if let Err(e) = insert_match_cs_data(&conn, m.match_id, lh_t, dn_t) {
                eprintln!("Failed to insert CS data for match {}: {}", m.match_id, e);
            } else {
                // Store item purchase timings if available
                if let Some(purchase_log) = &player_data.purchase_log {
                    for purchase in purchase_log {
                        if let Some(item_id) = items::get_item_id(&purchase.key) {
                            let timing = NewItemTiming {
                                match_id: m.match_id,
                                item_id,
                                timing_seconds: purchase.time,
                            };
                            let _ = insert_item_timing(&conn, &timing);
                        }
                    }
                }

                update_match_state(&conn, m.match_id, MatchState::Parsed)?;
                let _ = app.emit(
                    "match-state-changed",
                    serde_json::json!({
                        "match_id": m.match_id,
                        "state": "Parsed"
                    }),
                );
                parsed_count += 1;
            }
        } else {
            update_match_state(&conn, m.match_id, MatchState::Failed)?;
            let _ = app.emit(
                "match-state-changed",
                serde_json::json!({
                    "match_id": m.match_id,
                    "state": "Failed"
                }),
            );
        }

        // Small delay to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(format!(
        "Backfill complete! Added {} new matches, parsed {} matches.",
        new_count, parsed_count
    ))
}

/// Reparse all unparsed or failed matches
#[tauri::command]
async fn reparse_pending_matches(
    app: tauri::AppHandle,
    steam_id: String,
) -> Result<String, String> {
    let conn = init_db()?;

    // Get all unparsed or failed matches
    let matches = get_unparsed_matches(&conn)?;

    if matches.is_empty() {
        return Ok("No pending matches to reparse.".to_string());
    }

    let total_matches = matches.len();
    let mut parsed_count = 0;
    let mut failed_count = 0;

    // Convert Steam ID for parsing
    let account_id = steam_id64_to_id32(&steam_id)?;

    // Parse each match
    for m in &matches {
        // Request parse
        if let Err(e) = opendota::request_match_parse(m.match_id).await {
            eprintln!("Failed to request parse for match {}: {}", m.match_id, e);
            failed_count += 1;
            continue;
        }

        update_match_state(&conn, m.match_id, MatchState::Parsing)?;
        let _ = app.emit(
            "match-state-changed",
            serde_json::json!({
                "match_id": m.match_id,
                "state": "Parsing"
            }),
        );

        // Wait a bit for the parse to complete
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        // Fetch detailed match data
        let detailed_match = match opendota::fetch_match_details(m.match_id).await {
            Ok(dm) => dm,
            Err(e) => {
                eprintln!("Failed to fetch match details for {}: {}", m.match_id, e);
                update_match_state(&conn, m.match_id, MatchState::Failed)?;
                let _ = app.emit(
                    "match-state-changed",
                    serde_json::json!({
                        "match_id": m.match_id,
                        "state": "Failed"
                    }),
                );
                failed_count += 1;
                continue;
            }
        };

        // Find the player's data
        let player_data = match detailed_match
            .players
            .iter()
            .find(|p| p.account_id == Some(account_id))
        {
            Some(p) => p,
            None => {
                eprintln!("Player not found in match {}", m.match_id);
                update_match_state(&conn, m.match_id, MatchState::Failed)?;
                let _ = app.emit(
                    "match-state-changed",
                    serde_json::json!({
                        "match_id": m.match_id,
                        "state": "Failed"
                    }),
                );
                failed_count += 1;
                continue;
            }
        };

        // Store CS data if available
        if let (Some(lh_t), Some(dn_t)) = (&player_data.lh_t, &player_data.dn_t) {
            if let Err(e) = insert_match_cs_data(&conn, m.match_id, lh_t, dn_t) {
                eprintln!("Failed to insert CS data for match {}: {}", m.match_id, e);
                update_match_state(&conn, m.match_id, MatchState::Failed)?;
                let _ = app.emit(
                    "match-state-changed",
                    serde_json::json!({
                        "match_id": m.match_id,
                        "state": "Failed"
                    }),
                );
                failed_count += 1;
            } else {
                // Store item purchase timings if available
                if let Some(purchase_log) = &player_data.purchase_log {
                    for purchase in purchase_log {
                        if let Some(item_id) = items::get_item_id(&purchase.key) {
                            let timing = NewItemTiming {
                                match_id: m.match_id,
                                item_id,
                                timing_seconds: purchase.time,
                            };
                            let _ = insert_item_timing(&conn, &timing);
                        }
                    }
                }

                update_match_state(&conn, m.match_id, MatchState::Parsed)?;
                let _ = app.emit(
                    "match-state-changed",
                    serde_json::json!({
                        "match_id": m.match_id,
                        "state": "Parsed"
                    }),
                );
                parsed_count += 1;
            }
        } else {
            update_match_state(&conn, m.match_id, MatchState::Failed)?;
            let _ = app.emit(
                "match-state-changed",
                serde_json::json!({
                    "match_id": m.match_id,
                    "state": "Failed"
                }),
            );
            failed_count += 1;
        }

        // Small delay to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(format!(
        "Reparse complete! Successfully parsed {} of {} matches. {} failed.",
        parsed_count, total_matches, failed_count
    ))
}

/// Clear all matches from the database
#[tauri::command]
fn clear_matches() -> Result<String, String> {
    let conn = init_db()?;
    clear_all_matches(&conn)?;
    Ok("All matches cleared successfully.".to_string())
}

/// Toggle hero favorite status
#[tauri::command]
fn toggle_favorite_hero(hero_id: i32) -> Result<bool, String> {
    let conn = init_db()?;
    toggle_hero_favorite(&conn, hero_id)
}

/// Get all favorite hero IDs
#[tauri::command]
fn get_favorite_heroes() -> Result<Vec<i32>, String> {
    let conn = init_db()?;
    get_favorite_hero_ids(&conn)
}

/// Get all trackable items
#[tauri::command]
fn get_all_items() -> Vec<items::Item> {
    items::get_all_items()
}

/// Get item timings for a specific match
#[tauri::command]
fn get_match_item_timings(match_id: i64) -> Result<Vec<database::ItemTiming>, String> {
    let conn = init_db()?;
    get_item_timings_for_match(&conn, match_id)
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
        .plugin(tauri_plugin_updater::Builder::new().build())
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
            get_hero_goal_suggestion,
            refresh_hero_goal_suggestion,
            parse_match,
            get_database_folder_path,
            open_database_folder,
            get_last_hits_analysis_data,
            backfill_historical_matches,
            reparse_pending_matches,
            clear_matches,
            toggle_favorite_hero,
            get_favorite_heroes,
            get_all_items,
            get_match_item_timings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
