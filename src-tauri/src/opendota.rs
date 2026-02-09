use crate::database::{Match, MatchState};
use serde::Deserialize;

const OPENDOTA_API_BASE: &str = "https://api.opendota.com/api";

/// Response from the OpenDota recent matches endpoint
#[derive(Debug, Deserialize)]
struct OpenDotaMatch {
    match_id: i64,
    hero_id: i32,
    start_time: i64,
    duration: i32,
    game_mode: i32,
    lobby_type: i32,
    radiant_win: Option<bool>,
    player_slot: i32,
    kills: i32,
    deaths: i32,
    assists: i32,
    xp_per_min: Option<i32>,
    gold_per_min: Option<i32>,
    last_hits: Option<i32>,
    denies: Option<i32>,
    hero_damage: Option<i32>,
    tower_damage: Option<i32>,
    hero_healing: Option<i32>,
}

impl From<OpenDotaMatch> for Match {
    fn from(m: OpenDotaMatch) -> Self {
        Match {
            match_id: m.match_id,
            hero_id: m.hero_id,
            start_time: m.start_time,
            duration: m.duration,
            game_mode: m.game_mode,
            lobby_type: m.lobby_type,
            radiant_win: m.radiant_win.unwrap_or(false),
            player_slot: m.player_slot,
            kills: m.kills,
            deaths: m.deaths,
            assists: m.assists,
            xp_per_min: m.xp_per_min.unwrap_or(0),
            gold_per_min: m.gold_per_min.unwrap_or(0),
            last_hits: m.last_hits.unwrap_or(0),
            denies: m.denies.unwrap_or(0),
            hero_damage: m.hero_damage.unwrap_or(0),
            tower_damage: m.tower_damage.unwrap_or(0),
            hero_healing: m.hero_healing.unwrap_or(0),
            parse_state: MatchState::Unparsed,
        }
    }
}

/// Detailed match data from OpenDota
#[derive(Debug, Deserialize)]
pub struct DetailedMatch {
    pub match_id: i64,
    pub players: Vec<DetailedPlayer>,
}

#[derive(Debug, Deserialize)]
pub struct DetailedPlayer {
    pub account_id: Option<u32>,
    pub player_slot: i32,
    pub lh_t: Option<Vec<i32>>,  // Last hits at each minute
    pub dn_t: Option<Vec<i32>>,  // Denies at each minute
}

/// Parse request status
#[derive(Debug, Deserialize)]
pub struct ParseRequestResponse {
    pub job: Option<JobInfo>,
}

#[derive(Debug, Deserialize)]
pub struct JobInfo {
    pub jobId: Option<i64>,
}

/// Convert Steam ID64 to Steam ID32 (account ID) for OpenDota API
fn steam_id64_to_id32(steam_id64: &str) -> Result<u32, String> {
    let id64: u64 = steam_id64
        .parse()
        .map_err(|_| "Invalid Steam ID format".to_string())?;

    // Steam ID32 = Steam ID64 - 76561197960265728
    const STEAM_ID64_BASE: u64 = 76561197960265728;

    if id64 < STEAM_ID64_BASE {
        // Already a Steam ID32
        return Ok(id64 as u32);
    }

    Ok((id64 - STEAM_ID64_BASE) as u32)
}

/// Fetch recent matches from OpenDota API
pub async fn fetch_recent_matches(steam_id: &str, limit: usize) -> Result<Vec<Match>, String> {
    let account_id = steam_id64_to_id32(steam_id)?;

    let url = format!(
        "{}/players/{}/recentMatches",
        OPENDOTA_API_BASE, account_id
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch matches: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "OpenDota API returned error: {}",
            response.status()
        ));
    }

    let matches: Vec<OpenDotaMatch> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse matches: {}", e))?;

    // Take only the requested number of matches
    let matches: Vec<Match> = matches
        .into_iter()
        .take(limit)
        .map(|m| m.into())
        .collect();

    Ok(matches)
}

/// Fetch matches before a specific timestamp using the matches endpoint
pub async fn fetch_matches_before(
    steam_id: &str,
    before_timestamp: i64,
    limit: usize,
) -> Result<Vec<Match>, String> {
    let account_id = steam_id64_to_id32(steam_id)?;
    let client = reqwest::Client::new();

    let mut all_matches: Vec<Match> = Vec::new();
    let mut offset = 0;
    const BATCH_SIZE: usize = 100;

    // Fetch matches in batches until we have enough matches before the timestamp
    while all_matches.len() < limit && offset < 500 {
        // Limit to 500 total matches to avoid infinite loops
        let url = format!(
            "{}/players/{}/matches?limit={}&offset={}",
            OPENDOTA_API_BASE, account_id, BATCH_SIZE, offset
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch matches: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "OpenDota API returned error: {}",
                response.status()
            ));
        }

        let matches: Vec<OpenDotaMatch> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse matches: {}", e))?;

        // If no more matches, break
        if matches.is_empty() {
            break;
        }

        // Filter and collect matches that are before the timestamp
        for m in matches {
            if m.start_time < before_timestamp {
                all_matches.push(m.into());
                if all_matches.len() >= limit {
                    break;
                }
            }
        }

        offset += BATCH_SIZE;

        // Small delay to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    Ok(all_matches)
}

/// Request OpenDota to parse a match
pub async fn request_match_parse(match_id: i64) -> Result<(), String> {
    let url = format!("{}/request/{}", OPENDOTA_API_BASE, match_id);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to request match parse: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "OpenDota API returned error: {}",
            response.status()
        ));
    }

    Ok(())
}

/// Fetch detailed match data from OpenDota
pub async fn fetch_match_details(match_id: i64) -> Result<DetailedMatch, String> {
    let url = format!("{}/matches/{}", OPENDOTA_API_BASE, match_id);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch match details: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "OpenDota API returned error: {}",
            response.status()
        ));
    }

    let match_details: DetailedMatch = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse match details: {}", e))?;

    Ok(match_details)
}
