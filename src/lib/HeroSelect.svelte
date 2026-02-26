<script>
  import { getHeroIconUrl } from './heroes.js';

  let {
    value = $bindable(''),
    heroes = [],
    favoriteIds = new Set(),
    anyLabel = 'Any Hero',
    anyValue = '',
    groupOptions = [],  // [{value, label}] shown between "Any Hero" and heroes
    onchange = null,
  } = $props();

  let isOpen = $state(false);
  let containerEl = $state(null);
  let triggerImgError = $state(false);

  let favoriteHeroList = $derived(heroes.filter(h => favoriteIds.has(h.id)));
  let otherHeroList = $derived(heroes.filter(h => !favoriteIds.has(h.id)));

  // Loose == handles string/number mismatch from native select behaviour
  let selectedHero = $derived(
    value !== null && value !== '' && value !== undefined
      ? heroes.find(h => h.id == value) ?? null
      : null
  );

  let selectedGroup = $derived(
    groupOptions.find(g => g.value === value) ?? null
  );

  $effect(() => {
    // Reset trigger image error whenever selected hero changes
    selectedHero;
    triggerImgError = false;
  });

  function selectHero(heroId) {
    value = heroId;
    isOpen = false;
    onchange?.();
  }

  function selectAny() {
    value = anyValue;
    isOpen = false;
    onchange?.();
  }

  function toggle() {
    isOpen = !isOpen;
  }

  $effect(() => {
    if (!isOpen) return;
    function handleClick(e) {
      if (containerEl && !containerEl.contains(e.target)) {
        isOpen = false;
      }
    }
    function handleKeydown(e) {
      if (e.key === 'Escape') isOpen = false;
    }
    document.addEventListener('click', handleClick);
    document.addEventListener('keydown', handleKeydown);
    return () => {
      document.removeEventListener('click', handleClick);
      document.removeEventListener('keydown', handleKeydown);
    };
  });
</script>

{#snippet heroOptionRow(hero, selected)}
  {@const iconUrl = getHeroIconUrl(hero.id)}
  <div
    class="hero-option"
    class:selected
    role="option"
    aria-selected={selected}
    tabindex="0"
    onclick={() => selectHero(hero.id)}
    onkeydown={(e) => e.key === 'Enter' && selectHero(hero.id)}
  >
    <span class="option-icon-wrap">
      {#if iconUrl}
        <img
          src={iconUrl}
          alt={hero.name}
          class="option-icon"
          loading="lazy"
          onerror={(e) => { e.currentTarget.style.display = 'none'; e.currentTarget.nextElementSibling.style.display = 'flex'; }}
        />
        <span class="option-icon-fallback" style="display:none">{hero.name.substring(0, 2).toUpperCase()}</span>
      {:else}
        <span class="option-icon-fallback">{hero.name.substring(0, 2).toUpperCase()}</span>
      {/if}
    </span>
    <span class="option-name">{hero.name}</span>
  </div>
{/snippet}

<div class="hero-select" bind:this={containerEl}>
  <button type="button" class="hero-select-trigger" class:open={isOpen} onclick={toggle}>
    {#if selectedHero}
      {@const iconUrl = getHeroIconUrl(selectedHero.id)}
      {#if iconUrl && !triggerImgError}
        <img
          src={iconUrl}
          alt={selectedHero.name}
          class="trigger-icon"
          onerror={() => (triggerImgError = true)}
          loading="lazy"
        />
      {:else}
        <div class="trigger-icon-fallback">{selectedHero.name.substring(0, 2).toUpperCase()}</div>
      {/if}
      <span class="trigger-name">{selectedHero.name}</span>
    {:else if selectedGroup}
      <span class="trigger-group">{selectedGroup.label}</span>
    {:else}
      <span class="trigger-placeholder">{anyLabel}</span>
    {/if}
    <span class="chevron" class:rotated={isOpen}>▾</span>
  </button>

  {#if isOpen}
    <div class="hero-select-dropdown" role="listbox">
      <div
        class="hero-option"
        class:selected={!selectedHero && !selectedGroup}
        role="option"
        aria-selected={!selectedHero && !selectedGroup}
        tabindex="0"
        onclick={selectAny}
        onkeydown={(e) => e.key === 'Enter' && selectAny()}
      >
        <span class="option-any">{anyLabel}</span>
      </div>

      {#if groupOptions.length > 0}
        <div class="group-label">By Role</div>
        {#each groupOptions as opt}
          <div
            class="hero-option"
            class:selected={value === opt.value}
            role="option"
            aria-selected={value === opt.value}
            tabindex="0"
            onclick={() => { value = opt.value; isOpen = false; onchange?.(); }}
            onkeydown={(e) => e.key === 'Enter' && (() => { value = opt.value; isOpen = false; onchange?.(); })()}
          >
            <span class="option-group">{opt.label}</span>
          </div>
        {/each}
      {/if}

      {#if favoriteHeroList.length > 0}
        <div class="group-label">⭐ Favorites</div>
        {#each favoriteHeroList as hero (hero.id)}
          {@render heroOptionRow(hero, hero.id == value)}
        {/each}
      {/if}

      <div class="group-label">All Heroes</div>
      {#each otherHeroList as hero (hero.id)}
        {@render heroOptionRow(hero, hero.id == value)}
      {/each}
    </div>
  {/if}
</div>

<style>
  .hero-select {
    position: relative;
    display: inline-block;
    width: 100%;
  }

  .hero-select-trigger {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    font-family: 'Barlow', sans-serif;
    font-size: 13px;
    padding: 6px 10px;
    cursor: pointer;
    text-align: left;
    transition: border-color 0.2s;
    min-height: 34px;
  }

  .hero-select-trigger:hover,
  .hero-select-trigger.open {
    border-color: var(--border-active);
    outline: none;
  }

  .trigger-icon {
    width: 22px;
    height: 22px;
    border-radius: 3px;
    object-fit: cover;
    object-position: center top;
    flex-shrink: 0;
    border: 1px solid rgba(139, 92, 46, 0.4);
  }

  .trigger-icon-fallback {
    width: 22px;
    height: 22px;
    border-radius: 3px;
    background: rgba(60, 60, 70, 0.8);
    border: 1px solid rgba(139, 92, 46, 0.4);
    color: #d4af37;
    font-size: 9px;
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .trigger-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .trigger-placeholder {
    flex: 1;
    color: var(--text-secondary);
  }

  .chevron {
    margin-left: auto;
    font-size: 11px;
    color: var(--text-secondary);
    transition: transform 0.15s;
    flex-shrink: 0;
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  .hero-select-dropdown {
    position: absolute;
    top: calc(100% + 3px);
    left: 0;
    right: 0;
    z-index: 200;
    background: var(--bg-elevated);
    border: 1px solid var(--border-active);
    border-radius: 4px;
    max-height: 260px;
    overflow-y: auto;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);
  }

  .hero-option {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 10px;
    cursor: pointer;
    font-size: 13px;
    color: var(--text-primary);
    transition: background 0.1s;
  }

  .hero-option:hover {
    background: rgba(255, 200, 80, 0.08);
  }

  .hero-option.selected {
    background: rgba(255, 200, 80, 0.12);
    color: var(--gold);
  }

  .option-any {
    color: var(--text-secondary);
    font-style: italic;
  }

  .hero-option.selected .option-any {
    color: var(--gold);
    font-style: normal;
  }

  .option-group {
    color: var(--text-secondary);
  }

  .hero-option.selected .option-group {
    color: var(--gold);
  }

  .trigger-group {
    flex: 1;
    color: var(--text-secondary);
    font-style: italic;
  }

  .group-label {
    font-size: 11px;
    color: var(--text-muted);
    padding: 4px 10px 2px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    border-top: 1px solid rgba(255, 200, 80, 0.06);
    margin-top: 2px;
  }

  .option-icon-wrap {
    flex-shrink: 0;
    width: 20px;
    height: 20px;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .option-icon {
    width: 20px;
    height: 20px;
    border-radius: 2px;
    object-fit: cover;
    object-position: center top;
    border: 1px solid rgba(139, 92, 46, 0.3);
    display: block;
  }

  .option-icon-fallback {
    width: 20px;
    height: 20px;
    border-radius: 2px;
    background: rgba(60, 60, 70, 0.8);
    border: 1px solid rgba(139, 92, 46, 0.3);
    color: #d4af37;
    font-size: 8px;
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .option-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .hero-select-dropdown::-webkit-scrollbar {
    width: 4px;
  }

  .hero-select-dropdown::-webkit-scrollbar-track {
    background: transparent;
  }

  .hero-select-dropdown::-webkit-scrollbar-thumb {
    background: rgba(255, 200, 80, 0.2);
    border-radius: 2px;
  }
</style>
