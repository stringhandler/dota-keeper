# Auto Create Local Guides

## Summary
Add a button that generates a Dota 2 local hero guide file, populated with item timings derived from the user's actual match history for that hero.

## Background
Dota 2 supports custom guides (item builds) stored locally. By analysing match data, we can create guides that reflect the user's real item timings — e.g. "You typically get Blink Dagger at 18 minutes on Axe."

## Requirements

### Guide Generation
- Button per hero (on the hero detail/analysis page) to "Generate Local Guide"
- Analyse match history for that hero to extract item timings:
  - First purchase time for each common item
  - Average timing across recent N games
- Output a valid Dota 2 guide file (JSON format) with items ordered by typical timing
- Save the file to the Dota 2 local guides directory (e.g. `Steam/steamapps/common/dota 2 beta/game/dota/guides/`)

### UI
- Button: "Generate Guide" on hero stats view
- On success: toast notification confirming file was written and where
- Show timings in the UI before/after generation (e.g. a table of item → avg timing)

### Goal Data in Guide
- Include the user's active goals relevant to that hero in the guide description/notes
- Examples: "Goal: Average less than 5 deaths", "Goal: 55% win rate on carry heroes"
- Show current goal pass rate alongside the item timings so the guide reflects both build and performance targets

### Settings
- Allow user to configure Dota 2 install path (or auto-detect via Steam library)

## Open Questions
- Should guides auto-update on a schedule, or only on button press? (Start with button press)
- How many recent games to use for timing averages? (Default: last 20 games on that hero)

## Acceptance Criteria
- Guide file is written to the correct Dota 2 directory
- Items are sorted by average purchase timing
- User sees a success/error toast after pressing the button
- Timings are visible in the app UI
