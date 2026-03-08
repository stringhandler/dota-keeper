# Feature: Report Suspected Smurf / Cheater from Match History

## Backend: Supabase (chosen)
## Config: Build-time env vars (same pattern as POSTHOG_API_KEY)

---

## One-time Supabase Setup (do this before implementing)

### 1. Create a project
- Go to https://supabase.com → New project
- Name: `dota-keeper` (or anything)
- Note down:
  - **Project URL** (e.g. `https://abcxyz.supabase.co`)
  - **Anon/public key** (from Project Settings → API)

### 2. Create the reports table
Run this in the Supabase SQL editor:

```sql
create table reports (
  id            bigint generated always as identity primary key,
  match_id      bigint       not null,
  hero_id       int          not null,
  reason        text         not null default 'smurfing',
  reporter_steam_id text     not null,
  created_at    timestamptz  not null default now()
);

-- Prevent spam: one report per (match, hero, reporter)
create unique index reports_dedup on reports (match_id, hero_id, reporter_steam_id);

-- Enable Row Level Security
alter table reports enable row level security;

-- Allow anonymous inserts (the anon key will be embedded in the app)
create policy "allow_insert" on reports for insert to anon with check (true);

-- Only authenticated users (you, via the Supabase dashboard) can read
-- No read policy needed for the app — the dashboard + SQL editor is enough for now
```

### 3. Set env vars
Add to `.env` (local dev) and CI secrets:
```
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_ANON_KEY=eyJ...your-anon-key...
```

---

## App Implementation

### Tauri (Rust) — `src-tauri/src/lib.rs`

Read the env vars at **compile time** (same as POSTHOG_API_KEY pattern):
```rust
const SUPABASE_URL: &str = env!("SUPABASE_URL", "");
const SUPABASE_ANON_KEY: &str = env!("SUPABASE_ANON_KEY", "");
```

New command:
```rust
#[tauri::command]
async fn submit_smurf_report(
    match_id: i64,
    hero_id: i32,
    reason: String,  // "smurfing" | "cheating"
) -> Result<(), String> {
    let steam_id = Settings::load()
        .steam_id
        .ok_or_else(|| "No Steam ID configured".to_string())?;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/rest/v1/reports", SUPABASE_URL))
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Authorization", format!("Bearer {}", SUPABASE_ANON_KEY))
        .header("Content-Type", "application/json")
        .header("Prefer", "resolution=ignore-duplicates")
        .json(&serde_json::json!({
            "match_id": match_id,
            "hero_id": hero_id,
            "reason": reason,
            "reporter_steam_id": steam_id,
        }))
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("Report failed (HTTP {})", resp.status()));
    }
    Ok(())
}
```

### `tauri.conf.json` / `Cargo.toml` env injection
In `src-tauri/build.rs` (create if needed):
```rust
fn main() {
    // Forward env vars to rustc so env!() macros resolve at compile time
    for var in ["SUPABASE_URL", "SUPABASE_ANON_KEY"] {
        if let Ok(val) = std::env::var(var) {
            println!("cargo:rustc-env={var}={val}");
        }
    }
    tauri_build::build()
}
```

### Frontend — Matches page
- Add a small **⚑ Report** icon button to each match row
- Clicking opens a modal:
  - Match ID shown
  - Hero picker: searchable list from `heroes.js` (default selected = the hero you played)
  - Reason radio: **Smurfing** (default) / **Cheating**
  - Submit + Cancel buttons
- On submit → `invoke("submit_smurf_report", { matchId, heroId, reason })`
- Show success/error toast

---

## Acceptance Criteria

- [ ] ⚑ Report button visible on each match row
- [ ] Modal shows hero picker (searchable, 120+ heroes) with the played hero pre-selected
- [ ] Reason selector: Smurfing / Cheating
- [ ] Submit → POST to Supabase; duplicate (same match+hero+reporter) silently ignored
- [ ] Success toast shown; modal closes
- [ ] Error toast on network failure; match data unaffected
- [ ] If no Steam ID configured, button is disabled
- [ ] Build fails clearly if SUPABASE_URL / SUPABASE_ANON_KEY are empty (not silent)

---

## Querying your data

In the Supabase SQL editor or Table Editor:
```sql
select hero_id, count(*) as reports, reason
from reports
group by hero_id, reason
order by reports desc;
```

Or via the REST API with your service role key (never embed this in the app):
```
GET https://your-project.supabase.co/rest/v1/reports?select=*
Authorization: Bearer YOUR_SERVICE_ROLE_KEY
apikey: YOUR_SERVICE_ROLE_KEY
```

---

## Notes

- The **anon key** is safe to embed in a desktop app — Supabase's RLS blocks reads, only inserts are allowed.
- The `Prefer: resolution=ignore-duplicates` header silently ignores duplicate insert attempts.
- The hero picker can default to the `hero_id` stored on the match row so most reports are one click + submit.
