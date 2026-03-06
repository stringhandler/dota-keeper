# Show App Version in UI

## Problem Statement

Users have no way to see what version of Dota Keeper they are running. This makes it difficult to:
- Report bugs accurately (can't reference version number)
- Verify an update was applied successfully
- Match their version to release notes in the changelog

## Requirements

### Where to Display

**Primary Location: Settings or About section**
- Bottom of a sidebar/nav menu, or
- Settings page footer, or
- Dedicated "About" section

**Secondary Location (Optional):**
- App footer/status bar (very subtle, small text)

### Format

```
Dota Keeper v0.1.7
```

Or more compact for footers:
```
v0.1.7
```

### Behavior

- Version is read from Tauri at runtime (not hardcoded in frontend)
- Use Tauri's `app.getVersion()` API so it always matches the actual build
- Display should be static (no interaction needed), but optionally:
  - **Click to copy** version string to clipboard
  - **Click to open** releases page / changelog

## Technical Implementation

### Tauri API

Use the `@tauri-apps/api/app` package to read the version at runtime:

```typescript
import { getVersion } from '@tauri-apps/api/app';

let appVersion = '';

onMount(async () => {
    appVersion = await getVersion();
});
```

### Display Options

#### Option A: Nav/Sidebar Footer (Recommended)
If there is a sidebar or nav menu, add version at the bottom:

```svelte
<nav>
    <!-- nav items -->
    <div class="nav-footer">
        <span class="version">v{appVersion}</span>
    </div>
</nav>
```

#### Option B: Settings Page
Add an "About" section at the bottom of the settings/config page:

```svelte
<section class="about">
    <h3>About</h3>
    <p>Dota Keeper <span class="version">v{appVersion}</span></p>
</section>
```

#### Option C: Page Footer
Small, subtle footer at the bottom of all pages:

```svelte
<footer>
    <span>Dota Keeper v{appVersion}</span>
</footer>
```

## Styling

```css
.version {
    font-size: 0.75rem;
    color: var(--text-muted);
    opacity: 0.6;
}

/* Optional: subtle hover effect if clickable */
.version:hover {
    opacity: 1;
    cursor: pointer;
}
```

## Success Criteria

- [ ] Version number is visible somewhere in the app
- [ ] Version is read from Tauri runtime (not hardcoded)
- [ ] Version matches what is in `package.json` / `tauri.conf.json`
- [ ] Display is subtle and doesn't distract from main content

## Priority
**LOW** - Small quality-of-life improvement

## Estimated Complexity
**Very Low** - Single component change, one Tauri API call
