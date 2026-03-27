# Test: Show App Version in UI

## Steps

1. Run the app
2. Look at the bottom of the sidebar (desktop/wide view)
3. Verify a version string like `v0.4.5` appears below the feedback button, in small muted text
4. Confirm it matches the version in `src-tauri/Cargo.toml` / `tauri.conf.json`
5. On mobile (narrow window), the sidebar is hidden — version is not shown there (that's expected)
