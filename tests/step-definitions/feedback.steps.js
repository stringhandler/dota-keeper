import { When, Then } from '@wdio/cucumber-framework'
import { browser, $ } from '@wdio/globals'

When('I click the {string} button', async function (buttonText) {
  const btn = await $(`button=${buttonText}`)
  await btn.waitForClickable({ timeout: 5000 })
  await btn.click()
})

When('I select the {string} category', async function (categoryLabel) {
  // Category buttons contain a span with the label text
  const btn = await $(`button*=${categoryLabel}`)
  await btn.waitForClickable({ timeout: 5000 })
  await btn.click()
})

When('I fill in the feedback text with {string}', async function (text) {
  const textarea = await $('#feedback-text')
  await textarea.waitForDisplayed({ timeout: 5000 })
  await textarea.setValue(text)
})

When('I select priority {string}', async function (priorityLabel) {
  // Priority radios: label contains the priority text
  const label = await $(`label*=${priorityLabel}`)
  await label.waitForDisplayed({ timeout: 5000 })
  const radio = await label.$('input[type="radio"]')
  await radio.click()
})

When('I click {string}', async function (buttonText) {
  const btn = await $(`button=${buttonText}`)
  await btn.waitForClickable({ timeout: 5000 })
  await btn.click()
})

Then('I should see a success toast {string}', async function (message) {
  const toast = await $('.toast-success .toast-msg')
  await toast.waitForDisplayed({ timeout: 10000 })
  const text = await toast.getText()
  expect(text).toBe(message)
})

Then('I should see a toast {string}', async function (message) {
  const toast = await $('.toast-msg')
  await toast.waitForDisplayed({ timeout: 10000 })
  const text = await toast.getText()
  expect(text).toContain(message)
})

Then('the feedback modal should be closed', async function () {
  const modal = await $('[role="dialog"][aria-label="Send feedback"]')
  await browser.waitUntil(
    async () => !(await modal.isDisplayed()),
    { timeout: 5000, timeoutMsg: 'Feedback modal did not close' }
  )
})
