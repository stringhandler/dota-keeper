# Changelog

All notable changes to Dota Keeper will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

