# Hero Performance Benchmark Ranking

## Overview

Import hero benchmark data (mean, standard deviation per stat per hero/rank/mode) from a CSV and use it to show users how their performance compares across skill brackets — giving an approximate "equivalent rank" for each stat.

## Data Source

- CSV file hosted in the GitHub repo (e.g. `meta/benchmarks/hero_benchmarks.csv`), similar to how `meta/autoupdate/latest.json` is hosted
- Fetched from GitHub raw URL on app startup (e.g. `https://raw.githubusercontent.com/stringhandler/dota-keeper/main/meta/benchmarks/hero_benchmarks.csv`)
- CSV columns:
  - Hero
  - Mode (Ranked / Turbo)
  - Bracket (Herald, Guardian, Crusader, Archon, Legend, Ancient, Divine, Immortal)
  - Stat name (e.g. last hits at 10 min, GPM, XPM, deaths, etc.)
  - Mean
  - Standard Deviation
- On startup: fetch CSV, parse, and upsert into local SQLite — so benchmark data stays up to date without manual user action
- Cache locally so the app works offline (use last-fetched data if network unavailable)
- CSV should include a date/version header or the fetch should record the last-modified/fetch timestamp
- Display a small subtext on the UI (e.g. "Data as of 2026-03-15") showing when the benchmark data was last updated

## Features

### Bracket Comparison Table

For a given hero + mode + stat + user value, show a table like:

| Bracket  | Mean | SD   | z-score | Interpretation          |
|----------|------|------|---------|-------------------------|
| Herald   | 22.0 | 4.4  | +0.68   | Above average           |
| Guardian | 27.2 | 11.7 | -0.19   | Slightly below average  |
| Crusader | 25.4 | 9.4  | -0.04   | Almost exactly average  |
| Archon   | 25.2 | 3.6  | -0.06   | About average           |
| Legend   | 32.0 | 5.6  | -1.25   | Below average           |
| Ancient  | 20.5 | 4.9  | +0.92   | Above average           |
| Immortal | 27.0 | 1.4  | -1.43   | Below average           |

- z-score = (user_value - mean) / sd
- Interpretation text derived from z-score thresholds

### Summary Rank Estimate

- Determine which bracket the user's value is closest to "average" in (z-score closest to 0)
- Display as e.g. "Last hitting rank equiv = Crusader"

### User's Own Standard Deviation

- Calculate the user's SD for the stat across their recent matches (for that hero + mode)
- Show alongside the bracket data so the user can see their own consistency

## Where It Appears

1. **Analysis page** — aggregate view across recent matches, per hero/mode/stat
   - **Distribution chart**: show the user's last hits at 10 min distribution as a chart, with the ability to overlay the normal distributions for each bracket (toggle on/off per bracket)
   - **Filters**: ability to filter/split the user's distribution by **win/loss** and **ranked/turbo**
2. **Match details page** — show "Your Last Hitting Rank for this game: Herald" (best-fit bracket for that single match's last hits at 10 min), plus the full bracket comparison table

## Technical Notes

- Store imported CSV data in a new SQLite table (e.g. `hero_benchmarks`)
- Schema: `hero_id, mode, bracket, stat_name, mean, std_dev`
- Rust backend: startup fetch + parse of CSV from GitHub, querying benchmarks, computing z-scores
- Frontend: reusable benchmark table component used on both analysis and match detail pages
- User's own SD computed from `matches` table data at query time

## Open Questions

- ~~Which stats to benchmark?~~ → **Last hits at 10 min only** for now. Other stats (GPM, XPM, KDA, deaths) will be added later.
- ~~How often will the CSV be updated?~~ → Roughly monthly, but may be less frequent. The data date subtext keeps the user informed of staleness.
- ~~Should we highlight the best fit bracket row?~~ → **Yes.** Highlight the best-fit row in the table. Additionally, on the analysis page show a medal/badge above the table: "Your Last Hitting Rank — Crusader" with separate medals for **Ranked** and **Turbo**.
