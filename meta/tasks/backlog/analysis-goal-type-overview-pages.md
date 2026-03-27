# Analysis: Overview Pages for All Goal Types

## Description

Expand the analysis section to include overview/breakdown rows or sub-pages for every goal type supported by the app. Currently analysis may focus on win rate and KDA — this task extends it to cover all stat categories that goals can be defined against.

## Goal Types to Cover

Each should show averages broken down by: All Heroes / Carries / Cores (or other relevant groupings where applicable).

- **Last Hits** — avg last hits per game
- **Denies** — avg denies per game
- **Net Worth / Gold** — avg net worth or GPM
- **Item Timings** — avg timing for key items (e.g. BF at X minutes)
- **Deaths** — avg deaths per game
- **Kills / KDA** — avg kills, assists, KDA ratio
- **XPM** — avg experience per minute
- **GPM** — avg gold per minute
- **Hero Damage** — avg hero damage
- **Tower Damage** — avg tower damage
- **Healing** — avg healing output
- **Wards Placed** — avg observer/sentry wards placed
- **Camps Stacked** — avg camps stacked per game

## Acceptance Criteria

- Each goal type has a dedicated section or row group in the analysis page
- Breakdowns follow the same pattern: All Heroes / Carries / Cores (adjust grouping where it makes sense, e.g. Wards for supports)
- Consistent styling with existing analysis rows
- Only show stat types for which the app already stores data — add a note if data needs to be captured first
