# UI Redesign — Progress Tracker

> Reference design: `meta/tasks/backlog/redesign/dota-keeper-redesign.html`
> Full task spec: `meta/tasks/backlog/redesign/dotakeeper-ui-tasks.md`

## Design System

New color palette and fonts (matching reference HTML):
- Fonts: Rajdhani (headings), Barlow Condensed (labels/buttons), Barlow (body)
- Background: `#080c10` base, `#0d1318` cards, `#16202a` elevated
- Gold: `#f0b429`, Teal: `#2dd4bf`, Green: `#4ade80`, Red: `#f87171`
- Text: `#e8dcc8` primary, `#8a7f6e` secondary, `#504840` muted

## Files to Modify

### ✅ `src/app.html`
- Added Google Fonts: Rajdhani, Barlow Condensed, Barlow

### ✅ `src/app.css`
- Full replacement with new CSS design system variables
- Added shared component classes (`.btn`, `.goal-row`, `.dot`, `.mode-tag`, `.goals-chip`, `.filter-chip`, `.analysis-card`, etc.)
- Set `html, body, #svelte` to `height: 100%; overflow: hidden` for fixed-height layout

### ✅ `src/routes/+layout.svelte`
Changes needed:
- Sidebar width: 280px → 220px
- New sidebar styles: `var(--bg-card)` background, clean border
- Brand: Rajdhani font, gold color, Steam ID badge
- Nav items: SVG icons (heroicons), gold `::before` left-border active indicator, `rgba(240,180,41,0.06)` active bg
- Sidebar footer: rank pill (★ + "RANK" label + "N/A" value) + logout button
- Add **Topbar** div inside `.main`:
  - Left: page title (derived from `$page.url.pathname`) in Rajdhani
  - Right: challenge badge (⚡ Daily Challenge: X/Y) + "Refresh" ghost btn + "+ New Goal" primary btn
  - Load daily challenge progress on mount for the badge
- `.main-panel` → `.main` (flex column, no padding, `overflow: hidden`)
- `.content-area`: `flex: 1; overflow-y: auto; padding: 28px 32px`
- Login/loading screens: update colors to match new theme

### ✅ `src/routes/+page.svelte` (Dashboard)
Changes needed:
- Remove `page-header` div (title now in topbar)
- Add **quick stats strip** (4 cards at top):
  - Win Rate (7d) — load from `invoke("get_matches")` + compute
  - Avg CS @ 10min — load from `invoke("get_last_hits_analysis_data", ...)`
  - Goals Hit (7d) — compute from `goalCalendar`
  - Active Goals — `goals.length`
- Move **Today's Challenge** above the goals grid, styled as banner card with gold border
- Replace calendar grid with **goal card rows** (`.goal-row` class from app.css):
  - Hero avatar (HeroIcon), goal name, progress bar, goal meta, 7 colored dots
  - Entire row is clickable → navigate to `/goals/{id}`
  - No "View Details" buttons
- Add trend label per goal (Improving/Declining/Stable based on recent vs earlier days)
- Add `Manage Goals` button in section header
- Keep weekly challenge section (styled consistently)
- Keep hero suggestion section (styled consistently)

### ✅ `src/routes/matches/+page.svelte`
Changes needed:
- Remove `page-header` div
- Add **filter chips row**: All | Wins | Losses | Ranked | Turbo | [hero chips]
  - `activeFilter` state, `getFilteredMatches()` function
  - Tracked heroes derived from matches list
- Move **Refresh Matches** button into filter row (right side)
- Replace "Parse" column with no column (auto-parse handles it); parse state shown as small icon
- Replace goals button with **Goals chip** (`.goals-chip` / `.goals-chip.has-goals`)
- Style **game mode** column with `.mode-tag.mode-turbo` or `.mode-tag.mode-ranked` pills
- Make rows visually clickable (cursor: pointer, hover bg)
- Keep all existing pagination, modal, parse logic

### ✅ `src/routes/analysis/+page.svelte`
Changes needed:
- Remove `page-header` div
- Move all filters to a **horizontal row** (remove `filters-section` card wrapper)
- Remove `heroFilter` "Show" dropdown (always show all, sorted by favorites first)
- Replace sections with **2-column card grid** (`.analysis-grid`):
  - Card 1: Big LH stat + trend label + change indicator + mini sparkline SVG
  - Card 2: Hero breakdown bar chart + insight callout box
  - Card 3: Performance range (min/avg/max + visual range bar)
  - Card 4: Top/selected hero individual trend
- Load `goals` from backend for insight generation
- Keep all existing filter logic and hero breakdown logic

### ✅ `src/routes/goals/+page.svelte`
Changes needed:
- Remove `page-header` div
- Replace stacked form with **inline create form** (horizontal row):
  - Hero | Metric | Target/Item | Time/Seconds | Mode | Add button
  - Item Timing extra fields shown below row when metric = ItemTiming
- Display goals as `.goal-row` cards (same as dashboard style)
- Edit/Delete buttons inside each card (inline at right, replacing the separate actions)
- Keep "View Details" link as the card being clickable (navigate to `/goals/{id}`)
- Add contextual warning for goals where player average already exceeds target
- Add "Archive All" ghost button in section header

## Key Architecture Notes
- All backend `invoke()` calls are preserved — only visual layer changes
- Svelte 5 runes (`$state`, `$derived`, `$derived.by()`) used throughout
- Google Fonts loaded from app.html; fallback fonts in CSS
- Layout uses fixed-height (100vh) with scrollable content area
- The `.goal-row` / `.dot` shared classes in `app.css` are used by both Dashboard and Goals pages
