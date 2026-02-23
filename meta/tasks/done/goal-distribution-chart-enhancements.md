# Goal Distribution Chart Enhancements

## Problem Statement

The current goal distribution chart shows performance data but lacks key contextual information:
- No visual indicator of the average/mean performance
- No highlighting of recent performance trends
- No proactive suggestions when user is consistently underperforming

Users need to see at a glance:
1. Their overall average performance relative to the goal
2. Recent performance trend (last 10 games)
3. Whether their goal difficulty is appropriate

## Design Principle

**Target Achievement Rate: 75%**

Goals should be challenging but achievable. Users should aim to hit their goals approximately **75% of the time**:
- Too easy (>90% success): Goal should be raised
- Optimal (70-80% success): Goal is well-calibrated
- Too hard (<60% success): Goal should be lowered
- Way too hard (<50% success): Urgent suggestion to lower goal

This creates a "stretch goal" mentality while maintaining motivation through regular success.

## Requirements

### 1. Show Average Line on Distribution Chart

**Visual Design:**
- Horizontal line across the chart at the average value
- Distinct color (e.g., yellow/gold) different from goal line
- Label: "Avg: {value}" positioned near the line
- Dashed or dotted style to differentiate from goal line

**Data:**
- Calculate mean of all games included in the distribution
- Update dynamically based on filters (date range, hero, etc.)

**Example:**
```
Goal: 70 CS @ 10min  (solid red line)
Avg:  58 CS          (dashed yellow line)
```

### 2. Highlight Last 10 Games as Dots

**Visual Design:**
- Individual dots/markers for each of the last 10 games
- Color-coded:
  - ✅ Green: Passed goal
  - ❌ Red: Failed goal
- Position:
  - X-axis: Game chronology (1st = oldest, 10th = newest)
  - Y-axis: Performance value for that game
- Larger or differently styled than histogram bins

**Interaction:**
- Hover over dot shows:
  - Match ID
  - Date
  - Hero played
  - Actual value
  - Pass/Fail status
- Click to navigate to match details

**Example Visualization:**
```
  100 |                                    ● (last game, 95 CS, passed)
   90 |              ●                  ●
   80 |          ●       ●          ●
   70 |━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  (Goal line)
   60 |      ●   ●                         (Average line - - -)
   50 |  ●
   40 |  ●   (1st game of last 10)
      |________________________________
```

### 3. Recent Performance Warning Banner

**Trigger Conditions:**

Check if mean of last 10 games is significantly below goal:
```
last_10_mean < goal_value * 0.95  // More than 5% below goal
```

**Banner Content:**

**Severity Levels:**

1. **Critical** (mean < 50% of goal):
   ```
   ⚠️ Your last 10 games averaged {value}, which is significantly below your goal of {goal}.
   Consider lowering your goal to {suggested_value} for more consistent progress.
   [Lower Goal] [Dismiss]
   ```

2. **Warning** (mean between 50-70% of goal):
   ```
   ⚠️ You're hitting this goal only {percentage}% of the time in your last 10 games.
   Aim for ~75% success rate. Consider lowering to {suggested_value}.
   [Lower Goal] [Keep Current] [Dismiss]
   ```

3. **Info** (mean between 70-95% of goal, just below target):
   ```
   ℹ️ You're close! Your recent average is {value}. Keep pushing to consistently hit {goal}.
   [Dismiss]
   ```

**Suggested Value Calculation:**
```rust
fn calculate_suggested_goal(last_10_mean: f64, current_goal: f64) -> f64 {
    // Set goal to be 10-15% above recent performance
    let suggested = last_10_mean * 1.125; // 12.5% stretch

    // Round to sensible number
    round_to_meaningful_value(suggested)
}
```

**Banner Position:**
- Top of goal detail page, below title
- Dismissible (stores dismissal in database)
- Re-appears after N more games if condition persists

### 4. Achievement Rate Display

**Show Success Percentage:**
```
┌─────────────────────────────────────────┐
│ Goal: 70 CS by 10 minutes               │
│                                          │
│ Achievement Rate: 68% (34/50 games)     │
│ Target: ~75%                             │
│                                          │
│ Status: 🟡 Close - Keep trying!         │
└─────────────────────────────────────────┘
```

**Status Indicators:**
- 🟢 **Excellent** (75-85%): "Well-calibrated goal!"
- 🟡 **Good** (65-75%): "Close! Keep pushing."
- 🟠 **Low** (50-65%): "Consider lowering goal"
- 🔴 **Critical** (<50%): "Goal too ambitious"
- 🔵 **Too Easy** (>85%): "Time to raise the bar!"

## Technical Implementation

### Data Structures

```rust
#[derive(Serialize)]
struct GoalDistributionData {
    distribution: Vec<DistributionBin>,
    average: f64,
    last_10_games: Vec<GameDot>,
    achievement_rate: AchievementRate,
    suggestion: Option<GoalSuggestion>,
}

struct GameDot {
    match_id: u64,
    date: String,
    hero_name: String,
    value: f64,
    passed: bool,
    x_position: usize, // 0-9 for chronological order
}

struct AchievementRate {
    percentage: f64,
    passed_count: u32,
    total_count: u32,
    status: AchievementStatus, // Excellent, Good, Low, Critical, TooEasy
}

#[derive(Serialize)]
enum AchievementStatus {
    Excellent,  // 75-85%
    Good,       // 65-75%
    Low,        // 50-65%
    Critical,   // <50%
    TooEasy,    // >85%
}

struct GoalSuggestion {
    severity: SuggestionSeverity, // Critical, Warning, Info
    message: String,
    suggested_value: Option<f64>,
}
```

### Backend Logic

```rust
fn calculate_goal_distribution(goal_id: u32) -> Result<GoalDistributionData, String> {
    let matches = get_goal_matches(goal_id)?;
    let goal = get_goal(goal_id)?;

    // Calculate distribution
    let distribution = build_distribution(&matches);

    // Calculate average
    let average = matches.iter()
        .map(|m| m.value)
        .sum::<f64>() / matches.len() as f64;

    // Get last 10 games
    let last_10: Vec<GameDot> = matches.iter()
        .rev()
        .take(10)
        .enumerate()
        .map(|(i, m)| GameDot {
            match_id: m.id,
            date: m.date.clone(),
            hero_name: m.hero.clone(),
            value: m.value,
            passed: m.value >= goal.target_value,
            x_position: 9 - i, // Reverse so newest is rightmost
        })
        .collect();

    // Calculate achievement rate
    let passed_count = matches.iter().filter(|m| m.value >= goal.target_value).count();
    let achievement_rate = AchievementRate {
        percentage: (passed_count as f64 / matches.len() as f64) * 100.0,
        passed_count: passed_count as u32,
        total_count: matches.len() as u32,
        status: determine_achievement_status(passed_count, matches.len()),
    };

    // Check if suggestion needed
    let suggestion = check_goal_suggestion(&last_10, &goal, &achievement_rate);

    Ok(GoalDistributionData {
        distribution,
        average,
        last_10_games: last_10,
        achievement_rate,
        suggestion,
    })
}

fn check_goal_suggestion(
    last_10: &[GameDot],
    goal: &Goal,
    achievement_rate: &AchievementRate,
) -> Option<GoalSuggestion> {
    if last_10.len() < 10 {
        return None; // Need 10 games for suggestion
    }

    let last_10_mean = last_10.iter().map(|g| g.value).sum::<f64>() / 10.0;
    let ratio = last_10_mean / goal.target_value;

    if ratio < 0.50 {
        // Critical: way below goal
        Some(GoalSuggestion {
            severity: SuggestionSeverity::Critical,
            message: format!(
                "Your last 10 games averaged {:.1}, which is significantly below your goal of {:.1}. Consider lowering your goal for more consistent progress.",
                last_10_mean, goal.target_value
            ),
            suggested_value: Some(calculate_suggested_goal(last_10_mean, goal.target_value)),
        })
    } else if ratio < 0.95 && achievement_rate.percentage < 75.0 {
        // Warning: below goal and not hitting 75% target
        Some(GoalSuggestion {
            severity: SuggestionSeverity::Warning,
            message: format!(
                "You're hitting this goal only {:.0}% of the time. Aim for ~75% success rate.",
                achievement_rate.percentage
            ),
            suggested_value: Some(calculate_suggested_goal(last_10_mean, goal.target_value)),
        })
    } else {
        None
    }
}
```

### Chart Component Updates (Chart.js)

```javascript
// Add to existing chart config
const chartConfig = {
    // ... existing config

    plugins: {
        annotation: {
            annotations: {
                goalLine: {
                    type: 'line',
                    yMin: goalValue,
                    yMax: goalValue,
                    borderColor: 'rgb(255, 99, 132)',
                    borderWidth: 2,
                    label: {
                        content: `Goal: ${goalValue}`,
                        enabled: true,
                        position: 'end'
                    }
                },
                averageLine: {
                    type: 'line',
                    yMin: averageValue,
                    yMax: averageValue,
                    borderColor: 'rgb(255, 205, 86)',
                    borderWidth: 2,
                    borderDash: [5, 5],
                    label: {
                        content: `Avg: ${averageValue.toFixed(1)}`,
                        enabled: true,
                        position: 'start'
                    }
                }
            }
        }
    },

    datasets: [
        // ... existing histogram dataset
        {
            type: 'scatter',
            label: 'Last 10 Games',
            data: last10Games.map((game, i) => ({
                x: i,
                y: game.value
            })),
            backgroundColor: last10Games.map(g =>
                g.passed ? 'rgb(75, 192, 192)' : 'rgb(255, 99, 132)'
            ),
            pointRadius: 8,
            pointHoverRadius: 10
        }
    ]
};
```

## UI/UX Design

### Layout

```
┌─────────────────────────────────────────────────────────┐
│ 70 CS by 10 minutes - Anti-Mage                         │
├─────────────────────────────────────────────────────────┤
│ ⚠️ Warning Banner (if applicable)                       │
│ Your last 10 games averaged 52 CS. Consider lowering... │
│ [Lower to 58 CS] [Keep Current] [✕]                     │
├─────────────────────────────────────────────────────────┤
│ Achievement Rate: 68% (34/50 games) 🟡 Target: ~75%     │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  100 │                                   ●               │
│      │                               ●   ● (last 10)     │
│   80 │          ●   ●   ●   ●                            │
│   70 │━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ (Goal)            │
│   60 │- - - - - - - - - - - - - - - - - (Avg: 58)       │
│   40 │  ●   ●                                            │
│      │_______________________________________________    │
│                                                          │
│      Distribution of all 50 games                       │
└─────────────────────────────────────────────────────────┘
```

### Banner Styling

```css
.goal-suggestion-banner {
    padding: 1rem;
    margin-bottom: 1rem;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    gap: 1rem;
}

.banner-critical {
    background: rgba(255, 0, 0, 0.1);
    border-left: 4px solid #ff0000;
}

.banner-warning {
    background: rgba(255, 165, 0, 0.1);
    border-left: 4px solid #ffa500;
}

.banner-info {
    background: rgba(0, 123, 255, 0.1);
    border-left: 4px solid #007bff;
}
```

## Success Criteria

- [ ] Average line is displayed on distribution chart
- [ ] Average value is accurate and updates with filters
- [ ] Last 10 games appear as dots on the chart
- [ ] Dots are color-coded (green=pass, red=fail)
- [ ] Hovering over dots shows game details
- [ ] Clicking dots navigates to match details
- [ ] Warning banner appears when last 10 mean < 95% of goal
- [ ] Banner severity levels work correctly
- [ ] Suggested goal value is reasonable (10-15% above recent average)
- [ ] Achievement rate percentage is accurate
- [ ] Status indicator matches achievement rate correctly
- [ ] Banner can be dismissed and doesn't re-appear immediately

## Testing Checklist

- [ ] Chart renders correctly with all elements
- [ ] Average calculation is accurate
- [ ] Last 10 games are chronologically correct (newest rightmost)
- [ ] Color coding of dots matches pass/fail status
- [ ] Tooltip hover works on all dots
- [ ] Click navigation to match works
- [ ] Banner appears at correct thresholds
- [ ] Banner dismissal works and persists
- [ ] Suggested goal values are sensible
- [ ] Achievement rate status matches percentage ranges
- [ ] Works with different goal types (CS, KDA, GPM, etc.)
- [ ] Performance with large datasets (100+ games)

## Edge Cases

### Fewer than 10 Games
- Don't show warning banner if < 10 games played
- Still show dots for available games (e.g., 5 dots for 5 games)
- Label: "Last {N} games" instead of "Last 10 games"

### All Games Pass
- Achievement rate: 100%
- Status: "Too Easy" → suggest raising goal
- Banner: "You're crushing this goal! Time to raise the bar?"

### All Games Fail
- Achievement rate: 0%
- Status: "Critical"
- Banner: Strong suggestion to lower goal significantly

### No Games Yet
- Don't show distribution chart
- Show: "Play some games to see your performance!"

### Exactly at Goal
- Average line coincides with goal line
- Show both but make average line slightly offset or different style

## Priority
**MEDIUM-HIGH** - Significantly improves goal management and user feedback

## Dependencies
- Chart.js with annotation plugin
- Goal evaluation system (existing)
- Match data (existing)

## Estimated Complexity
**Medium** - Chart updates, backend calculations, banner system

## Related Features
- Goal suggestion system (task: suggest-goal-adjustments.md)
- Goal evaluation (existing)
- Distribution charts (existing)
