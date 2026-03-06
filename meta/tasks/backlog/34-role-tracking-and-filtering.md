# Role Tracking and Role-Based Goal Filtering

## Problem Statement

Currently, the application doesn't track or display what role (position) the player was playing in each match. Goals apply universally across all games regardless of role, which doesn't reflect the reality that performance expectations differ significantly between positions (e.g., CS goals for Position 1 vs Position 5).

Users need to:
1. See what role they played in each match
2. Create goals that apply only to specific roles
3. Filter goals by one or multiple roles

## Requirements

### Part 1: Role Detection and Display

#### Data Source
- **Primary**: Use OpenDota API role/lane detection
  - `lane_role` field in match data
  - Maps to positions 1-5
- **Fallback**: Infer from hero and farm priority if API doesn't provide role

#### Role Definitions
```
Position 1 (Carry): Safe lane farmer, highest farm priority
Position 2 (Mid): Middle lane, second farm priority
Position 3 (Offlane): Offlane, third farm priority
Position 4 (Soft Support): Roaming/lane support, fourth priority
Position 5 (Hard Support): Ward bitch, lowest farm priority
```

#### Display Requirements
- **Match List**: Show role badge/icon next to hero name
- **Match Details**: Display role prominently
- **Statistics**: Show role distribution in analytics
  - "You played Position 1 in 35% of games"
  - "Most played: Position 3 (42 games)"

### Part 2: Role-Based Goal Filtering

#### Goal Configuration
- Goals can be assigned to one or more roles
- Options:
  - **All Roles** (default): Goal applies to every game
  - **Specific Roles**: Select 1-5 positions
  - Examples:
    - "80 CS by 10 min" → Positions 1, 2 only
    - "Ward at least 15 times" → Positions 4, 5 only
    - "Win rate > 55%" → All roles

#### UI Components
- **Goal Creation/Edit Form**:
  - Role selector with checkboxes or multi-select
  - Visual indicators (position icons/badges)
  - Quick presets:
    - "Cores" (Pos 1, 2, 3)
    - "Supports" (Pos 4, 5)
    - "Farm Priority" (Pos 1, 2)

- **Goal List View**:
  - Show role badges on each goal
  - Filter dropdown: "Show goals for: [All/Pos1/Pos2/etc]"

#### Goal Evaluation Logic
- Only evaluate goals against matches where player's role matches goal's roles
- Example:
  - Goal: "70 CS by 10 min" (Positions 1, 2)
  - Match #1: Position 1, 75 CS → ✅ Evaluated, Passed
  - Match #2: Position 5, 20 CS → ⊘ Not evaluated (wrong role)
  - Match #3: Position 2, 65 CS → ✅ Evaluated, Failed

### Part 3: Statistics and Analytics

#### Role-Based Insights
- Performance by role
  - Win rate per position
  - Average KDA per position
  - Average GPM/XPM per position
- Goal achievement rate by role
- Suggest goals based on most-played roles

## Technical Implementation

### Database Changes

#### matches table
```sql
ALTER TABLE matches ADD COLUMN role INTEGER; -- 1-5 for positions
ALTER TABLE matches ADD COLUMN lane TEXT; -- 'safe', 'mid', 'off', 'jungle', 'roaming'
```

#### goals table
```sql
ALTER TABLE goals ADD COLUMN roles TEXT; -- JSON array: '["1","2"]' or null for all roles
```

Or use a junction table for better normalization:
```sql
CREATE TABLE goal_roles (
    goal_id INTEGER NOT NULL,
    role INTEGER NOT NULL, -- 1-5
    PRIMARY KEY (goal_id, role),
    FOREIGN KEY (goal_id) REFERENCES goals(id)
);
```

### API Integration

#### OpenDota API
```rust
// In opendota.rs
pub struct MatchPlayer {
    // ... existing fields
    pub lane_role: Option<i32>,  // 1-5 for positions
    pub lane: Option<String>,    // "safe", "mid", "off", etc.
}

fn determine_role(player: &MatchPlayer) -> i32 {
    // Use lane_role if available
    if let Some(role) = player.lane_role {
        return role;
    }

    // Fallback: Infer from lane + farm metrics
    // TODO: Implement heuristic
    0 // Unknown
}
```

### UI Components

#### Role Badge Component (Svelte)
```svelte
<script>
    export let role; // 1-5

    const roleInfo = {
        1: { label: 'Pos 1', color: 'red', icon: '⚔️' },
        2: { label: 'Pos 2', color: 'blue', icon: '🔮' },
        3: { label: 'Pos 3', color: 'green', icon: '🛡️' },
        4: { label: 'Pos 4', color: 'yellow', icon: '👁️' },
        5: { label: 'Pos 5', color: 'purple', icon: '🌟' }
    };
</script>

<span class="role-badge role-{role}">
    {roleInfo[role].icon} {roleInfo[role].label}
</span>
```

#### Role Filter Component
```svelte
<script>
    let selectedRoles = [1, 2, 3, 4, 5]; // All selected by default

    function toggleRole(role) {
        if (selectedRoles.includes(role)) {
            selectedRoles = selectedRoles.filter(r => r !== role);
        } else {
            selectedRoles = [...selectedRoles, role];
        }
    }
</script>

<div class="role-filter">
    {#each [1, 2, 3, 4, 5] as role}
        <button
            class:active={selectedRoles.includes(role)}
            on:click={() => toggleRole(role)}>
            Pos {role}
        </button>
    {/each}
</div>
```

### Goal Evaluation Updates

```rust
// In goal evaluation logic
fn should_evaluate_goal(goal: &Goal, match_role: i32) -> bool {
    // If goal has no role restriction, always evaluate
    if goal.roles.is_none() || goal.roles.as_ref().unwrap().is_empty() {
        return true;
    }

    // Check if match role is in goal's allowed roles
    goal.roles.as_ref()
        .unwrap()
        .contains(&match_role)
}
```

## User Stories

### Story 1: Support Player
> "As a position 5 player, I want to create a goal for warding that only applies to my support games, so my carry games don't negatively affect my ward goal statistics."

### Story 2: Core Player
> "As a player who plays both mid and carry, I want to set different CS goals for each role (80 for carry, 70 for mid), so I can track role-specific improvement."

### Story 3: Flex Player
> "As a player who fills all roles, I want to see my performance broken down by position, so I can identify which roles I need to improve in."

### Story 4: Specialized Goals
> "As a carry player, I want goals that only count for my carry games (GPM, CS, etc.), while having separate goals for the occasional support game."

## Success Criteria

- [ ] Role is correctly detected and stored for all matches
- [ ] Role badges are visible in match lists and details
- [ ] Users can assign roles when creating/editing goals
- [ ] Goals are only evaluated for matches with matching roles
- [ ] Multi-role selection works correctly (e.g., goal applies to Pos 1 AND 2)
- [ ] Role statistics are accurate and insightful
- [ ] UI clearly indicates which roles a goal applies to

## Edge Cases

### Unknown Roles
- **Problem**: Some matches may not have role data
- **Solution**: Mark as "Unknown" role, allow goals to opt-in to include unknown roles

### Role Mismatch
- **Problem**: Player picks carry hero but ends up playing support (or vice versa)
- **Solution**: Trust OpenDota's lane_role detection; manual override option (future)

### Retroactive Role Assignment
- **Problem**: Existing matches don't have role data
- **Solution**:
  - Run migration to fetch role data for existing matches
  - Backfill from OpenDota API
  - Show progress indicator during backfill

### Role-less Goals
- **Problem**: Existing goals don't have role assignments
- **Solution**: Default to "all roles" to maintain backward compatibility

## Migration Plan

### Step 1: Database Migration
```sql
-- Add role columns
ALTER TABLE matches ADD COLUMN role INTEGER DEFAULT 0;
ALTER TABLE goals ADD COLUMN roles TEXT DEFAULT NULL;

-- Backfill role data for existing matches
-- This would be done via Rust code calling OpenDota API
```

### Step 2: Data Backfill
- Fetch role data for all existing matches
- Update matches table with role information
- Show progress in UI or run as background task

### Step 3: UI Updates
- Add role badges to match displays
- Add role selector to goal form
- Add role filter to goal list

## Future Enhancements

### Advanced Features
- **Role Prediction**: Use ML to predict role from draft/game state
- **Role-Specific Hero Suggestions**: "You have a 65% win rate on Pos 3 Mars"
- **Role Performance Trends**: Chart showing improvement over time per role
- **Meta Role Analysis**: Compare your role performance to current meta
- **Manual Role Override**: Allow users to correct incorrect role detection

### Analytics
- Role distribution over time (am I playing more support recently?)
- Best/worst roles by win rate
- Role-specific strengths and weaknesses
- Recommended role based on performance

## Priority
**HIGH** - Improves goal accuracy significantly and enables role-based analytics

## Dependencies
- OpenDota API must provide lane_role data
- Database migrations must complete successfully
- UI components for role selection

## Estimated Complexity
**High** - Requires database changes, API updates, UI work, and migration strategy

## Related Tasks
- Goal evaluation system (existing)
- Match fetching and storage (existing)
- Statistics and analytics (existing)
- Backfilling system (existing)
