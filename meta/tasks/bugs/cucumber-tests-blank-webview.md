# Bug: Cucumber E2E Tests — Blank WebView2 Window

## Branch
`feat/cucumber-tests`

## Status
Broken — tests run but the app window is blank white under WebDriver control.

## What Works
- `tauri-driver` starts successfully
- WebDriverIO connects and a session is created
- The Tauri app launches (window appears)
- DevTools is reachable: `DevTools listening on ws://127.0.0.1:{port}/devtools/browser/...`
- The release build runs fine when launched directly (not via WebDriver)
- `npm test` wiring, feature file, step definitions, and hooks are all correct

## Root Cause (Suspected)
`tauri-driver` launches the app but WebView2 does not load the app content when under
WebDriver control. All WebDriverIO commands (`browser.execute`, element queries) fail
because the webview has no DOM.

Two leading candidates:

### 1. msedgedriver version mismatch
- WebView2 runtime: `145.0.3800.82`
- Installed msedgedriver: `145.0.3800.97`
- The fourth version segment differs. Download the exact `145.0.3800.82` build from
  the Edge WebDriver archive and replace `msedgedriver.exe` in PATH.

### 2. tauri-driver version incompatible with Tauri v2
- If `tauri-driver --version` shows `0.x` or `1.x`, it will not work with Tauri v2.
- Fix: `cargo install tauri-driver --version "^2" --force`

## Steps to Fix
1. Check tauri-driver version: `tauri-driver --version` — must be 2.x
2. Check msedgedriver version: `msedgedriver --version` — must match WebView2 runtime
3. Check WebView2 version (PowerShell):
   ```
   Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" | Select-Object pv
   ```
4. If msedgedriver doesn't match, download the correct version from:
   https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/
5. Run `npm test` and confirm the app renders inside the WebDriver session

## Files Changed on This Branch
- `tests/features/pre-release.feature` — Gherkin scenarios for the pre-release checklist
- `tests/step-definitions/common.steps.js` — app launch + login
- `tests/step-definitions/feedback.steps.js` — feedback modal
- `tests/step-definitions/goals.steps.js` — goals CRUD
- `tests/step-definitions/matches.steps.js` — match refresh
- `tests/support/hooks.js` — BeforeAll pause, screenshot on failure
- `wdio.conf.js` — WebDriverIO + Cucumber + tauri-driver config
- `src-tauri/tauri.test.conf.json` — build config without signing (for local test builds)
- `package.json` — test scripts + `@wdio/*` devDependencies
- `src-tauri/build.rs` — forward POSTHOG_API_KEY via cargo:rustc-env

## Notes
- The `--native-driver msedgedriver` flag was tested but reverted; auto-detection is
  currently used in `wdio.conf.js`. May need to be re-added once versions are aligned.
- `decorations: false` in tauri.conf.json is fine — not related to this issue.
- All WebView2 log noise (`dpi awareness`, `Edge LLM`, `fallback task provider`) is
  harmless and can be ignored.
