use crate::database::{Match, MatchState};
use serde::Deserialize;
use tracing::debug;

const OPENDOTA_API_BASE: &str = "https://api.opendota.com/api";

// ── User-friendly error helpers ─────────────────────────────────────────────

fn friendly_network_err(err: &reqwest::Error) -> String {
    if err.is_connect() || err.is_timeout() {
        "Couldn't reach OpenDota — check your internet connection. Matches will sync automatically when you're back online.".to_string()
    } else {
        "Network error while contacting OpenDota. Will retry automatically.".to_string()
    }
}

fn friendly_status_err(status: reqwest::StatusCode) -> String {
    match status.as_u16() {
        429 => "Too many requests to OpenDota. Will try again shortly.".to_string(),
        404 => "No data found for this request. The match may not be tracked by OpenDota yet.".to_string(),
        500..=599 => "OpenDota is unavailable right now. Your matches will sync automatically when it comes back.".to_string(),
        code => format!("OpenDota returned an unexpected error (HTTP {}). Will retry later.", code),
    }
}

/// Send an OpenDota API error to Sentry for monitoring.
/// Uses endpoint names only — no account IDs or match IDs are included.
fn log_api_error(endpoint: &str, status: u16) {
    let level = match status {
        429 => sentry::Level::Warning,
        500..=599 => sentry::Level::Error,
        _ => sentry::Level::Warning,
    };
    sentry::with_scope(
        |scope| {
            scope.set_tag("component", "opendota");
            scope.set_tag("endpoint", endpoint);
            scope.set_tag("http_status", status.to_string());
        },
        || {
            sentry::capture_message(
                &format!("OpenDota {} HTTP {}", endpoint, status),
                level,
            );
        },
    );
}

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
    rank_tier: Option<i32>,
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
            role: 0,
            rank_tier: m.rank_tier,
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
    pub lane_role: Option<i32>,  // 1=carry, 2=mid, 3=offlane, 4=soft support, 5=hard support
    pub lh_t: Option<Vec<i32>>,  // Last hits at each minute
    pub dn_t: Option<Vec<i32>>,  // Denies at each minute
    pub gold_t: Option<Vec<i32>>,  // Gold (net worth) at each minute — OpenDota field name
    pub purchase_log: Option<Vec<PurchaseLogEntry>>,  // Item purchases
}

#[derive(Debug, Deserialize)]
pub struct PurchaseLogEntry {
    pub time: i32,  // Game time in seconds when item was purchased
    pub key: String,  // Item name/key (e.g., "blink", "armlet")
}

/// Find the lane partner for a support player (pos 4 or 5).
/// Pos 5 (hard support) lanes with pos 1 (carry) in the safe lane.
/// Pos 4 (soft support) lanes with pos 3 (offlaner) in the off lane.
/// Returns None if the player is not a support or no partner is found.
pub fn find_lane_partner<'a>(
    all_players: &'a [DetailedPlayer],
    player_slot: i32,
    player_role: i32,
) -> Option<&'a DetailedPlayer> {
    let partner_role = match player_role {
        5 => 1, // hard support partners with carry
        4 => 3, // soft support partners with offlaner
        _ => return None,
    };

    let is_radiant = player_slot < 128;

    all_players.iter().find(|p| {
        p.player_slot != player_slot
            && (p.player_slot < 128) == is_radiant
            && p.lane_role == Some(partner_role)
    })
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
        .map_err(|_| "No matches found for this Steam ID. Double-check the ID in Settings.".to_string())?;

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

    debug!("fetch_recent_matches GET {} limit={}", url, limit);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            let msg = friendly_network_err(&e);
            debug!("fetch_recent_matches network error: {}", msg);
            msg
        })?;

    let status = response.status();
    debug!("fetch_recent_matches response status={}", status.as_u16());

    if !status.is_success() {
        if status.as_u16() == 404 {
            return Err("No matches found for this Steam ID. Double-check the ID in Settings.".to_string());
        }
        log_api_error("recentMatches", status.as_u16());
        return Err(friendly_status_err(status));
    }

    let matches: Vec<OpenDotaMatch> = response
        .json()
        .await
        .map_err(|_| "OpenDota returned unexpected data. Check that your Steam ID is correct in Settings.".to_string())?;

    // Take only the requested number of matches
    let matches: Vec<Match> = matches
        .into_iter()
        .take(limit)
        .map(|m| m.into())
        .collect();

    debug!("fetch_recent_matches returning {} matches", matches.len());
    Ok(matches)
}

/// Fetch matches before a specific timestamp using offset-based pagination
pub async fn fetch_matches_before(
    steam_id: &str,
    before_timestamp: i64,
    limit: usize,
) -> Result<Vec<Match>, String> {
    let account_id = steam_id64_to_id32(steam_id)?;
    let client = reqwest::Client::new();

    debug!("fetch_matches_before: looking for {} matches before timestamp {}", limit, before_timestamp);

    let mut all_matches: Vec<Match> = Vec::new();
    let mut offset = 0;
    const BATCH_SIZE: usize = 100;
    const MAX_ATTEMPTS: usize = 50; // Try up to 5000 matches to find what we need
    let mut attempts = 0;

    while all_matches.len() < limit && attempts < MAX_ATTEMPTS {
        let url = format!(
            "{}/players/{}/matches?limit={}&offset={}&significant=0",
            OPENDOTA_API_BASE, account_id, BATCH_SIZE, offset
        );

        debug!("fetch_matches_before GET {} offset={} collected={}", url, offset, all_matches.len());

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| {
                let msg = friendly_network_err(&e);
                debug!("fetch_matches_before network error at offset {}: {}", offset, msg);
                msg
            })?;

        let status = response.status();
        debug!("fetch_matches_before response status={}", status.as_u16());

        if !status.is_success() {
            log_api_error("matchesBackfill", status.as_u16());
            return Err(friendly_status_err(status));
        }

        let matches: Vec<OpenDotaMatch> = response
            .json()
            .await
            .map_err(|_| "OpenDota returned unexpected data while backfilling. Backfill may be incomplete.".to_string())?;

        debug!("fetch_matches_before received {} matches from API", matches.len());

        // If no more matches, we've reached the end
        if matches.is_empty() {
            debug!("fetch_matches_before: no more matches from API, stopping");
            break;
        }

        // Collect matches that are before the timestamp
        let mut matches_in_range = 0;
        for m in matches {
            if m.start_time < before_timestamp {
                all_matches.push(m.into());
                matches_in_range += 1;
                if all_matches.len() >= limit {
                    break;
                }
            }
        }
        debug!("fetch_matches_before: {} matches in this batch are older than target timestamp", matches_in_range);

        offset += BATCH_SIZE;
        attempts += 1;

        // Small delay to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    }

    // Sort by start_time descending (most recent first)
    all_matches.sort_by(|a, b| b.start_time.cmp(&a.start_time));

    // Take only the requested number of matches
    all_matches.truncate(limit);

    debug!("fetch_matches_before: returning {} total matches", all_matches.len());

    Ok(all_matches)
}

/// Request OpenDota to parse a match.
/// Returns the job ID if OpenDota queued a parse job, or None if the match
/// was already parsed (response had no job).
pub async fn request_match_parse(match_id: i64) -> Result<Option<i64>, String> {
    let url = format!("{}/request/{}", OPENDOTA_API_BASE, match_id);
    debug!("request_match_parse POST {}", url);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .send()
        .await
        .map_err(|e| {
            let msg = friendly_network_err(&e);
            debug!("request_match_parse network error: {}", msg);
            msg
        })?;

    let status = response.status();
    debug!("request_match_parse response status={}", status.as_u16());

    if !status.is_success() {
        log_api_error("parseRequest", status.as_u16());
        let err = format!("OpenDota couldn't parse this match — it will be skipped and retried later. ({})", friendly_status_err(status));
        debug!("request_match_parse failed: {}", err);
        return Err(err);
    }

    let parsed: ParseRequestResponse = response
        .json()
        .await
        .unwrap_or(ParseRequestResponse { job: None });

    let job_id = parsed.job.and_then(|j| j.jobId);
    debug!("request_match_parse match_id={} job_id={:?}", match_id, job_id);
    Ok(job_id)
}

/// Poll the parse job until it completes or the timeout is reached.
/// Returns true if the job completed cleanly, false if it timed out or errored.
/// OpenDota returns `null` (or 404) for the job once parsing is done.
pub async fn wait_for_parse_job(job_id: i64) -> bool {
    let url = format!("{}/request/{}", OPENDOTA_API_BASE, job_id);
    let client = reqwest::Client::new();
    debug!("wait_for_parse_job polling job_id={} url={}", job_id, url);

    // Poll every 5 s for up to 90 s (18 attempts).
    for attempt in 0..18 {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let response = match client.get(&url).send().await {
            Ok(r) => r,
            Err(e) => {
                debug!("wait_for_parse_job attempt={} network error: {}", attempt, e);
                break; // network issue — proceed and try fetching details anyway
            }
        };

        let status = response.status().as_u16();
        match status {
            // 404 means the job is gone (done or never existed) — ready to fetch.
            404 => {
                debug!("wait_for_parse_job attempt={} status=404 → done", attempt);
                return true;
            }
            s if s >= 500 => {
                debug!("wait_for_parse_job attempt={} server error {} → give up", attempt, s);
                break; // server error — give up polling
            }
            _ => {}
        }

        let body = response.text().await.unwrap_or_default();
        debug!("wait_for_parse_job attempt={} status={} body={:?}", attempt, status, &body[..body.len().min(80)]);
        // OpenDota returns the literal string "null" when the job is finished.
        if body.trim() == "null" || body.trim().is_empty() {
            debug!("wait_for_parse_job attempt={} body=null → done", attempt);
            return true;
        }
        // Otherwise the job is still in the queue — keep polling.
    }

    debug!("wait_for_parse_job timed out or errored for job_id={}", job_id);
    false // timed out or error; caller should still attempt fetch_match_details
}

/// Fetch detailed match data from OpenDota
pub async fn fetch_match_details(match_id: i64) -> Result<DetailedMatch, String> {
    let url = format!("{}/matches/{}", OPENDOTA_API_BASE, match_id);

    debug!("fetch_match_details GET {}", url);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            let msg = friendly_network_err(&e);
            debug!("fetch_match_details network error: {}", msg);
            msg
        })?;

    let status = response.status();
    debug!("fetch_match_details match_id={} response status={}", match_id, status.as_u16());

    if !status.is_success() {
        if status.as_u16() == 404 {
            return Err("This match hasn't been parsed by OpenDota yet. It will be picked up on the next sync.".to_string());
        }
        log_api_error("matchDetails", status.as_u16());
        return Err(friendly_status_err(status));
    }

    let match_details: DetailedMatch = response
        .json()
        .await
        .map_err(|e| format!("Unexpected response from OpenDota for this match ({}). Try again or check https://www.opendota.com/matches/{}", e, match_id))?;

    debug!("fetch_match_details match_id={} OK players={}", match_id, match_details.players.len());
    Ok(match_details)
}
