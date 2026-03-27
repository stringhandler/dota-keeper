# Test: What's New Dialog

## Steps

1. Open DevTools → Application → Local Storage and delete the `last_seen_version` key
2. Run the app — dialog should **not** appear (first install, no previous version)
3. Set `last_seen_version` to an older version e.g. `0.4.3` in Local Storage
4. Reload the app — the "What's New in v0.4.5" dialog should appear with the 0.4.5 release notes
5. Click "Got it" — dialog dismisses
6. Reload again — dialog should **not** appear again (version already seen)
7. Verify `last_seen_version` in Local Storage is now `0.4.5`
