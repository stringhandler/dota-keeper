/**
 * Sentry error monitoring helper
 *
 * Captures unhandled errors and explicit exceptions.
 * No PII (Steam IDs, user details) is ever attached.
 * Only active when PUBLIC_SENTRY_DSN is set in the build env.
 */

import * as Sentry from '@sentry/browser';

let initialised = false;
let sendingEnabled = false;

export function initSentry() {
  const dsn = import.meta.env.PUBLIC_SENTRY_DSN;
  if (!dsn) return;

  if (!initialised) {
    Sentry.init({
      dsn,
      release: `dota-keeper@${__APP_VERSION__}`,
      environment: import.meta.env.DEV ? 'development' : 'production',
      // Don't send errors in dev unless DSN is explicitly set
      enabled: !import.meta.env.DEV || !!dsn,
      integrations: [
        Sentry.browserTracingIntegration(),
      ],
      // Capture 10% of sessions for performance monitoring
      tracesSampleRate: 0.1,
      // Don't attach PII
      sendDefaultPii: false,
      beforeSend(event) {
        // Respect user's analytics consent — drop event if opted out
        if (!sendingEnabled) return null;
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
  if (!initialised) return;
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
