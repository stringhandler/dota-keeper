<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { goto } from "$app/navigation";
  import { getHeroName } from "$lib/heroes.js";
  import HeroIcon from "$lib/HeroIcon.svelte";

  let isLoading = $state(true);
  let error = $state("");
  let goalCalendar = $state([]);
  let heroSuggestion = $state(null);
  let goals = $state([]);
  let items = $state([]);
  let dailyProgress = $state(null);
  let dailyStreak = $state(0);
  let timeUntilMidnight = $state("");
  let midnightTimer = null;
  let weeklyProgress = $state(null);

  // Quick stats
  let recentMatches = $state([]);
  let analysisData = $state(null);

  const DAYS_TO_SHOW = 7;

  let isSuggestionAdopted = $derived(
    heroSuggestion !== null &&
    goals.some(
      (g) =>
        g.hero_id === heroSuggestion.hero_id &&
        g.metric === "LastHits" &&
        g.target_time_minutes === 10
    )
  );

  // Quick stats derived
  let winRate7d = $derived.by(() => {
    const sevenDaysAgo = Date.now() / 1000 - 7 * 24 * 3600;
    const recent = recentMatches.filter(m => m.start_time > sevenDaysAgo);
    if (recent.length === 0) return null;
    const wins = recent.filter(m => {
      const isRadiant = m.player_slot < 128;
      return (isRadiant && m.radiant_win) || (!isRadiant && !m.radiant_win);
    });
    return { rate: Math.round((wins.length / recent.length) * 100), count: recent.length };
  });

  let goalsHit7d = $derived.by(() => {
    let total = 0, hit = 0;
    for (const g of goalCalendar) {
      for (const d of g.daily_progress) {
        total += d.total;
        hit += d.achieved;
      }
    }
    return { hit, total };
  });

  let avgCS = $derived(
    analysisData?.current_period?.count > 0
      ? analysisData.current_period.average
      : null
  );

  onMount(async () => {
    await Promise.all([
      loadGoalCalendar(),
      loadHeroSuggestion(),
      loadItems(),
      loadDailyChallenge(),
      loadWeeklyChallenge(),
      loadQuickStats(),
    ]);
    updateMidnightCountdown();
    midnightTimer = setInterval(updateMidnightCountdown, 60000);
  });

  onDestroy(() => {
    if (midnightTimer) clearInterval(midnightTimer);
  });

  function updateMidnightCountdown() {
    const now = new Date();
    const midnight = new Date(now);
    midnight.setHours(24, 0, 0, 0);
    const diffMs = midnight - now;
    const hours = Math.floor(diffMs / 3600000);
    const mins = Math.floor((diffMs % 3600000) / 60000);
    timeUntilMidnight = hours > 0 ? `${hours}h ${mins}m` : `${mins}m`;
  }

  async function loadQuickStats() {
    try {
      [recentMatches, analysisData] = await Promise.all([
        invoke("get_matches"),
        invoke("get_last_hits_analysis_data", {
          timeMinutes: 10,
          windowSize: 30,
          heroId: null,
          gameMode: null,
        }),
      ]);
    } catch (e) {
      console.error("Failed to load quick stats:", e);
    }
  }

  async function loadDailyChallenge() {
    try {
      [dailyProgress, dailyStreak] = await Promise.all([
        invoke("get_daily_challenge_progress_cmd"),
        invoke("get_daily_streak_cmd"),
      ]);
    } catch (e) {
      console.error("Failed to load daily challenge:", e);
    }
  }

  async function loadWeeklyChallenge() {
    try {
      weeklyProgress = await invoke("get_weekly_challenge_progress_cmd");
    } catch (e) {
      console.error("Failed to load weekly challenge:", e);
    }
  }

  function weeklyProgressPct(p) {
    if (!p || p.target <= 0) return 0;
    return Math.min(100, Math.round((p.current_value / p.target) * 100));
  }

  function getDailyProgressPct(progress) {
    if (!progress) return 0;
    return Math.min(100, Math.round((progress.current_value / progress.target) * 100));
  }

  function formatDailyDescription(progress) {
    if (!progress) return "";
    const c = progress.challenge;
    let desc = c.challenge_description;
    if (c.hero_id !== null) desc += ` (${getHeroName(c.hero_id)})`;
    return desc;
  }

  async function loadItems() {
    try {
      items = await invoke("get_all_items");
    } catch (e) {
      console.error("Failed to load items:", e);
    }
  }

  async function loadGoalCalendar() {
    try {
      [goalCalendar, goals] = await Promise.all([
        invoke("get_goals_calendar", { days: DAYS_TO_SHOW }),
        invoke("get_goals"),
      ]);
    } catch (e) {
      error = `Failed to load goal calendar: ${e}`;
      console.error("Failed to load goal calendar:", e);
    } finally {
      isLoading = false;
    }
  }

  async function loadHeroSuggestion() {
    try {
      heroSuggestion = await invoke("get_hero_goal_suggestion");
    } catch (e) {
      console.error("Failed to load hero suggestion:", e);
    }
  }

  async function acceptSuggestion(suggestion) {
    try {
      await invoke("create_goal", {
        goal: {
          hero_id: suggestion.hero_id,
          metric: "LastHits",
          target_value: suggestion.suggested_last_hits,
          target_time_minutes: 10,
          game_mode: "Ranked"
        }
      });
      await loadGoalCalendar();
    } catch (e) {
      error = `Failed to create goal: ${e}`;
    }
  }

  async function refreshSuggestion() {
    try {
      heroSuggestion = await invoke("refresh_hero_goal_suggestion");
    } catch (e) {
      error = `Failed to refresh suggestion: ${e}`;
    }
  }

  function getMetricLabel(metric) {
    switch (metric) {
      case "Networth": return "Net Worth";
      case "Kills": return "Kills";
      case "LastHits": return "Last Hits";
      case "Level": return "Level";
      default: return metric;
    }
  }

  function getMetricUnit(metric) {
    switch (metric) {
      case "Networth": return "gold";
      case "Kills": return "kills";
      case "LastHits": return "CS";
      case "Level": return "";
      default: return "";
    }
  }

  function getItemName(itemId) {
    const item = items.find(i => i.id === itemId);
    return item ? item.display_name : `Item ${itemId}`;
  }

  function formatGoalDescription(goal) {
    const heroName = goal.hero_id !== null ? getHeroName(goal.hero_id) : "Any Hero";
    if (goal.metric === "ItemTiming") {
      const itemName = goal.item_id !== null ? getItemName(goal.item_id) : "Unknown Item";
      const minutes = Math.floor(goal.target_value / 60);
      const seconds = goal.target_value % 60;
      const timeStr = seconds > 0 ? `${minutes}:${seconds.toString().padStart(2, '0')}` : `${minutes}:00`;
      return `${heroName} — ${itemName} by ${timeStr}`;
    } else {
      const metricLabel = getMetricLabel(goal.metric);
      const unit = getMetricUnit(goal.metric);
      const valueStr = unit ? `${goal.target_value} ${unit}` : `Level ${goal.target_value}`;
      return `${heroName} — ${valueStr} by ${goal.target_time_minutes} min`;
    }
  }

  function getDotClass(day) {
    if (day.total === 0) return 'empty';
    return day.achieved > 0 ? 'hit' : 'miss';
  }

  function getDotContent(day) {
    if (day.total === 0) return '';
    return day.achieved > 0 ? '✓' : '✗';
  }

  function getHitRate(goalData) {
    const total = goalData.daily_progress.reduce((s, d) => s + d.total, 0);
    const hit = goalData.daily_progress.reduce((s, d) => s + d.achieved, 0);
    if (total === 0) return 0;
    return (hit / total) * 100;
  }

  function getTrendLabel(goalData) {
    const days = goalData.daily_progress;
    const recent = days.slice(-3);
    const earlier = days.slice(0, 4);
    const recentTotal = recent.reduce((s, d) => s + d.total, 0);
    const earlierTotal = earlier.reduce((s, d) => s + d.total, 0);
    if (recentTotal === 0 && earlierTotal === 0) return null;
    const recentRate = recentTotal > 0
      ? recent.reduce((s, d) => s + d.achieved, 0) / recentTotal
      : 0;
    const earlierRate = earlierTotal > 0
      ? earlier.reduce((s, d) => s + d.achieved, 0) / earlierTotal
      : 0;
    if (recentRate > earlierRate + 0.15) return { label: 'Improving', cls: 'improving' };
    if (recentRate < earlierRate - 0.15) return { label: 'Declining', cls: 'declining' };
    return { label: 'Stable', cls: 'stable' };
  }

  function formatDayLabel(dateString) {
    const date = new Date(dateString + "T00:00:00Z");
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    const diffTime = date.getTime() - today.getTime();
    const diffDays = Math.round(diffTime / (1000 * 60 * 60 * 24));
    if (diffDays === 0) return "Today";
    if (diffDays === -1) return "Yesterday";
    return date.toLocaleDateString('en-US', { weekday: 'short' });
  }
</script>

<div class="dashboard">
  {#if isLoading}
    <div class="loading-state">Loading...</div>
  {:else}
    {#if error}
      <div class="error-banner">{error}</div>
    {/if}

    <!-- QUICK STATS STRIP -->
    <div class="stats-row">
      <div class="stat-card">
        <div class="stat-label">Win Rate (7d)</div>
        {#if winRate7d}
          <div class="stat-value">{winRate7d.rate}<span class="stat-unit">%</span></div>
          <div class="stat-sub">{winRate7d.count} games</div>
        {:else}
          <div class="stat-value stat-na">—</div>
          <div class="stat-sub">No recent games</div>
        {/if}
      </div>

      <div class="stat-card">
        <div class="stat-label">Avg CS @ 10min</div>
        {#if avgCS !== null}
          <div class="stat-value">{avgCS.toFixed(1)}</div>
          <div class="stat-sub">last {analysisData?.current_period?.count || 0} games</div>
        {:else}
          <div class="stat-value stat-na">—</div>
          <div class="stat-sub">Parse matches to track</div>
        {/if}
      </div>

      <div class="stat-card">
        <div class="stat-label">Goals Hit (7d)</div>
        {#if goalsHit7d.total > 0}
          <div class="stat-value">{goalsHit7d.hit}<span class="stat-unit">/{goalsHit7d.total}</span></div>
          <div class="stat-sub">{Math.round((goalsHit7d.hit / goalsHit7d.total) * 100)}% success rate</div>
        {:else}
          <div class="stat-value stat-na">—</div>
          <div class="stat-sub">No goal attempts yet</div>
        {/if}
      </div>

      <div class="stat-card">
        <div class="stat-label">Active Goals</div>
        <div class="stat-value">{goals.length}</div>
        <div class="stat-sub">
          {#if goals.length === 0}No goals set{:else}being tracked{/if}
        </div>
      </div>
    </div>

    <!-- TODAY'S CHALLENGE -->
    {#if dailyProgress}
      <div class="section-header">
        <div class="section-title">⚡ Today's Challenge</div>
        <div class="reset-text">Resets in {timeUntilMidnight}</div>
      </div>
      <div class="challenge-card" class:completed={dailyProgress.completed}>
        <div class="challenge-icon">
          {#if dailyProgress.challenge.hero_id !== null}
            <HeroIcon heroId={dailyProgress.challenge.hero_id} size="small" showName={false} />
          {:else}
            {dailyProgress.completed ? '✅' : '⚔️'}
          {/if}
        </div>
        <div class="challenge-info">
          <div class="challenge-title">{formatDailyDescription(dailyProgress)}</div>
          <div class="challenge-bar-wrap">
            <div class="challenge-bar-fill" style="width: {getDailyProgressPct(dailyProgress)}%"></div>
          </div>
          <div class="challenge-meta">
            <span>{dailyProgress.current_value} / {dailyProgress.target} completed</span>
            {#if dailyProgress.completed}
              <span class="complete-tag">✓ Complete!</span>
            {:else if dailyStreak > 0}
              <span class="streak-tag">🔥 {dailyStreak} day streak</span>
            {/if}
          </div>
        </div>
        <a href="/challenges" class="btn btn-ghost">View Stats</a>
      </div>
    {/if}

    <!-- GOAL PROGRESS -->
    <div class="section-header" style="margin-top: 8px;">
      <div class="section-title">Goal Progress — Last 7 Days</div>
      <a href="/goals" class="btn btn-ghost">Manage Goals</a>
    </div>

    {#if goalCalendar.length === 0}
      <div class="empty-goals">
        <p>No goals yet.</p>
        <a href="/goals" class="btn btn-primary" style="margin-top:12px">Create your first goal →</a>
      </div>
    {:else}
      <div class="goals-grid">
        {#each goalCalendar as goalData}
          {@const trend = getTrendLabel(goalData)}
          {@const hitRate = getHitRate(goalData)}
          <div class="goal-row" onclick={() => goto(`/goals/${goalData.goal.id}`)}>
            <div class="hero-avatar">
              {#if goalData.goal.hero_id !== null}
                <HeroIcon heroId={goalData.goal.hero_id} size="small" showName={false} />
              {:else}
                🌟
              {/if}
            </div>
            <div class="goal-info">
              <div class="goal-name">{formatGoalDescription(goalData.goal)}</div>
              <div class="goal-progress-bar">
                <div class="goal-fill {hitRate >= 70 ? 'success' : ''}" style="width:{hitRate}%"></div>
              </div>
              <div class="goal-meta">
                {#if trend}
                  <span class="trend-{trend.cls}">{trend.label}</span>
                {/if}
                <span>{goalData.goal.metric === 'ItemTiming' ? 'Item Goal' : getMetricLabel(goalData.goal.metric) + ' Goal'}</span>
                <span>{goalData.goal.game_mode}</span>
              </div>
            </div>
            <div class="goal-dots">
              {#each goalData.daily_progress as day}
                <div class="dot {getDotClass(day)}" title="{formatDayLabel(day.date)}: {day.achieved}/{day.total}">
                  {getDotContent(day)}
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    {/if}

    <!-- WEEKLY CHALLENGE -->
    <div class="section-header" style="margin-top: 28px;">
      <div class="section-title">🏆 Weekly Challenge</div>
    </div>
    {#if weeklyProgress}
      <a href="/challenges" class="weekly-card {weeklyProgress.completed ? 'completed' : ''}">
        <div class="weekly-desc">{weeklyProgress.challenge.challenge_description}</div>
        <div class="weekly-bar-wrap">
          <div class="weekly-bar">
            <div class="weekly-fill {weeklyProgress.completed ? 'done' : ''}"
                 style="width:{weeklyProgressPct(weeklyProgress)}%"></div>
          </div>
          <span class="weekly-label">{weeklyProgress.current_value}/{weeklyProgress.target}</span>
        </div>
        <div class="weekly-meta">
          {#if weeklyProgress.completed}
            <span class="complete-tag">✓ Complete!</span>
          {:else}
            <span class="reset-text">{weeklyProgress.days_remaining} days left</span>
          {/if}
        </div>
      </a>
    {:else}
      <a href="/challenges" class="weekly-card empty">
        <span class="weekly-empty-text">Choose this week's challenge →</span>
      </a>
    {/if}

    <!-- HERO SUGGESTION -->
    {#if heroSuggestion && !isSuggestionAdopted}
      <div class="section-header" style="margin-top: 28px;">
        <div class="section-title">🎯 Suggested Goal</div>
      </div>
      <div class="suggestion-card">
        <div class="suggestion-hero">
          <HeroIcon heroId={heroSuggestion.hero_id} size="medium" />
        </div>
        <div class="suggestion-info">
          <div class="suggestion-hero-name">{getHeroName(heroSuggestion.hero_id)}</div>
          <div class="suggestion-stats">
            <div class="sug-stat">
              <div class="sug-label">Current Avg</div>
              <div class="sug-value">{Math.round(heroSuggestion.current_average)} CS</div>
            </div>
            <div class="sug-stat">
              <div class="sug-label">Suggested</div>
              <div class="sug-value gold">{heroSuggestion.suggested_last_hits} CS</div>
            </div>
            <div class="sug-stat">
              <div class="sug-label">Improvement</div>
              <div class="sug-value green">+{heroSuggestion.suggested_last_hits - Math.round(heroSuggestion.current_average)} CS</div>
            </div>
          </div>
          <div class="sug-games">Based on {heroSuggestion.games_analyzed} games</div>
        </div>
        <div class="suggestion-actions">
          <button class="btn btn-ghost" onclick={() => refreshSuggestion()}>🔄 Refresh</button>
          <button class="btn btn-primary" onclick={() => acceptSuggestion(heroSuggestion)}>Create Goal</button>
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  .dashboard {
    max-width: 1400px;
    margin: 0 auto;
  }

  /* ── QUICK STATS ── */
  .stats-row {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;
    margin-bottom: 28px;
  }

  .stat-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 18px 20px;
    position: relative;
    overflow: hidden;
    transition: border-color 0.2s;
  }

  .stat-card:hover { border-color: var(--border-active); }

  .stat-card::before {
    content: '';
    position: absolute;
    top: 0; left: 0; right: 0;
    height: 2px;
    background: linear-gradient(90deg, transparent, var(--gold), transparent);
    opacity: 0;
    transition: opacity 0.2s;
  }

  .stat-card:hover::before { opacity: 1; }

  .stat-label {
    font-family: 'Barlow Condensed', sans-serif;
    font-size: 10px;
    letter-spacing: 2px;
    color: var(--text-muted);
    text-transform: uppercase;
    margin-bottom: 8px;
  }

  .stat-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 32px;
    font-weight: 700;
    color: var(--text-primary);
    line-height: 1;
  }

  .stat-value.stat-na { color: var(--text-muted); }

  .stat-unit {
    font-size: 18px;
    color: var(--text-secondary);
  }

  .stat-sub {
    font-size: 11px;
    color: var(--text-secondary);
    margin-top: 4px;
  }

  /* ── TODAY'S CHALLENGE ── */
  .challenge-card {
    background: var(--bg-card);
    border: 1px solid rgba(240, 180, 41, 0.25);
    border-radius: 8px;
    padding: 18px 22px;
    display: flex;
    align-items: center;
    gap: 18px;
    margin-bottom: 28px;
    position: relative;
    overflow: hidden;
    transition: border-color 0.2s;
  }

  .challenge-card::before {
    content: '';
    position: absolute;
    top: 0; left: 0; right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, var(--gold), transparent);
  }

  .challenge-card.completed {
    border-color: rgba(74, 222, 128, 0.3);
  }

  .challenge-card.completed::before {
    background: linear-gradient(90deg, transparent, var(--green), transparent);
  }

  .challenge-icon {
    font-size: 26px;
    flex-shrink: 0;
  }

  .challenge-info { flex: 1; }

  .challenge-title {
    font-family: 'Rajdhani', sans-serif;
    font-size: 17px;
    font-weight: 700;
    letter-spacing: 1px;
    color: var(--gold);
    margin-bottom: 8px;
  }

  .challenge-bar-wrap {
    height: 5px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 3px;
    overflow: hidden;
    margin-bottom: 4px;
  }

  .challenge-bar-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--gold-dim), var(--gold-bright));
    border-radius: 3px;
    transition: width 0.8s ease;
  }

  .challenge-meta {
    font-size: 11px;
    color: var(--text-muted);
    display: flex;
    gap: 14px;
  }

  .reset-text {
    font-size: 11px;
    color: var(--text-muted);
    font-family: 'Barlow Condensed', sans-serif;
  }

  .complete-tag {
    color: var(--green);
    font-weight: 600;
    font-family: 'Barlow Condensed', sans-serif;
  }

  .streak-tag {
    color: var(--gold);
    font-weight: 600;
    font-family: 'Barlow Condensed', sans-serif;
  }

  /* ── GOALS GRID ── */
  .goals-grid {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-bottom: 8px;
  }

  /* Inherit .goal-row, .hero-avatar, .goal-info, .goal-name, etc. from app.css */

  .trend-improving { color: var(--green); }
  .trend-declining { color: var(--red); }
  .trend-stable { color: var(--teal); }

  .empty-goals {
    background: var(--bg-card);
    border: 1px dashed var(--border);
    border-radius: 8px;
    padding: 40px;
    text-align: center;
    color: var(--text-muted);
    font-size: 13px;
    margin-bottom: 28px;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  /* ── WEEKLY CHALLENGE ── */
  .weekly-card {
    display: block;
    padding: 18px 22px;
    border-radius: 8px;
    border: 1px solid rgba(100, 100, 200, 0.3);
    background: var(--bg-card);
    text-decoration: none;
    transition: border-color 0.2s;
    margin-bottom: 8px;
  }

  .weekly-card:hover { border-color: rgba(130, 130, 255, 0.5); }
  .weekly-card.completed { border-color: rgba(74, 222, 128, 0.3); }

  .weekly-card.empty {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 60px;
    border-style: dashed;
    border-color: rgba(100, 100, 200, 0.2);
  }

  .weekly-empty-text { color: var(--text-secondary); font-size: 13px; }

  .weekly-desc {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 10px;
  }

  .weekly-bar-wrap {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 6px;
  }

  .weekly-bar {
    flex: 1;
    height: 5px;
    background: rgba(40, 40, 70, 0.8);
    border-radius: 3px;
    overflow: hidden;
  }

  .weekly-fill {
    height: 100%;
    background: linear-gradient(90deg, rgba(130, 130, 255, 0.8), rgba(150, 150, 255, 1));
    border-radius: 3px;
    transition: width 0.4s ease;
  }

  .weekly-fill.done {
    background: linear-gradient(90deg, #16a34a, var(--green));
  }

  .weekly-label {
    font-size: 11px;
    color: var(--text-secondary);
    white-space: nowrap;
    min-width: 3rem;
    text-align: right;
    font-family: 'Barlow Condensed', sans-serif;
  }

  .weekly-meta { font-size: 11px; }

  /* ── HERO SUGGESTION ── */
  .suggestion-card {
    background: var(--bg-card);
    border: 1px solid rgba(139, 92, 200, 0.3);
    border-radius: 8px;
    padding: 20px 24px;
    display: flex;
    align-items: center;
    gap: 20px;
    margin-bottom: 8px;
  }

  .suggestion-hero { flex-shrink: 0; }

  .suggestion-info { flex: 1; }

  .suggestion-hero-name {
    font-family: 'Rajdhani', sans-serif;
    font-size: 18px;
    font-weight: 700;
    color: var(--gold);
    margin-bottom: 10px;
  }

  .suggestion-stats {
    display: grid;
    grid-template-columns: repeat(3, auto);
    gap: 20px;
    margin-bottom: 6px;
  }

  .sug-stat { display: flex; flex-direction: column; }

  .sug-label {
    font-size: 10px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 1px;
    font-family: 'Barlow Condensed', sans-serif;
  }

  .sug-value {
    font-family: 'Rajdhani', sans-serif;
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .sug-value.gold { color: var(--gold); }
  .sug-value.green { color: var(--green); }

  .sug-games {
    font-size: 11px;
    color: var(--text-muted);
    font-style: italic;
  }

  .suggestion-actions {
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex-shrink: 0;
  }
</style>
