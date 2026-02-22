# Dota Keeper - GitHub Pages Site

This directory contains the GitHub Pages landing page for Dota Keeper.

## Structure

- `index.html` - Main landing page
- `style.css` - Styles matching the Dota 2 theme
- `script.js` - Dynamic release information fetching
- `screenshot.png` - App screenshot (add your own)
- `assets/` - Icons and other assets

## Deployment

This site is automatically deployed to GitHub Pages when changes are pushed to the main branch.

The site will be available at: `https://stringhandler.github.io/dota-keeper/`

## Local Testing

To test locally, simply open `index.html` in a web browser. For best results, use a local server:

```bash
# Using Python
python -m http.server 8000

# Using Node.js
npx serve

# Using PHP
php -S localhost:8000
```

Then navigate to `http://localhost:8000`

## Updating

- **Screenshot**: Replace `screenshot.png` with an actual app screenshot
- **Version**: The version is automatically fetched from the latest GitHub release
- **Download links**: Automatically updated from GitHub API

## Notes

- The page uses vanilla HTML/CSS/JS with no build step required
- Release information is fetched dynamically from the GitHub API
- Fallback URLs are hardcoded in the HTML if the API fetch fails
