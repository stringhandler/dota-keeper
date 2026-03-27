# Bug: Stratz API last-hit-per-minute is one minute ahead

## Description

When using the Stratz API as the data source, the last-hit-per-minute chart shows values offset by one minute. For example, the value for minute 11 is displayed at minute 10.

OpenDota data is unaffected and remains correct — do not change OpenDota logic.

## Steps to Reproduce

1. Use Stratz API as data source (Settings → API Source → Stratz)
2. Open a match detail page and view the last-hits-per-minute chart
3. Observe that each value appears one minute earlier than it should (minute N value shown at minute N-1)

## Expected Behaviour

The last-hit value for minute N should be displayed at minute N.

## Root Cause (suspected)

Stratz returns per-minute arrays that are likely 0-indexed (index 0 = minute 1), while the chart plots them as 0-indexed positions (index 0 = minute 0). This creates a consistent -1 minute offset.

Fix: when parsing Stratz last-hit data, shift the array or adjust the x-axis mapping so index 0 maps to minute 1.

## Affected Files

- `src-tauri/src/stratz.rs` — where Stratz per-minute data is parsed into the database
- Possibly the frontend chart if the offset is applied there instead
