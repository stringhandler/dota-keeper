import { writable } from 'svelte/store';

/** @type {import('svelte/store').Writable<Array<{id: number, message: string, type: 'success'|'error'|'info'}>>} */
export const toasts = writable([]);

let _nextId = 0;

/**
 * Show a toast notification.
 * @param {string} message
 * @param {'success'|'error'|'info'} type
 * @param {number} duration  milliseconds before auto-dismiss
 */
export function showToast(message, type = 'success', duration = 3000) {
  const id = ++_nextId;
  toasts.update(t => [...t, { id, message, type }]);
  setTimeout(() => {
    toasts.update(t => t.filter(toast => toast.id !== id));
  }, duration);
}
