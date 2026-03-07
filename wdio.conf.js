/**
 * WebDriverIO + Cucumber configuration for Dota Keeper pre-release tests.
 *
 * Prerequisites:
 *   1. Build the app:   cargo tauri build --config src-tauri/tauri.test.conf.json
 *   2. Install driver:  cargo install tauri-driver
 *   3. Add to .env:     TEST_STEAM_ID=<your-steam-id>
 *   4. Run tests:       npm test
 *
 * The @manual tag is excluded by default. Scenario 5 (auto-updater) must be
 * tested manually against a previous release build.
 */

import { spawn } from 'node:child_process'
import { resolve } from 'node:path'
import { config as loadEnv } from 'dotenv'

loadEnv() // loads TEST_STEAM_ID (and others) from .env

const APP_PATH = resolve('./src-tauri/target/release/dota-keeper.exe')

let tauriDriver

export const config = {
  runner: 'local',
  hostname: '127.0.0.1',
  port: 4444,
  path: '/',

  specs: ['./tests/features/**/*.feature'],
  maxInstances: 1,

  capabilities: [
    {
      maxInstances: 1,
      browserName: 'wry',
      'tauri:options': {
        application: APP_PATH,
      },
    },
  ],

  logLevel: 'warn',
  bail: 0,
  waitforTimeout: 10000,
  connectionRetryTimeout: 120000,
  connectionRetryCount: 3,

  framework: 'cucumber',
  reporters: ['spec'],

  cucumberOpts: {
    require: [
      './tests/step-definitions/**/*.js',
      './tests/support/**/*.js',
    ],
    backtrace: false,
    dryRun: false,
    failFast: false,
    snippets: true,
    source: true,
    strict: false,
    // Skip @manual scenarios — they require manual verification
    tags: 'not @manual',
    timeout: 60000,
    ignoreUndefinedDefinitions: false,
  },

  /**
   * Start tauri-driver before the test suite.
   * Make sure `tauri-driver` is on your PATH (cargo install tauri-driver).
   */
  onPrepare() {
    // tauri-driver will auto-locate msedgedriver matching the WebView2 runtime.
    // If this still shows a blank window, check that msedgedriver version matches
    // the WebView2 runtime version (not just the Edge browser version):
    //   Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" | Select-Object pv
    tauriDriver = spawn('tauri-driver', [], {
      stdio: [null, process.stdout, process.stderr],
    })
  },

  /**
   * Kill tauri-driver once all tests are done.
   */
  onComplete() {
    if (tauriDriver) {
      tauriDriver.kill()
    }
  },
}
