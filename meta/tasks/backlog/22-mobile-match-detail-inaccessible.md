# Match Detail Page Inaccessible on Mobile

## Priority
**HIGH** — Feature exists but is hidden on mobile

## Problem

The match detail view link (🔍 icon) lives inside `.match-id-cell`, which is explicitly hidden on mobile:

```css
/* In src/routes/matches/+page.svelte */
@media (max-width: 640px) {
  .match-id-cell { display: none !important; }
}
```

This means mobile users can:
- Tap a match row → opens the Goals modal (useful)
- But cannot navigate to the full detail page at `/matches/[matchId]`

The full detail page likely shows per-minute CS charts and richer data.

## Solution

Make the match row tap on mobile navigate to the detail page (`/matches/[matchId]`) instead of opening the Goals modal. The goals modal is fine on desktop (where the detail link is visible), but on mobile the row tap should go straight to the detail page.

### Implementation

In [src/routes/matches/+page.svelte](src/routes/matches/+page.svelte):

1. Detect if mobile (`window.innerWidth < 640` or use a prop/store)
2. On mobile: `onclick` on match row → `goto('/matches/${match.match_id}')`
3. On desktop: keep existing behaviour (open goals modal)

Alternatively, add a "View Details" action to the goals modal footer on mobile, so users can navigate deeper after seeing the goal summary.

### Option B — Add "View Full Details" to the Goals Modal

In the modal footer on mobile, add:
```svelte
<a href="/matches/{selectedMatch.match_id}" class="btn btn-ghost">View Full Details</a>
```

This is less disruptive and works alongside the existing modal behaviour.

## Acceptance Criteria

- [ ] Mobile users can reach `/matches/[matchId]` for any match
- [ ] The transition feels natural (no jarring jump)
- [ ] Back navigation from the detail page returns to the matches list
