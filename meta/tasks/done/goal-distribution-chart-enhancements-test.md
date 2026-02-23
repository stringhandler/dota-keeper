# Test: Goal Distribution Chart Enhancements

## Setup

You'll need a goal that has at least a few games tracked against it. Ideally one with 10+ games and a mix of passes/failures.

## Tests

### 1. Average Line on Histogram

- Open a goal detail page that has tracked matches
- **Expected**: A dashed semi-transparent yellow line appears on the histogram, labeled "Avg: {value}"
- **Expected**: The average line is positioned at the correct value on the X-axis
- **Expected**: It appears alongside (and distinct from) the solid gold Target line
- Check legend shows "Target" and "Average" line entries with correct styling

### 2. Achievement Rate Card

- In the Statistics section, below the 6-stat grid
- **Expected**: Shows "Achievement Rate: X% (N/M games)"
- **Expected**: Shows "Target: ~75%"
- **Expected**: Shows a status label and description with colour-coded text:
  - >85%: "Too Easy — Time to raise the bar!" (blue)
  - 75-85%: "Excellent — Well-calibrated goal!" (green)
  - 65-75%: "Good — Close! Keep pushing." (yellow)
  - 50-65%: "Low — Consider lowering goal" (orange)
  - <50%: "Critical — Goal too ambitious" (red)
- **Expected**: Progress bar fills to the achievement rate % with a tick mark at 75%

### 3. Warning Banner

- For a goal where you're passing <75% of the time (in last 5+ games):
- **Expected**: Banner appears above the histogram
- **Expected**: Severity based on recent pass rate:
  - <50% pass rate → Red critical banner with suggested lower value + "Lower to X" button
  - 50-70% pass rate → Orange warning banner with suggested lower value + "Lower to X" button
  - 70-75% pass rate → Blue info banner (no lower button)
- Click "Lower to X" → **Expected**: Edit form opens with new target value pre-populated
- Click ✕ → **Expected**: Banner dismisses and stays dismissed (refresh page to confirm)
- After 3+ new games are tracked, banner should re-appear if condition persists

### 4. Last N Games Scatter Chart

- Below the histogram legend, a "Last N Games" section appears (if you have any tracked games)
- **Expected**: Scatter chart shows dots for up to last 10 games
- **Expected**: Dots are green for passed, red for failed
- **Expected**: Horizontal solid gold "Goal" line at target value
- **Expected**: Horizontal dashed yellow "Avg" line at overall average
- **Expected**: Numbers 1–N along X-axis (1=oldest, N=newest)
- Hover over a dot → **Expected**: Tooltip shows value, hero name, date, pass/fail status, "Click to view match" hint
- Click a dot → **Expected**: Navigates to `/matches/{matchId}`

### 5. Filters Interaction

- Apply a hero filter or date filter
- **Expected**: Average line on histogram updates to reflect filtered data
- **Expected**: Achievement rate card updates
- **Expected**: Last 10 games scatter updates (only shows filtered games)
- **Expected**: Suggestion banner re-evaluates based on filtered data

### 6. Edge Cases

- Goal with <5 tracked games → **Expected**: No warning banner shown
- Goal with 5-9 games → **Expected**: Scatter shows all available games (label says "Last 5 games" etc.)
- ItemTiming goal → **Expected**: Values display as M:SS format in all locations
- Goal with 100% pass rate → **Expected**: Achievement status shows "Too Easy" in blue
