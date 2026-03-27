# Test Guide: Play Store Screenshot Automation

## Prerequisites
- Node 18+, Playwright installed (`npm install` already handles this)
- Your Steam64 ID (find it at https://steamid.io/ or via your Steam profile URL)
- Your OpenDota profile must be **public**

## Steps

### 1. Configure your Steam ID
Copy the example env file and fill it in:
```
cp .env.screenshots.example .env.screenshots
# Edit .env.screenshots and set SCREENSHOT_STEAM_ID=76561198XXXXXXXXX
```

### 2. Fetch mock data from OpenDota
```
npm run screenshots:fetch
```
Expected output:
- Fetches 30 matches from OpenDota public API
- Prints: `Saved mock data to: scripts/mock-data.json`
- Check `scripts/mock-data.json` exists and contains matches/goals

### 3. Capture screenshots
```
npm run screenshots
```
(The script starts the Vite dev server automatically, takes screenshots, then stops it.)

Or if your dev server is already running:
```
node scripts/screenshots.js --no-server
```

Expected output:
- Prints progress for each device × route
- All 4 devices × up to 6 routes = up to 24 screenshots

### 4. Check output
Screenshots land in `meta/screenshots/<device>/<route>.png`:
```
meta/screenshots/
  phone/
    dashboard.png
    matches.png
    goals.png
    analysis.png
    match-detail.png
    goal-detail.png
  tablet7/  ...
  tablet10/ ...
  chromebook/ ...
```

## What to verify
- [ ] All 4 device folders exist with PNG files
- [ ] Screenshots show realistic data (matches, goals, progress charts) — not empty states
- [ ] Steam ID is NOT visible in screenshots (redacted in layout mock)
- [ ] Phone screenshots look good at 1080×1920
- [ ] Tablet screenshots look good at 1200×1920 and 1600×2560
- [ ] No error banners or loading spinners visible in screenshots

## One-shot (fetch + screenshot)
```
npm run screenshots:all
```
