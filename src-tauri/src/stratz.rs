use crate::database::{Match, MatchState};
use crate::items;
use crate::opendota::{DetailedMatch, DetailedPlayer, PurchaseLogEntry};
use serde::{Deserialize, Serialize};
use tracing::debug;

const STRATZ_API_BASE: &str = "https://api.stratz.com/graphql";

// ── Helpers ───────────────────────────────────────────────────────────────────

fn steam_id64_to_id32(steam_id64: &str) -> Result<u64, String> {
    let id64: u64 = steam_id64
        .parse()
        .map_err(|_| "No matches found for this Steam ID. Double-check the ID in Settings.".to_string())?;

    const STEAM_ID64_BASE: u64 = 76561197960265728;
    if id64 >= STEAM_ID64_BASE {
        Ok(id64 - STEAM_ID64_BASE)
    } else {
        Ok(id64) // already a Steam ID32
    }
}

/// Extract an i32 from a JSON Value that may be an integer or a string.
fn value_to_i32(val: &Option<serde_json::Value>) -> i32 {
    match val {
        Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0) as i32,
        Some(serde_json::Value::String(s)) => s.parse().unwrap_or(0),
        _ => 0,
    }
}

/// Map a Stratz GameModeEnumType string to the Valve game_mode integer.
fn stratz_game_mode_to_int(s: &str) -> i32 {
    match s {
        "NONE"                    =>  0,
        "ALL_PICK"                =>  1,
        "CAPTAINS_MODE"           =>  2,
        "RANDOM_DRAFT"            =>  3,
        "SINGLE_DRAFT"            =>  4,
        "ALL_RANDOM"              =>  5,
        "INTRO"                   =>  6,
        "DIRETIDE"                =>  7,
        "REVERSE_CAPTAINS_MODE"   =>  8,
        "THE_GREEVILING"          =>  9,
        "TUTORIAL"                => 10,
        "MID_ONLY"                => 11,
        "LEAST_PLAYED"            => 12,
        "NEW_PLAYER_POOL"         => 13,
        "COMPENDIUM_MATCHMAKING"  => 14,
        "CUSTOM"                  => 15,
        "CAPTAINS_DRAFT"          => 16,
        "BALANCED_DRAFT"          => 17,
        "ABILITY_DRAFT"           => 18,
        "EVENT"                   => 19,
        "ALL_RANDOM_DEATH_MATCH"  => 20,
        "SOLO_MID"                => 21,
        "ALL_DRAFT"               => 22,
        "TURBO"                   => 23,
        "MUTATION"                => 24,
        _ => s.parse().unwrap_or(0),
    }
}

/// Map a Stratz LobbyTypeEnum string to the Valve lobby_type integer.
fn stratz_lobby_type_to_int(s: &str) -> i32 {
    match s {
        "NORMAL"            => 0,
        "PRACTICE"          => 1,
        "TOURNAMENT"        => 2,
        "TUTORIAL"          => 3,
        "COOP_WITH_BOTS"    => 4,
        "TEAM_MATCHMAKING"  => 5,
        "SOLO_MATCHMAKING"  => 6,
        "RANKED"            => 7,
        "SOLO_PRACTICE"     => 8,
        _ => s.parse().unwrap_or(0),
    }
}

/// Convert a Stratz position value (int or "POSITION_N" string) to the
/// lane_role integer used throughout the app (1=carry … 5=hard support).
fn parse_position(val: &Option<serde_json::Value>) -> i32 {
    match val {
        Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0) as i32,
        Some(serde_json::Value::String(s)) => {
            // "POSITION_1" → 1
            s.trim_start_matches("POSITION_").parse().unwrap_or(0)
        }
        _ => 0,
    }
}

fn friendly_network_err(err: &reqwest::Error) -> String {
    if err.is_connect() || err.is_timeout() {
        "Couldn't reach Stratz — check your internet connection.".to_string()
    } else {
        "Network error while contacting Stratz. Will retry automatically.".to_string()
    }
}

fn friendly_status_err(status: reqwest::StatusCode) -> String {
    match status.as_u16() {
        401 => "Stratz API key is invalid or missing. Check the API key in Settings.".to_string(),
        429 => "Too many requests to Stratz. Will try again shortly.".to_string(),
        500..=599 => "Stratz is unavailable right now. Your matches will sync when it comes back.".to_string(),
        code => format!("Stratz returned an unexpected error (HTTP {}). Will retry later.", code),
    }
}

// ── GraphQL request ───────────────────────────────────────────────────────────

#[derive(Serialize)]
struct GraphQLRequest<'a> {
    query: &'a str,
    variables: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize, Debug)]
struct GraphQLError {
    message: String,
}

async fn graphql_post<T: for<'de> Deserialize<'de>>(
    api_key: &str,
    query: &str,
    variables: serde_json::Value,
) -> Result<T, String> {
    let api_key = api_key.trim();
    if api_key.is_empty() {
        return Err("Stratz API key is empty. Add your API key in Settings.".to_string());
    }

    let client = reqwest::Client::new();
    let response = client
        .post(STRATZ_API_BASE)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("User-Agent", "DotaKeeper/1.0")
        .json(&GraphQLRequest { query, variables })
        .send()
        .await
        .map_err(|e| {
            let msg = friendly_network_err(&e);
            debug!("stratz graphql_post network error: {}", msg);
            msg
        })?;

    let status = response.status();
    debug!("stratz graphql_post response status={}", status.as_u16());

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        debug!("stratz graphql_post error body: {}", &body[..body.len().min(500)]);
        return Err(friendly_status_err(status));
    }

    let body = response.text().await.map_err(|e| format!("Failed to read Stratz response: {}", e))?;
    debug!("stratz graphql_post response body (first 500 chars): {}", &body[..body.len().min(500)]);

    let resp: GraphQLResponse<T> = serde_json::from_str(&body)
        .map_err(|e| format!("Stratz returned unexpected data: {} — body snippet: {}", e, &body[..body.len().min(200)]))?;

    if let Some(errors) = resp.errors {
        if !errors.is_empty() {
            let msg = errors.iter().map(|e| e.message.as_str()).collect::<Vec<_>>().join("; ");
            debug!("stratz graphql_post GraphQL errors: {}", msg);
            return Err(format!("Stratz API error: {}", msg));
        }
    }

    resp.data.ok_or_else(|| "Stratz returned no data".to_string())
}

// ── Player match list ─────────────────────────────────────────────────────────

#[derive(Deserialize, Debug)]
struct PlayerMatchesData {
    player: Option<PlayerData>,
}

#[derive(Deserialize, Debug)]
struct PlayerData {
    matches: Option<Vec<StratzMatch>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct StratzMatch {
    id: i64,
    start_date_time: Option<i64>,
    duration_seconds: Option<i32>,
    #[serde(default)]
    game_mode: Option<serde_json::Value>,
    #[serde(default)]
    lobby_type: Option<serde_json::Value>,
    did_radiant_win: Option<bool>,
    players: Option<Vec<StratzMatchPlayer>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct StratzMatchPlayer {
    steam_account_id: Option<u64>,
    hero_id: Option<i32>,
    is_radiant: Option<bool>,
    position: Option<serde_json::Value>,
    kills: Option<i32>,
    deaths: Option<i32>,
    assists: Option<i32>,
    experience_per_minute: Option<i32>,
    gold_per_minute: Option<i32>,
    num_last_hits: Option<i32>,
    num_denies: Option<i32>,
    hero_damage: Option<i32>,
    tower_damage: Option<i32>,
    hero_healing: Option<i32>,
    player_slot: Option<i32>,
}

impl StratzMatch {
    fn into_match(self, player: StratzMatchPlayer) -> Match {
        let is_radiant = player.is_radiant.unwrap_or(false);
        // Derive player_slot from isRadiant + position (position 1=slot 0, etc.)
        let position = parse_position(&player.position);
        let slot_offset = if position >= 1 { position - 1 } else { 0 };
        let player_slot = player.player_slot.unwrap_or_else(|| {
            if is_radiant { slot_offset } else { 128 + slot_offset }
        });

        Match {
            match_id: self.id,
            hero_id: player.hero_id.unwrap_or(0),
            start_time: self.start_date_time.unwrap_or(0),
            duration: self.duration_seconds.unwrap_or(0),
            game_mode: match &self.game_mode {
                Some(serde_json::Value::String(s)) => stratz_game_mode_to_int(s),
                Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0) as i32,
                _ => 0,
            },
            lobby_type: match &self.lobby_type {
                Some(serde_json::Value::String(s)) => stratz_lobby_type_to_int(s),
                Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0) as i32,
                _ => 0,
            },
            radiant_win: self.did_radiant_win.unwrap_or(false),
            player_slot,
            kills: player.kills.unwrap_or(0),
            deaths: player.deaths.unwrap_or(0),
            assists: player.assists.unwrap_or(0),
            xp_per_min: player.experience_per_minute.unwrap_or(0),
            gold_per_min: player.gold_per_minute.unwrap_or(0),
            last_hits: player.num_last_hits.unwrap_or(0),
            denies: player.num_denies.unwrap_or(0),
            hero_damage: player.hero_damage.unwrap_or(0),
            tower_damage: player.tower_damage.unwrap_or(0),
            hero_healing: player.hero_healing.unwrap_or(0),
            parse_state: MatchState::Unparsed,
            role: 0,
            rank_tier: None,
            patch: None, // assigned after insert using patch lookup
        }
    }
}

const PLAYER_MATCHES_QUERY: &str = r#"
query GetPlayerMatches($steamAccountId: Long!, $take: Int!, $skip: Int) {
  player(steamAccountId: $steamAccountId) {
    matches(request: { take: $take, skip: $skip }) {
      id
      startDateTime
      durationSeconds
      gameMode
      lobbyType
      didRadiantWin
      players {
        steamAccountId
        heroId
        isRadiant
        position
        kills
        deaths
        assists
        experiencePerMinute
        goldPerMinute
        numLastHits
        numDenies
        heroDamage
        towerDamage
        heroHealing
        playerSlot
      }
    }
  }
}
"#;

/// Fetch recent matches from Stratz API.
pub async fn fetch_recent_matches(steam_id: &str, api_key: &str, limit: usize) -> Result<Vec<Match>, String> {
    let account_id = steam_id64_to_id32(steam_id)?;
    debug!("stratz fetch_recent_matches account_id={} limit={}", account_id, limit);

    let data: PlayerMatchesData = graphql_post(
        api_key,
        PLAYER_MATCHES_QUERY,
        serde_json::json!({
            "steamAccountId": account_id,
            "take": limit,
            "skip": 0
        }),
    )
    .await?;

    let matches = extract_matches(data, account_id)?;
    debug!("stratz fetch_recent_matches returning {} matches", matches.len());
    Ok(matches)
}

/// Fetch matches before a timestamp using skip-based pagination.
pub async fn fetch_matches_before(
    steam_id: &str,
    api_key: &str,
    before_timestamp: i64,
    limit: usize,
) -> Result<Vec<Match>, String> {
    let account_id = steam_id64_to_id32(steam_id)?;
    debug!("stratz fetch_matches_before account_id={} before={} limit={}", account_id, before_timestamp, limit);

    let mut all_matches: Vec<Match> = Vec::new();
    let mut skip = 0;
    const BATCH_SIZE: usize = 100;
    const MAX_ATTEMPTS: usize = 50;
    let mut attempts = 0;

    while all_matches.len() < limit && attempts < MAX_ATTEMPTS {
        let data: PlayerMatchesData = graphql_post(
            api_key,
            PLAYER_MATCHES_QUERY,
            serde_json::json!({
                "steamAccountId": account_id,
                "take": BATCH_SIZE,
                "skip": skip
            }),
        )
        .await?;

        let batch = extract_matches(data, account_id)?;
        debug!("stratz fetch_matches_before batch={} matches from API (skip={})", batch.len(), skip);

        if batch.is_empty() {
            break;
        }

        for m in batch {
            if m.start_time < before_timestamp {
                all_matches.push(m);
                if all_matches.len() >= limit {
                    break;
                }
            }
        }

        skip += BATCH_SIZE;
        attempts += 1;

        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    }

    all_matches.sort_by(|a, b| b.start_time.cmp(&a.start_time));
    all_matches.truncate(limit);
    debug!("stratz fetch_matches_before returning {} matches", all_matches.len());
    Ok(all_matches)
}

fn extract_matches(data: PlayerMatchesData, account_id: u64) -> Result<Vec<Match>, String> {
    let matches = data
        .player
        .ok_or("No player data returned from Stratz. Check your Steam ID in Settings.")?
        .matches
        .unwrap_or_default();

    Ok(matches
        .into_iter()
        .filter_map(|m| {
            // Filter players to find ours by steamAccountId, fall back to first player
            let players = m.players.as_ref()?;
            let player = players
                .iter()
                .find(|p| p.steam_account_id == Some(account_id))
                .or_else(|| players.first())
                .cloned()?;
            Some(m.into_match(player))
        })
        .collect())
}

// ── Match details ─────────────────────────────────────────────────────────────

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MatchDetailsData {
    #[serde(rename = "match")]
    match_data: Option<StratzDetailedMatch>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StratzDetailedMatch {
    id: i64,
    players: Option<Vec<StratzDetailedPlayer>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StratzDetailedPlayer {
    steam_account_id: Option<u64>,
    player_slot: Option<i32>,
    position: Option<serde_json::Value>,
    stats: Option<StratzPlayerStats>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StratzPlayerStats {
    last_hits_per_minute: Option<Vec<i32>>,
    denies_per_minute: Option<Vec<i32>>,
    networth_per_minute: Option<Vec<i32>>,
    item_purchases: Option<Vec<StratzItemPurchase>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StratzItemPurchase {
    time: i32,
    item_id: i32,
}

const MATCH_DETAILS_QUERY: &str = r#"
query GetMatchDetails($matchId: Long!) {
  match(id: $matchId) {
    id
    players {
      steamAccountId
      playerSlot
      position
      stats {
        lastHitsPerMinute
        deniesPerMinute
        networthPerMinute
        itemPurchases {
          time
          itemId
        }
      }
    }
  }
}
"#;

/// Fetch detailed match data from Stratz.
/// Unlike OpenDota, no parse request is needed — Stratz data is available immediately.
pub async fn fetch_match_details(match_id: i64, api_key: &str) -> Result<DetailedMatch, String> {
    debug!("stratz fetch_match_details match_id={}", match_id);

    let data: MatchDetailsData = graphql_post(
        api_key,
        MATCH_DETAILS_QUERY,
        serde_json::json!({ "matchId": match_id }),
    )
    .await?;

    let stratz_match = data
        .match_data
        .ok_or_else(|| format!("No data returned for match {}. It may not be tracked by Stratz yet.", match_id))?;

    let players: Vec<DetailedPlayer> = stratz_match
        .players
        .unwrap_or_default()
        .into_iter()
        .map(|p| {
            let lane_role = parse_position(&p.position);
            let purchase_log = p.stats.as_ref().and_then(|s| s.item_purchases.as_ref()).map(|purchases| {
                purchases
                    .iter()
                    .filter_map(|purchase| {
                        let key = items::get_item_name(purchase.item_id)?;
                        Some(PurchaseLogEntry {
                            time: purchase.time,
                            key: key.to_string(),
                        })
                    })
                    .collect()
            });

            DetailedPlayer {
                account_id: p.steam_account_id.map(|id| id as u32),
                player_slot: p.player_slot.unwrap_or(0),
                lane_role: Some(lane_role),
                // Stratz returns per-minute deltas; convert to cumulative totals
                // to match the OpenDota lh_t / dn_t format the rest of the app expects.
                lh_t: p.stats.as_ref().and_then(|s| s.last_hits_per_minute.as_ref()).map(|v| {
                    let mut total = 0;
                    v.iter().map(|&x| { total += x; total }).collect()
                }),
                dn_t: p.stats.as_ref().and_then(|s| s.denies_per_minute.as_ref()).map(|v| {
                    let mut total = 0;
                    v.iter().map(|&x| { total += x; total }).collect()
                }),
                gold_t: p.stats.as_ref().and_then(|s| s.networth_per_minute.clone()),
                purchase_log,
            }
        })
        .collect();

    debug!("stratz fetch_match_details match_id={} players={}", match_id, players.len());
    Ok(DetailedMatch {
        match_id: stratz_match.id,
        players,
    })
}
