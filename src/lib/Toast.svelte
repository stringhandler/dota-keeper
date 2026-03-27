<script>
  import { toasts } from './toast.js';
  import { fly, fade } from 'svelte/transition';
  import { flip } from 'svelte/animate';
</script>

<div class="toast-container" aria-live="polite" aria-atomic="false">
  {#each $toasts as toast (toast.id)}
    <div
      class="toast toast-{toast.type}"
      animate:flip={{ duration: 150 }}
      in:fly={{ y: 16, duration: 200 }}
      out:fade={{ duration: 150 }}
    >
      <span class="toast-icon" aria-hidden="true">
        {#if toast.type === 'success'}✓{:else if toast.type === 'error'}✕{:else}ℹ{/if}
      </span>
      <span class="toast-msg">{toast.message}</span>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: calc(72px + env(safe-area-inset-bottom, 0px) + 8px);
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    gap: 6px;
    z-index: 9999;
    pointer-events: none;
    min-width: 220px;
    max-width: min(400px, calc(100vw - 32px));
  }

  .toast {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 16px;
    border-radius: 6px;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.5px;
    border-left: 3px solid transparent;
    background: var(--bg-elevated);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);
    color: var(--text-primary);
    pointer-events: auto;
  }

  .toast-success {
    border-left-color: var(--green);
    background: rgba(24, 35, 46, 0.97);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5), inset 0 0 0 1px rgba(74, 222, 128, 0.12);
  }

  .toast-error {
    border-left-color: var(--red);
    background: rgba(24, 35, 46, 0.97);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5), inset 0 0 0 1px rgba(248, 113, 113, 0.12);
  }

  .toast-info {
    border-left-color: var(--gold);
    background: rgba(24, 35, 46, 0.97);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5), inset 0 0 0 1px rgba(240, 180, 41, 0.12);
  }

  .toast-icon {
    font-size: 14px;
    flex-shrink: 0;
  }

  .toast-success .toast-icon { color: var(--green); }
  .toast-error   .toast-icon { color: var(--red); }
  .toast-info    .toast-icon { color: var(--gold); }

  .toast-msg {
    flex: 1;
  }

  /* On desktop (sidebar visible), shift right so toast doesn't sit over sidebar */
  @media (min-width: 641px) {
    .toast-container {
      bottom: 24px;
      left: calc(var(--sidebar-w, 200px) + 50%);
      transform: translateX(-50%);
    }
  }
</style>
