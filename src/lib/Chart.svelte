<script>
  import { onDestroy } from 'svelte';
  import {
    Chart,
    LineController,
    LineElement,
    BarController,
    BarElement,
    PointElement,
    LinearScale,
    CategoryScale,
    Title,
    Tooltip,
    Legend,
    Filler
  } from 'chart.js';

  // Register Chart.js components
  Chart.register(
    LineController,
    LineElement,
    BarController,
    BarElement,
    PointElement,
    LinearScale,
    CategoryScale,
    Title,
    Tooltip,
    Legend,
    Filler
  );

  let { config, height = '300px' } = $props();

  let canvas;
  let chart;

  onDestroy(() => {
    if (chart) {
      chart.destroy();
    }
  });

  // Create or recreate chart whenever config changes.
  // Recreating ensures custom plugins always have fresh closure values.
  // Also handles the case where config is null on first render (async data load).
  $effect(() => {
    if (!canvas || !config) return;
    if (chart) {
      chart.destroy();
    }
    chart = new Chart(canvas, config);
  });
</script>

<div class="chart-wrapper" style="height: {height}">
  <canvas bind:this={canvas}></canvas>
</div>

<style>
  .chart-wrapper {
    position: relative;
    width: 100%;
  }

  canvas {
    max-height: 100%;
  }
</style>
