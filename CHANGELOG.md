# Changelog

All notable changes to Dota Keeper will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.2] - 2026-03-10

### Fixed
- **Landing Page HTML Entities**: `&amp;` was rendering literally in the "Free & Open Source" badge and Linux platform description due to HTML entities being used inside JavaScript translation strings assigned via `textContent`.
- **Duplicate Refresh Icons**: The refresh button on the Matches page showed two `↻` arrows, and the dashboard refresh button showed two `🔄` icons. The icon was present in both the template and the translation string.

### Added
- **Russian Language Support**: Full Russian translation of the landing page and app UI.
- **FAQ Section**: Added a Frequently Asked Questions section to the landing page.
- **Google Analytics**: Added analytics to the landing page.

## [0.3.0] - 2026-03-07

### Fixed
- **Net Worth Goals**: Net Worth goals now correctly evaluate and display match history. The backend was returning no data for this metric despite per-minute gold data being available in the database. The player's own networth at the target minute is now correctly queried from the `player_networth` table using their match `player_slot`.

## [0.2.16] - 2026-03-06

### Added
- **Feedback Button**: Send feedback directly from within the app without leaving. Tap the chat icon above the bottom navigation bar to report a bug, request a missing feature, or share what you like. Feedback is submitted instantly and includes the current screen, app version, and platform automatically — no extra steps needed.

### Changed
- **Android build format**: Switched from APK to AAB (Android App Bundle) for smaller installs and better Play Store compatibility.
- **Separate beta app**: The beta build now installs as a distinct app (`Dota Keeper Beta`) alongside the production app, so you can run both without conflict.

## [0.2.12] - 2026-03-06

### Added
- **Feedback Button**: New persistent feedback button in the sidebar (desktop) and as a floating icon above the bottom nav (mobile). Opens a compact modal to report bugs, request features, or share what's working well. Feedback is submitted to Supabase and includes auto-captured context (current page, app version, platform).

### Fixed
- **Android CI Build**: Fixed Android build workflow — restored required `--apk true` value and added a `cargo-tauri` wrapper script so the Tauri CLI resolves correctly in CI.

## [0.2.11] - 2026-03-04

### Added
- **Background Parse**: Match data is now parsed in the background, so the app remains responsive while fetching and processing new matches.
- **First Run Experience**: New users are guided through an onboarding flow on first launch to set up their Steam ID and initial preferences.
- **Factory Reset**: New option in Settings to completely reset the app — clears all data and returns to the first-run state.
- **Hero Pick on Startup**: Hero pool data is refreshed automatically on startup so the hero list is always up to date.

### Fixed
- **Database Locking**: Eliminated intermittent "database is locked" errors on startup by replacing per-command `Connection` creation with a single shared `Mutex<Connection>` for the app lifetime. Also enabled WAL journal mode and a 5-second busy timeout as additional hardening.
- **Android CI Build**: Fixed `tauri android build --apk` argument syntax for the updated Tauri CLI. Android builds now also trigger on the `beta` branch.
- **Bug Fixes**: Various stability and correctness improvements.

## [0.2.7] - 2026-03-02

### Added
- **Mental Health / Mind Tab**: New "Mind" section for tracking tilt and emotional state after games
  - Post-game mood check-in prompt appears after matches with a mood slider and optional notes
  - Tilt assessment based on recent check-in history (calm / watch yourself / tilted / on tilt)
  - Full check-in history with mood trend visualisation and personalised suggestions
  - Check-in frequency setting (after every game, every 3 games, or every 5 games)
- **Steam Login**: Authenticate via Steam OpenID directly in the app as an alternative login path
- **Toast Notifications**: Global toast notification system for in-app feedback messages

### Fixed
- **OpenDota Parsing**: Resolved parsing edge cases that could cause match data to be incorrectly marked as parsed
- **Goal Sparklines**: Fixed sparkline charts on the dashboard not rendering correctly for goals

## [0.2.6] - 2026-02-26

### Added
- **Custom Title Bar**: Native window frame replaced with a custom in-app title bar. Includes drag-to-move, minimise, maximise/restore, and close buttons. Close button turns red on hover.
- **Window Resize Handles**: Invisible resize handles at all window edges and corners restore the ability to resize the window after removing native decorations.
- **Goal Hero Group Filters**: When creating or editing a goal, you can now scope it to "Any Hero", "Any Core (pos 1–3)", "Any Carry (pos 1)", or "Any Support (pos 4–5)" in addition to a specific hero. Goals evaluate correctly based on the detected lane role of each match.
- **Role Detection**: Each match now stores the detected lane role (1–5) from OpenDota parsed data, enabling role-aware goal evaluation.
- **Denies Goal Metric**: New goal metric to track denies at a given game minute.
- **Partner Networth Goal Metric**: New goal metric to track your lane partner's networth, useful for support players measuring their impact on the carry.

### Fixed
- **Font Weight**: Barlow font weights 600 and 700 are now properly loaded from Google Fonts (previously only 300–500 were included, causing browser-synthesised bold that appeared too heavy).
- **Weekly Challenge UI**: After completing a weekly challenge, the challenges page now correctly shows the completed challenge and its progress instead of showing the option-selection screen again.

## [0.2.5] - 2026-02-25

### Added
- **Hero Icons in Dropdowns**: Hero selection dropdowns across Goals, Analysis, and Goal Details pages now display hero portrait icons alongside hero names for easier selection
- **Item Icons**: Item icons are now displayed on the Goals page and Goal Details page for item timing goals
- **New Heroes**: Added Kez and Lara (Largo) to the hero roster

### Changed
- **Default Window Size**: Increased default window size to 1280x800 for a better out-of-the-box experience
- **Weekly Goals Display**: Fixed name and display of weekly goals on the dashboard

## [0.2.4] - 2026-02-23

### Added
- **Goal Distribution Enhancements**: Significant improvements to the goal detail page
  - Average line on histogram: dashed yellow vertical line showing your mean performance
  - Achievement rate card: shows pass rate % with colour-coded status (Too Easy / Excellent / Good / Low / Critical) and a progress bar with a 75% target marker
  - Warning banner: appears when recent pass rate drops below 75%, with severity levels (critical/warning/info) and a one-click "Lower to X" button to adjust the goal
  - Last N games scatter chart: chronological dot chart below the histogram showing pass/fail per recent game, with hover tooltips and click-to-match navigation

### Fixed
- **Item Timing Goal Description**: Fixed incorrect description text for item timing goals

## [0.2.3] - 2026-02-22

### Changed
- **Match Details Redesign**: Enhanced match details page with improved layout and visual presentation
- **Goal Details Redesign**: Refreshed goal details page for better readability and user experience
- **Challenges Redesign**: Updated challenges page with new design and improved navigation
- **UI Updates**: Various user interface improvements across multiple pages

### Added
- **Logout in Settings**: Added logout functionality in settings page
- **Enhanced Analytics**: Improved PostHog tracking for better insights
- **New Icons**: Updated iconography throughout the application
- **Contributing Guidelines**: Added contribution documentation for developers

## [0.2.2] - 2026-02-20

### Fixed
- **Auto-update URLs**: Fixed repository name in auto-update URLs (changed from `dota-goals` to `dota-keeper`)
- **Windows installer URL**: Removed incorrect `.zip` extension from Windows MSI download URL

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

