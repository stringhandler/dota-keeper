# Play Store Screenshot Automation

Automate capturing screenshots for all required Google Play Store device form factors.

## Required device types & sizes

| Type | Min size | Notes |
|------|----------|-------|
| Phone | 1080 x 1920 px (min) | Required |
| 7" Tablet | 1200 x 1920 px | Recommended |
| 10" Tablet | 1600 x 2560 px | Recommended |
| Chromebook | 1280 x 800 px | Optional |

Play Store accepts JPEG or 24-bit PNG, max 8MB per image, 2–8 screenshots per device type.

## Approach

Use **Playwright** (or similar headless browser tool) running against the SvelteKit dev server to:
1. Launch the app at each viewport size
2. Navigate to key screens (dashboard, matches, goals, analysis)
3. Inject fixture/mock data so screenshots look realistic (no empty states)
4. Hide any sensitive data (Steam ID, personal stats) — see task `hide-steam-id-screenshots.md`
5. Save screenshots to `meta/screenshots/<device>/` at the correct resolution

## Screens to capture (suggested set of 6)

1. Dashboard / overview
2. Match history list
3. Match detail
4. Goals list
5. Goal detail / progress chart
6. Analysis / hero stats

## Script outline

- `scripts/screenshots.ts` (or `.js`) — Playwright script
- Configurable: device profiles, routes, output dir
- Can be run with: `npm run screenshots` or `npx playwright ...`
- Output: ready-to-upload PNGs in `meta/screenshots/`

## Notes

- Mock data injection can be done by seeding a test SQLite DB and pointing the app at it, or by intercepting Tauri IPC calls via a Playwright fixture
- Consider `--hide-steam-id` flag that blurs/redacts the Steam ID field before capture
- Screenshots should reflect a realistic, active account (goals passing, several matches logged)
