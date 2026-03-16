# Task: Load More button on last page of matches

## Overview

On the Matches page, when the user is viewing the last page of their stored matches, show a **Load More** button that fetches older matches — unless we already know there are no more matches to load (i.e. the oldest stored match is their very first ever match).

## Requirements

- Show a "Load More" button at the bottom of the matches list when on the last page
- Clicking it triggers a backfill: fetch the next batch of matches older than the oldest currently stored match (same logic as the existing "Backfill 100 matches" feature)
- After loading, append the new matches and stay on the current page / scroll position
- If the previous backfill returned 0 new matches (or reached the user's first match), hide the button permanently for that session
- Do not show the button if we already know there are no more matches (i.e. backfill was previously exhausted)

## Implementation Notes

- Reuse the existing backfill Tauri command — pass the oldest stored `match_id` as the `before_match_id` parameter
- Track "backfill exhausted" state in the component (no need to persist to DB unless we want to remember across sessions)
- The button should be disabled / show a spinner while loading

## Acceptance Criteria

- [ ] "Load More" button appears only on the last page of matches
- [ ] Clicking fetches older matches via the same backfill logic
- [ ] Button disappears / is disabled once no more matches are available
- [ ] Button is not shown if the user is not on the last page
