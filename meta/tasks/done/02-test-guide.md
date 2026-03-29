# Test Guide: Goal Game Mode Filter (Task 02)

## Setup
- Have at least a few goals already created (Ranked/Turbo)
- Have some ranked and turbo matches in the DB

## Test Cases

### 1. Create a goal with "Any" mode
- Go to Goals page
- Click Add Goal
- Verify the Mode dropdown defaults to **Any**
- Create a goal with Mode = Any
- Verify it appears in the list with no mode label (or "Any")

### 2. Create goals with Ranked and Turbo
- Create a goal with Mode = Ranked
- Create a goal with Mode = Turbo
- Verify each shows the correct label in the goal list

### 3. Goal evaluation respects mode filter
- Open a goal set to Ranked — its match history/histogram should only show ranked matches (game_mode 22)
- Open a goal set to Turbo — should only show turbo matches (game_mode 23)
- Open a goal set to Any — should show all matches

### 4. Edit an existing goal
- Open any goal's detail page
- Click Edit
- Verify the Mode dropdown shows the current mode
- Change to a different mode, save, verify it persists

### 5. Backward compat
- Existing goals that had "ranked" or "turbo" stored should load correctly
- Any goal with an unrecognised game_mode value should fall back to "Any"

### 6. Description string
- Goals with mode = Any should NOT show "(Any)" in the description string
- Goals with mode = Ranked should show "(Ranked)"
- Goals with mode = Turbo should show "(Turbo)"
