# Guide Builder / Editor

## Summary
A UI for manually building and editing Dota 2 hero guides, complementing the auto-generation feature. Users can tweak auto-generated guides or build one from scratch, then export it to the local Dota 2 guides directory.

## Related
- `auto-create-local-guides.md` — auto-generation populates initial guide data; this editor allows manual refinement

## Requirements

### Guide Editor UI
- Hero selector to choose which hero the guide is for
- Item section builder:
  - Add/remove item sections (e.g. "Starting Items", "Early Game", "Core", "Situational")
  - Add items to each section via search/browse
  - Reorder sections and items via drag-and-drop
- Timing annotations: optionally attach a timing note to each item (e.g. "~18 min")
- Guide title and description/notes field (pre-populated with goal data if using auto-gen as a base)

### Import from Auto-Generated
- "Start from my data" button: pre-fills the editor with auto-generated timings and items from match history
- User can then adjust before saving

### Export
- Save guide file to Dota 2 local guides directory
- Toast confirmation with file path on success

### Guide Management
- List of saved guides (per hero)
- Edit or delete existing guides
- Optionally re-sync timings from latest match data

## Acceptance Criteria
- User can build a guide from scratch and export it
- User can load an auto-generated guide into the editor and modify it
- Guide sections and items can be reordered
- Exported file is valid Dota 2 guide JSON
