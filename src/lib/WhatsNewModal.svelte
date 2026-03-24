<script>
  let { version = '', notes = [], onClose = () => {} } = $props();
</script>

<div class="modal-backdrop" onclick={onClose} role="presentation">
  <div class="modal-card" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true" aria-label="What's new">
    <div class="modal-header">
      <h2>What's New in v{version}</h2>
    </div>

    <div class="modal-body">
      <ul>
        {#each notes as note}
          <li>{note}</li>
        {/each}
      </ul>
    </div>

    <div class="modal-actions">
      <button class="btn btn-primary" onclick={onClose}>Got it</button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    backdrop-filter: blur(4px);
  }

  .modal-card {
    background: var(--bg-card, #0d1318);
    border: 2px solid var(--border, rgba(240, 180, 41, 0.3));
    border-radius: 8px;
    max-width: 480px;
    width: 90%;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.8);
  }

  .modal-header {
    padding: 24px 28px 20px;
    border-bottom: 1px solid var(--border, rgba(240, 180, 41, 0.2));
  }

  .modal-header h2 {
    margin: 0;
    font-family: 'Rajdhani', sans-serif;
    font-size: 22px;
    font-weight: 700;
    letter-spacing: 2px;
    color: var(--gold, #f0b429);
    text-transform: uppercase;
  }

  .modal-body {
    padding: 20px 28px;
    color: var(--text-primary, #e8dcc8);
  }

  ul {
    margin: 0;
    padding-left: 20px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  li {
    font-size: 14px;
    line-height: 1.6;
    color: var(--text-secondary, #b8a895);
  }

  .modal-actions {
    padding: 16px 28px 24px;
    border-top: 1px solid var(--border, rgba(240, 180, 41, 0.2));
    display: flex;
    justify-content: flex-end;
  }
</style>
