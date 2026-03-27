# Dota Keeper - Problem Statement

## Overview

I want an application that tracks my Dota 2 games and analyzes my performance against a set of personal goals I define.

## Core Requirements

### Game Tracking
- Fetch match data using my Steam ID via the OpenDota or Steam API
- Automatically retrieve and store match history
- Track relevant match statistics (heroes played, win/loss, KDA, GPM, XPM, etc.)

### Goal Analysis
- Allow me to define custom goals (e.g., "Maintain 55% win rate on carry heroes", "Average less than 5 deaths per game")
- Analyze matches against these goals
- Provide progress tracking and insights

#### Goal Achievement Design Principle
**Target Achievement Rate: ~75%**

Goals should be challenging but achievable. When evaluating whether a user is passing or failing a goal, the system should aim for users to hit their goals approximately **75% of the time**:

- **Too Easy** (>85% success rate): Suggest raising the goal
- **Optimal** (70-80% success rate): Goal is well-calibrated, encourage user
- **Below Target** (60-70% success rate): Goal is challenging, user is close
- **Too Hard** (<60% success rate): Suggest lowering the goal
- **Way Too Hard** (<50% success rate): Strongly suggest lowering the goal

This creates a "stretch goal" mentality while maintaining motivation through regular success. The 75% target applies to all goal suggestions, notifications, and achievement status indicators throughout the application.

### Data Storage
- Store all data in a SQLite database
- Database location: User's local AppData directory (`%LOCALAPPDATA%/DotaKeeper/`)

## Technical Notes

> **AI Context Note**: Future AI prompts working on this project should reference this `project management` directory for project context, requirements, and documentation. This serves as the central location for project planning and specifications.


## Project management

Tasks are stored in `meta/tasks/`. It has `upnext`, `backlog` and `done`. When asked to create a new task, create it either in `backlog` or `upnext` as a `.md` file.
When a task is completed by you and requires manual verification, move to `meta/tasks/to_test`. You should also create a small .md doc with steps for me to test, in the same directory.Bugs go direct to the `bugs` folder, because we will fix those first.

## Use Case Specifications

Use case specs for all implemented features are stored in `meta/spec/`, organised by feature area (auth, matches, goals, dashboard, analysis, challenges, settings). They use Gherkin syntax and are the source of truth for future automated tests.

- When implementing a new feature, write or update the corresponding spec in `meta/spec/` using the same numbering convention (UC-00x auth, UC-01x matches, UC-02x goals, UC-03x dashboard, UC-04x analysis, UC-05x challenges, UC-06x settings).
- The long-term goal is to convert these into Cucumber-based automated tests (see task `meta/tasks/backlog/automated-cucumber-tests.md`).

## Releases

Where creating a release, you need to update package.json, src-tauri/Cargo.toml, tauri.conf.json. Also, populate the CHANGELOG.md with the changes

**Do NOT commit the release.** Just prepare the files (version bumps + changelog) and leave it staged or unstaged for the user to review and commit themselves.


## Status

- [ ] Initial setup
- [ ] API integration
- [ ] Database schema design
- [ ] Goal definition system
- [ ] Analysis engine
- [ ] User interface
