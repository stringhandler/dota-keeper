# Dota Keeper — Project Memory

Index of durable, non-obvious project facts. One line per memory; details live in the linked file.

## Build / Release
- [Android release minify JNI crash](android-release-minify-jni-crash.md) — release APK crashes on launch if R8 minify strips Tauri/wry JNI methods; keep `isMinifyEnabled = false` (or verify proguard keeps) on every Tauri upgrade.
