# Pre-Release Manual Testing Checklist

Run these checks on a release build (or the latest beta installer) before publishing a release.

---

## 1. Feedback

**Goal:** Verify the in-app feedback form submits successfully to Supabase.

1. Launch the app.
2. Click the **Feedback** button in the sidebar (desktop) or the floating icon (mobile/narrow).
3. Select category **Bug / Something broken**.
4. Fill in the follow-up text field with any text (e.g. "Test submission — please ignore").
5. Select a priority (e.g. "Minor").
6. Click **Send Feedback**.
7. **Expected:** A green success toast appears: _"Feedback submitted — thank you!"_ and the modal closes.
8. Verify the entry appears in the Supabase `feedback` table.

**If you see "Feedback is not configured yet.":** The build env vars `PUBLIC_SUPABASE_URL` / `PUBLIC_SUPABASE_ANON_KEY` were not injected at build time. Check GitHub repository secrets.

---

## 2. Create a Goal

**Goal:** Verify that a new goal can be created and appears in the goals list.

1. Navigate to the **Goals** page.
2. In the **Create New Goal** form (left panel on desktop, or tap **+ New Goal** on mobile):
   - Set **Metric** to e.g. `Win Rate`.
   - Set a **Target** value (e.g. `55`).
   - Optionally set a **Hero** or leave as "All heroes".
3. Click **Add Goal**.
4. **Expected:** The goal appears in the goals grid on the right.
5. Verify the goal card shows the correct metric, target, and hero scope.

---

## 3. Edit and Delete a Goal

1. From the goals list, click **Edit** on the goal created above.
2. Change the target value.
3. Click **Update**.
4. **Expected:** The goal card reflects the updated value.
5. Click **Delete** on the same goal and confirm.
6. **Expected:** The goal is removed from the list.

---

## 4. Match Sync

**Goal:** Verify match data can be fetched from OpenDota.

1. Navigate to **Matches**.
2. Click **Sync** (or equivalent refresh button).
3. **Expected:** Recent matches load without error. At least the last few games are shown.

---

## 5. Auto-updater (release builds only)

If testing a release build that is not the latest version:

1. Launch the app.
2. **Expected:** An update notification appears within a few seconds (or on next launch).
