/**
 * Sentry error monitoring helper
 *
 * Captures unhandled errors and explicit exceptions.
 * No PII (Steam IDs, user details) is ever attached.
 * Only active when PUBLIC_SENTRY_DSN is set in the build env.
 */

import * as Sentry from '@sentry/browser';
import { PUBLIC_SENTRY_DSN } from '$env/static/public';

let initialised = false;
let sendingEnabled = false;

export function initSentry() {
  const dsn = PUBLIC_SENTRY_DSN;
  if (!dsn) {
    console.warn('[Sentry] No DSN configured — skipping init');
    return;
  }

  if (!initialised) {
    const env = import.meta.env.DEV ? 'development' : 'production';
    console.log(`[Sentry] Initialising (env=${env})`);
    Sentry.init({
      dsn,
      release: `dota-keeper@${__APP_VERSION__}`,
      environment: env,
      // Don't send errors in dev unless DSN is explicitly set
      enabled: !import.meta.env.DEV || !!PUBLIC_SENTRY_DSN,
      integrations: [
        Sentry.browserTracingIntegration(),
      ],
      // Capture 10% of sessions for performance monitoring
      tracesSampleRate: 0.1,
      // Don't attach PII
      sendDefaultPii: false,
      beforeSend(event) {
        // Respect user's analytics consent — drop event if opted out
        if (!sendingEnabled) {
          console.warn('[Sentry] beforeSend: dropping event (sendingEnabled=false)');
          return null;
        }
        console.log('[Sentry] beforeSend: sending event', event.event_id);
        // Strip any URL that looks like it contains a Steam ID
        if (event.request?.url) {
          event.request.url = event.request.url.replace(/\d{17}/g, '[steamid]');
        }
        return event;
      },
    });
    initialised = true;
  }

  sendingEnabled = true;
  console.log('[Sentry] Ready (initialised=true, sendingEnabled=true)');
}

/** Stop sending events to Sentry (user opted out of telemetry). */
export function disableSentry() {
  sendingEnabled = false;
}

/**
 * Capture an exception with optional context tags.
 * Safe to call even if Sentry is not initialised.
 * @param {unknown} error
 * @param {{ [key: string]: string }} [tags]
 */
export function captureException(error, tags) {
  if (!initialised) {
    console.warn('[Sentry] captureException called but Sentry is not initialised');
    return;
  }
  console.log('[Sentry] Capturing exception:', error?.message);
  Sentry.withScope((scope) => {
    if (tags) scope.setTags(tags);
    Sentry.captureException(error);
  });
}

/**
 * Capture a message (non-fatal event).
 * @param {string} message
 * @param {'fatal'|'error'|'warning'|'info'|'debug'} [level]
 */
export function captureMessage(message, level = 'info') {
  if (!initialised) return;
  Sentry.captureMessage(message, level);
}
