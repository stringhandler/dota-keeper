# Feedback Button

## Goal
Add a persistent feedback button in the app so users can report problems, share what they like, and request missing features. This data shapes prioritisation and validates UX decisions.

## UX Rationale

Keep the form short and opinionated. Every extra field reduces completion rate. The design targets three distinct signals:
- **Bugs** — something is broken and needs fixing
- **Missing features** — what the user expected but couldn't do
- **Positive signal** — what is worth protecting when making changes

Context (current page/route) is captured automatically so the user never has to describe where they were.

---

## Feedback Form Design

### Step 1 — Category (required, pick one)
> "What kind of feedback is this?"

- Bug / Something broken
- Missing feature or improvement
- Works great — tell us what you like

This single choice routes the rest of the form and determines the follow-up question.

---

### Step 2 — Follow-up (required, short textarea ~100 chars)

| Category selected | Prompt shown |
|---|---|
| Bug | "What happened? What did you expect?" |
| Missing feature | "What were you trying to do that wasn't possible?" |
| Works great | "What specifically do you like? Why is it useful?" |

---

### Step 3 — Priority signal (optional, single select)
> "How much is this affecting your use of the app?"

- Blocking — I can't use the app properly
- Annoying — I work around it
- Minor — Nice to have
- Not applicable

Only shown for Bug and Missing feature categories.

---

### Auto-captured context (not shown to user, attached to submission)
- Current route/page
- App version
- Platform (win/mac/linux)
- Date/time

---

## Delivery Mechanism

Submit feedback to **Supabase** via a direct REST insert (no Tauri backend command needed — the frontend can call the Supabase API directly).

### Supabase setup
- Create a `feedback` table with columns:
  - `id` uuid primary key default gen_random_uuid()
  - `created_at` timestamptz default now()
  - `category` text — 'bug' | 'feature' | 'positive'
  - `feedback` text
  - `priority` text nullable — 'blocking' | 'annoying' | 'minor'
  - `page` text — auto-captured current route
  - `app_version` text
  - `platform` text
- Enable Row Level Security with an INSERT-only anon policy (no reads for anon)
- Store the Supabase URL and anon key in a `.env` file as `PUBLIC_SUPABASE_URL` and `PUBLIC_SUPABASE_ANON_KEY`

The anon key is safe to ship in the client since the RLS policy only permits inserts.

---

## Placement

- **Trigger:** A small "Feedback" link in the bottom-left of the sidebar / nav, or a floating button on all pages
- **Modal:** Opens a compact modal overlay — no new route needed
- Keep it visible but unobtrusive; not a primary nav item

---

## Acceptance Criteria

- [ ] Feedback button visible on all pages
- [ ] Modal opens with the 3-field form described above
- [ ] Category selection changes the follow-up prompt dynamically
- [ ] Priority field only appears for Bug / Missing feature
- [ ] Submitting POSTs to Supabase and shows a success/error toast
- [ ] Auto-context (route, version, platform) is captured and stored
- [ ] Form resets after successful submission
- [ ] Supabase URL + anon key read from env vars (not hardcoded)
