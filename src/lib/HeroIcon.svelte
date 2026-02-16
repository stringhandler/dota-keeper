<script>
  import { getHeroIconUrl, getHeroName } from './heroes.js';

  let {
    heroId,
    size = 'medium',
    showName = true
  } = $props();

  let imageError = $state(false);

  const iconUrl = $derived(getHeroIconUrl(heroId));
  const heroName = $derived(getHeroName(heroId));

  function handleImageError() {
    imageError = true;
  }
</script>

<div class="hero-icon-wrapper {size}">
  {#if iconUrl && !imageError}
    <img
      src={iconUrl}
      alt={heroName}
      class="hero-icon"
      onerror={handleImageError}
      loading="lazy"
    />
  {:else}
    <div class="hero-icon-fallback">
      {heroName.substring(0, 2).toUpperCase()}
    </div>
  {/if}
  {#if showName}
    <span class="hero-name">{heroName}</span>
  {/if}
</div>

<style>
  .hero-icon-wrapper {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
  }

  .hero-icon {
    display: block;
    border-radius: 4px;
    border: 2px solid rgba(139, 92, 46, 0.4);
    background-color: rgba(0, 0, 0, 0.3);
    transition: all 0.2s ease;
    object-fit: cover;
  }

  .hero-icon:hover {
    border-color: rgba(139, 92, 46, 0.7);
    box-shadow: 0 0 10px rgba(212, 175, 55, 0.3);
    transform: scale(1.05);
  }

  .hero-icon-fallback {
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    border: 2px solid rgba(139, 92, 46, 0.4);
    background: linear-gradient(135deg, rgba(60, 60, 70, 0.8) 0%, rgba(40, 40, 50, 0.8) 100%);
    color: #d4af37;
    font-weight: bold;
    font-size: 0.7rem;
  }

  /* Size variants */
  .small .hero-icon,
  .small .hero-icon-fallback {
    width: 32px;
    height: 32px;
    min-width: 32px;
  }

  .medium .hero-icon,
  .medium .hero-icon-fallback {
    width: 48px;
    height: 48px;
    min-width: 48px;
  }

  .large .hero-icon,
  .large .hero-icon-fallback {
    width: 64px;
    height: 64px;
    min-width: 64px;
  }

  .hero-name {
    color: #e0e0e0;
    font-size: 0.9rem;
    white-space: nowrap;
  }
</style>
