/**
 * Anonymous analytics tracking helper
 *
 * Tracks usage events to help improve Dota Keeper.
 * Respects user consent via Settings (Accepted/Declined/NotYet).
 * No personal information is ever collected.
 */

import { invoke } from "@tauri-apps/api/core";

let analyticsConsent = "NotYet"; // "Accepted" | "Declined" | "NotYet"
let settingsLoaded = false;

// Load settings on module initialization
async function loadSettings() {
  if (settingsLoaded) return;
  try {
    const settings = await invoke("get_settings");
    analyticsConsent = settings.analytics_consent ?? "NotYet";
    settingsLoaded = true;
  } catch (e) {
    // Fail silently - analytics should never break the app
    console.error("Failed to load analytics settings:", e);
  }
}

/**
 * Get the current analytics consent status
 * @returns {Promise<string>} "Accepted" | "Declined" | "NotYet"
 */
export async function getAnalyticsConsent() {
  if (!settingsLoaded) {
    await loadSettings();
  }
  return analyticsConsent;
}

/**
 * Track an analytics event
 * @param {string} event - Event name (e.g. "page_view", "goal_created")
 * @param {object} properties - Optional event properties (no PII!)
 */
export async function trackEvent(event, properties = null) {
  // Lazy-load settings on first call
  if (!settingsLoaded) {
    await loadSettings();
  }

  // Only track if user has explicitly accepted
  if (analyticsConsent !== "Accepted") {
    return;
  }

  try {
    await invoke("track_event", { event, properties });
  } catch (e) {
    // Fail silently - analytics should never break the UI
    console.error("Analytics tracking failed:", e);
  }
}

/**
 * Track a page view
 * @param {string} pageName - Name of the page (e.g. "Dashboard", "Goals")
 */
export async function trackPageView(pageName) {
  await trackEvent("page_view", { page: pageName });
}

/**
 * Update the cached analytics consent state
 * Call this when the user changes their analytics preference
 * @param {string} consent - "Accepted" | "Declined" | "NotYet"
 */
export function updateAnalyticsConsent(consent) {
  analyticsConsent = consent;
  settingsLoaded = true;
}
