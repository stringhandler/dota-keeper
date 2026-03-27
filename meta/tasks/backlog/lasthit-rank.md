# Task: Show user's last-hitting rank

## Overview

Determine and display the user's last-hitting "rank" by comparing their per-hero last-hit data against a pre-created benchmark dataset of average last-hit values per hero per rank bracket.

## Requirements

- Display the user's last-hitting rank on the Analysis page (or a dedicated section)
- Rank is determined per-hero: find the rank bracket whose average last-hit curve is closest to the user's actual data
- Use Euclidean distance (or similar) across the per-minute last-hit values to find the closest benchmark
- Show the rank label (e.g. "Archon", "Legend", "Ancient") with a confidence indicator if useful

## Data

A benchmark dataset will be provided as a JSON or CSV file (to be created separately). Format will be:

```json
{
  "hero_id": {
    "Herald": [lh_min1, lh_min2, ..., lh_min10],
    "Guardian": [...],
    ...
  }
}
```

Store this file in `src/lib/data/lasthit-benchmarks.json` (or similar).

## Implementation Notes

- Benchmark data should be loaded at build time (static import) — no DB storage needed
- Comparison logic: for each hero the user has played, average their actual last-hit-per-minute curves across recent games, then find the closest bracket in the benchmark data
- Show an overall rank (aggregate across all heroes) and optionally a per-hero breakdown
- Only compute for heroes with sufficient data (e.g. ≥ 5 parsed games)

## Acceptance Criteria

- [ ] Benchmark data file created and imported
- [ ] Rank computed correctly per hero
- [ ] Aggregate rank displayed on the UI
- [ ] Graceful handling when insufficient data exists
- [ ] Works with both OpenDota and Stratz parsed data
