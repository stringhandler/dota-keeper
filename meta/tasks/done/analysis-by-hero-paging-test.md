# Test Guide: Analysis By Hero Pagination

## Steps

1. Open the app and navigate to the **Analysis** tab.
2. If you have played many heroes, the "By Hero" card should now show **at most 10 rows**.
3. If there are more than 10 heroes with data, you should see **‹ 1 / N ›** pagination controls at the bottom of the card.
4. Click **›** to advance to the next page — the hero list should update to show the next batch of heroes.
5. Click **‹** to go back — the previous page should be restored.
6. The **‹** button should be disabled on page 1; **›** should be disabled on the last page.
7. Change a filter (e.g. Time Marker or Sample Size) — pagination should **reset to page 1**.
8. Toggle a favourite star — pagination should **reset to page 1**.
9. If you have 10 or fewer heroes with data, **no pagination controls should appear** at all.
10. Confirm the other cards (Performance Range, Top Hero Trend) are still fully visible without excessive scrolling.
