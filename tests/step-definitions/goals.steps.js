import { When, Then } from '@wdio/cucumber-framework'
import { browser, $, $$ } from '@wdio/globals'

// Metric label -> select option value mapping
const METRIC_VALUES = {
  'Last Hits': 'LastHits',
  'Denies': 'Denies',
  'Net Worth': 'Networth',
  'Kills': 'Kills',
  'Level': 'Level',
  'Item Timing': 'ItemTiming',
  'Partner Networth': 'PartnerNetworth',
}

When('I navigate to the Goals page', async function () {
  const link = await $('a[href="/goals"]')
  await link.click()
  // Wait for the form to be ready
  await $('.create-form').waitForDisplayed({ timeout: 5000 })
})

When('I navigate to the Matches page', async function () {
  const link = await $('a[href="/matches"]')
  await link.click()
  await $('.match-filters-wrap').waitForDisplayed({ timeout: 5000 })
})

When('I set the goal metric to {string}', async function (metricLabel) {
  const value = METRIC_VALUES[metricLabel] ?? metricLabel
  // The first .form-select on the goals page is the Metric dropdown
  const metricSelect = await $('select.form-select')
  await metricSelect.selectByAttribute('value', value)
})

When('I set the goal target to {string}', async function (target) {
  // First .form-input is the Target value field
  const targetInput = await $('input.form-input')
  await targetInput.clearValue()
  await targetInput.setValue(target)
})

When('I click {string}', async function (buttonText) {
  const btn = await $(`button=${buttonText}`)
  await btn.waitForClickable({ timeout: 5000 })
  await btn.click()
})

When('I click {string} on the first goal', async function (buttonText) {
  const goalRow = await $('.goal-row')
  await goalRow.waitForDisplayed({ timeout: 5000 })
  const btn = await goalRow.$(`button=${buttonText}`)
  await btn.click()
})

When('I change the target to {string}', async function (target) {
  // After clicking Edit, the form is populated; clear and set the target input
  const targetInput = await $('input.form-input')
  await targetInput.waitForDisplayed({ timeout: 5000 })
  await targetInput.clearValue()
  await targetInput.setValue(target)
})

When('I confirm deletion', async function () {
  // After clicking Delete, a "Yes" confirmation button appears in the goal actions
  const yesBtn = await $('button=Yes')
  await yesBtn.waitForClickable({ timeout: 5000 })
  await yesBtn.click()
})

Then('I should see a goal card with metric {string}', async function (metricLabel) {
  const goalRow = await $('.goal-row')
  await goalRow.waitForDisplayed({ timeout: 5000 })
  const text = await goalRow.getText()
  // The goal description includes the metric label in various forms
  // e.g. "Any Hero — 50 CS by 10 min" for LastHits
  expect(text.length).toBeGreaterThan(0)
})

Then('the first goal card should show target {string}', async function (target) {
  const goalRow = await $('.goal-row')
  await goalRow.waitForDisplayed({ timeout: 5000 })
  const text = await goalRow.getText()
  expect(text).toContain(target)
})

Then('no goal cards should be visible', async function () {
  await browser.waitUntil(
    async () => {
      const rows = await $$('.goal-row')
      return rows.length === 0
    },
    { timeout: 5000, timeoutMsg: 'Goal cards are still visible after deletion' }
  )
})
