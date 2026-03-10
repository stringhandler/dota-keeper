/**
 * Module-level parse queue — persists across page navigation.
 * All state is shared; only one parse runs at a time.
 *
 * Uses a class with $state fields because Svelte 5 does not allow exporting
 * reassigned $state variables from a module — only mutating class properties.
 */
import { invoke } from "@tauri-apps/api/core";

class ParseQueueStore {
  queue      = $state([]);
  active     = $state(new Set());
  errors     = $state(new Map());
  countdowns = $state(new Map());
}

export const pqs = new ParseQueueStore();

let _isDraining   = false;
let _steamId      = "";
let _onComplete   = null;
let _timerStarted = false;

// ── Public API ────────────────────────────────────────────────────────────────

/**
 * Call on component mount. Supplies the steam ID and a callback to reload
 * matches after each parse. Resumes any drain that was in progress before
 * the user navigated away.
 */
export function initQueue(steamId, onComplete) {
  _steamId    = steamId;
  _onComplete = onComplete;
  _startCountdownTimer();
  if (pqs.queue.length > 0 && !_isDraining) _drain();
}

/** Add a match to the queue. No-op if already queued or active. */
export function enqueueParse(matchId) {
  if (pqs.active.has(matchId) || pqs.queue.includes(matchId)) return;
  pqs.queue = [...pqs.queue, matchId];
  if (!_isDraining && _steamId) _drain();
}

export function isPendingError(msg) {
  return (
    msg.includes('not yet')   ||
    msg.includes('next sync') ||
    msg.includes('will be')   ||
    msg.includes('per-minute')
  );
}

// ── Internal ──────────────────────────────────────────────────────────────────

async function _drain() {
  if (_isDraining) return;
  _isDraining = true;
  try {
    while (pqs.queue.length > 0) {
      const [matchId, ...rest] = pqs.queue;
      pqs.queue = rest;
      await _parseMatch(matchId);
    }
  } finally {
    _isDraining = false;
  }
}

async function _parseMatch(matchId) {
  pqs.active = new Set([...pqs.active, matchId]);

  if (pqs.errors.has(matchId))    { const m = new Map(pqs.errors);     m.delete(matchId); pqs.errors     = m; }
  if (pqs.countdowns.has(matchId)){ const m = new Map(pqs.countdowns); m.delete(matchId); pqs.countdowns = m; }

  try {
    await invoke("parse_match", { matchId, steamId: _steamId });
    if (_onComplete) await _onComplete();
  } catch (e) {
    const errMsg = String(e);
    console.error(`[parse-queue] parse_match failed for ${matchId}:`, errMsg);
    const m = new Map(pqs.errors);
    m.set(matchId, errMsg);
    pqs.errors = m;
    if (isPendingError(errMsg)) {
      const rc = new Map(pqs.countdowns);
      rc.set(matchId, 60);
      pqs.countdowns = rc;
    }
  } finally {
    const s = new Set(pqs.active);
    s.delete(matchId);
    pqs.active = s;
  }
}

function _startCountdownTimer() {
  if (_timerStarted) return;
  _timerStarted = true;
  setInterval(() => {
    if (pqs.countdowns.size === 0) return;
    const next    = new Map();
    const toRetry = [];
    for (const [matchId, secs] of pqs.countdowns) {
      if (secs <= 1) toRetry.push(matchId);
      else next.set(matchId, secs - 1);
    }
    pqs.countdowns = next;
    for (const matchId of toRetry) enqueueParse(matchId);
  }, 1000);
}
