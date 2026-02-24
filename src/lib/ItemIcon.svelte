<script>
  let {
    itemName,
    displayName = itemName,
    size = 'medium',
    showName = true
  } = $props();

  let imageError = $state(false);

  const iconUrl = $derived(
    itemName
      ? `https://cdn.cloudflare.steamstatic.com/apps/dota2/images/dota_react/items/${itemName}.png`
      : null
  );

  function handleImageError() {
    imageError = true;
  }
</script>

<div class="item-icon-wrapper {size}">
  {#if iconUrl && !imageError}
    <img
      src={iconUrl}
      alt={displayName}
      class="item-icon"
      onerror={handleImageError}
      loading="lazy"
    />
  {:else}
    <div class="item-icon-fallback">
      {displayName.substring(0, 2).toUpperCase()}
    </div>
  {/if}
  {#if showName}
    <span class="item-name">{displayName}</span>
  {/if}
</div>

<style>
  .item-icon-wrapper {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
  }

  .item-icon {
    display: block;
    border-radius: 4px;
    border: 1px solid rgba(212, 175, 55, 0.35);
    background-color: rgba(0, 0, 0, 0.3);
    transition: all 0.2s ease;
    object-fit: cover;
  }

  .item-icon:hover {
    border-color: rgba(212, 175, 55, 0.65);
    box-shadow: 0 0 8px rgba(212, 175, 55, 0.3);
  }

  .item-icon-fallback {
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    border: 1px solid rgba(212, 175, 55, 0.35);
    background: linear-gradient(135deg, rgba(60, 60, 70, 0.8) 0%, rgba(40, 40, 50, 0.8) 100%);
    color: #d4af37;
    font-weight: bold;
    font-size: 0.65rem;
  }

  /* Size variants */
  .small .item-icon,
  .small .item-icon-fallback {
    width: 28px;
    height: 20px;
    min-width: 28px;
  }

  .medium .item-icon,
  .medium .item-icon-fallback {
    width: 44px;
    height: 32px;
    min-width: 44px;
  }

  .large .item-icon,
  .large .item-icon-fallback {
    width: 64px;
    height: 46px;
    min-width: 64px;
  }

  .item-name {
    color: #e0e0e0;
    font-size: 0.9rem;
    white-space: nowrap;
  }
</style>
