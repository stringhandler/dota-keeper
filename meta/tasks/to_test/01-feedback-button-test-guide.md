# Test Guide: Feedback Button

## Pre-requisites

### Supabase setup (required for submission to work)
1. In your Supabase project, create a `feedback` table:
   ```sql
   create table feedback (
     id uuid primary key default gen_random_uuid(),
     created_at timestamptz default now(),
     category text not null,
     feedback text not null,
     priority text,
     page text,
     app_version text,
     platform text
   );
   ```
2. Enable Row Level Security on the table:
   ```sql
   alter table feedback enable row level security;
   create policy "anon insert" on feedback for insert to anon with check (true);
   ```
3. Add your Supabase URL and anon key to `.env` at the project root:
   ```
   PUBLIC_SUPABASE_URL=https://your-project.supabase.co
   PUBLIC_SUPABASE_ANON_KEY=your-anon-key
   ```
4. Rebuild/restart the dev server after editing `.env`.

---

## Test Checklist

### Visibility
- [ ] On desktop: a "Feedback" button appears at the bottom of the left sidebar (below the Rank pill)
- [ ] On mobile: a small floating chat-bubble icon appears above the bottom navigation bar

### Modal behaviour
- [ ] Clicking the button opens a modal overlay
- [ ] Clicking the backdrop (outside the card) closes the modal
- [ ] Clicking the ✕ button closes the modal

### Category selection
- [ ] Three options visible: Bug / Missing feature / Works great
- [ ] Selecting a category highlights that button
- [ ] Follow-up textarea appears only after a category is selected

### Dynamic follow-up prompt
- [ ] Bug → prompt reads "What happened? What did you expect?"
- [ ] Feature → prompt reads "What were you trying to do that wasn't possible?"
- [ ] Positive → prompt reads "What specifically do you like? Why is it useful?"

### Priority field
- [ ] Priority radios appear for Bug and Feature categories
- [ ] Priority radios are NOT shown when "Works great" is selected

### Submit button state
- [ ] Disabled when no category selected
- [ ] Disabled when category selected but textarea is empty
- [ ] Enabled once both category and textarea have values

### Submission (requires Supabase configured)
- [ ] Submit shows "Sending..." on the button while in flight
- [ ] On success: success toast appears, modal closes, form resets
- [ ] Row appears in Supabase `feedback` table with correct category, feedback text, page, app_version, platform
- [ ] For Bug/Feature with priority selected: `priority` column is populated
- [ ] For Bug/Feature with no priority selected: `priority` column is null
- [ ] For Positive: `priority` column is always null

### Error handling
- [ ] If `PUBLIC_SUPABASE_URL` / `PUBLIC_SUPABASE_ANON_KEY` are empty, an error toast is shown instead of crashing
