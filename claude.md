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

### Data Storage
- Store all data in a SQLite database
- Database location: User's local AppData directory (`%LOCALAPPDATA%/DotaKeeper/`)

## Technical Notes

> **AI Context Note**: Future AI prompts working on this project should reference this `project management` directory for project context, requirements, and documentation. This serves as the central location for project planning and specifications.


## Project management

Tasks are stored in `meta/tasks/`. It has `upnext`, `backlog` and `done`. When asked to create a new task, create it either in `backlog` or `upnext` as a `.md` file.

## Status

- [ ] Initial setup
- [ ] API integration
- [ ] Database schema design
- [ ] Goal definition system
- [ ] Analysis engine
- [ ] User interface
