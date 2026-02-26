<script>
  import { getCurrentWindow } from '@tauri-apps/api/window';

  const appWindow = getCurrentWindow();

  async function minimize() {
    await appWindow.minimize();
  }

  async function toggleMaximize() {
    await appWindow.toggleMaximize();
  }

  async function close() {
    await appWindow.close();
  }
</script>

<div class="titlebar" data-tauri-drag-region role="banner">
  <span class="titlebar-title" data-tauri-drag-region>Dota Keeper</span>
  <div class="titlebar-buttons">
    <button class="tb-btn" onclick={minimize} title="Minimize" aria-label="Minimize">
      <svg width="10" height="1" viewBox="0 0 10 1"><rect width="10" height="1" fill="currentColor"/></svg>
    </button>
    <button class="tb-btn" onclick={toggleMaximize} title="Maximize" aria-label="Maximize">
      <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
        <rect x="0.5" y="0.5" width="9" height="9" stroke="currentColor"/>
      </svg>
    </button>
    <button class="tb-btn close" onclick={close} title="Close" aria-label="Close">
      <svg width="10" height="10" viewBox="0 0 10 10"><line x1="0" y1="0" x2="10" y2="10" stroke="currentColor" stroke-width="1.2"/><line x1="10" y1="0" x2="0" y2="10" stroke="currentColor" stroke-width="1.2"/></svg>
    </button>
  </div>
</div>

<style>
  .titlebar {
    height: 32px;
    background: var(--bg-card);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px 0 20px;
    flex-shrink: 0;
    user-select: none;
    position: relative;
    z-index: 100;
  }

  .titlebar-title {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 3px;
    text-transform: uppercase;
    color: var(--gold-dim);
    pointer-events: none;
  }

  .titlebar-buttons {
    display: flex;
    gap: 0;
  }

  .tb-btn {
    width: 46px;
    height: 32px;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
    border-radius: 0;
  }

  .tb-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: var(--text-primary);
  }

  .tb-btn.close:hover {
    background: #c42b1c;
    color: #fff;
  }
</style>
