<script>
  import { _ } from "svelte-i18n";

  const MEDAL_COLORS = {
    herald: "#9e9e9e",
    guardian: "#4caf50",
    crusader: "#00bcd4",
    archon: "#2196f3",
    legend: "#9c27b0",
    ancient: "#ff9800",
    divine: "#f44336",
    immortal: "#f0b429",
  };

  const BRACKET_RANK = {
    herald: 1,
    guardian: 2,
    crusader: 3,
    archon: 4,
    legend: 5,
    ancient: 6,
    divine: 7,
    immortal: 8,
  };

  function getMedalIconUrl(/** @type {string} */ bracket) {
    const rank = BRACKET_RANK[/** @type {keyof typeof BRACKET_RANK} */ (bracket)] || 0;
    return `https://dotakeeper.com/meta/rank_icons/rank_icon_${rank}.png`;
  }

  /** Convert z-score to percentile (within that bracket's distribution) */
  function zToPercentile(/** @type {number} */ z) {
    // Approximation of the standard normal CDF
    const a1 = 0.254829592, a2 = -0.284496736, a3 = 1.421413741, a4 = -1.453152027, a5 = 1.061405429;
    const p = 0.3275911;
    const sign = z < 0 ? -1 : 1;
    const x = Math.abs(z) / Math.sqrt(2);
    const t = 1.0 / (1.0 + p * x);
    const y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * Math.exp(-x * x);
    return ((1.0 + sign * y) / 2.0) * 100;
  }

  /** @type {{ benchmarkData: { rows: any[], best_fit_bracket: string, data_date: string, user_std_dev: number | null, user_mean: number | null }, compact?: boolean }} */
  let { benchmarkData, compact = false } = $props();

  function capitalise(/** @type {string} */ s) {
    return s.charAt(0).toUpperCase() + s.slice(1);
  }

  function getBracketColor(/** @type {string} */ bracket) {
    return MEDAL_COLORS[/** @type {keyof typeof MEDAL_COLORS} */ (bracket)] || "#555";
  }

  function formatZ(/** @type {number} */ z) {
    return (z >= 0 ? "+" : "") + z.toFixed(2);
  }
</script>

{#if benchmarkData && benchmarkData.rows.length > 0}
  <div class="benchmark-wrapper" class:compact>
    <!-- Medal badge -->
    <div class="medal-badge-row">
      <div class="medal-badge" style="border-color: {getBracketColor(benchmarkData.best_fit_bracket)}60; background: {getBracketColor(benchmarkData.best_fit_bracket)}15;">
        <img class="medal-icon" src={getMedalIconUrl(benchmarkData.best_fit_bracket)} alt={benchmarkData.best_fit_bracket} />
        <span class="medal-label" style="color: {getBracketColor(benchmarkData.best_fit_bracket)}">
          {capitalise(benchmarkData.best_fit_bracket)}
        </span>
      </div>
      <div class="medal-stats">
        {#if benchmarkData.user_std_dev != null && benchmarkData.user_mean != null}
          {@const low = Math.max(0, benchmarkData.user_mean - 1.04 * benchmarkData.user_std_dev)}
          {@const high = benchmarkData.user_mean + 1.04 * benchmarkData.user_std_dev}
          <span class="user-range">70% of games: {low.toFixed(0)}–{high.toFixed(0)} LH</span>
        {/if}
        {#if benchmarkData.user_std_dev != null}
          <span class="user-sd">SD: {benchmarkData.user_std_dev.toFixed(1)}</span>
        {/if}
      </div>
    </div>

    {#if !compact}
      <!-- Full table -->
      <div class="benchmark-table">
        <div class="bt-header">
          <div class="bt-col bracket-col">Bracket</div>
          <div class="bt-col num-col">Mean</div>
          <div class="bt-col num-col">SD</div>
          <div class="bt-col num-col">z-score</div>
          <div class="bt-col num-col">Percentile</div>
          <div class="bt-col interp-col">Interpretation</div>
          <div class="bt-col num-col">n</div>
        </div>
        {#each benchmarkData.rows as row}
          <div class="bt-row" class:best-fit={row.is_best_fit} class:low-data={row.low_data} style={row.is_best_fit ? `border-left-color: ${getBracketColor(row.bracket)}` : ''}>
            <div class="bt-col bracket-col">
              <img class="bracket-medal-icon" src={getMedalIconUrl(row.bracket)} alt={row.bracket} />
              {capitalise(row.bracket)}
            </div>
            <div class="bt-col num-col">{row.mean.toFixed(1)}</div>
            <div class="bt-col num-col">{row.std_dev.toFixed(1)}</div>
            <div class="bt-col num-col" class:positive={row.z_score > 0} class:negative={row.z_score < 0}>
              {formatZ(row.z_score)}
            </div>
            <div class="bt-col num-col" class:positive={row.z_score > 0} class:negative={row.z_score < 0}>
              {zToPercentile(row.z_score).toFixed(0)}%
            </div>
            <div class="bt-col interp-col">
              {row.interpretation}
              {#if row.low_data}
                <span class="low-data-badge">Low data</span>
              {/if}
            </div>
            <div class="bt-col num-col sample-col" class:low-data-text={row.low_data}>{row.sample_size}</div>
          </div>
        {/each}
      </div>
    {/if}

    {#if benchmarkData.data_date}
      <div class="data-date">Data as of {benchmarkData.data_date}</div>
    {/if}
    <div class="experimental-warning">This analysis is experimental, could be wrong and might change in future.</div>
  </div>
{/if}

<style>
  .benchmark-wrapper {
    width: 100%;
  }

  .medal-badge-row {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px;
  }

  .medal-badge {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 6px 16px;
    border-radius: 6px;
    border: 1.5px solid;
  }

  .medal-icon {
    width: 32px;
    height: 32px;
    object-fit: contain;
  }

  .medal-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 16px;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
  }

  .medal-stats {
    display: flex;
    gap: 12px;
    align-items: center;
    flex-wrap: wrap;
  }

  .user-sd {
    font-size: 11px;
    color: var(--text-muted);
  }

  .user-range {
    font-size: 11px;
    color: var(--text-secondary);
  }

  /* Table */
  .benchmark-table {
    display: flex;
    flex-direction: column;
    gap: 2px;
    margin-top: 8px;
  }

  .bt-header {
    display: grid;
    grid-template-columns: 120px 70px 70px 80px 75px 1fr 50px;
    gap: 0.5rem;
    padding: 8px 12px;
    background: var(--bg-elevated);
    border-radius: 4px;
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .bt-row {
    display: grid;
    grid-template-columns: 120px 70px 70px 80px 75px 1fr 50px;
    gap: 0.5rem;
    padding: 8px 12px;
    background: var(--bg-surface);
    border-left: 3px solid transparent;
    border-radius: 4px;
    font-size: 12px;
    color: var(--text-secondary);
    transition: background 0.15s;
  }

  .bt-row:hover {
    background: var(--bg-elevated);
  }

  .bt-row.best-fit {
    background: var(--bg-elevated);
    border-left-width: 3px;
  }

  .bracket-col {
    display: flex;
    align-items: center;
    gap: 6px;
    font-weight: 600;
  }

  .bracket-medal-icon {
    width: 18px;
    height: 18px;
    object-fit: contain;
    flex-shrink: 0;
  }

  .num-col {
    font-family: 'Rajdhani', sans-serif;
    font-weight: 600;
    text-align: right;
  }

  .positive { color: var(--green); }
  .negative { color: var(--red); }

  .interp-col {
    font-size: 11px;
    color: var(--text-muted);
  }

  .data-date {
    margin-top: 8px;
    font-size: 10px;
    color: var(--text-muted);
    letter-spacing: 0.5px;
  }

  /* Compact mode */
  .compact .medal-badge-row {
    margin-bottom: 0;
  }

  .compact .medal-label {
    font-size: 13px;
  }

  /* Low data warning */
  .low-data-badge {
    display: inline-block;
    font-size: 9px;
    font-family: 'Barlow Condensed', sans-serif;
    letter-spacing: 0.5px;
    text-transform: uppercase;
    color: var(--gold);
    background: rgba(240, 180, 41, 0.12);
    padding: 1px 5px;
    border-radius: 3px;
    margin-left: 4px;
    vertical-align: middle;
  }

  .bt-row.low-data {
    opacity: 0.7;
  }

  .low-data-text {
    color: var(--gold) !important;
  }

  .sample-col {
    font-size: 10px;
    color: var(--text-muted);
  }

  .experimental-warning {
    margin-top: 8px;
    font-size: 10px;
    font-style: italic;
    color: var(--text-muted);
    opacity: 0.7;
  }
</style>
