# Custom Title Bar

## Overview

Replace the native Windows title bar with a custom in-app title bar so we can control its appearance (colour, height, styling).

## Implementation

### 1. Disable native decorations

In `src-tauri/tauri.conf.json`, add `"decorations": false` to the window config:

```json
"windows": [
  {
    "title": "Dota Keeper",
    "width": 1280,
    "height": 800,
    "decorations": false
  }
]
```

### 2. Custom title bar component

Create a `TitleBar` component that:
- Uses `data-tauri-drag-region` on the drag area so the window is still draggable
- Has Close, Minimise, and Maximise buttons wired to Tauri's window API
- Sits fixed at the top of the app layout

```svelte
<!-- src/lib/components/TitleBar.svelte -->
<script>
  import { getCurrentWindow } from '@tauri-apps/api/window';
  const appWindow = getCurrentWindow();
</script>

<div class="titlebar" data-tauri-drag-region>
  <span class="titlebar-title" data-tauri-drag-region>Dota Keeper</span>
  <div class="titlebar-buttons">
    <button on:click={() => appWindow.minimize()}>–</button>
    <button on:click={() => appWindow.toggleMaximize()}>□</button>
    <button class="close" on:click={() => appWindow.close()}>✕</button>
  </div>
</div>
```

### 3. Insert into root layout

Add `<TitleBar />` at the very top of the root layout component, and add `padding-top` equal to the title bar height so content doesn't sit underneath it.

### 4. Styling

- Height: ~32px (match native Windows feel) or adjust to taste
- Background: match the app's sidebar/nav colour
- Close button hover: red (`#c42b1c`)
- Min/Max hover: subtle highlight

## Acceptance Criteria

- [ ] Native title bar is hidden
- [ ] Window is draggable by clicking and dragging the title bar area
- [ ] Minimise, Maximise/Restore, and Close buttons work correctly
- [ ] Double-clicking the title bar toggles maximise
- [ ] Title bar colour matches the app theme
- [ ] App content is not obscured by the title bar
