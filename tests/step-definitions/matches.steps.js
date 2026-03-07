import { When, Then } from '@wdio/cucumber-framework'
import { browser, $, $$ } from '@wdio/globals'

When('I click {string}', async function (buttonText) {
  const btn = await $(`button*=${buttonText}`)
  await btn.waitForClickable({ timeout: 5000 })
  await btn.click()
})

Then('at least one match row should be displayed', async function () {
  // Wait for the refreshing state to end (button text returns to normal)
  await browser.waitUntil(
    async () => {
      const btn = await $('button*=Refresh Matches')
      return btn.isDisplayed()
    },
    { timeout: 30000, timeoutMsg: 'Match refresh did not complete within 30s' }
  )

  // Assert at least one match row is present
  await browser.waitUntil(
    async () => {
      const rows = await $$('.match-row')
      return rows.length > 0
    },
    { timeout: 5000, timeoutMsg: 'No match rows appeared after refresh' }
  )

  const rows = await $$('.match-row')
  expect(rows.length).toBeGreaterThan(0)
})
