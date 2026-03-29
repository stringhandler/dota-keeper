# Test Guide — Mental Health Check-in Frequency Setting

## Setup
- Make sure the app is built and running
- Log in with a Steam ID that has some match history

## Steps

### 1. Frequency selector visibility
- Go to **Settings → Mental Wellbeing**
- With tracking **OFF**: confirm the frequency selector is **not visible**
- Toggle tracking **ON**: confirm a "How often to check in?" selector appears below the toggle

### 2. Default value
- The selector should default to **Once per session**

### 3. Save on change
- Change the selector to **Every 3 games**
- Navigate away from Settings and return
- Confirm the selector still shows **Every 3 games** (persisted)

### 4. Frequency modes (manual verification using DB or match count)

#### every_game
- Set to **Every game**
- Open the dashboard — the check-in card should appear if there is any unchecked match

#### once_per_session
- Set to **Once per session**
- Open the dashboard — check-in appears (if unchecked matches exist)
- Dismiss it, navigate away and back — check-in should **not** appear again this session

#### after_loss
- Set to **After every loss**
- If your most recent match was a win → no check-in card
- If your most recent match was a loss → check-in card should appear

#### every_3 / every_5 / every_10
- These require unchecked matches since the last real check-in
- With `every_3`: complete a check-in, then play/import 3 more matches without checking in → check-in should appear

### 5. Loss streak override
- Set frequency to **Every 10 games**
- Ensure there are 3+ consecutive losses in recent match history
- Open dashboard — check-in should appear regardless of the game count

### 6. Restart persistence
- Change setting to **Every 5 games**
- Close and reopen the app
- Go to Settings → confirm selector still shows **Every 5 games**
