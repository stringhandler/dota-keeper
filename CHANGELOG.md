# Changelog

All notable changes to Dota Keeper will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2026-02-20

### Added
- **Anonymous Analytics Tracking**: Optional usage analytics via PostHog to help improve Dota Keeper
  - User consent modal on first launch
  - Opt-in/opt-out in Settings page
  - No personal information collected
  - Analytics disabled in development builds (only active in production)

### Changed
- **Development Build Configuration**: Analytics tracking is now completely disabled in debug builds for cleaner local development

## [0.2.0] - 2026-02-20

### Changed
- **Complete UI Redesign**: New tactical dark theme with refined color palette (dark blue-grey base, gold accents, teal highlights)
- **New Typography**: Upgraded to Google Fonts — Rajdhani (headings), Barlow Condensed (labels/buttons), Barlow (body text)
- **Redesigned Layout**: 220px sidebar with streamlined navigation, new topbar with page titles and quick actions, fixed-height layout with scrollable content area
- **Dashboard Overhaul**: Added quick stats strip (Win Rate 7d, Avg CS @ 10min, Goals Hit 7d, Active Goals), goal cards with 7-dot hit/miss indicators, clickable goal rows
- **Matches Page**: Added filter chips (All/Wins/Losses/Ranked/Turbo/per-hero), mode tags, goals achievement chips, streamlined table layout
- **Analysis Page**: 2-column card grid layout, horizontal filters, contextual insights based on goals, decorative sparkline visualizations
- **Goals Page**: Inline horizontal create form, goal cards with inline Edit/Delete buttons, contextual warnings when player average exceeds goal target
- **Sidebar Updates**: Rank pill display at bottom, gold active state indicators, refreshed navigation icons

### Fixed
- Svelte 5 `{@const}` placement errors in analysis and matches pages

## [0.1.10] - 2026-02-19

### Added
- **Weekly Challenges**: New Challenges page (🏆 nav item) shows three weekly challenge options (easy/medium/hard). Accept one to track progress, reroll up to twice, or skip the week. Dashboard shows active challenge progress at a glance.
- **Challenge History**: `/challenges/history` page lists all completed and failed challenges (daily + weekly) with filter tabs and grouped by date.
- **Difficulty Levels for Goal Suggestions**: Settings now has an Easy/Medium/Hard/Custom difficulty selector that controls how aggressively suggestions push beyond current baseline.
- **Edit Goal from Details Page**: Inline edit form on the goal details page — change hero, metric, target, or game mode without leaving the page.
- **Favourite Heroes in Filters**: Hero dropdowns across Analysis, Goals, and goal edit forms now group favourited heroes at the top under a ⭐ Favourites section.

### Fixed
- **View on OpenDota**: Button now correctly opens the system browser (was silently failing due to incorrect import name)
- **Updater permission**: Auto-update check was failing with a permissions error; `updater:default` capability now included
- **Hero dropdown pre-selection in goal edit**: Opening the edit form now correctly pre-selects the goal's hero
- **Game mode shown in goal description**: Goal details page now shows `(Ranked)` / `(Turbo)` in the goal description
- **Refresh Matches error handling**: OpenDota API errors now show a friendly message and fall back to displaying cached local matches

## [0.1.9] - 2026-02-18

### Fixed
- Release workflow: sig files now correctly classified and inserted into latest.json

## [0.1.8] - 2026-02-18

### Added
- **Match Details View**: New per-match page with hero stats, KDA, GPM/XPM, damage, and a last-hits/denies chart showing progress against goal target lines
- **Daily Challenges**: Personalized daily challenges generated from recent performance with a progress bar, streak counter, and dashboard widget
- **Item Timing Goal Display**: Goal details page now correctly shows item name and M:SS formatted times for item timing goals

### Fixed
- **Suggested goal panel** now hides automatically once the matching goal has been created
- **Dev/prod database separation**: dev builds (`tauri dev`) use `dota_keeper_dev.db`; release builds use `dota_keeper.db` — no more risk of polluting production data during development
- **Fresh database crash**: app no longer crashes on first launch with a new database (cleanup query was running before tables were created)

## [0.1.7] - 2026-02-16

### Fixed
- Fixed autoupdate signature generation by enabling `createUpdaterArtifacts` in bundle configuration
- Fixed version placeholder substitution in release workflow (was using `v__VERSION__` instead of actual version)
- Enhanced release workflow debugging for better signature verification

## [0.1.5] - 2026-02-15

### Fixed
- Fixed release workflow to handle missing signature files gracefully
- Ensured latest.json is always generated and uploaded to releases
- Fixed goal evaluation showing incorrect results for unparsed matches
- Fixed last-hit goals using inaccurate linear estimation instead of actual per-minute data
- Parse process now requires per-minute data from OpenDota before marking match as "Parsed"
- Added clear error messages when OpenDota hasn't finished parsing a match yet

## [0.1.4] - 2026-02-14

### Changed
- Improved release workflow signature detection for more reliable multi-platform builds
- Enhanced signature file discovery to support different platform naming conventions

## [0.1.3] - 2026-02-12

### Added
- **Weekly Challenges**: New challenge system to track weekly improvement goals
- **Goal Suggestions**: Intelligent goal recommendations based on play patterns
- **Hero Favourites**: Mark and track favorite heroes for quick access
- **Tasks System**: Set and track personal Dota 2 improvement tasks
- **Charting Library**: Visual charts for performance metrics and trends
- **Last Hit Analysis**: Detailed last-hitting performance statistics
- **Per-Hero Detail Pages**: Dedicated pages for in-depth hero performance
- **Match Backfilling**: Automatic backfill of historical match data
- **Real-time Events**: Live updates for match data and statistics
- **Clear Matches**: Option to clear match history
- **Auto-update System**: Automatic application updates

### Changed
- **Analysis Range**: Increased number of games that can be analyzed at once
- **Font Rendering**: Improved font rendering for better readability
- **Documentation**: Enhanced README with comprehensive usage guidelines

### Fixed
- Backfilling reliability issues that could cause incomplete data retrieval
- Real-time event handling stability improvements

