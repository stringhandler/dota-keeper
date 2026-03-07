import { Before, After, BeforeAll } from '@wdio/cucumber-framework'
import { browser } from '@wdio/globals'

/**
 * Give the WebView2 instance time to fully initialize before any scenario runs.
 * Without this, tauri-driver may connect before the renderer has loaded content.
 */
BeforeAll(async function () {
  await browser.pause(2000)
})

/**
 * Before each scenario: dismiss any open modals or toasts that might
 * have carried over from a previous scenario.
 */
Before(async function () {
  // Close any open dialog (e.g. analytics consent, onboarding)
  try {
    const consentBtn = await $('button=Accept')
    if (await consentBtn.isDisplayed()) {
      await consentBtn.click()
    }
  } catch (_) { /* ignore — modal may not be present */ }

  try {
    const skipBtn = await $('button*=Skip')
    if (await skipBtn.isDisplayed()) {
      await skipBtn.click()
    }
  } catch (_) { /* ignore */ }
})

/**
 * After each scenario: take a screenshot on failure for debugging.
 */
After(async function (scenario) {
  if (scenario.result?.status === 'FAILED') {
    const screenshot = await browser.takeScreenshot()
    await this.attach(screenshot, 'image/png')
  }
})
