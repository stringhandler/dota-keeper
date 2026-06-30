---
name: android-release-minify-jni-crash
description: Android release builds crash on launch if R8 minify strips Tauri/wry JNI methods (e.g. WryActivity.getId())
metadata:
  type: project
---

Android **release** builds crash immediately on launch (app "starts and stops") when R8 minification strips/renames classes that Tauri (tao/wry) and the Tauri plugin framework call **reflectively by name over JNI**. The original failure: `tao-0.35.0` `ndk_glue.rs:onActivityCreate` calls `activity.getId()` (the Kotlin getter generated from `var id: Int` in the auto-generated `WryActivity.kt`); R8 can't see the JNI call site, removes/renames the method, so the lookup throws `NoSuchMethodError` → surfaces as `JavaException` → Rust panics in a non-unwind FFI context → `SIGABRT`.

Tell-tale logcat: `RustStdoutStderr: panicked at tao-.../android/ndk_glue.rs:393:6: called Result::unwrap() on an Err value: JavaException`, then `panic in a function that cannot unwind` and a native `SIGABRT`. A framework method (`getIntent()`) succeeds right before the app method (`getId()`) fails — that asymmetry is the fingerprint of minification stripping the app's own method.

Surfaced after the Tauri 2.11 upgrade (tao 0.35.0 / wry 0.55.0) because the newer tao started calling `getId()` over JNI; minify had been silently on the whole time.

**Why:** Debug builds (`isMinifyEnabled = false`) never hit it, so it passes local testing and only breaks the signed release APK on-device.

**How to apply:** In [src-tauri/gen/android/app/build.gradle.kts](../../src-tauri/gen/android/app/build.gradle.kts) the release buildType is set to `isMinifyEnabled = false`. If re-enabling minify, first verify the keep rules in [src-tauri/gen/android/app/proguard-rules.pro](../../src-tauri/gen/android/app/proguard-rules.pro) (`-keep class com.volthawk.dota_keeper.** { *; }`, `-keep class app.tauri.** { *; }`) still cover everything the current tao/wry version reaches via JNI — re-check this on every Tauri/tao/wry version bump. Validate by sideloading the release (not debug) APK on a real device, not just by building. These `gen/android` files are committed and used as-is by the release-branch CI (it does not re-run `tauri android init`); the beta workflow DOES re-init, which would regenerate them.
