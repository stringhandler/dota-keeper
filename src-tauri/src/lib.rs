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

/// Save the check-in frequency preference
#[tauri::command]
fn save_checkin_frequency(frequency: String) -> Result<Settings, String> {
    let mut settings = Settings::load();
    settings.checkin_frequency = frequency;
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

    // Check for 3+ consecutive losses (look at 5 most recent matches) — always overrides frequency
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

    // Check frequency-based trigger condition
    let frequency = settings.checkin_frequency.as_str();
    let frequency_triggered = match frequency {
        // every_game and once_per_session: always return (frontend manages session limit)
        "every_game" | "once_per_session" => true,

        // Threshold-based: count unchecked matches since the last successful check-in
        "every_3" | "every_5" | "every_10" => {
            let threshold: i64 = match frequency {
                "every_3" => 3,
                "every_5" => 5,
                _ => 10,
            };
            // Count matches not in mood_checkins that are newer than the last successful check-in
            let unchecked_count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM matches \
                     WHERE match_id NOT IN (SELECT match_id FROM mood_checkins) \
                     AND start_time > COALESCE( \
                         (SELECT m.start_time FROM mood_checkins mc \
                          JOIN matches m ON mc.match_id = m.match_id \
                          WHERE mc.skipped = 0 \
                          ORDER BY m.start_time DESC LIMIT 1), \
                         0)",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0);
            unchecked_count >= threshold
        }

        // after_loss: only trigger if the pending match was a loss
        "after_loss" => {
            let result: Option<(i32, i32)> = conn
                .query_row(
                    "SELECT radiant_win, player_slot FROM matches WHERE match_id = ?1",
                    rusqlite::params![match_id],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .optional()
                .unwrap_or(None);
            result.map_or(false, |(radiant_win, player_slot)| {
                let is_radiant = player_slot < 128;
                (is_radiant && radiant_win == 0) || (!is_radiant && radiant_win == 1)
            })
        }

        // Unknown value: fall back to once_per_session behaviour
        _ => true,
    };

    if is_loss_streak || frequency_triggered {
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

// ── Mental Health: Tilt Assessment ──────────────────────────────────────────

#[derive(Debug, serde::Serialize)]
struct TiltSignal {
    signal_type: String,
    value: f64,
    threshold: f64,
}

#[derive(Debug, serde::Serialize)]
struct TiltSuggestion {
    title: String,
    body: String,
    severity: String,
    actions: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
struct TiltAssessment {
    tilt_score: i32,
    pattern: String,
    signals: Vec<TiltSignal>,
    energy_avg_7d: Option<f64>,
    calm_avg_7d: Option<f64>,
    trend: String,
    has_sufficient_data: bool,
    suggestion: Option<TiltSuggestion>,
}

/// Compute a tilt assessment from objective match data + subjective mood check-ins.
/// Returns None if mental health tracking is disabled.
#[tauri::command]
fn get_tilt_assessment() -> Result<Option<TiltAssessment>, String> {
    let settings = Settings::load();
    if !settings.mental_health_tracking_enabled {
        return Ok(None);
    }

    let conn = init_db()?;

    // ── Objective Signals ──────────────────────────────────────────────────
    let mut obj_score: f64 = 0.0;
    let mut signals: Vec<TiltSignal> = Vec::new();

    // Loss streak — look at last 5 matches
    let mut stmt = conn
        .prepare("SELECT radiant_win, player_slot FROM matches ORDER BY start_time DESC LIMIT 5")
        .map_err(|e| format!("DB error: {}", e))?;
    let recent_results: Vec<bool> = stmt
        .query_map([], |row| {
            let rw: i32 = row.get(0)?;
            let ps: i32 = row.get(1)?;
            Ok((ps < 128 && rw == 1) || (ps >= 128 && rw == 0))
        })
        .map_err(|e| format!("DB: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    let loss_streak_len = recent_results.iter().take_while(|&&w| !w).count();
    let has_loss_streak = loss_streak_len >= 3;
    if has_loss_streak {
        obj_score += 20.0;
        signals.push(TiltSignal {
            signal_type: "loss_streak".to_string(),
            value: loss_streak_len as f64,
            threshold: 3.0,
        });
    }

    // Deaths elevated — last 3 vs 30-game baseline
    struct MatchStats {
        deaths: f64,
        kills: f64,
        assists: f64,
    }
    let mut stat_stmt = conn
        .prepare("SELECT deaths, kills, assists FROM matches ORDER BY start_time DESC LIMIT 30")
        .map_err(|e| format!("DB error: {}", e))?;
    let all_stats: Vec<MatchStats> = stat_stmt
        .query_map([], |row| {
            Ok(MatchStats {
                deaths: row.get::<_, f64>(0)?,
                kills: row.get::<_, f64>(1)?,
                assists: row.get::<_, f64>(2)?,
            })
        })
        .map_err(|e| format!("DB: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    let deaths_elevated;
    let kda_depressed;
    if all_stats.len() >= 6 {
        let last3_deaths = all_stats.iter().take(3).map(|s| s.deaths).sum::<f64>() / 3.0;
        let baseline_deaths = all_stats.iter().map(|s| s.deaths).sum::<f64>() / all_stats.len() as f64;
        deaths_elevated = baseline_deaths > 0.0 && last3_deaths >= baseline_deaths * 1.5;
        if deaths_elevated {
            obj_score += 25.0;
            signals.push(TiltSignal {
                signal_type: "deaths_elevated".to_string(),
                value: last3_deaths,
                threshold: baseline_deaths * 1.5,
            });
        }

        let kda = |s: &MatchStats| (s.kills + s.assists) / (s.deaths + 1.0);
        let last3_kda = all_stats.iter().take(3).map(kda).sum::<f64>() / 3.0;
        let baseline_kda = all_stats.iter().map(kda).sum::<f64>() / all_stats.len() as f64;
        kda_depressed = baseline_kda > 0.0 && last3_kda <= baseline_kda * 0.6;
        if kda_depressed {
            obj_score += 20.0;
            signals.push(TiltSignal {
                signal_type: "kda_depressed".to_string(),
                value: last3_kda,
                threshold: baseline_kda * 0.6,
            });
        }
    } else {
        deaths_elevated = false;
        kda_depressed = false;
    }

    // Session length — games in the last 24 hours
    let day_ago = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
        - 86400;
    let session_games: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM matches WHERE start_time > ?1",
            rusqlite::params![day_ago],
            |row| row.get(0),
        )
        .unwrap_or(0);
    let long_session = session_games >= 5;
    if long_session {
        obj_score += 15.0;
        signals.push(TiltSignal {
            signal_type: "long_session".to_string(),
            value: session_games as f64,
            threshold: 5.0,
        });
    }

    // ── Subjective Signals ─────────────────────────────────────────────────
    struct RecentCheckin {
        energy: Option<i32>,
        calm: Option<i32>,
        attribution: Option<String>,
    }
    let mut ci_stmt = conn
        .prepare(
            "SELECT energy, calm, attribution FROM mood_checkins \
             WHERE skipped = 0 AND hidden = 0 \
             ORDER BY checked_at DESC LIMIT 10",
        )
        .map_err(|e| format!("DB error: {}", e))?;
    let checkins: Vec<RecentCheckin> = ci_stmt
        .query_map([], |row| {
            Ok(RecentCheckin {
                energy: row.get(0)?,
                calm: row.get(1)?,
                attribution: row.get(2)?,
            })
        })
        .map_err(|e| format!("DB: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    let has_checkin_data = !checkins.is_empty();
    let mut subj_score: f64 = 0.0;

    let last_energy = checkins.first().and_then(|c| c.energy);
    let last_calm = checkins.first().and_then(|c| c.calm);

    if let Some(e) = last_energy {
        if e <= 2 {
            subj_score += 10.0;
            signals.push(TiltSignal {
                signal_type: "low_energy".to_string(),
                value: e as f64,
                threshold: 2.0,
            });
        }
    }
    if let Some(c) = last_calm {
        if c <= 2 {
            subj_score += 10.0;
            signals.push(TiltSignal {
                signal_type: "low_calm".to_string(),
                value: c as f64,
                threshold: 2.0,
            });
        }
    }

    // Calm trend — is it declining over last 3 check-ins?
    let calm_trend_declining = checkins.len() >= 3 && {
        let calms: Vec<i32> = checkins.iter().take(3).filter_map(|c| c.calm).collect();
        calms.len() == 3 && calms[0] < calms[1] && calms[1] <= calms[2]
    };
    if calm_trend_declining {
        subj_score += 15.0;
        signals.push(TiltSignal {
            signal_type: "calm_declining".to_string(),
            value: 1.0,
            threshold: 1.0,
        });
    }

    // Team attribution count in last 5 non-skipped check-ins
    let team_attrib_count = checkins.iter().take(5)
        .filter(|c| c.attribution.as_deref() == Some("Teammates"))
        .count();
    if team_attrib_count >= 2 {
        subj_score += 10.0;
        signals.push(TiltSignal {
            signal_type: "team_friction".to_string(),
            value: team_attrib_count as f64,
            threshold: 2.0,
        });
    }
    let self_attrib_count = checkins.iter().take(5)
        .filter(|c| c.attribution.as_deref() == Some("My own mistakes"))
        .count();

    // Composite tilt score
    let tilt_score = if has_checkin_data {
        (obj_score * 0.6 + subj_score * 0.4).min(100.0) as i32
    } else {
        obj_score.min(100.0) as i32
    };

    // ── 7-day averages ─────────────────────────────────────────────────────
    let seven_days_ago = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
        - 7 * 86400;

    let mut avg_stmt = conn
        .prepare(
            "SELECT energy, calm FROM mood_checkins \
             WHERE skipped = 0 AND hidden = 0 AND checked_at > ?1",
        )
        .map_err(|e| format!("DB error: {}", e))?;
    struct AvgRow { energy: Option<i32>, calm: Option<i32> }
    let week_checkins: Vec<AvgRow> = avg_stmt
        .query_map(rusqlite::params![seven_days_ago], |row| {
            Ok(AvgRow { energy: row.get(0)?, calm: row.get(1)? })
        })
        .map_err(|e| format!("DB: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    let energy_avg_7d = if week_checkins.is_empty() {
        None
    } else {
        let vals: Vec<f64> = week_checkins.iter().filter_map(|r| r.energy.map(|v| v as f64)).collect();
        if vals.is_empty() { None } else { Some(vals.iter().sum::<f64>() / vals.len() as f64) }
    };
    let calm_avg_7d = if week_checkins.is_empty() {
        None
    } else {
        let vals: Vec<f64> = week_checkins.iter().filter_map(|r| r.calm.map(|v| v as f64)).collect();
        if vals.is_empty() { None } else { Some(vals.iter().sum::<f64>() / vals.len() as f64) }
    };

    // ── Trend ──────────────────────────────────────────────────────────────
    let trend = if checkins.len() < 3 {
        "insufficient_data".to_string()
    } else {
        // Compare avg of last 3 vs next 3
        let last3_calm: Vec<f64> = checkins.iter().take(3).filter_map(|c| c.calm.map(|v| v as f64)).collect();
        let prev3_calm: Vec<f64> = checkins.iter().skip(3).take(3).filter_map(|c| c.calm.map(|v| v as f64)).collect();
        if last3_calm.len() < 3 || prev3_calm.len() < 3 {
            "insufficient_data".to_string()
        } else {
            let last_avg = last3_calm.iter().sum::<f64>() / 3.0;
            let prev_avg = prev3_calm.iter().sum::<f64>() / 3.0;
            if last_avg >= prev_avg + 0.4 {
                "improving".to_string()
            } else if last_avg <= prev_avg - 0.4 {
                "declining".to_string()
            } else {
                "stable".to_string()
            }
        }
    };

    // ── Pattern Classification ─────────────────────────────────────────────
    let pattern = if tilt_score <= 20 && !has_loss_streak {
        "peak_performance".to_string()
    } else if long_session || last_energy.map_or(false, |e| e <= 2) {
        "fatigue".to_string()
    } else if has_loss_streak && deaths_elevated {
        "loss_spiral".to_string()
    } else if team_attrib_count >= 2 {
        "team_friction".to_string()
    } else if self_attrib_count >= 2 && calm_trend_declining {
        "self_doubt".to_string()
    } else if has_loss_streak {
        "loss_spiral".to_string()
    } else {
        "mild_tilt".to_string()
    };

    // ── Suggestion ─────────────────────────────────────────────────────────
    let severity = if tilt_score >= 56 { "high" } else if tilt_score >= 31 { "mild" } else { "none" };
    let suggestion = if tilt_score < 31 {
        if pattern == "peak_performance" {
            Some(TiltSuggestion {
                title: "You're in the zone".to_string(),
                body: "You're playing well and feeling good about it. This is the state that produces your best Dota — notice what's working so you can recreate it.".to_string(),
                severity: "positive".to_string(),
                actions: vec![],
            })
        } else {
            None
        }
    } else {
        let (title, body, actions) = match (pattern.as_str(), severity) {
            ("fatigue", "high") => (
                "You've been playing a lot",
                "5+ games in one sitting is a lot. Fatigue is one of the biggest hidden performance drains in Dota. A 15-minute break — walk, water, stretch — often makes the next game feel completely different.",
                vec!["Take a Break".to_string(), "Play One More Anyway".to_string()],
            ),
            ("fatigue", _) => (
                "Consider taking a break",
                "You've played quite a bit today. Even top pros schedule regular breaks — your reaction time and decision-making genuinely improve with rest.",
                vec!["Take a Break".to_string()],
            ),
            ("loss_spiral", "high") => (
                "You're in a loss streak",
                "You're in a loss streak and your deaths are climbing. This is the classic tilt spiral — each loss makes the next one more likely. The single most effective thing you can do right now is stop and come back in an hour.",
                vec!["Step Away".to_string(), "Switch to Turbo for Fun".to_string(), "Keep Grinding".to_string()],
            ),
            ("loss_spiral", _) => (
                "3 losses in a row",
                "It happens to everyone. Consider whether a different hero or role might give you a mental reset.",
                vec!["Switch Hero".to_string(), "Take a Break".to_string()],
            ),
            ("team_friction", "high") => (
                "Frustration with teammates",
                "When frustration comes from teammates, it often makes us play more aggressively or alone — which backfires. Try focusing purely on your own farm, positioning, and cooldown usage this next game.",
                vec!["Focus on Self".to_string()],
            ),
            ("team_friction", _) => (
                "Rough teammates lately?",
                "It's worth remembering that in a 5-player team game, your individual impact is highest when you focus on what you can control.",
                vec![],
            ),
            ("self_doubt", "high") => (
                "Paralysis by analysis",
                "You seem focused on your own errors. Replaying mistakes mentally during a game is called 'paralysis by analysis' — try to notice mistakes once, then let them go and focus on the next decision.",
                vec![],
            ),
            ("self_doubt", _) => (
                "You're being self-aware",
                "That self-awareness is actually a strength — most players never notice their mistakes at all.",
                vec![],
            ),
            _ => (
                "Performance dip detected",
                "Your recent games show some signs of tilt. Consider taking a short break or switching to a more comfortable hero.",
                vec!["Take a Break".to_string()],
            ),
        };
        Some(TiltSuggestion {
            title: title.to_string(),
            body: body.to_string(),
            severity: severity.to_string(),
            actions,
        })
    };

    Ok(Some(TiltAssessment {
        tilt_score,
        pattern,
        signals,
        energy_avg_7d,
        calm_avg_7d,
        trend,
        has_sufficient_data: checkins.len() >= 3,
        suggestion,
    }))
}

// ── Mental Health: Check-in History ─────────────────────────────────────────

#[derive(Debug, serde::Serialize)]
struct CheckinHistoryItem {
    match_id: i64,
    checked_at: i64,
    energy: Option<i32>,
    calm: Option<i32>,
    attribution: Option<String>,
    skipped: bool,
    hero_id: Option<i32>,
    won: Option<bool>,
    match_start_time: Option<i64>,
}

/// Return the most recent check-in history entries (including skipped ones).
#[tauri::command]
fn get_checkin_history(limit: i32) -> Result<Vec<CheckinHistoryItem>, String> {
    let settings = Settings::load();
    if !settings.mental_health_tracking_enabled {
        return Ok(vec![]);
    }

    let conn = init_db()?;
    let mut stmt = conn
        .prepare(
            "SELECT mc.match_id, mc.checked_at, mc.energy, mc.calm, mc.attribution, mc.skipped, \
                    m.hero_id, m.radiant_win, m.player_slot, m.start_time \
             FROM mood_checkins mc \
             LEFT JOIN matches m ON mc.match_id = m.match_id \
             WHERE mc.hidden = 0 \
             ORDER BY mc.checked_at DESC \
             LIMIT ?1",
        )
        .map_err(|e| format!("DB error: {}", e))?;

    let items: Vec<CheckinHistoryItem> = stmt
        .query_map(rusqlite::params![limit], |row| {
            let radiant_win: Option<i32> = row.get(7)?;
            let player_slot: Option<i32> = row.get(8)?;
            let won = match (radiant_win, player_slot) {
                (Some(rw), Some(ps)) => Some((ps < 128 && rw == 1) || (ps >= 128 && rw == 0)),
                _ => None,
            };
            Ok(CheckinHistoryItem {
                match_id: row.get(0)?,
                checked_at: row.get(1)?,
                energy: row.get(2)?,
                calm: row.get(3)?,
                attribution: row.get(4)?,
                skipped: row.get::<_, i32>(5)? == 1,
                hero_id: row.get(6)?,
                won,
                match_start_time: row.get(9)?,
            })
        })
        .map_err(|e| format!("DB error: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(items)
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
            save_checkin_frequency,
            mark_mental_health_intro_shown,
            clear_mood_data,
            get_pending_checkin,
            save_mood_checkin,
            dismiss_checkin,
            get_tilt_assessment,
            get_checkin_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
