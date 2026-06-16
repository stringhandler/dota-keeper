# Pull-to-Refresh Matches (Mobile)

## Summary

On mobile, add pull-down-to-refresh gesture to the matches list to trigger a match refresh without needing the "Refresh Matches" button.

## Behaviour

- User pulls down from the top of the matches list
- A loading indicator appears (e.g. spinner or "Refreshing..." text)
- Triggers the same `refresh_matches` invoke as the existing refresh button
- Releases and shows updated matches (or error banner on failure)

## Implementation Notes

- The content area uses `-webkit-overflow-scrolling: touch` — detect scroll position reaching top + touchstart/touchmove/touchend to calculate pull distance
- Show a pull indicator above the list (translate the list down while pulling)
- Only active on mobile (`isMobile` flag already exists in layout, or use `window.innerWidth < 640`)
- Can be implemented as a reusable Svelte component `PullToRefresh.svelte` wrapping the matches content

## Acceptance Criteria

- [ ] Pulling down ≥60px from the top triggers a refresh
- [ ] Visual indicator shown during pull and while loading
- [ ] Works alongside the existing scroll behaviour (doesn't interfere with normal scrolling)
- [ ] Error state handled same as the button refresh
