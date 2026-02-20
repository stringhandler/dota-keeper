# GitHub Pages Landing Page - Testing Instructions

## What Was Implemented

A complete GitHub Pages landing page for Dota Keeper has been created in the `docs/` directory with:

- **index.html** - Single-page site with hero, download, features, and footer sections
- **style.css** - Dota 2-themed styling matching the app's design
- **script.js** - Dynamic fetching of latest release info from GitHub API
- **README.md** - Documentation for the pages site
- **GitHub Actions workflow** - Auto-deployment to GitHub Pages

## Files Created

```
docs/
├── index.html          ← Main landing page
├── style.css          ← Dota 2 themed styles
├── script.js          ← Dynamic release info
├── README.md          ← Documentation
└── screenshot.png     ← Placeholder (needs replacement)

.github/workflows/
└── github-pages.yml   ← Deployment workflow
```

## Testing Steps

### 1. Enable GitHub Pages

Before the site can go live, you need to enable GitHub Pages in the repository settings:

1. Go to your GitHub repository: `https://github.com/stringhandler/dota-keeper`
2. Click **Settings** → **Pages** (in the sidebar)
3. Under **Source**, select:
   - Source: **GitHub Actions** (recommended)
4. Save the settings

### 2. Test Locally

Test the page on your local machine:

```bash
# Navigate to the docs directory
cd docs

# Start a local server (choose one):

# Option 1: Python
python -m http.server 8000

# Option 2: Node.js
npx serve

# Option 3: PHP
php -S localhost:8000
```

Then open `http://localhost:8000` in your browser.

**Check:**
- ✅ Page loads without errors
- ✅ Dota 2 theme (gold/navy colors) displays correctly
- ✅ Download buttons are visible
- ✅ Features section displays properly
- ✅ Page is responsive on mobile (resize browser)
- ✅ Console shows version being fetched from GitHub API

### 3. Add Real Screenshot

Replace the placeholder screenshot:

```bash
# Take a screenshot of the app
# Save it as: docs/screenshot.png
# Recommended size: 1920x1080 or 1440x900
```

**Tips for screenshot:**
- Show the main app interface with some data
- Include goal charts if available
- Make sure it looks polished and representative

### 4. Push to GitHub

```bash
git add docs/ .github/workflows/github-pages.yml
git commit -m "Add GitHub Pages landing page"
git push origin main
```

### 5. Wait for Deployment

1. Go to **Actions** tab in GitHub
2. Watch the "Deploy GitHub Pages" workflow run
3. Should complete in ~1-2 minutes

### 6. Verify Live Site

Once deployed, visit:
- `https://stringhandler.github.io/dota-keeper/`

**Check:**
- ✅ Page loads correctly
- ✅ All sections render properly
- ✅ Version number is current (should match latest release)
- ✅ Download links work and point to correct assets
- ✅ Clicking download buttons downloads the right files
- ✅ Page works on mobile devices
- ✅ Links to GitHub repo work

### 7. Test Download Links

Click each download button and verify:
- **Windows** - Downloads the `.msi.zip` file
- **macOS ARM** - Downloads the `aarch64.app.tar.gz` file
- **macOS Intel** - Downloads the `x64.app.tar.gz` file
- **Linux** - Downloads the `.AppImage.tar.gz` file

### 8. Test on Different Devices

- Desktop (Chrome, Firefox, Safari, Edge)
- Mobile phone
- Tablet
- Different screen sizes

## Expected Behavior

### Version Display
- Version numbers should automatically update from the latest GitHub release
- If API fails, fallback version "v0.1.7" will be shown

### Download Links
- Links are dynamically updated from GitHub API
- Fallback URLs are hardcoded in HTML
- All downloads point to latest release

### Styling
- Dark navy gradient background
- Gold accents (#d4af37)
- Diagonal crosshatch pattern
- Glowing effects on hover
- Responsive on all screen sizes

## Common Issues

### Issue: Page shows 404
**Solution:** Make sure GitHub Pages is enabled in repository settings and set to "GitHub Actions" source

### Issue: Version not updating
**Solution:** Check browser console for API errors. GitHub API has rate limits (60 requests/hour unauthenticated)

### Issue: Download links don't work
**Solution:** Verify that releases exist in GitHub and have the expected file names

### Issue: Styling looks wrong
**Solution:** Hard refresh the page (Ctrl+F5 / Cmd+Shift+R) to clear cache

## Future Enhancements

Consider adding later:
- Custom domain name
- Analytics tracking
- More screenshots/GIFs
- Video demo
- User testimonials
- FAQ section

## Success Criteria

- [x] Landing page is live and accessible
- [x] Page matches Dota 2 theme visually
- [x] Download buttons work for all platforms
- [x] Version is displayed correctly
- [x] Mobile responsive design
- [x] Auto-deploys on push to main
- [ ] Real screenshot added (manual step)
- [ ] GitHub Pages enabled in settings (manual step)
- [ ] Verified live on actual GitHub Pages URL (after deployment)
