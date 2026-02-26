# Test: Font Weight Fix

## What changed

- Added `wght@600;700` to the Barlow font Google Fonts URL (previously only 300/400/500 were loaded, causing browser-synthesized bold which looks heavy)
- Added explicit `font-weight: 400` to `body` in `app.css`

## Steps to test

1. Launch the app
2. Navigate through all pages (Dashboard, Matches, Analysis, Goals, Challenges, Settings)
3. Verify that body text reads cleanly and is not excessively bold or dark
4. Check the Analysis page specifically — it had the most complaints about heavy text
5. Nav items, filter chips, and section labels should look crisp but not overpowering

## Note

This fix requires an internet connection at app startup to download the new font weights from Google Fonts. On first load after the update, the fonts will be cached for subsequent offline use.
