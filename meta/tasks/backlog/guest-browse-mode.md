# Guest Browse Mode (App Store Compliance)

## Problem

Apple App Store guidelines require that apps do not force users to log in with a third-party account (e.g., Steam) before being able to use the app. The current onboarding flow requires Steam login as the very first step, which may cause rejection during App Store review.

Reference: [App Store Review Guideline 5.1.1](https://developer.apple.com/app-store/review/guidelines/#data-collection-and-storage) — apps must not require users to log in to a social network or third-party service to use core app functionality.

## Goal

Allow users to browse key screens of the app without providing a Steam ID, so that App Store reviewers (and curious new users) can explore the app before committing to connecting their account.

## Desired Behaviour

- On first launch, show an onboarding/welcome screen with two options:
  - **"Get Started"** — prompts the user to enter their Steam ID (existing flow)
  - **"Browse as Guest"** — skips login and enters the app with sample/demo data
- In guest mode, the app shows:
  - The Goals screen populated with example goals
  - The Matches screen populated with a small set of fake/demo match data
  - The Analysis/Progress screens using the demo data
- A persistent banner or indicator shows "Guest Mode — connect your Steam account to track your real games"
- Tapping the banner (or visiting Settings) lets the user exit guest mode and enter their Steam ID at any time
- When the user enters their Steam ID, demo data is cleared and real data is fetched

## Implementation Notes

- Demo data can be hardcoded JSON bundled with the app (a small set of realistic-looking matches and a few example goals)
- Guest mode state should be stored in app settings (`is_guest: bool`)
- All Tauri commands that fetch real data should short-circuit and return demo data when `is_guest` is true
- No network requests should be made in guest mode (keep it fully offline)

## Acceptance Criteria

- [ ] First-launch screen offers "Get Started" and "Browse as Guest" options
- [ ] Guest mode populates the app with demo matches and goals
- [ ] A clear visual indicator shows the user they are in guest mode
- [ ] User can exit guest mode at any time via Settings
- [ ] No Steam ID or network request is required to enter guest mode
- [ ] App Store reviewer can fully explore the UI without a Steam account
