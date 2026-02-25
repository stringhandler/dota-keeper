# Goal Hero Scope: Any Core / Any Carry / Any Support

## Overview

When creating a goal, the hero selector currently offers individual heroes plus "Any Hero". Add three new group options so goals can target a role category rather than a specific hero.

## New Options

| Value | Meaning |
|-------|---------|
| `any` | All heroes (existing) |
| `any_carry` | Pos 1 heroes (carry) |
| `any_core` | Pos 1 + 2 + 3 (carry, mid, offlane) |
| `any_support` | Pos 4 + 5 (soft + hard support) |

## Implementation

### Backend (Rust)

#### 1. Goal struct

Replace `hero_id: Option<i32>` with a `hero_scope` field (or keep `hero_id` as `NULL` and add a separate `hero_scope TEXT` column):

Recommended: add a `hero_scope` column to the `goals` table:

```sql
ALTER TABLE goals ADD COLUMN hero_scope TEXT;
-- NULL = specific hero (use hero_id), or one of: 'any', 'any_carry', 'any_core', 'any_support'
```

When `hero_scope` is set, `hero_id` should be `NULL`.

#### 2. Goal evaluation (`database.rs`)

In `evaluate_goal`, when filtering matches for a goal, use `hero_scope` to match against `matches.role`:

```rust
match goal.hero_scope.as_deref() {
    Some("any") | None if goal.hero_id.is_none() => true,      // all matches
    Some("any_carry")  => match_data.role == 1,
    Some("any_core")   => matches!(match_data.role, 1 | 2 | 3),
    Some("any_support")=> matches!(match_data.role, 4 | 5),
    _ => match_data.hero_id == goal.hero_id.unwrap_or(-1),      // specific hero
}
```

> **Prerequisite**: role detection task must be complete (already done — `matches.role` is populated).

### Frontend (Svelte)

In the goal creation form, the hero dropdown should show:

```
── Any Hero
── Any Core  (pos 1/2/3)
── Any Carry (pos 1)
── Any Support (pos 4/5)
── ── Heroes ──
   Lion, Crystal Maiden, ...
```

Set `hero_scope` to the appropriate string and `hero_id` to `null` when a group option is chosen.

## Acceptance Criteria

- [ ] Goal creation UI shows "Any Hero", "Any Core", "Any Carry", "Any Support" options
- [ ] Goals created with a group scope evaluate correctly against `matches.role`
- [ ] Existing goals (specific hero or "Any Hero") are unaffected
- [ ] Goal cards/titles display the scope name (e.g. "Any Support — Last Hits @ 10 min")
