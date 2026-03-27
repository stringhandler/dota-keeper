# Feature: Fetch and Store Stratz Impact Score for Matches

## Summary

Stratz provides a per-match "impact score" that rates a player's performance. Fetch and store this value alongside each match so it can be displayed and used in goal/analysis metrics.

## Background

- Stratz API: https://stratz.com/api (GraphQL)
- Impact score is a float (0–100 range roughly) per player per match
- Requires a Stratz API key (user must provide in settings)

## Acceptance Criteria

- [ ] Settings page has a Stratz API key field (stored securely alongside Steam ID)
- [ ] When fetching/refreshing matches, the app also calls the Stratz API for the impact score
- [ ] Impact score is stored in the `matches` table (new column: `impact_score REAL`)
- [ ] Match list and match detail views display the impact score (if available)
- [ ] If Stratz API is unavailable or key is missing, impact score is null — app still works without it
- [ ] Impact score can be used as a goal metric (GoalMetric::ImpactScore)

## Notes

- Stratz GraphQL endpoint: `https://api.stratz.com/graphql`
- Query needed: `match(id: $matchId) { players(steamAccountId: $steamId) { imp } }`
- The field is `imp` (impact score) on the player node
- Need to decide whether to fetch on match import or on-demand
- Rate limits apply — batch carefully or fetch lazily
