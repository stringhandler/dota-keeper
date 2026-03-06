<script>
  import { page } from "$app/stores";
  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";
  import { showToast } from "$lib/toast.js";
  import { PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY } from "$env/static/public";

  let { onClose = () => {} } = $props();

  let category = $state(""); // 'bug' | 'feature' | 'positive'
  let feedbackText = $state("");
  let priority = $state(""); // 'blocking' | 'annoying' | 'minor'
  let isSubmitting = $state(false);
  let appVersion = $state("unknown");

  onMount(async () => {
    try {
      appVersion = await getVersion();
    } catch (_) {
      // fallback — stays "unknown"
    }
  });

  const followUpPrompts = {
    bug: "What happened? What did you expect?",
    feature: "What were you trying to do that wasn't possible?",
    positive: "What specifically do you like? Why is it useful?",
  };

  function getPlatform() {
    const ua = navigator.userAgent.toLowerCase();
    if (ua.includes("windows")) return "windows";
    if (ua.includes("mac os")) return "macos";
    if (ua.includes("linux")) return "linux";
    return "unknown";
  }

  async function handleSubmit() {
    if (!category || !feedbackText.trim()) return;

    if (!PUBLIC_SUPABASE_URL || !PUBLIC_SUPABASE_ANON_KEY) {
      showToast("Feedback is not configured yet.", "error");
      return;
    }

    isSubmitting = true;
    try {
      const payload = {
        category,
        feedback: feedbackText.trim(),
        priority: category !== "positive" && priority ? priority : null,
        page: $page.url.pathname,
        app_version: appVersion,
        platform: getPlatform(),
      };

      const res = await fetch(`${PUBLIC_SUPABASE_URL}/rest/v1/feedback`, {
        method: "POST",
        headers: {
          apikey: PUBLIC_SUPABASE_ANON_KEY,
          Authorization: `Bearer ${PUBLIC_SUPABASE_ANON_KEY}`,
          "Content-Type": "application/json",
          Prefer: "return=minimal",
        },
        body: JSON.stringify(payload),
      });

      if (!res.ok) throw new Error(`HTTP ${res.status}`);

      showToast("Feedback submitted — thank you!", "success");
      category = "";
      feedbackText = "";
      priority = "";
      onClose();
    } catch (e) {
      showToast("Failed to submit feedback. Please try again.", "error");
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="modal-backdrop" onclick={onClose} role="presentation">
  <div class="modal-card" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true" aria-label="Send feedback">

    <div class="modal-header">
      <h2>Send Feedback</h2>
      <button class="close-btn" onclick={onClose} aria-label="Close">✕</button>
    </div>

    <div class="modal-body">
      <!-- Step 1: Category -->
      <div class="field-group">
        <label class="field-label">What kind of feedback is this?</label>
        <div class="category-options">
          <button
            class="category-btn"
            class:selected={category === "bug"}
            onclick={() => { category = "bug"; priority = ""; }}
            type="button"
          >
            <span class="cat-icon">🐛</span>
            <span>Bug / Something broken</span>
          </button>
          <button
            class="category-btn"
            class:selected={category === "feature"}
            onclick={() => { category = "feature"; priority = ""; }}
            type="button"
          >
            <span class="cat-icon">💡</span>
            <span>Missing feature or improvement</span>
          </button>
          <button
            class="category-btn"
            class:selected={category === "positive"}
            onclick={() => { category = "positive"; priority = ""; }}
            type="button"
          >
            <span class="cat-icon">⭐</span>
            <span>Works great — tell us what you like</span>
          </button>
        </div>
      </div>

      <!-- Step 2: Follow-up (shown once category selected) -->
      {#if category}
        <div class="field-group">
          <label class="field-label" for="feedback-text">
            {followUpPrompts[category]}
          </label>
          <textarea
            id="feedback-text"
            class="feedback-textarea"
            bind:value={feedbackText}
            placeholder="Your answer..."
            rows="3"
            maxlength="500"
          ></textarea>
        </div>

        <!-- Step 3: Priority (only for bug / feature) -->
        {#if category !== "positive"}
          <div class="field-group">
            <label class="field-label">How much is this affecting your use of the app? <span class="optional">(optional)</span></label>
            <div class="priority-options">
              {#each [
                { value: "blocking", label: "Blocking — I can't use the app properly" },
                { value: "annoying", label: "Annoying — I work around it" },
                { value: "minor",    label: "Minor — Nice to have" },
              ] as opt}
                <label class="priority-radio">
                  <input
                    type="radio"
                    name="priority"
                    value={opt.value}
                    bind:group={priority}
                  />
                  <span>{opt.label}</span>
                </label>
              {/each}
            </div>
          </div>
        {/if}
      {/if}
    </div>

    <div class="modal-actions">
      <button class="btn btn-ghost" onclick={onClose} type="button">Cancel</button>
      <button
        class="btn btn-primary"
        onclick={handleSubmit}
        disabled={!category || !feedbackText.trim() || isSubmitting}
        type="button"
      >
        {isSubmitting ? "Sending..." : "Send Feedback"}
      </button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    backdrop-filter: blur(4px);
  }

  .modal-card {
    background: var(--bg-card, #0d1318);
    border: 1px solid var(--border, rgba(240, 180, 41, 0.3));
    border-radius: 8px;
    width: min(520px, calc(100vw - 32px));
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.8);
    display: flex;
    flex-direction: column;
  }

  .modal-header {
    padding: 20px 24px 16px;
    border-bottom: 1px solid var(--border, rgba(240, 180, 41, 0.2));
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .modal-header h2 {
    margin: 0;
    font-family: 'Rajdhani', sans-serif;
    font-size: 20px;
    font-weight: 700;
    letter-spacing: 2px;
    color: var(--gold, #f0b429);
    text-transform: uppercase;
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted, #8a7f6e);
    font-size: 14px;
    cursor: pointer;
    padding: 4px 6px;
    border-radius: 4px;
    line-height: 1;
    transition: color 0.15s;
  }

  .close-btn:hover { color: var(--text-primary); }

  .modal-body {
    padding: 20px 24px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .field-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-muted, #8a7f6e);
  }

  .optional {
    font-size: 10px;
    opacity: 0.7;
    text-transform: none;
    letter-spacing: 0;
  }

  /* Category buttons */
  .category-options {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .category-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    background: var(--bg-elevated, rgba(255,255,255,0.03));
    border: 1px solid var(--border, rgba(240,180,41,0.15));
    border-radius: 6px;
    color: var(--text-secondary, #b8a895);
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.5px;
    cursor: pointer;
    text-align: left;
    transition: border-color 0.15s, color 0.15s, background 0.15s;
  }

  .category-btn:hover {
    border-color: rgba(240, 180, 41, 0.4);
    color: var(--text-primary);
  }

  .category-btn.selected {
    border-color: var(--gold, #f0b429);
    color: var(--gold, #f0b429);
    background: rgba(240, 180, 41, 0.06);
  }

  .cat-icon {
    font-size: 15px;
    flex-shrink: 0;
  }

  /* Textarea */
  .feedback-textarea {
    width: 100%;
    min-height: 80px;
    resize: vertical;
    background: var(--bg-elevated, rgba(255,255,255,0.03));
    border: 1px solid var(--border, rgba(240,180,41,0.15));
    border-radius: 6px;
    color: var(--text-primary, #e8dcc8);
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 14px;
    padding: 10px 12px;
    box-sizing: border-box;
    transition: border-color 0.15s;
    outline: none;
  }

  .feedback-textarea:focus {
    border-color: rgba(240, 180, 41, 0.5);
  }

  .feedback-textarea::placeholder {
    color: var(--text-muted, #8a7f6e);
  }

  /* Priority radios */
  .priority-options {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .priority-radio {
    display: flex;
    align-items: center;
    gap: 8px;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 13px;
    color: var(--text-secondary, #b8a895);
    cursor: pointer;
  }

  .priority-radio input[type="radio"] {
    accent-color: var(--gold, #f0b429);
    width: 14px;
    height: 14px;
    flex-shrink: 0;
  }

  /* Actions */
  .modal-actions {
    padding: 16px 24px 20px;
    border-top: 1px solid var(--border, rgba(240, 180, 41, 0.2));
    display: flex;
    gap: 10px;
    justify-content: flex-end;
  }
</style>
