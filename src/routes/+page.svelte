<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { getHeroName } from "$lib/heroes.js";

  let isLoading = $state(true);
  let error = $state("");
  let goalCalendar = $state([]);
  let heroSuggestion = $state(null);

  const DAYS_TO_SHOW = 7;

  onMount(async () => {
    await loadGoalCalendar();
    await loadHeroSuggestion();
  });

  async function loadGoalCalendar() {
    try {
      goalCalendar = await invoke("get_goals_calendar", { days: DAYS_TO_SHOW });
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
      alert("Goal created successfully!");
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
      console.error("Failed to refresh suggestion:", e);
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

  function formatGoalDescription(goal) {
    const heroName = goal.hero_id !== null ? getHeroName(goal.hero_id) : "Any Hero";
    const metricLabel = getMetricLabel(goal.metric);
    const unit = getMetricUnit(goal.metric);
    const valueStr = unit ? `${goal.target_value} ${unit}` : `Level ${goal.target_value}`;
    return `${heroName}: ${valueStr} by ${goal.target_time_minutes} min`;
  }

  function getProgressPercentage(day) {
    if (day.total === 0) return 0;
    return (day.achieved / day.total) * 100;
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

<div class="dashboard-content">
  {#if isLoading}
    <div class="loading">
      <p>Loading...</p>
    </div>
  {:else}
    <div class="page-header">
      <h1>Dashboard</h1>
      <p class="subtitle">Track your Dota 2 progress and goals</p>
    </div>

    {#if error}
      <p class="error">{error}</p>
    {/if}

    <div class="calendar-section">
      <h2>Goal Progress - Last 7 Days</h2>
      {#if goalCalendar.length === 0}
        <p class="no-goals">No goals found. <a href="/goals">Create your first goal</a> to start tracking your progress.</p>
      {:else}
        <div class="calendar-wrapper">
          <div class="calendar-header">
            <div class="goal-label-header">Goal</div>
            {#each goalCalendar[0]?.daily_progress || [] as day}
              <div class="day-header">{formatDayLabel(day.date)}</div>
            {/each}
          </div>
          {#each goalCalendar as goalData}
            <div class="calendar-row">
              <div class="goal-label">
                <span class="goal-text">{formatGoalDescription(goalData.goal)}</span>
                <a href="/goals/{goalData.goal.id}" class="details-btn">View Details</a>
              </div>
              {#each goalData.daily_progress as day}
                <div class="day-cell" title="{day.achieved}/{day.total} matches">
                  {#if day.total === 0}
                    <div class="progress-bar empty"></div>
                  {:else}
                    <div class="progress-bar">
                      <div
                        class="progress-fill"
                        style="height: {getProgressPercentage(day)}%"
                      ></div>
                    </div>
                    <div class="day-stats">{day.achieved}/{day.total}</div>
                  {/if}
                </div>
              {/each}
            </div>
          {/each}
        </div>
      {/if}
    </div>

    {#if heroSuggestion}
      <div class="suggestion-section">
        <h2>🎯 Suggested Goal This Week</h2>
        <div class="suggestion-card">
          <div class="hero-info">
            <div class="hero-name">{getHeroName(heroSuggestion.hero_id)}</div>
            <div class="time-marker">At 10 Minutes</div>
            <div class="suggestion-stats">
              <div class="stat-item">
                <span class="label">Current Average</span>
                <span class="value">{Math.round(heroSuggestion.current_average)} CS</span>
              </div>
              <div class="stat-item">
                <span class="label">Suggested Target</span>
                <span class="value highlight">{heroSuggestion.suggested_last_hits} CS</span>
              </div>
              <div class="stat-item">
                <span class="label">Improvement</span>
                <span class="value improvement">
                  +{heroSuggestion.suggested_last_hits - Math.round(heroSuggestion.current_average)} CS
                  (+{Math.round(((heroSuggestion.suggested_last_hits - heroSuggestion.current_average) / heroSuggestion.current_average) * 100)}%)
                </span>
              </div>
            </div>
            <div class="games-info">
              Based on your last {heroSuggestion.games_analyzed} games
            </div>
          </div>
          <div class="suggestion-actions">
            <button class="refresh-btn" onclick={() => refreshSuggestion()}>
              🔄 Refresh
            </button>
            <button class="accept-btn" onclick={() => acceptSuggestion(heroSuggestion)}>
              Create Goal
            </button>
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
.dashboard-content {
  max-width: 1400px;
  margin: 0 auto;
}

.loading {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 3rem;
  color: #d4af37;
  font-size: 1rem;
  letter-spacing: 2px;
  text-transform: uppercase;
}

.page-header {
  margin-bottom: 2rem;
  padding: 25px 30px;
  background:
    linear-gradient(180deg, rgba(30, 30, 40, 0.9) 0%, rgba(20, 20, 30, 0.9) 100%),
    repeating-linear-gradient(90deg, transparent, transparent 2px, rgba(139, 92, 46, 0.08) 2px, rgba(139, 92, 46, 0.08) 4px);
  background-size: 100%, 4px 4px;
  border: 2px solid rgba(139, 92, 46, 0.5);
  border-radius: 8px;
  box-shadow:
    0 4px 20px rgba(0, 0, 0, 0.5),
    inset 0 1px 0 rgba(255, 255, 255, 0.05);
}

.page-header h1 {
  margin: 0 0 0.5rem 0;
  font-size: 2em;
  color: #d4af37;
  text-shadow:
    0 0 20px rgba(212, 175, 55, 0.5),
    2px 2px 4px rgba(0, 0, 0, 0.8);
  letter-spacing: 3px;
  text-transform: uppercase;
}

.subtitle {
  color: #a0a0a0;
  margin: 0;
  font-size: 0.9rem;
  letter-spacing: 1px;
}

.error {
  color: #ff6b6b;
  background-color: rgba(220, 53, 69, 0.2);
  border: 1px solid rgba(220, 53, 69, 0.4);
  border-radius: 3px;
  padding: 0.75rem 1rem;
  margin-bottom: 1rem;
  font-size: 0.9rem;
}

.calendar-section {
  padding: 30px;
  background:
    linear-gradient(135deg, rgba(25, 25, 35, 0.8) 0%, rgba(20, 20, 30, 0.9) 100%),
    repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.1) 3px, rgba(0, 0, 0, 0.1) 6px),
    repeating-linear-gradient(-45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.05) 3px, rgba(0, 0, 0, 0.05) 6px);
  background-size: 100%, 6px 6px, 6px 6px;
  border: 2px solid rgba(139, 92, 46, 0.4);
  border-radius: 8px;
  box-shadow:
    0 4px 20px rgba(0, 0, 0, 0.5),
    inset 0 1px 0 rgba(255, 255, 255, 0.03);
}

.calendar-section h2 {
  margin-bottom: 1.5rem;
  font-size: 1.5em;
  color: #d4af37;
  text-transform: uppercase;
  letter-spacing: 2px;
  text-shadow: 0 0 10px rgba(212, 175, 55, 0.3);
  border-bottom: 2px solid rgba(139, 92, 46, 0.5);
  padding-bottom: 15px;
}

.no-goals {
  color: #a0a0a0;
  font-style: italic;
}

.no-goals a {
  color: #d4af37;
  text-decoration: none;
  font-weight: 600;
}

.no-goals a:hover {
  text-shadow: 0 0 10px rgba(212, 175, 55, 0.5);
}

.calendar-wrapper {
  background:
    linear-gradient(135deg, rgba(25, 25, 35, 0.8) 0%, rgba(20, 20, 30, 0.9) 100%),
    repeating-linear-gradient(45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.1) 3px, rgba(0, 0, 0, 0.1) 6px),
    repeating-linear-gradient(-45deg, transparent, transparent 3px, rgba(0, 0, 0, 0.05) 3px, rgba(0, 0, 0, 0.05) 6px);
  background-size: 100%, 6px 6px, 6px 6px;
  border: 2px solid rgba(139, 92, 46, 0.4);
  border-radius: 3px;
  padding: 1.5rem;
  box-shadow:
    0 4px 20px rgba(0, 0, 0, 0.5),
    inset 0 1px 0 rgba(255, 255, 255, 0.03);
  overflow-x: auto;
}

.calendar-header {
  display: grid;
  grid-template-columns: minmax(250px, 1fr) repeat(7, 80px);
  gap: 0.5rem;
  margin-bottom: 0.75rem;
  padding-bottom: 0.75rem;
  border-bottom: 2px solid rgba(139, 92, 46, 0.4);
}

.goal-label-header {
  font-weight: 600;
  font-size: 0.9rem;
  color: #d4af37;
  padding: 0.5rem;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.day-header {
  font-weight: 600;
  font-size: 0.85rem;
  color: #d4af37;
  text-align: center;
  padding: 0.5rem;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.calendar-row {
  display: grid;
  grid-template-columns: minmax(250px, 1fr) repeat(7, 80px);
  gap: 0.5rem;
  margin-bottom: 0.75rem;
  align-items: center;
  padding: 0.5rem;
  background: rgba(30, 30, 40, 0.3);
  border-radius: 3px;
  border-left: 3px solid rgba(139, 92, 46, 0.5);
}

.goal-label {
  font-size: 0.9rem;
  font-weight: 500;
  padding: 0.5rem;
  color: #e0e0e0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
}

.goal-text {
  flex: 1;
}

.details-btn {
  background: linear-gradient(180deg, rgba(40, 80, 100, 0.8) 0%, rgba(30, 60, 80, 0.8) 100%);
  color: #e0e0e0;
  padding: 6px 12px;
  font-size: 0.75rem;
  border: 2px solid rgba(92, 139, 139, 0.6);
  border-radius: 3px;
  text-decoration: none;
  font-weight: bold;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  transition: all 0.3s ease;
  box-shadow:
    0 2px 8px rgba(0, 0, 0, 0.4),
    inset 0 1px 0 rgba(255, 255, 255, 0.1);
  white-space: nowrap;
}

.details-btn:hover {
  background: linear-gradient(180deg, rgba(50, 95, 120, 0.9) 0%, rgba(40, 75, 100, 0.9) 100%);
  border-color: rgba(92, 139, 139, 0.8);
  box-shadow:
    0 4px 12px rgba(0, 0, 0, 0.6),
    0 0 15px rgba(100, 150, 200, 0.3);
  transform: translateY(-1px);
}

.day-cell {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
}

.progress-bar {
  width: 50px;
  height: 50px;
  background: rgba(40, 40, 50, 0.8);
  border: 1px solid rgba(139, 92, 46, 0.4);
  border-radius: 3px;
  position: relative;
  display: flex;
  align-items: flex-end;
  overflow: hidden;
  box-shadow: inset 0 2px 5px rgba(0, 0, 0, 0.8);
}

.progress-bar.empty {
  background-color: rgba(30, 30, 40, 0.5);
  border: 2px dashed rgba(80, 80, 90, 0.4);
}

.progress-fill {
  width: 100%;
  background: linear-gradient(90deg, #60c040 0%, #80e060 50%, #60c040 100%);
  box-shadow: 0 0 15px rgba(100, 255, 100, 0.5);
  transition: height 0.5s ease;
}

.day-stats {
  font-size: 0.75rem;
  color: #b0b0b0;
  font-weight: 600;
}

.suggestion-section {
  margin-top: 2rem;
  padding: 30px;
  background: linear-gradient(135deg, rgba(40, 30, 60, 0.8) 0%, rgba(30, 20, 50, 0.9) 100%);
  border: 2px solid rgba(139, 92, 200, 0.4);
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(138, 43, 226, 0.3);
}

.suggestion-section h2 {
  margin-bottom: 1.5rem;
  font-size: 1.5em;
  color: #b89bdb;
  text-transform: uppercase;
  letter-spacing: 2px;
  text-shadow: 0 0 10px rgba(184, 155, 219, 0.5);
}

.suggestion-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  background: rgba(30, 25, 40, 0.8);
  border-radius: 5px;
  border-left: 4px solid #b89bdb;
}

.hero-name {
  font-size: 1.8em;
  font-weight: bold;
  color: #d4af37;
  margin-bottom: 0.5rem;
}

.time-marker {
  font-size: 0.9rem;
  color: #b89bdb;
  text-transform: uppercase;
  letter-spacing: 1.5px;
  margin-bottom: 1rem;
  font-weight: 600;
}

.suggestion-stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
  margin-bottom: 0.75rem;
}

.stat-item {
  display: flex;
  flex-direction: column;
}

.stat-item .label {
  font-size: 0.85rem;
  color: #a0a0a0;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.stat-item .value {
  font-size: 1.3rem;
  font-weight: bold;
  color: #e0e0e0;
}

.stat-item .highlight {
  color: #b89bdb;
  text-shadow: 0 0 10px rgba(184, 155, 219, 0.5);
}

.stat-item .improvement {
  color: #60c040;
}

.games-info {
  font-size: 0.85rem;
  color: #a0a0a0;
  font-style: italic;
  margin-top: 0.5rem;
}

.suggestion-actions {
  display: flex;
  gap: 0.75rem;
  flex-direction: column;
}

.refresh-btn {
  background: linear-gradient(180deg, rgba(60, 60, 80, 0.8) 0%, rgba(40, 40, 60, 0.8) 100%);
  color: #e0e0e0;
  padding: 10px 20px;
  border: 2px solid rgba(139, 139, 139, 0.6);
  border-radius: 3px;
  font-weight: bold;
  text-transform: uppercase;
  letter-spacing: 1px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 0.9rem;
}

.refresh-btn:hover {
  background: linear-gradient(180deg, rgba(80, 80, 100, 0.9) 0%, rgba(60, 60, 80, 0.9) 100%);
  box-shadow: 0 0 15px rgba(139, 139, 139, 0.5);
  transform: translateY(-2px);
}

.accept-btn {
  background: linear-gradient(180deg, rgba(100, 70, 140, 0.8) 0%, rgba(80, 50, 120, 0.8) 100%);
  color: #e0e0e0;
  padding: 12px 24px;
  border: 2px solid rgba(184, 155, 219, 0.6);
  border-radius: 3px;
  font-weight: bold;
  text-transform: uppercase;
  letter-spacing: 1px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.accept-btn:hover {
  background: linear-gradient(180deg, rgba(120, 90, 160, 0.9) 0%, rgba(100, 70, 140, 0.9) 100%);
  box-shadow: 0 0 20px rgba(184, 155, 219, 0.5);
  transform: translateY(-2px);
}
</style>
