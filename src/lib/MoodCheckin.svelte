<script>
  import { invoke } from "@tauri-apps/api/core";

  let { matchId, onComplete } = $props();

  let energy = $state(null);
  let calm = $state(null);
  let attribution = $state(null);
  let isSubmitting = $state(false);

  // Show Q3 only when calm score is 1 or 2 (very uncalm)
  let showQ3 = $derived(calm !== null && calm <= 2);

  const energyEmojis = ["😴", "😐", "🙂", "😄", "⚡"];
  const calmEmojis = ["😤", "😠", "😐", "😊", "🧘"];
  const attributionOptions = [
    "Teammates",
    "The enemy",
    "My own mistakes",
    "Nothing — I was fine actually",
  ];

  async function submit() {
    if (energy === null || calm === null) return;
    isSubmitting = true;
    try {
      await invoke("save_mood_checkin", {
        matchId,
        energy,
        calm,
        attribution: showQ3 ? attribution : null,
      });
      onComplete();
    } catch (e) {
      console.error("Failed to save mood checkin:", e);
    } finally {
      isSubmitting = false;
    }
  }

  async function skip() {
    try {
      await invoke("dismiss_checkin", { matchId });
    } catch (e) {
      console.error("Failed to dismiss checkin:", e);
    }
    onComplete();
  }
</script>

<div class="checkin-card">
  <div class="checkin-header">
    <span class="checkin-icon">🧠</span>
    <h3 class="checkin-title">How was that session?</h3>
  </div>

  <div class="question-block">
    <p class="question-label">How energised did you feel?</p>
    <div class="emoji-scale">
      {#each energyEmojis as emoji, i}
        <button
          class="emoji-btn"
          class:selected={energy === i + 1}
          onclick={() => (energy = i + 1)}
          title={`${i + 1}`}
        >
          <span class="emoji">{emoji}</span>
          <span class="score">{i + 1}</span>
        </button>
      {/each}
    </div>
  </div>

  <div class="question-block">
    <p class="question-label">How calm were you?</p>
    <div class="emoji-scale">
      {#each calmEmojis as emoji, i}
        <button
          class="emoji-btn"
          class:selected={calm === i + 1}
          onclick={() => (calm = i + 1)}
          title={`${i + 1}`}
        >
          <span class="emoji">{emoji}</span>
          <span class="score">{i + 1}</span>
        </button>
      {/each}
    </div>
  </div>

  {#if showQ3}
    <div class="question-block">
      <p class="question-label">What got under your skin?</p>
      <div class="attribution-options">
        {#each attributionOptions as option}
          <button
            class="attribution-btn"
            class:selected={attribution === option}
            onclick={() => (attribution = attribution === option ? null : option)}
          >
            {option}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <div class="checkin-actions">
    <button class="skip-btn" onclick={skip} disabled={isSubmitting}>
      Skip
    </button>
    <button
      class="submit-btn"
      onclick={submit}
      disabled={isSubmitting || energy === null || calm === null}
    >
      {isSubmitting ? "Saving..." : "Submit"}
    </button>
  </div>
</div>

<style>
.checkin-card {
  background: var(--bg-card);
  border: 1px solid var(--border-active);
  border-radius: 10px;
  padding: 20px 24px;
  margin-bottom: 20px;
}

.checkin-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 18px;
}

.checkin-icon {
  font-size: 20px;
  line-height: 1;
}

.checkin-title {
  font-family: 'Rajdhani', sans-serif;
  font-size: 17px;
  font-weight: 700;
  letter-spacing: 1px;
  color: var(--text-primary);
  margin: 0;
}

.question-block {
  margin-bottom: 16px;
}

.question-label {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0 0 10px 0;
}

.emoji-scale {
  display: flex;
  gap: 6px;
}

.emoji-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  background: var(--bg-elevated);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 8px 10px;
  cursor: pointer;
  transition: all 0.15s;
  min-width: 44px;
  min-height: 44px;
  flex: 1;
}

.emoji-btn:hover {
  border-color: var(--border-active);
  background: rgba(255, 200, 80, 0.05);
}

.emoji-btn.selected {
  border-color: var(--gold);
  background: rgba(255, 200, 80, 0.1);
}

.emoji {
  font-size: 22px;
  line-height: 1;
}

.score {
  font-size: 11px;
  color: var(--text-muted);
  font-family: 'Barlow Condensed', sans-serif;
}

.emoji-btn.selected .score {
  color: var(--gold);
}

.attribution-options {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.attribution-btn {
  font-family: 'Barlow Condensed', sans-serif;
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.5px;
  background: var(--bg-elevated);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-secondary);
  padding: 7px 14px;
  cursor: pointer;
  transition: all 0.15s;
  min-height: 44px;
}

.attribution-btn:hover {
  border-color: var(--border-active);
  color: var(--text-primary);
}

.attribution-btn.selected {
  border-color: var(--gold);
  color: var(--gold);
  background: rgba(255, 200, 80, 0.08);
}

.checkin-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 18px;
  gap: 12px;
}

.skip-btn {
  font-family: 'Barlow Condensed', sans-serif;
  font-weight: 600;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  font-size: 11px;
  background: transparent;
  color: var(--text-muted);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 8px 18px;
  cursor: pointer;
  transition: all 0.2s;
}

.skip-btn:hover:not(:disabled) {
  color: var(--text-secondary);
  border-color: var(--border-active);
}

.skip-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.submit-btn {
  font-family: 'Barlow Condensed', sans-serif;
  font-weight: 600;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  font-size: 11px;
  background: var(--gold);
  color: #080c10;
  border: none;
  border-radius: 4px;
  padding: 8px 28px;
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
}

.submit-btn:hover:not(:disabled) {
  background: #ffdf80;
}

.submit-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

@media (max-width: 600px) {
  .emoji-btn {
    padding: 6px 4px;
    min-width: 44px;
  }

  .emoji {
    font-size: 20px;
  }

  .checkin-actions {
    flex-direction: row;
  }

  .submit-btn {
    flex: 1;
  }
}
</style>
