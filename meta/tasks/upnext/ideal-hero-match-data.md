# Ideal Hero Match Data (Per Bracket)

## Overview

A CSV hosted on GitHub (alongside the benchmark CSV) containing "ideal game" stats per hero, per bracket, per mode. This gives users a reference point for what a typical good game looks like for their hero at each skill level — useful for comparison features throughout the app.

## Data Source

- CSV hosted in the GitHub repo (e.g. `meta/benchmarks/ideal_hero_matches.csv`)
- Fetched from GitHub raw URL on app startup, same mechanism as the hero benchmark CSV
- Generated externally (separate process), updated periodically
- Cached locally in SQLite; works offline with last-fetched data
- Display "Data as of ..." subtext showing data freshness

## CSV Contents

Per hero / bracket / mode (Ranked, Turbo):

- **Last hits per minute** (time-series, not just a single number — e.g. LH at 5, 10, 15, 20, 25, 30 min)
- **Purchase order** — typical item build timing (item + approximate purchase time)
- **Other stats** — GPM, XPM, KDA, deaths, damage, healing, etc. (expand over time)
- **example_match_id_avg** — a real match ID representing a typical/average game for this hero+bracket+mode. Used as a compare target (fetched from OpenDota on demand). **Not** a synthetic/fake match — a real game selected as the closest to the bracket average.
- **example_match_id_top** — a real match ID representing a top-performing game for this hero+bracket+mode. Same approach — a real game selected as a high-performing example.

## Schema

SQLite table (e.g. `ideal_hero_matches`):
- `hero_id, mode, bracket, stat_name, stat_value, time_minute, example_match_id_avg, example_match_id_top` (for time-series stats)
- `hero_id, mode, bracket, item_name, purchase_order, purchase_time` (for item build)

## Usage

### Bracket Compare Buttons

Add bracket comparison buttons (using the `example_match_id_avg` / `example_match_id_top`) in two places:

#### 1. Goal detail page — comparison selection
- Show buttons per bracket, e.g. `Herald (Avg)`, `Herald (Top)`, `Crusader (Avg)`, etc.
- **Mode logic:**
  - Goal mode = Ranked → only show Ranked bracket buttons
  - Goal mode = Turbo → only show Turbo bracket buttons
  - Goal mode = Any → show **both** Ranked and Turbo buttons, e.g. `Herald (Ranked)`, `Herald (Turbo)`

#### 2. Match details page — compare-to feature
- Add bracket buttons alongside the existing compare options (Last Game, Last Same Hero)
- **Mode logic:**
  - Match is Turbo (game_mode 23) → only show Turbo bracket buttons
  - Match is Ranked (game_mode 22) → only show Ranked bracket buttons
  - Match is other mode → infer Ranked or Turbo based on the current game; default to Ranked if unclear

#### Button behaviour
- Clicking a bracket button fetches the `example_match_id_avg` (or `_top`) from the local DB
- If the match isn't already in the user's local DB, fetch it from OpenDota on demand and load it as the compare target
- Uses the same comparison UI that already exists (CS, networth, XP, item timing charts side-by-side)

### Other usage
- **Analysis page**: overlay ideal LH/min curve on user's charts
- **Item build comparison**: show how the user's item timings compare to the ideal

## Open Questions

- Exact CSV format/schema TBD once the generation process is built
- Which stats to include in v1? Start with last hits per minute + purchase order, expand later.
