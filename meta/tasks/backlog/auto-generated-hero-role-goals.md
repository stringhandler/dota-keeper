# Auto-Generated Goals for Hero & Role

## User Request
> "I would like goals for a specific hero and a certain role to be generated automatically and several options to choose from"

## Summary
When a user selects a hero (or their favourite heroes) and a role (carry, support, mid, etc.), the app should automatically suggest several pre-built goal options tailored to that hero/role combination. The user can then pick one or more to add to their goal list.

## Acceptance Criteria
- User can select a hero and/or role as context for goal generation
- System generates 3–5 relevant goal suggestions (e.g. "Average 500 GPM on Juggernaut", "Maintain 60% win rate as carry")
- Suggestions are calibrated to the 75% target achievement rate (per the goal design principle in CLAUDE.md)
- User can accept one or more suggestions and add them directly to their goals
- Suggestions can be regenerated or dismissed

## Design Notes
- Could be surfaced on the Goals page as a "Suggest goals" CTA
- Could leverage existing hero data (favourite heroes) to pre-populate
- Suggestions could be static rule-based (hero role → stat thresholds) or eventually AI-assisted
- Consider using the user's historical match data to calibrate thresholds to their actual skill level

## Related
- Goal definition system (`goals` route)
- Hero favourite selection (onboarding step 2, analysis page)
- UC-02x goals specs in `meta/spec/goals/`
