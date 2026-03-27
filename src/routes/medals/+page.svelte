<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { trackPageView } from "$lib/analytics.js";

  let isLoading = $state(true);
  let error = $state("");
  let history = $state(/** @type {any[]} */ ([]));
  let stats = $state(/** @type {any} */ (null));

  // rank_tier encoding: major = Math.floor(tier / 10), stars = tier % 10
  // 11-15 = Herald 1-5, 21-25 = Guardian 1-5, ..., 71-75 = Divine 1-5, 80 = Immortal
  const MEDAL_NAMES = ["", "Herald", "Guardian", "Crusader", "Archon", "Legend", "Ancient", "Divine", "Immortal"];
  const MEDAL_COLORS = [
    "",
    "#9e9e9e", // Herald
    "#4caf50", // Guardian
    "#00bcd4", // Crusader
    "#2196f3", // Archon
    "#9c27b0", // Legend
    "#ff9800", // Ancient
    "#f44336", // Divine
    "#f0b429", // Immortal
  ];

  /** @param {number} rankTier */
  function getMedalName(rankTier) {
    if (!rankTier) return "Unranked";
    const major = Math.floor(rankTier / 10);
    const stars = rankTier % 10;
    if (major === 8) return "Immortal";
    if (major >= 1 && major <= 7) return stars > 0 ? `${MEDAL_NAMES[major]} ${stars}` : MEDAL_NAMES[major];
    return "Unranked";
  }

  /** @param {number | null | undefined} rankTier */
  function getMedalColor(rankTier) {
    if (!rankTier) return "#555";
    const major = Math.floor(rankTier / 10);
    return MEDAL_COLORS[major] || "#555";
  }

  /** @param {number} rankTier */
  function getMedalMajor(rankTier) {
    if (!rankTier) return 0;
    return Math.floor(rankTier / 10);
  }

  /** @param {number} rankTier */
  function getMedalStars(rankTier) {
    if (!rankTier) return 0;
    const major = Math.floor(rankTier / 10);
    if (major === 8) return 0;
    return rankTier % 10;
  }

  /** @param {number} ts */
  function formatDate(ts) {
    return new Date(ts * 1000).toLocaleDateString(undefined, { month: "short", day: "numeric", year: "numeric" });
  }

  // Build deduplicated timeline — only show entries where rank changed
  let timeline = $derived.by(() => {
    if (!history.length) return [];
    const result = [];
    let lastTier = null;
    for (const entry of history) {
      if (entry.rank_tier !== lastTier) {
        result.push(entry);
        lastTier = entry.rank_tier;
      }
    }
    return result.reverse(); // most recent first
  });

  onMount(async () => {
    trackPageView("Medals");
    try {
      [history, stats] = await Promise.all([
        invoke("get_medal_history"),
        invoke("get_medal_stats"),
      ]);
    } catch (e) {
      error = `Failed to load medal data: ${e}`;
    } finally {
      isLoading = false;
    }
  });
</script>

<div class="medals-page">
  {#if isLoading}
    <div class="loading">Loading medal history...</div>
  {:else if error}
    <div class="error-banner">{error}</div>
  {:else if !history.length}
    <div class="empty-state">
      <div class="empty-icon">🏅</div>
      <div class="empty-title">No medal data yet</div>
      <div class="empty-sub">Sync your matches — rank is captured automatically from OpenDota.</div>
    </div>
  {:else}
    <!-- Top stats cards -->
    <div class="stats-row">
      <div class="stat-card" style="--medal-color: {getMedalColor(stats?.current_rank_tier)}">
        <div class="stat-label">Current Medal</div>
        <div class="medal-display">
          <div class="medal-badge" style="background: {getMedalColor(stats?.current_rank_tier)}20; border-color: {getMedalColor(stats?.current_rank_tier)}60;">
            <div class="medal-name" style="color: {getMedalColor(stats?.current_rank_tier)}">
              {getMedalName(stats?.current_rank_tier)}
            </div>
            {#if getMedalStars(stats?.current_rank_tier) > 0}
              <div class="medal-stars">
                {#each Array(getMedalStars(stats?.current_rank_tier)) as _}
                  <span class="star" style="color: {getMedalColor(stats?.current_rank_tier)}">★</span>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      </div>

      <div class="stat-card peak-card" style="--medal-color: {getMedalColor(stats?.peak_rank_tier)}">
        <div class="stat-label">Peak Medal</div>
        <div class="peak-badge-wrap">
          <span class="peak-tag">All-time Best</span>
        </div>
        <div class="medal-display">
          <div class="medal-badge" style="background: {getMedalColor(stats?.peak_rank_tier)}20; border-color: {getMedalColor(stats?.peak_rank_tier)}60;">
            <div class="medal-name" style="color: {getMedalColor(stats?.peak_rank_tier)}">
              {getMedalName(stats?.peak_rank_tier)}
            </div>
            {#if getMedalStars(stats?.peak_rank_tier) > 0}
              <div class="medal-stars">
                {#each Array(getMedalStars(stats?.peak_rank_tier)) as _}
                  <span class="star" style="color: {getMedalColor(stats?.peak_rank_tier)}">★</span>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-label">Tracked Games</div>
        <div class="stat-value">{history.length}</div>
        <div class="stat-sub">with rank data</div>
      </div>
    </div>

    <!-- Timeline -->
    <div class="section-title">Rank Changes</div>

    {#if timeline.length <= 1}
      <div class="no-changes">Your rank has not changed across tracked matches.</div>
    {:else}
      <div class="timeline">
        {#each timeline as entry, i}
          {@const isPeak = entry.rank_tier === stats?.peak_rank_tier}
          <div class="timeline-item">
            <div class="timeline-dot" style="background: {getMedalColor(entry.rank_tier)}; box-shadow: 0 0 8px {getMedalColor(entry.rank_tier)}60;"></div>
            {#if i < timeline.length - 1}
              <div class="timeline-line"></div>
            {/if}
            <div class="timeline-content">
              <div class="timeline-medal" style="color: {getMedalColor(entry.rank_tier)}">
                {getMedalName(entry.rank_tier)}
                {#if getMedalStars(entry.rank_tier) > 0}
                  <span class="inline-stars">
                    {#each Array(getMedalStars(entry.rank_tier)) as _}<span>★</span>{/each}
                  </span>
                {/if}
                {#if isPeak}
                  <span class="peak-chip">Peak</span>
                {/if}
              </div>
              <div class="timeline-date">{formatDate(entry.start_time)}</div>
              <a class="timeline-match-link" href="/matches/{entry.match_id}">Match #{entry.match_id}</a>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .medals-page {
    max-width: 720px;
  }

  .loading, .no-changes {
    color: var(--text-muted);
    font-size: 13px;
    padding: 20px 0;
  }

  .error-banner {
    background: rgba(220, 60, 60, 0.1);
    border: 1px solid rgba(220, 60, 60, 0.3);
    border-radius: 6px;
    padding: 12px 16px;
    color: #e57373;
    font-size: 13px;
  }

  /* ── Empty state ── */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    padding: 60px 20px;
    color: var(--text-muted);
    text-align: center;
  }

  .empty-icon { font-size: 40px; }

  .empty-title {
    font-family: 'Rajdhani', sans-serif;
    font-size: 18px;
    font-weight: 700;
    letter-spacing: 2px;
    color: var(--text-secondary);
    text-transform: uppercase;
  }

  .empty-sub { font-size: 13px; max-width: 340px; }

  /* ── Stats row ── */
  .stats-row {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 16px;
    margin-bottom: 32px;
  }

  @media (max-width: 640px) {
    .stats-row { grid-template-columns: 1fr 1fr; }
  }

  .stat-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    position: relative;
  }

  .peak-card {
    border-color: rgba(240, 180, 41, 0.3);
  }

  .stat-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .stat-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 32px;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1;
  }

  .stat-sub {
    font-size: 11px;
    color: var(--text-muted);
  }

  .peak-badge-wrap {
    position: absolute;
    top: 12px;
    right: 12px;
  }

  .peak-tag {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 9px;
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--gold);
    background: rgba(240, 180, 41, 0.1);
    border: 1px solid rgba(240, 180, 41, 0.3);
    border-radius: 3px;
    padding: 2px 6px;
  }

  /* ── Medal badge ── */
  .medal-display { display: flex; }

  .medal-badge {
    border: 1px solid;
    border-radius: 6px;
    padding: 8px 14px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .medal-name {
    font-family: 'Rajdhani', sans-serif;
    font-size: 18px;
    font-weight: 700;
    letter-spacing: 2px;
    text-transform: uppercase;
  }

  .medal-stars {
    display: flex;
    gap: 2px;
    font-size: 12px;
  }

  /* ── Timeline ── */
  .section-title {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 11px;
    letter-spacing: 2.5px;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 16px;
  }

  .timeline {
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .timeline-item {
    display: flex;
    align-items: flex-start;
    gap: 16px;
    position: relative;
    padding-bottom: 24px;
  }

  .timeline-item:last-child { padding-bottom: 0; }

  .timeline-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    flex-shrink: 0;
    margin-top: 4px;
    z-index: 1;
  }

  .timeline-line {
    position: absolute;
    left: 5px;
    top: 16px;
    bottom: 0;
    width: 2px;
    background: var(--border);
  }

  .timeline-content {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .timeline-medal {
    font-family: 'Rajdhani', sans-serif;
    font-size: 16px;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .inline-stars {
    font-size: 11px;
    display: flex;
    gap: 1px;
  }

  .peak-chip {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 9px;
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--gold);
    background: rgba(240, 180, 41, 0.1);
    border: 1px solid rgba(240, 180, 41, 0.3);
    border-radius: 3px;
    padding: 2px 6px;
  }

  .timeline-date {
    font-size: 11px;
    color: var(--text-muted);
  }

  .timeline-match-link {
    font-size: 11px;
    color: var(--gold-dim);
    text-decoration: none;
    transition: color 0.15s;
  }

  .timeline-match-link:hover { color: var(--gold); }
</style>
