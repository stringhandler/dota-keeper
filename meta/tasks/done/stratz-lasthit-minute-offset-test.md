# Test: Stratz last-hit-per-minute chart offset

## What was fixed
Stratz's `lastHitsPerMinute` array is 0-indexed starting at minute 1 (index 0 = end of minute 1), but `insert_match_cs_data` stores `minute = array_index`. This made every value appear one minute too early. Fixed by prepending 0 to the cumulative lh_t, dn_t, gold_t, and xp_t arrays in stratz.rs so index 0 = minute 0 (game start), matching OpenDota's format.

Note: previously parsed Stratz matches will still have the old offset in the DB. A reparse is needed for existing matches.

## Steps to test

1. Settings → API Source → Stratz (with a valid API key and Steam ID).
2. Reparse an existing match, or refresh matches to get a new one.
3. Open the match detail page → Last Hits per Minute chart.
4. Cross-reference the value shown at minute N against a Stratz match page (stratz.gg).
5. Expected: minute 10 value in the chart matches the "10 min CS" shown on Stratz.
6. Verify the networth / XP charts are also correctly aligned at each minute.
7. OpenDota source: confirm charts are unchanged (not affected by this fix).
