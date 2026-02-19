# Increase Default Window Size

## Overview

The starting window size feels too small. Increase the default width and height so the app has more breathing room on launch.

## Implementation

Update the window dimensions in `src-tauri/tauri.conf.json` under `app.windows[0]`:

```json
"width": 1280,
"height": 800
```

Adjust values based on what looks good — aim for something comfortable on a 1080p display without being full-screen.

## Acceptance Criteria
- [ ] App launches at a larger default size
- [ ] Content is not clipped or awkwardly laid out at the new size
