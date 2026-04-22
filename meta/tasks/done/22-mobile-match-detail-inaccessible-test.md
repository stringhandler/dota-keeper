# Test: Match Detail Page Accessible on Mobile

## What was fixed
On mobile (≤640px), tapping a match row now navigates directly to `/matches/[matchId]`
instead of opening the Goals modal. Desktop behaviour is unchanged (modal still opens).

## Steps to test

### Mobile
1. Open the app on a mobile device or resize the browser to ≤640px.
2. Go to the Matches page.
3. Tap any match row.
4. Expected: navigates directly to the match detail page.
5. Tap the back button — returns to the matches list.

### Desktop
1. Open the app at >640px width.
2. Click any match row.
3. Expected: Goals modal opens (unchanged behaviour).
4. Modal footer "View Details" link still navigates to detail page.

### Edge case
- Resize the window across the 640px boundary — tap behaviour should update accordingly.
