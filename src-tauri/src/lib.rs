use rusqlite::OptionalExtension;
mod analytics;
mod database;
mod items;
mod opendota;
mod settings;

use std::sync::OnceLock;
use uuid;

// Global session ID - generated once per app launch
static SESSION_ID: OnceLock<String> = OnceLock::new();

fn get_session_id() -> String {
    SESSION_ID
        .get_or_init(|| uuid::Uuid::new_v4().to_string())
        .clone()
}

use database::{
    accept_weekly_challenge, clear_all_matches, delete_goal, evaluate_match_goals,
    get_active_weekly_challenge, get_all_goals, get_all_matches, get_challenge_history,
    get_daily_challenge_progress, get_daily_streak, get_db_dir, get_favorite_hero_ids,
    get_goal_by_id, get_goal_match_data, get_goals_with_daily_progress, get_item_timings_for_match,
    get_last_hits_analysis, get_match_cs_data, get_matches_with_goals, get_oldest_match_timestamp,
    get_or_generate_daily_challenge, get_or_generate_hero_suggestion, get_unparsed_matches,
    get_weekly_challenge_options, get_weekly_challenge_progress, init_db, insert_goal,
    insert_item_timing, insert_match, insert_match_cs_data, insert_player_networth, match_exists,
    regenerate_hero_suggestion, reroll_weekly_challenges, set_db_dir, skip_weekly_challenge,
    toggle_hero_favorite, update_goal, update_match_partner_slot, update_match_role,
    update_match_state, ChallengeHistoryItem, ChallengeOption, DailyChallenge,
    DailyChallengeProgress, Goal, GoalEvaluation, GoalWithDailyProgress, HeroGoalSuggestion,
    LastHitsAnalysis, MatchCS, MatchDataPoint, MatchState, MatchWithGoals, NewGoal, NewItemTiming,
    WeeklyChallenge, WeeklyChallengeProgress,
};
use serde_json;
use settings::{set_settings_dir, AnalyticsConsent, Settings};
use tauri::{Emitter, Manager};
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

/// Save suggestion difficulty setting
#[tauri::command]
fn save_suggestion_difficulty(
    difficulty: String,
    custom_percentage: Option<f64>,
) -> Result<Settings, String> {
    let mut settings = Settings::load();
    settings.suggestion_difficulty = difficulty;
    settings.suggestion_custom_percentage = custom_percentage;
    settings.save()?;
    Ok(settings)
}

/// Update analytics consent setting
#[tauri::command]
fn save_analytics_consent(consent: String) -> Result<Settings, String> {
    let mut settings = Settings::load();
    settings.analytics_consent = match consent.as_str() {
        "Accepted" => AnalyticsConsent::Accepted,
        "Declined" => AnalyticsConsent::Declined,
        _ => AnalyticsConsent::NotYet,
    };
    settings.save()?;
    Ok(settings)
}

/// Identify the user in analytics (should be called once on app start)
#[tauri::command]
async fn identify_analytics_user() -> Result<(), String> {
    let settings = Settings::load();
    let is_accepted = settings.analytics_consent == AnalyticsConsent::Accepted;
    let installation_id = settings.installation_id.clone();
    analytics::identify_user(is_accepted, installation_id).await
}

/// Track an analytics event (async, fails silently)
#[tauri::command]
async fn track_event(event: String, properties: Option<serde_json::Value>) -> Result<(), String> {
    let settings = Settings::load();
    let is_accepted = settings.analytics_consent == AnalyticsConsent::Accepted;
    let installation_id = settings.installation_id.clone();
    let session_id = get_session_id();
    analytics::track_event(event, properties, is_accepted, installation_id, session_id).await
}

/// Clear the Steam ID (logout)
#[tauri::command]
fn logout() -> Result<Settings, String> {
    let mut settings = Settings::load();
    settings.steam_id = None;
    settings.save()?;
    Ok(settings)
}

/// Enable or disable mental health mood tracking
#[tauri::command]
fn save_mental_health_enabled(enabled: bool) -> Result<Settings, String> {
    let mut settings = Settings::load();
    settings.mental_health_tracking_enabled = enabled;
    settings.save()?;
    Ok(settings)
}

/// Mark the first-enable explanation modal as shown
#[tauri::command]
fn mark_mental_health_intro_shown() -> Result<(), String> {
    let mut settings = Settings::load();
    settings.mental_health_intro_shown = true;
    settings.save()?;
    Ok(())
}

/// Delete all mood check-in data (does not affect match history or goals)
#[tauri::command]
fn clear_mood_data() -> Result<(), String> {
    let conn = init_db()?;
    conn.execute("DELETE FROM mood_checkins", [])
        .map_err(|e| format!("Failed to clear mood data: {}", e))?;
    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct PendingCheckin {
    match_id: i64,
    is_loss_streak: bool,
}

/// Return the most recent match that needs a check-in, if trigger conditions are met.
/// Returns null if tracking is disabled, no qualifying match exists, or no trigger fires.
#[tauri::command]
fn get_pending_checkin() -> Result<Option<PendingCheckin>, String> {
    let settings = Settings::load();
    if !settings.mental_health_tracking_enabled {
        return Ok(None);
    }

    let conn = init_db()?;

    // Find the most recent match that hasn't been checked in or dismissed
    let pending: Option<i64> = conn
        .query_row(
            "SELECT match_id FROM matches \
             WHERE match_id NOT IN (SELECT match_id FROM mood_checkins) \
             ORDER BY start_time DESC LIMIT 1",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| format!("DB error getting pending checkin: {}", e))?;

    let Some(match_id) = pending else {
        return Ok(None);
    };

    // Check for 3+ consecutive losses (look at 5 most recent matches)
    struct RecentMatch {
        radiant_win: i32,
        player_slot: i32,
    }
    let mut stmt = conn
        .prepare("SELECT radiant_win, player_slot FROM matches ORDER BY start_time DESC LIMIT 5")
        .map_err(|e| format!("DB error: {}", e))?;
    let recent: Vec<bool> = stmt
        .query_map([], |row| {
            Ok(RecentMatch {
                radiant_win: row.get(0)?,
                player_slot: row.get(1)?,
            })
        })
        .map_err(|e| format!("DB error: {}", e))?
        .filter_map(|r| r.ok())
        .map(|m| {
            let is_radiant = m.player_slot < 128;
            (is_radiant && m.radiant_win == 1) || (!is_radiant && m.radiant_win == 0)
        })
        .collect();

    let is_loss_streak = recent.len() >= 3 && recent.iter().take(3).all(|&w| !w);

    // Check for a long session (4+ games in the last 6 hours)
    let six_hours_ago = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
        - 6 * 3600;
    let session_count: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM matches WHERE start_time > ?1",
            rusqlite::params![six_hours_ago],
            |row| row.get(0),
        )
        .map_err(|e| format!("DB error counting session games: {}", e))?;
    let is_long_session = session_count >= 4;

    // ~25% random trigger (deterministic per match)
    let is_random_trigger = match_id % 4 == 0;

    if is_loss_streak || is_long_session || is_random_trigger {
        Ok(Some(PendingCheckin {
            match_id,
            is_loss_streak,
        }))
    } else {
        Ok(None)
    }
}

/// Save a mood check-in linked to a match
#[tauri::command]
fn save_mood_checkin(
    match_id: i64,
    energy: i32,
    calm: i32,
    attribution: Option<String>,
) -> Result<(), String> {
    let conn = init_db()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    conn.execute(
        "INSERT OR REPLACE INTO mood_checkins \
         (match_id, checked_at, energy, calm, attribution, skipped) \
         VALUES (?1, ?2, ?3, ?4, ?5, 0)",
        rusqlite::params![match_id, now, energy, calm, attribution],
    )
    .map_err(|e| format!("Failed to save mood checkin: {}", e))?;
    Ok(())
}

/// Dismiss a check-in for a match (records a skip so it won't appear again)
#[tauri::command]
fn dismiss_checkin(match_id: i64) -> Result<(), String> {
    let conn = init_db()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    conn.execute(
        "INSERT OR IGNORE INTO mood_checkins (match_id, checked_at, skipped) VALUES (?1, ?2, 1)",
        rusqlite::params![match_id, now],
    )
    .map_err(|e| format!("Failed to dismiss checkin: {}", e))?;
    Ok(())
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

        // Store lane role
        let role = player_data.lane_role.unwrap_or(0);
        let _ = update_match_role(&conn, match_id, role);

        // Store per-minute networth for all players (used by PartnerNetworth goals)
        for p in &detailed_match.players {
            if let Some(nw_t) = &p.net_worth {
                let _ = insert_player_networth(&conn, match_id, p.player_slot, nw_t);
            }
        }
        // Identify and store lane partner slot
        let partner =
            opendota::find_lane_partner(&detailed_match.players, player_data.player_slot, role);
        let _ = update_match_partner_slot(&conn, match_id, partner.map(|p| p.player_slot));

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
    if let Some(dir) = get_db_dir() {
        return Ok(dir.to_string_lossy().to_string());
    }
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
                // Store lane role
                let role = player_data.lane_role.unwrap_or(0);
                let _ = update_match_role(&conn, m.match_id, role);

                // Store per-minute networth for all players (used by PartnerNetworth goals)
                for p in &detailed_match.players {
                    if let Some(nw_t) = &p.net_worth {
                        let _ = insert_player_networth(&conn, m.match_id, p.player_slot, nw_t);
                    }
                }
                // Identify and store lane partner slot
                let partner = opendota::find_lane_partner(
                    &detailed_match.players,
                    player_data.player_slot,
                    role,
                );
                let _ =
                    update_match_partner_slot(&conn, m.match_id, partner.map(|p| p.player_slot));

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
                // Store lane role
                let role = player_data.lane_role.unwrap_or(0);
                let _ = update_match_role(&conn, m.match_id, role);

                // Store per-minute networth for all players (used by PartnerNetworth goals)
                for p in &detailed_match.players {
                    if let Some(nw_t) = &p.net_worth {
                        let _ = insert_player_networth(&conn, m.match_id, p.player_slot, nw_t);
                    }
                }
                // Identify and store lane partner slot
                let partner = opendota::find_lane_partner(
                    &detailed_match.players,
                    player_data.player_slot,
                    role,
                );
                let _ =
                    update_match_partner_slot(&conn, m.match_id, partner.map(|p| p.player_slot));

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

/// Get per-minute CS data for a specific match
#[tauri::command]
fn get_match_cs(match_id: i64) -> Result<Vec<MatchCS>, String> {
    let conn = init_db()?;
    get_match_cs_data(&conn, match_id)
}

/// Get (or generate) today's daily challenge
#[tauri::command]
fn get_daily_challenge() -> Result<Option<DailyChallenge>, String> {
    let conn = init_db()?;
    get_or_generate_daily_challenge(&conn)
}

/// Get today's daily challenge with evaluated progress
#[tauri::command]
fn get_daily_challenge_progress_cmd() -> Result<Option<DailyChallengeProgress>, String> {
    let conn = init_db()?;
    get_daily_challenge_progress(&conn)
}

/// Get the current daily challenge completion streak
#[tauri::command]
fn get_daily_streak_cmd() -> Result<i32, String> {
    let conn = init_db()?;
    get_daily_streak(&conn)
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

// ===== Weekly Challenge Commands =====

#[tauri::command]
fn get_weekly_challenge_options_cmd() -> Result<Vec<ChallengeOption>, String> {
    let conn = init_db()?;
    get_weekly_challenge_options(&conn)
}

#[tauri::command]
fn reroll_weekly_challenges_cmd() -> Result<Vec<ChallengeOption>, String> {
    let conn = init_db()?;
    reroll_weekly_challenges(&conn)
}

#[tauri::command]
fn skip_weekly_challenge_cmd() -> Result<(), String> {
    let conn = init_db()?;
    skip_weekly_challenge(&conn)
}

#[tauri::command]
fn accept_weekly_challenge_cmd(option_id: i64) -> Result<WeeklyChallenge, String> {
    let conn = init_db()?;
    accept_weekly_challenge(&conn, option_id)
}

#[tauri::command]
fn get_active_weekly_challenge_cmd() -> Result<Option<WeeklyChallenge>, String> {
    let conn = init_db()?;
    get_active_weekly_challenge(&conn)
}

#[tauri::command]
fn get_weekly_challenge_progress_cmd() -> Result<Option<WeeklyChallengeProgress>, String> {
    let conn = init_db()?;
    get_weekly_challenge_progress(&conn)
}

#[tauri::command]
fn get_challenge_history_cmd(
    challenge_type: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<ChallengeHistoryItem>, String> {
    let conn = init_db()?;
    get_challenge_history(&conn, challenge_type, limit.unwrap_or(50))
}

// ── Steam OpenID login ────────────────────────────────────────────────────────

fn percent_encode(input: &str) -> String {
    let mut out = String::new();
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("00");
            if let Ok(b) = u8::from_str_radix(hex, 16) {
                out.push(b as char);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            out.push(' ');
            i += 1;
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

/// Start a Steam OpenID login flow.
/// Binds a temporary local HTTP server, returns the Steam authorisation URL.
/// When Steam redirects back, the callback is verified and a `steam-login-complete`
/// event is emitted with `{steam_id}` on success or `{error}` on failure.
#[tauri::command]
async fn start_steam_login(app: tauri::AppHandle) -> Result<String, String> {
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| format!("Failed to bind port: {e}"))?;

    let port = listener
        .local_addr()
        .map_err(|e| format!("Failed to get local address: {e}"))?
        .port();

    let return_to = format!("http://127.0.0.1:{port}/callback");
    let realm = format!("http://127.0.0.1:{port}");

    let steam_url = format!(
        "https://steamcommunity.com/openid/login\
         ?openid.ns=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0\
         &openid.mode=checkid_setup\
         &openid.return_to={}\
         &openid.realm={}\
         &openid.identity=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select\
         &openid.claimed_id=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select",
        percent_encode(&return_to),
        percent_encode(&realm),
    );

    tokio::spawn(handle_steam_callback(listener, app));

    Ok(steam_url)
}

async fn handle_steam_callback(listener: tokio::net::TcpListener, app: tauri::AppHandle) {
    use tokio::io::{AsyncBufReadExt, BufReader};

    let (stream, _) = match listener.accept().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Steam callback: failed to accept connection: {e}");
            let _ = app.emit(
                "steam-login-complete",
                serde_json::json!({"error": "Connection failed"}),
            );
            return;
        }
    };

    let (reader_half, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader_half);

    // Read the HTTP request line ("GET /callback?... HTTP/1.1")
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).await.is_err() {
        let _ = app.emit(
            "steam-login-complete",
            serde_json::json!({"error": "Failed to read request"}),
        );
        return;
    }

    // Drain remaining HTTP headers before sending a response
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line).await {
            Ok(0) | Err(_) => break,
            Ok(_) if line == "\r\n" || line == "\n" => break,
            _ => {}
        }
    }

    // Extract query string from "GET /callback?<qs> HTTP/1.1"
    let path_part = request_line.split_whitespace().nth(1).unwrap_or("");
    let query_string = path_part.split_once('?').map(|(_, q)| q).unwrap_or("");

    // Parse all OpenID parameters
    let mut params: Vec<(String, String)> = Vec::new();
    let mut claimed_id: Option<String> = None;
    let mut openid_mode: Option<String> = None;

    for pair in query_string.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            let key = percent_decode(k);
            let val = percent_decode(v);
            if key == "openid.claimed_id" {
                claimed_id = Some(val.clone());
            }
            if key == "openid.mode" {
                openid_mode = Some(val.clone());
            }
            params.push((key, val));
        }
    }

    // Steam sends mode=id_res on success, mode=cancel on cancellation
    if openid_mode.as_deref() != Some("id_res") {
        let _ =
            steam_html_response(&mut writer, 400, "Login cancelled. You can close this tab.").await;
        let _ = app.emit(
            "steam-login-complete",
            serde_json::json!({"error": "Login cancelled"}),
        );
        return;
    }

    // Extract Steam64 ID from "https://steamcommunity.com/openid/id/{ID}"
    let steam_id = match claimed_id
        .as_deref()
        .and_then(|id| id.strip_prefix("https://steamcommunity.com/openid/id/"))
        .map(|s| s.trim().to_string())
    {
        Some(id) if !id.is_empty() => id,
        _ => {
            let _ = steam_html_response(
                &mut writer,
                400,
                "Could not extract Steam ID. Please try again.",
            )
            .await;
            let _ = app.emit(
                "steam-login-complete",
                serde_json::json!({"error": "Could not extract Steam ID"}),
            );
            return;
        }
    };

    // Verify the assertion with Steam (switch mode to check_authentication)
    let verify_params: Vec<(String, String)> = params
        .into_iter()
        .map(|(k, v)| {
            if k == "openid.mode" {
                (k, "check_authentication".to_string())
            } else {
                (k, v)
            }
        })
        .collect();

    let verified = match reqwest::Client::new()
        .post("https://steamcommunity.com/openid/login")
        .form(&verify_params)
        .send()
        .await
    {
        Ok(resp) => resp
            .text()
            .await
            .unwrap_or_default()
            .contains("is_valid:true"),
        Err(e) => {
            eprintln!("Steam OpenID verification request failed: {e}");
            false
        }
    };

    if verified {
        let _ = steam_html_response(
            &mut writer,
            200,
            "Steam login successful! You can close this tab and return to Dota Keeper.",
        )
        .await;
        let _ = app.emit(
            "steam-login-complete",
            serde_json::json!({"steam_id": steam_id}),
        );
    } else {
        let _ = steam_html_response(
            &mut writer,
            400,
            "Steam verification failed. Please try again.",
        )
        .await;
        let _ = app.emit(
            "steam-login-complete",
            serde_json::json!({"error": "Steam verification failed"}),
        );
    }
}

async fn steam_html_response(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    status: u16,
    message: &str,
) -> Result<(), std::io::Error> {
    use tokio::io::AsyncWriteExt;
    let (status_text, color) = if status == 200 {
        ("OK", "#f0b429")
    } else {
        ("Bad Request", "#e53e3e")
    };
    let body = format!(
        "<html><head><title>Dota Keeper</title></head>\
         <body style='font-family:sans-serif;background:#1a1a1a;color:{color};\
         text-align:center;padding:60px'><h2>{message}</h2></body></html>"
    );
    let response = format!(
        "HTTP/1.1 {status} {status_text}\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\r\n{}",
        body.len(),
        body
    );
    writer.write_all(response.as_bytes()).await
}

// ── end Steam OpenID login ────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialise storage directories from Tauri's platform-aware path API.
            // This works on desktop, Android, and iOS.
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("could not resolve app data directory");
            set_db_dir(app_data_dir.clone());
            set_settings_dir(app_data_dir);
            Ok(())
        })
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
            get_match_item_timings,
            get_match_cs,
            get_daily_challenge,
            get_daily_challenge_progress_cmd,
            get_daily_streak_cmd,
            save_suggestion_difficulty,
            save_analytics_consent,
            identify_analytics_user,
            track_event,
            get_weekly_challenge_options_cmd,
            reroll_weekly_challenges_cmd,
            skip_weekly_challenge_cmd,
            accept_weekly_challenge_cmd,
            get_active_weekly_challenge_cmd,
            get_weekly_challenge_progress_cmd,
            get_challenge_history_cmd,
            start_steam_login,
            save_mental_health_enabled,
            mark_mental_health_intro_shown,
            clear_mood_data,
            get_pending_checkin,
            save_mood_checkin,
            dismiss_checkin
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
