# Hero Role Tagging & Role-Aware Goal Suggestions

## Overview

Each hero should be tagged as **core** or **support** in the hero data. Goal suggestions (weekly/daily) should then use this to:
1. Only suggest CS/kill/networth goals for core heroes
2. Only suggest deny/partner networth goals for support heroes
3. Label goals as `core_only`, `support_only`, or `anyone` so the UI can communicate who each goal is for

---

## Part 1 — Hero Role Tagging

### Data Source

OpenDota's hero data includes `primary_attr` and typical role lists. The simplest approach is a static map in `heroes.js` (or `heroes.rs`) marking each hero as core or support based on their most common position.

Rough split:
- **Core** (pos 1/2/3): carries, mids, offlaners — heroes primarily played for farm/damage
- **Support** (pos 4/5): heroes primarily played for utility, healing, disable

A hero can be tagged as **flex** if they're commonly played in both roles (e.g. Invoker, Rubick, Lone Druid).

### Frontend (`src/lib/heroes.js`)

Add a `heroRoles` map:
```js
export const heroRoles = {
  1: "core",     // Anti-Mage
  2: "core",     // Axe
  5: "support",  // Crystal Maiden
  // ...
};

export function getHeroRole(heroId) {
  return heroRoles[heroId] ?? "flex";
}
```

### Backend (`src-tauri/src/database.rs` or a new `heroes.rs`)

Add a Rust equivalent for use in suggestion generation:
```rust
fn get_hero_role(hero_id: i32) -> &'static str {
    match hero_id {
        1 => "core",
        5 => "support",
        // ...
        _ => "flex",
    }
}
```

---

## Part 2 — Goal Suggestion Role Awareness

### Existing suggestion system

`generate_hero_suggestion()` currently only generates last-hits suggestions — it doesn't differentiate core vs support.

### Changes needed

1. **Skip CS suggestions for support heroes** — if `get_hero_role(hero_id) == "support"`, don't suggest a last-hits goal. Instead suggest a deny goal.
2. **Skip deny suggestions for core heroes** — if `get_hero_role(hero_id) == "core"`, don't suggest deny goals.
3. **PartnerNetworth suggestions** — only relevant for support heroes (pos 4/5).

---

## Part 3 — Goal `applicability` Tag

Add an `applicability` field to the `Goal` struct (and DB column) to communicate who the goal is intended for:

```
core_only    — only evaluated for core heroes/matches
support_only — only evaluated for support heroes/matches
anyone       — evaluated for all heroes (default)
```

This can be:
- **Auto-derived** from `hero_scope` at display time:
  - `hero_scope = any_carry / any_core` → `core_only`
  - `hero_scope = any_support` → `support_only`
  - `hero_scope = null` with specific hero → derive from hero role
  - Otherwise → `anyone`
- Shown as a small badge on goal cards

---

## Acceptance Criteria

- [ ] All 140+ heroes tagged as core / support / flex in `heroes.js`
- [ ] Weekly goal suggestions only propose CS goals for core heroes
- [ ] Weekly goal suggestions only propose deny goals for support heroes
- [ ] Goal cards show a role applicability badge (Core, Support, or blank for Anyone)
- [ ] Role tag is derived automatically — no manual input required
