# Test Guide: Weekly Challenge Completed Display

## Changes Made

1. **Backend** (`database.rs`): `get_active_weekly_challenge` already queries `status IN ('active', 'completed')` — completed challenges are correctly returned.

2. **Dashboard** (`src/routes/+page.svelte`): When a weekly challenge is completed, the weekly card now shows both "✓ Complete!" AND "Resets in Xd" side by side.

3. **Challenges page** (`src/routes/challenges/+page.svelte`):
   - Added a green "✓ Challenge Complete!" banner at the top of the card
   - The card gets a green border when completed
   - "Games counted" meta item is hidden for completed challenges (it would show 0)
   - "Days remaining" label changes to "Resets in" for completed challenges

## Steps to Test

### Prerequisite
You need a completed weekly challenge. Either:
- Accept a challenge and manually complete it by playing games
- Or temporarily lower the target in the DB to make it easy to complete

### Test 1: Dashboard shows completed state
1. Complete a weekly challenge
2. Go to the Dashboard
3. **Expected**: Weekly challenge card shows challenge description, 100% fill bar, "✓ Complete!" badge, and "Resets in Xd" countdown
4. **Previously wrong**: Would show "Choose this week's challenge →" empty state

### Test 2: Challenges page shows completed state
1. Navigate to `/challenges`
2. **Expected**:
   - Green "✓ Challenge Complete!" banner at top of card
   - Green border on the card
   - Progress bar is green and 100% filled
   - "RESETS IN" label instead of "DAYS REMAINING"
   - "STATUS: Completed!" in green
   - No "Games counted" row (hidden when completed)
3. **Previously wrong**: Would show "Choose one of this week's challenges" options view

### Test 3: Refresh persists
1. After completing a challenge, close and reopen the app
2. Repeat tests 1 and 2
3. **Expected**: Same completed state shown correctly after restart
