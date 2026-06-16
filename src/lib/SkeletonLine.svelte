<script>
  /** @type {{ width?: string, height?: string }} */
  let { width = "100%", height = "12px" } = $props();
</script>

<div class="skeleton-line" style="width: {width}; height: {height}"></div>

<style>
  .skeleton-line {
    border-radius: 4px;
    background: linear-gradient(90deg, var(--bg-elevated) 25%, var(--border) 50%, var(--bg-elevated) 75%);
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
    flex-shrink: 0;
  }

  @keyframes shimmer {
    0%   { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }
</style>
