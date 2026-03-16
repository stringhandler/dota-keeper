import { writable } from 'svelte/store';

// Set by the matches page after a refresh that finds new matches.
// Read by the dashboard to show the post-game mood check-in modal.
export const pendingCheckinStore = writable(/** @type {any} */ (null));
