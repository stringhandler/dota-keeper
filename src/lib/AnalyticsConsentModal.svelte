<script>
  import { invoke } from "@tauri-apps/api/core";
  import { updateAnalyticsConsent } from "$lib/analytics.js";

  let { onClose = () => {} } = $props();
  let isSaving = $state(false);

  async function handleConsent(consent) {
    isSaving = true;
    try {
      await invoke("save_analytics_consent", { consent });
      updateAnalyticsConsent(consent);
      onClose();
    } catch (e) {
      console.error("Failed to save consent:", e);
      // Close anyway - user shouldn't be trapped
      onClose();
    }
  }
</script>

<div class="modal-backdrop" onclick={()  => {}}>
  <div class="modal-card" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <h2>Help Improve Dota Keeper</h2>
    </div>

    <div class="modal-body">
      <p class="intro">
        Would you like to help improve Dota Keeper by sending anonymous usage data?
      </p>

      <div class="info-section">
        <h3>✅ What we collect:</h3>
        <ul>
          <li>App version and platform (Windows/macOS/Linux)</li>
          <li>Page views (e.g. "opened Goals page")</li>
          <li>Feature usage (e.g. "goal created", "challenge accepted")</li>
          <li>Error events (type only, no personal details)</li>
        </ul>
      </div>

      <div class="info-section">
        <h3>🚫 What we DON'T collect:</h3>
        <ul>
          <li>Steam ID or any personally identifiable information</li>
          <li>Match IDs, hero choices, or game data</li>
          <li>Goal descriptions, targets, or any content you enter</li>
        </ul>
      </div>

      <p class="note">
        You can change this preference anytime in Settings → Privacy.
      </p>
    </div>

    <div class="modal-actions">
      <button
        class="btn btn-primary"
        onclick={() => handleConsent("Accepted")}
        disabled={isSaving}
      >
        {isSaving ? 'Saving...' : 'Accept'}
      </button>
      <button
        class="btn btn-ghost"
        onclick={() => handleConsent("Declined")}
        disabled={isSaving}
      >
        Decline
      </button>
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
    max-width: 560px;
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
    padding: 24px 28px;
    color: var(--text-primary, #e8dcc8);
  }

  .intro {
    font-size: 15px;
    margin: 0 0 20px 0;
    line-height: 1.6;
  }

  .info-section {
    margin-bottom: 18px;
  }

  .info-section h3 {
    font-size: 14px;
    font-weight: 600;
    margin: 0 0 10px 0;
    color: var(--text-primary, #e8dcc8);
  }

  .info-section ul {
    margin: 0;
    padding-left: 20px;
  }

  .info-section li {
    font-size: 13px;
    line-height: 1.8;
    color: var(--text-secondary, #b8a895);
  }

  .note {
    margin: 16px 0 0 0;
    font-size: 12px;
    font-style: italic;
    color: var(--text-muted, #8a7f6e);
  }

  .modal-actions {
    padding: 20px 28px 24px;
    border-top: 1px solid var(--border, rgba(240, 180, 41, 0.2));
    display: flex;
    gap: 12px;
    justify-content: flex-end;
  }
</style>
