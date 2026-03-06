# Better App Icon

## Overview

Replace the default Tauri placeholder icon with a proper Dota Keeper icon that reflects the app's identity.

## Design Direction

- **Theme**: Dota 2 / fantasy RPG aesthetic — dark background, gold accents
- **Concept ideas**: a shield or tome with a sword, a stylised "DK" monogram in gold, or a minimap-style compass rose
- **Style**: Should read clearly at small sizes (16x16 taskbar) and look good at large sizes (512x512 store/installer)

## Required Sizes (Tauri)

Tauri needs the following files in `src-tauri/icons/`:
- `32x32.png`
- `128x128.png`
- `128x128@2x.png` (256x256)
- `icon.icns` (macOS)
- `icon.ico` (Windows — multi-size)
- `icon.png` (512x512 source)

Use `cargo tauri icon <source.png>` to auto-generate all sizes from a single 1024x1024 source PNG.

## Acceptance Criteria
- [ ] Custom icon replaces the Tauri default across the app
- [ ] Looks sharp in the Windows taskbar and start menu
- [ ] `.ico` and `.icns` formats generated correctly
- [ ] Installer uses the new icon
