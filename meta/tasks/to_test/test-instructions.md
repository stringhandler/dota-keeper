# Test Instructions – Batch: upnext tasks (2026-02-18)

## Prerequisites
- Build and run the app (`cargo tauri dev` or a release build)
- Have at least a few parsed matches in the database (needed for challenge generation and goal suggestions)
- Have at least one hero marked as a favourite

---

## 1. Fix: View on OpenDota (`fix-view-on-opendota.md`)

**Steps:**
1. Go to **Matches** page
2. Click on any match row to open the match detail page
3. Click the **"View on OpenDota"** button
4. Verify that your system browser opens to the correct OpenDota match URL

**Also test:** From the Matches list page, if there is a quick "View on OpenDota" link/button there, confirm it also opens the browser correctly.

**Expected:** Browser opens with `https://www.opendota.com/matches/<matchId>`. No error or silent failure.

---

## 2. Difficulty Levels for Suggestions (`difficulty-levels-for-suggestions.md`)

**Steps:**
1. Go to **Settings**
2. Scroll to the **"Goal Suggestions"** section
3. Change the difficulty dropdown to **Easy**, click Save
4. Go to **Analysis**, pick a hero with enough matches, click "Suggest Goal"
5. Note the suggested target value
6. Go back to Settings, change to **Hard**, Save
7. Suggest a goal for the same hero
8. Verify the Hard suggestion target is noticeably higher than the Easy suggestion

**Also test:** Select **Custom**, enter a custom percentage (e.g. 12%), Save, and verify suggestions reflect that range.

**Expected:** Easy = ~3–5% above baseline; Medium = ~5–10%; Hard = ~10–15%; Custom uses your entered %.

---

## 3. Edit Goal from Details Page (`edit-goal-from-details-page.md`)

**Steps:**
1. Go to **Goals**, click any existing goal
2. On the goal details page, find the **Edit** button (pencil icon or "Edit" label) in the header
3. Click it — an inline edit form should appear
4. Change the **target value** and/or **hero filter** and/or **game mode**
5. Click **Save**
6. Verify the page refreshes and shows the updated values
7. Click Edit again and click **Cancel** — verify no changes occur

**Expected:** Goal is updated in the database and reflected immediately on the page. Cancel discards edits.

---

## 4. Favourite Heroes in Filters (`favorite-heroes-in-filters.md`)

**Steps:**
1. Ensure at least one hero is marked as favourite in Settings
2. Go to **Analysis** page — open the hero filter dropdown
3. Verify that favourited heroes appear in a **"⭐ Favourites"** group at the top, followed by an "All Heroes" group
4. Go to **Goals** page — open the hero filter dropdown
5. Verify the same optgroup structure
6. Open any goal's detail page, click Edit
7. Verify the hero dropdown in the edit form also shows favourites at top

**Expected:** Favourited heroes are always visually separated at the top of hero dropdowns in Analysis, Goals, and Goal edit forms.

---

## 5–7. Weekly Challenges: Generation, Progress & UI (`weekly-challenge-generation.md`, `weekly-challenge-progress.md`, `weekly-challenge-ui.md`)

### 5a. Challenges page – view options
1. Click **Challenges** (🏆) in the left nav
2. You should see **3 challenge cards** (one Easy, one Medium, one Hard)
3. Each card shows a description, difficulty badge, and target

### 5b. Reroll
1. Click **Reroll** on any card — the 3 cards should refresh with new options
2. Reroll again — should work a second time
3. Reroll a third time — should show "No more rerolls" (max 2 rerolls per week)

### 5c. Skip
1. Click **Skip** on any card — a different set of 3 options should be shown (or message if exhausted)

### 5d. Accept a challenge
1. Click **Accept** on any card
2. The page should switch from the selection view to a **progress view** showing:
   - Challenge description
   - Progress bar (current value / target)
   - Games counted
   - Days remaining this week
3. Once accepted, the Reroll and other Accept buttons should no longer be visible

### 5e. Progress tracking
1. Parse a new match (via the main match sync if available)
2. Return to the Challenges page
3. Verify the progress value updates if the match contributes to the challenge

### 5f. Dashboard widget
1. Go to the **Dashboard** (home page)
2. A weekly challenge widget should appear showing:
   - Challenge description
   - Progress bar
   - Days remaining
3. Clicking the widget should navigate to `/challenges`

---

## 8. Challenge History (`challenge-history-page.md`)

**Steps:**
1. Accept a weekly challenge (or wait for an existing one to auto-complete/fail)
2. Navigate to **Challenges → View History** (link on the challenges page)
3. Verify completed/failed challenges appear in a list
4. Each item should show:
   - Type badge (daily / weekly)
   - Description
   - ✅ or ✗ status icon
   - Completion date (if completed)
5. Use the **All / Weekly / Daily** filter tabs to filter the list
6. Items should be grouped by date/week

**Expected:** History accurately reflects past challenges with correct statuses and dates.

---

## Notes
- Daily challenge history from before this batch should still appear on the history page under the "Daily" tab
- The challenges page should handle the case where no matches have been parsed yet (shows empty options or a "no data" message gracefully)
- Weekly challenges reset on Sunday — the `days_remaining` counter should show correct days until end of week
