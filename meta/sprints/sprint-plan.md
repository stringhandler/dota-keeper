# Sprint Plan
_Generated: 2026-04-20_
_Focus: Android + iOS App Store launch_

Sizing: S = ~hours (1 pt), M = 1–2 days (2 pts), L = 3–5 days (3 pts), XL = 5+ days (5 pts)
Target: 8–10 points per sprint for a solo developer with a 10–20% buffer.

---

## Sprint 1 — "Mobile-Ready"
**Theme:** Bug fixes + mobile UX blockers
**Estimated scope:** ~9 days / 9 points

| Task | File | Size | Notes |
|------|------|------|-------|
| Bug: No matches on first refresh | `bugs/no-matches-refresh-on-first.md` | S | Embarrassing first-run bug — fix before any store user sees it |
| Bug: Stratz last-hit minute offset | `bugs/stratz-lasthit-minute-offset.md` | S | Clear fix (shift array index); root cause is documented |
| Match detail inaccessible on mobile | `backlog/22-mobile-match-detail-inaccessible.md` | S | Marked HIGH priority — the detail page is literally hidden on mobile right now |
| Simplify mobile topbar | `backlog/25-mobile-topbar-simplify.md` | S | Cramped topbar is immediately visible to reviewers; Option A is a CSS one-liner |
| Match filter overflow — scroll indicator | `backlog/23-mobile-match-filter-overflow.md` | S | Short-term fix only (fade gradient); save the full filter sheet for later |
| Pull-to-refresh matches | `backlog/32-pull-to-refresh-matches.md` | M | Core mobile feel — store reviewers expect this gesture |
| Skeleton loaders | `backlog/24-mobile-skeleton-loaders.md` | M | Removes jarring "Loading..." text; much better first impression for store users |

**Why this sprint:** Clear every known bug and every mobile issue that would immediately signal "not a real mobile app" to a store reviewer. These are all self-contained and unambiguous — no design decisions needed.

**Deferred from this sprint:**
- `bugs/cucumber-tests-blank-webview.md` — dev infrastructure, not user-facing; won't affect store submission

---

## Sprint 2 — "Store Ready"
**Theme:** App Store compliance + onboarding polish
**Estimated scope:** ~8 days / 8 points

| Task | File | Size | Notes |
|------|------|------|-------|
| Guest browse mode | `backlog/13-guest-browse-mode.md` | L | **Required for Apple App Store** — Guideline 5.1.1 explicitly prohibits mandatory third-party login. This is the most important sprint 2 item. |
| Hero select bottom sheet on mobile | `backlog/21-mobile-hero-select-bottom-sheet.md` | M | Last remaining mobile UX fix; makes goal creation feel native |
| Onboarding: role selection | `backlog/28-onboarding-role-selection.md` | M | Great first-impression feature for new users arriving from stores |

**Why this sprint:** Guest mode is a hard requirement for Apple's review process — without it, the iOS app will be rejected. Role selection onboarding makes a strong first impression for organic store traffic and sets up goal suggestion quality from day one.

**Risk:** Guest mode (L) is the largest single task in the plan. If it runs long, drop role selection to Sprint 3 — it's a nice-to-have, not a blocker.

---

## Sprint 3 — "Ship It"
**Theme:** Submission + first post-launch feature
**Estimated scope:** ~6–7 days / 6–7 points

| Task | File | Size | Notes |
|------|------|------|-------|
| Submit to Google Play Store | _(coordination)_ | S | Prepare release build, store listing, screenshots, privacy policy link |
| Submit to Apple App Store | _(coordination)_ | S | Requires signing, App Store Connect setup, review wait time (allow 1–3 days) |
| Goal history & trend tracking | `upnext/goal-history-trend.md` | L | Most self-contained upnext feature; directly supports the core value prop of the app; gives users a reason to keep coming back post-launch |

**Why this sprint:** Submission is mostly coordination and waiting — there's capacity to build a high-value feature in parallel. Goal History is the best candidate: it uses existing data (no new API calls), reinforces the 75% achievement rate design principle, and is something you can demo in store screenshots.

---

## Remaining Upnext Queue (post-launch)
These are ready to build but not on the store launch critical path:

| Task | File | Size | Notes |
|------|------|------|-------|
| Hero benchmark ranking | `upnext/hero-benchmark-ranking.md` | L | Needs the benchmark CSV to be generated first (external dependency) |
| Ideal hero match data | `upnext/ideal-hero-match-data.md` | L | Depends on hero benchmark CSV; these two go together |
| Performance journal | `upnext/performance-journal.md` | L | High-value retention feature; good for Sprint 4 |
| Local match history from Dota install | `upnext/local-match-history-from-dota-install.md` | XL | Has open questions about which local files Dota 2 actually exposes — de-risk with a spike first |

---

## Backlog (not yet scheduled)
Lower priority or thematically distant from store launch:

| Task | File | Reason not scheduled |
|------|------|---------------------|
| Match filter full bottom sheet | `backlog/23-mobile-match-filter-overflow.md` | Sprint 1 covers the minimum fix; full sheet is a nice-to-have |
| Settings: OpenDota API key | `backlog/settings-opendota-api-key.md` | Useful but not store-blocking; pull into any sprint with spare capacity |
| Load more matches | `backlog/load-more-matches.md` | Polish; not launch-critical |
| BSJ goals (3 tasks) | `backlog/06–08-bsj-*.md` | Interesting but low priority vs. launch goals |
| Game State Integration | `backlog/11-game-state-integration.md` | Large scope; overlaps with local match history |
| Secret achievements, challenges, public page | various | Post-launch growth features |

---

## Needs Refining (not schedulable yet)
| Task | File | What's missing |
|------|------|----------------|
| Guide builder / editor | `needs_refining/14-guide-builder-editor.md` | Scope unclear; no acceptance criteria |
| Report smurf / cheater | `needs_refining/33-report-smurf-cheater.md` | External dependency (cheater DB API); needs API research first |
| Analysis graph Y-axis 110 | `needs_refining/analysis-graph-y-axis-110.md` | Too vague to estimate; needs a clearer problem statement |
