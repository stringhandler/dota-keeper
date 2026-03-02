<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import HeroIcon from "$lib/HeroIcon.svelte";
  import { trackPageView } from "$lib/analytics.js";

  let isLoading = $state(true);
  let error = $state("");
  let assessment = $state(null);
  let history = $state([]);
  let trackingEnabled = $state(false);

  onMount(async () => {
    await loadAll();
    trackPageView("MentalHealth");
  });

  async function loadAll() {
    isLoading = true;
    error = "";
    try {
      const settings = await invoke("get_settings");
      trackingEnabled = settings.mental_health_tracking_enabled ?? false;

      if (trackingEnabled) {
        const [a, h] = await Promise.all([
          invoke("get_tilt_assessment"),
          invoke("get_checkin_history", { limit: 40 }),
        ]);
        assessment = a;
        history = h;
      }
    } catch (e) {
      error = `Failed to load data: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  function formatDate(ts) {
    return new Date(ts * 1000).toLocaleDateString("en-GB", { day: "numeric", month: "short" });
  }

  function formatTime(ts) {
    return new Date(ts * 1000).toLocaleTimeString("en-GB", { hour: "2-digit", minute: "2-digit" });
  }

  const energyEmojis = ["", "😴", "😐", "🙂", "😄", "⚡"];
  const calmEmojis   = ["", "😤", "😠", "😐", "😊", "🧘"];
  const energyLabels = ["", "Drained", "Tired", "Neutral", "Good", "Energised"];
  const calmLabels   = ["", "Very frustrated", "Frustrated", "Neutral", "Calm", "Zen"];

  function trendArrow(trend) {
    if (trend === "improving") return "↗";
    if (trend === "declining") return "↘";
    if (trend === "stable")    return "→";
    return "";
  }

  function trendLabel(trend) {
    if (trend === "improving") return "Improving over recent sessions";
    if (trend === "declining") return "Declining — see suggestion below";
    if (trend === "stable")    return "Stable";
    return "";
  }

  function trendColor(trend) {
    if (trend === "improving") return "var(--green)";
    if (trend === "declining") return "var(--red)";
    return "var(--gold)";
  }

  function barPct(value, max = 5) {
    return `${Math.round((value / max) * 100)}%`;
  }

  let completedCheckins = $derived(history.filter(h => !h.skipped).length);
</script>

<div class="page">
  <header class="page-header">
    <h1>Mental Wellbeing</h1>
  </header>

  {#if isLoading}
    <div class="loading-state">Loading...</div>
  {:else if error}
    <div class="error-banner">{error}</div>
  {:else if !trackingEnabled}
    <div class="card disabled-state">
      <div class="disabled-icon">🧠</div>
      <h3>Mood tracking is off</h3>
      <p>Enable post-game mood check-ins in <a href="/settings">Settings → Mental Wellbeing</a> to start building your history.</p>
    </div>
  {:else}

    <!-- ─── Tilt Assessment ─────────────────────────────────────── -->
    {#if assessment}
      <div class="card assessment-card">
        <div class="assessment-header">
          <span class="assessment-icon">🧠</span>
          <h2 class="assessment-title">Mental State</h2>
          {#if assessment.tilt_score > 55}
            <span class="warn-badge">⚠</span>
          {/if}
        </div>

        {#if !assessment.has_sufficient_data && assessment.energy_avg_7d == null}
          <!-- Not enough data yet -->
          <p class="muted-note">Complete more check-ins to see your mood trends.</p>
          <div class="data-progress">
            {#each Array(3) as _, i}
              <span class="dot" class:filled={completedCheckins > i}></span>
            {/each}
            <span class="dot-count">{Math.min(completedCheckins, 3)} of 3 needed</span>
          </div>
        {:else}
          <!-- Averages -->
          <div class="avg-grid">
            {#if assessment.energy_avg_7d != null}
              <div class="avg-row">
                <span class="avg-label">Energy</span>
                <div class="bar-track">
                  <div class="bar-fill" style="width: {barPct(assessment.energy_avg_7d)}"></div>
                </div>
                <span class="avg-value">{assessment.energy_avg_7d.toFixed(1)} avg (7d)</span>
              </div>
            {/if}
            {#if assessment.calm_avg_7d != null}
              <div class="avg-row">
                <span class="avg-label">Calm</span>
                <div class="bar-track">
                  <div class="bar-fill" style="width: {barPct(assessment.calm_avg_7d)}"></div>
                </div>
                <span class="avg-value">{assessment.calm_avg_7d.toFixed(1)} avg (7d)</span>
              </div>
            {/if}
          </div>

          {#if assessment.trend !== "insufficient_data"}
            <div class="trend-line" style="color: {trendColor(assessment.trend)}">
              {trendArrow(assessment.trend)} {trendLabel(assessment.trend)}
            </div>
          {/if}
        {/if}
      </div>

      <!-- ─── Suggestion Card ──────────────────────────────────── -->
      {#if assessment.suggestion}
        {@const s = assessment.suggestion}
        <div class="card suggestion-card" class:suggestion-positive={s.severity === "positive"} class:suggestion-high={s.severity === "high"}>
          <div class="suggestion-header">
            <span class="suggestion-icon">🧠</span>
            <h3 class="suggestion-title">{s.title}</h3>
          </div>
          <p class="suggestion-body">{s.body}</p>
          {#if s.actions.length > 0}
            <div class="suggestion-actions">
              {#each s.actions as action}
                <button class="action-btn">{action}</button>
              {/each}
            </div>
          {/if}
        </div>
      {/if}
    {/if}

    <!-- ─── Check-in History ───────────────────────────────────── -->
    <h2 class="section-title">Check-in History</h2>

    {#if history.length === 0}
      <div class="card empty-history">
        <p>No check-ins yet. After your next match a prompt may appear based on your frequency setting.</p>
      </div>
    {:else}
      <div class="history-list">
        {#each history as item}
          <div class="history-row" class:row-skipped={item.skipped}>
            <div class="row-left">
              {#if item.hero_id}
                <HeroIcon heroId={item.hero_id} size={36} />
              {:else}
                <div class="hero-placeholder"></div>
              {/if}
              <div class="row-meta">
                <span class="row-date">{formatDate(item.checked_at)}</span>
                <span class="row-time">{formatTime(item.checked_at)}</span>
                {#if item.won != null}
                  <span class="result-pill" class:win-pill={item.won} class:loss-pill={!item.won}>
                    {item.won ? "W" : "L"}
                  </span>
                {/if}
              </div>
            </div>

            <div class="row-right">
              {#if item.skipped}
                <span class="skipped-tag">Skipped</span>
              {:else}
                {#if item.energy}
                  <span class="mood-cell" title="{energyLabels[item.energy]}">
                    {energyEmojis[item.energy]}
                    <span class="mood-score">{item.energy}</span>
                  </span>
                {/if}
                {#if item.calm}
                  <span class="mood-cell" title="{calmLabels[item.calm]}">
                    {calmEmojis[item.calm]}
                    <span class="mood-score">{item.calm}</span>
                  </span>
                {/if}
                {#if item.attribution}
                  <span class="attrib-tag">{item.attribution}</span>
                {/if}
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}

  {/if}
</div>

<style>
  .page {
    padding: 16px;
    padding-bottom: 90px;
    max-width: 700px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: 20px;
  }

  .page-header h1 {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 22px;
    font-weight: 700;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-primary);
    margin: 0;
  }

  .loading-state {
    color: var(--text-muted);
    padding: 40px 0;
    text-align: center;
    font-size: 13px;
  }

  .error-banner {
    background: rgba(248, 113, 113, 0.1);
    border: 1px solid rgba(248, 113, 113, 0.3);
    color: var(--red);
    border-radius: 6px;
    padding: 12px 16px;
    font-size: 13px;
  }

  /* ── Cards ──────────────────────────────────────────────────── */
  .card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 12px;
  }

  /* ── Disabled state ─────────────────────────────────────────── */
  .disabled-state {
    text-align: center;
    padding: 40px 24px;
  }

  .disabled-icon {
    font-size: 36px;
    margin-bottom: 12px;
  }

  .disabled-state h3 {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 18px;
    font-weight: 700;
    letter-spacing: 1px;
    color: var(--text-primary);
    margin: 0 0 8px;
  }

  .disabled-state p {
    font-size: 13px;
    color: var(--text-secondary);
    margin: 0;
  }

  .disabled-state a {
    color: var(--gold);
    text-decoration: none;
  }

  /* ── Assessment card ────────────────────────────────────────── */
  .assessment-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 14px;
  }

  .assessment-icon {
    font-size: 18px;
  }

  .assessment-title {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 16px;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--text-primary);
    margin: 0;
    flex: 1;
  }

  .warn-badge {
    font-size: 14px;
    color: var(--red);
  }

  .muted-note {
    font-size: 13px;
    color: var(--text-secondary);
    margin: 0 0 12px;
  }

  .data-progress {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
  }

  .dot.filled {
    background: var(--gold);
    border-color: var(--gold);
  }

  .dot-count {
    font-size: 12px;
    color: var(--text-muted);
    margin-left: 4px;
  }

  .avg-grid {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-bottom: 12px;
  }

  .avg-row {
    display: grid;
    grid-template-columns: 60px 1fr 100px;
    align-items: center;
    gap: 10px;
  }

  .avg-label {
    font-size: 12px;
    font-family: 'Barlow Condensed', sans-serif;
    letter-spacing: 0.8px;
    text-transform: uppercase;
    color: var(--text-secondary);
  }

  .bar-track {
    height: 6px;
    background: var(--bg-elevated);
    border-radius: 3px;
    overflow: hidden;
  }

  .bar-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--gold-dim), var(--gold));
    border-radius: 3px;
    transition: width 0.3s ease;
  }

  .avg-value {
    font-size: 12px;
    color: var(--text-secondary);
    text-align: right;
    white-space: nowrap;
  }

  .trend-line {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.5px;
  }

  /* ── Suggestion card ────────────────────────────────────────── */
  .suggestion-card {
    border-color: rgba(45, 212, 191, 0.3);
    background: rgba(45, 212, 191, 0.04);
  }

  .suggestion-positive {
    border-color: rgba(74, 222, 128, 0.3);
    background: rgba(74, 222, 128, 0.04);
  }

  .suggestion-high {
    border-color: rgba(248, 113, 113, 0.3);
    background: rgba(248, 113, 113, 0.04);
  }

  .suggestion-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
  }

  .suggestion-icon {
    font-size: 16px;
  }

  .suggestion-title {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 15px;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--text-primary);
    margin: 0;
  }

  .suggestion-body {
    font-size: 14px;
    color: var(--text-secondary);
    line-height: 1.5;
    margin: 0 0 12px;
  }

  .suggestion-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .action-btn {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 1px;
    text-transform: uppercase;
    padding: 6px 14px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--bg-elevated);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s;
  }

  .action-btn:hover {
    border-color: var(--teal);
    color: var(--teal);
  }

  /* ── Section title ──────────────────────────────────────────── */
  .section-title {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 14px;
    font-weight: 700;
    letter-spacing: 1.5px;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 20px 0 10px;
  }

  /* ── Empty history ──────────────────────────────────────────── */
  .empty-history p {
    font-size: 13px;
    color: var(--text-secondary);
    margin: 0;
  }

  /* ── History list ───────────────────────────────────────────── */
  .history-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .history-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 6px;
    gap: 12px;
  }

  .history-row.row-skipped {
    opacity: 0.5;
  }

  .row-left {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .hero-placeholder {
    width: 36px;
    height: 36px;
    border-radius: 4px;
    background: var(--bg-elevated);
    flex-shrink: 0;
  }

  .row-meta {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
  }

  .row-date {
    font-size: 13px;
    color: var(--text-primary);
    white-space: nowrap;
  }

  .row-time {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
  }

  .result-pill {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    font-weight: 700;
    padding: 1px 6px;
    border-radius: 3px;
    letter-spacing: 0.5px;
  }

  .win-pill {
    background: rgba(74, 222, 128, 0.15);
    color: var(--green);
  }

  .loss-pill {
    background: rgba(248, 113, 113, 0.15);
    color: var(--red);
  }

  .row-right {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .skipped-tag {
    font-size: 11px;
    color: var(--text-muted);
    font-style: italic;
  }

  .mood-cell {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1px;
  }

  .mood-cell span:first-child {
    font-size: 18px;
    line-height: 1;
  }

  .mood-score {
    font-size: 10px;
    color: var(--text-muted);
    font-family: 'Barlow Condensed', sans-serif;
  }

  .attrib-tag {
    font-size: 10px;
    color: var(--text-muted);
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 2px 6px;
    white-space: nowrap;
    max-width: 90px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* ── Mobile ─────────────────────────────────────────────────── */
  @media (max-width: 480px) {
    .avg-row {
      grid-template-columns: 55px 1fr 80px;
    }

    .avg-value {
      font-size: 11px;
    }

    .attrib-tag {
      display: none;
    }
  }
</style>
