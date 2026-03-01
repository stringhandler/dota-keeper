# Mobile Build — Test Instructions

## Prerequisites

### Android
1. Install [Android Studio](https://developer.android.com/studio) and the Android SDK.
2. Set the `ANDROID_HOME` environment variable to your SDK location.
3. Install NDK via Android Studio → SDK Manager → SDK Tools → NDK.
4. Have a connected Android device (USB debugging on) **or** an AVD emulator running.

### iOS (macOS only)
1. Install Xcode 14+ and `xcode-select --install`.
2. An Apple Developer account is required for device builds; the Simulator is free.

---

## Step 1 — Initialise mobile targets (one-time)

```bash
cd dota-keeper

# Android
yarn tauri android init

# iOS (macOS only)
yarn tauri ios init
```

These generate `src-tauri/gen/android/` and `src-tauri/gen/apple/` respectively.

---

## Step 2 — Run on Android emulator / device

```bash
yarn tauri android dev
```

Or build a release APK:

```bash
yarn tauri android build
```

---

## Step 3 — Run on iOS simulator (macOS only)

```bash
yarn tauri ios dev
```

---

## Things to verify

### Layout
- [ ] The sidebar is **hidden** on mobile; a bottom tab bar appears instead
- [ ] All 5 tabs work: Home, Matches, Goals, Challenges, Settings
- [ ] Active tab is highlighted in gold
- [ ] No custom title bar or resize handles appear
- [ ] Content area scrolls correctly within the screen (not the whole page)
- [ ] Bottom nav does not overlap content (padding-bottom applied)

### Safe area (iOS)
- [ ] Content does not sit under the notch/Dynamic Island (top padding)
- [ ] Bottom nav respects the home indicator area

### Database
- [ ] On first launch the app creates and writes to its database without errors
- [ ] Matches sync from OpenDota after entering a Steam ID
- [ ] Data persists across app restarts

### Network
- [ ] OpenDota API calls succeed (Goals and Matches load)

### Responsive desktop check (resize window < 640 px wide)
- [ ] Sidebar disappears and BottomNav appears when the desktop window is narrowed below 640 px
- [ ] Sidebar and normal title bar return when window is widened above 640 px

---

## Known limitations in this phase

- **Auto-update** is not available on mobile (the updater plugin is desktop-only). The update check fails silently — this is expected.
- **"Open database folder"** in Settings will fail on mobile — this is expected and can be hidden in a future task.
- iOS distribution requires a paid Apple Developer account ($99/yr). Simulator testing is free.
