# Onboarding: Role Selection for Goal Prioritisation

## Overview

Extend the first-run onboarding flow with a role selection step. Ask the user which roles they primarily play, then use that information to rank and prioritise goal suggestions so the most relevant goals appear first.

## New Onboarding Step

Insert after Step 2 (Favourite Heroes) as **Step 3 — Your Roles** *(skippable)*:

**"Which roles do you play?"**

Multi-select chips (choose all that apply):

| Role | Example Heroes |
|------|---------------|
| Carry | AM, Jugg, Spectre |
| Mid | Invoker, OD, SF |
| Offlane | Axe, Tide, Bristle |
| Support (4) | ES, Ogre, Bane |
| Hard Support (5) | CM, Oracle, Jakiro |

- Store selections as `preferred_roles: Vec<String>` in Settings.
- Skip copy: *"You can update this in Settings later."*

## Backend

1. Add `preferred_roles: Vec<String>` (default `[]`) to the `Settings` struct with `#[serde(default)]`.
2. Add `save_preferred_roles(roles: Vec<String>) -> Result<Settings, String>` Tauri command.
3. Update goal suggestion logic (`get_or_generate_hero_suggestion`, or wherever goal templates are ranked) to boost goals that match the user's preferred roles. For example:
   - Carry players → last hits, GPM, net worth goals ranked higher
   - Support players → ward/assist goals, deaths goals ranked higher
   - Mid players → XPM, rune control goals ranked higher

## Frontend

- Add the role step between hero step and mental health step in `src/lib/OnboardingFlow.svelte`.
- Update `TOTAL_STEPS` from 4 → 5.
- Add a role preference section in Settings (`/settings`) so users can update this later.

## Acceptance Criteria

- [ ] Role step shown in onboarding, skippable
- [ ] Selected roles persisted to settings
- [ ] Goal suggestions prioritised based on preferred roles
- [ ] Settings page allows updating roles after onboarding
- [ ] Skipping role step has no effect on goal suggestions (neutral ordering)
