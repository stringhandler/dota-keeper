import { Given } from '@wdio/cucumber-framework'
import { browser } from '@wdio/globals'

const STEAM_ID = process.env.TEST_STEAM_ID || '76561198000000000'

Given('the app is running and I am logged in', async function () {
  // tauri-driver connects to WebView2 before the app content loads.
  // Wait for either the login screen or main app to appear using element queries,
  // which are more forgiving than browser.execute() in the early webview state.
  await browser.waitUntil(
    async () => {
      try {
        const loginBox = await $('.login-box')
        if (await loginBox.isDisplayed()) return true
      } catch { /* not ready */ }
      try {
        const appLayout = await $('.app-layout')
        if (await appLayout.isDisplayed()) return true
      } catch { /* not ready */ }
      return false
    },
    { timeout: 20000, timeoutMsg: 'App did not start within 20 seconds' }
  )

  // Now that the webview has content, force desktop layout
  await browser.setWindowSize(1280, 800)

  // If the login screen is shown, enter the Steam ID
  try {
    const loginBox = await $('.login-box')
    if (await loginBox.isDisplayed()) {
      await $('#steam-id').setValue(STEAM_ID)
      await $('button=Save & Continue').click()
      await loginBox.waitForDisplayed({ reverse: true, timeout: 10000 })
    }
  } catch { /* already logged in */ }

  // Confirm the main app layout is visible
  await $('.app-layout').waitForDisplayed({ timeout: 10000 })
})
