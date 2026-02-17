# GitHub Pages Download / Landing Page

## Problem Statement

There is currently no public-facing website for Dota Keeper. Users who want to download the app have to navigate GitHub's release UI, which is unfamiliar to non-technical users and doesn't reflect the app's identity or quality.

A dedicated landing page hosted on GitHub Pages will:
- Give the project a professional presence
- Make downloads obvious and easy to find
- Reinforce the app's Dota 2 theme

## Requirements

### Hosting

- Hosted on **GitHub Pages** from the `gh-pages` branch (or `/docs` folder on `main`)
- URL: `https://stringhandler.github.io/dota-keeper/` (or custom domain if available)
- Auto-deployed on release via a GitHub Actions workflow step

### Pages

Single-page site (no routing needed):

1. **Hero section** — App name, tagline, screenshot/preview
2. **Download section** — Buttons for Windows, macOS, Linux
3. **Features section** — Brief bullet points of what the app does
4. **Footer** — Version, GitHub link, license

### Visual Theme

Match the app's Dota 2-inspired design from [src/app.css](src/app.css):

| Property | Value |
|----------|-------|
| Background | Dark navy gradient: `#1a1a2e` → `#16213e` → `#0f3460` |
| Background texture | Repeating diagonal crosshatch pattern (same as app) |
| Headings | Gold `#d4af37`, uppercase, letter-spacing, gold text-shadow |
| Body text | Light grey `#e0e0e0` |
| Font | `'Trebuchet MS', Arial, sans-serif` |
| Accent/links | Gold `#d4af37`, glow on hover |
| Buttons | Gold border, transparent background; filled gold on hover |
| Cards/panels | `rgba(26, 26, 46, 0.8)` with subtle border |

### Download Section

Three platform buttons, dynamically linking to the latest GitHub release assets:

```
┌──────────────────────────────────────────────────────┐
│                    DOWNLOAD                          │
│                                                      │
│  ┌──────────────┐ ┌──────────────┐ ┌─────────────┐  │
│  │   Windows    │ │    macOS     │ │    Linux    │  │
│  │  ⊞ .msi     │ │   macOS ARM  │ │  .AppImage  │  │
│  │  Download   │ │  Download   │ │  Download  │  │
│  └──────────────┘ └──────────────┘ └─────────────┘  │
│                                                      │
│              v0.1.7  •  Free & Open Source           │
└──────────────────────────────────────────────────────┘
```

**macOS:** Show two download options:
- Apple Silicon (aarch64)
- Intel (x86_64)

**Download URL pattern** (GitHub releases):
```
https://github.com/stringhandler/dota-keeper/releases/latest/download/<filename>
```

Or link directly to the releases page as a fallback.

### Features Section

Brief highlights:
- Track match history automatically via OpenDota
- Set personal performance goals (CS, KDA, GPM, and more)
- Analyse goal progress with distribution charts
- Get weekly goal suggestions tailored to your playstyle
- Free, open source, runs locally — your data stays on your machine

### Screenshot / Preview

Include a screenshot or animated GIF of the app in action. Placeholder can be used initially.

## Technical Implementation

### Stack

Plain **HTML + CSS** (no framework needed for a static page). Optionally add minimal vanilla JS for dynamic version/download links fetched from the GitHub API.

### File Structure

```
docs/                        ← or gh-pages branch root
├── index.html
├── style.css
├── screenshot.png           ← App screenshot
└── assets/
    ├── icon.png             ← App icon
    ├── windows-icon.svg
    ├── macos-icon.svg
    └── linux-icon.svg
```

### Dynamic Version (Optional)

Fetch the latest release version from the GitHub API to always show the current version:

```javascript
async function loadLatestVersion() {
    const res = await fetch(
        'https://api.github.com/repos/stringhandler/dota-keeper/releases/latest'
    );
    const data = await res.json();
    document.querySelectorAll('.version').forEach(el => {
        el.textContent = data.tag_name;
    });

    // Update download links with actual asset URLs
    data.assets.forEach(asset => {
        if (asset.name.endsWith('.msi.zip')) {
            document.querySelector('#dl-windows').href = asset.browser_download_url;
        } else if (asset.name.endsWith('.AppImage.tar.gz')) {
            document.querySelector('#dl-linux').href = asset.browser_download_url;
        } else if (asset.name.includes('aarch64') && asset.name.endsWith('.tar.gz')) {
            document.querySelector('#dl-macos-arm').href = asset.browser_download_url;
        } else if (asset.name.includes('x64') && asset.name.endsWith('.tar.gz')) {
            document.querySelector('#dl-macos-x64').href = asset.browser_download_url;
        }
    });
}
```

### CSS Snippet (Theme)

```css
:root {
    font-family: 'Trebuchet MS', Arial, sans-serif;
    color: #e0e0e0;
}

body {
    margin: 0;
    background:
        linear-gradient(135deg, rgba(26,26,46,0.9) 0%, rgba(22,33,62,0.9) 50%, rgba(15,52,96,0.9) 100%),
        repeating-linear-gradient(45deg, transparent, transparent 2px, rgba(0,0,0,.1) 2px, rgba(0,0,0,.1) 4px),
        linear-gradient(180deg, #1a1a2e 0%, #16213e 50%, #0f3460 100%);
    background-attachment: fixed;
    min-height: 100vh;
}

h1, h2 {
    color: #d4af37;
    text-transform: uppercase;
    letter-spacing: 2px;
    text-shadow: 0 0 10px rgba(212,175,55,0.3), 2px 2px 4px rgba(0,0,0,0.8);
}

.btn-download {
    display: inline-block;
    padding: 0.75rem 1.5rem;
    border: 2px solid #d4af37;
    color: #d4af37;
    background: transparent;
    text-transform: uppercase;
    letter-spacing: 1px;
    cursor: pointer;
    transition: all 0.3s ease;
    text-decoration: none;
}

.btn-download:hover {
    background: #d4af37;
    color: #1a1a2e;
    box-shadow: 0 0 20px rgba(212,175,55,0.4);
}
```

### GitHub Actions: Auto-deploy on Release

Add a step to the release workflow to update the version displayed on the page (or rely on the JS API fetch to always show the latest).

Alternatively, deploy the `docs/` folder automatically:

```yaml
- name: Deploy GitHub Pages
  uses: peaceiris/actions-gh-pages@v3
  with:
    github_token: ${{ secrets.GITHUB_TOKEN }}
    publish_dir: ./docs
```

## Success Criteria

- [ ] Page is live at the GitHub Pages URL
- [ ] Page visually matches the app's Dota 2 theme
- [ ] Download buttons link to the correct release assets for Windows, macOS (ARM + Intel), and Linux
- [ ] Version number is correct (static or dynamically fetched)
- [ ] Page is mobile-responsive
- [ ] Page loads fast (no heavy frameworks)
- [ ] Auto-deploys or stays up to date when new releases are published

## Priority
**MEDIUM** - Improves discoverability and first impressions for new users

## Estimated Complexity
**Low** - Static HTML/CSS page with optional minimal JS
