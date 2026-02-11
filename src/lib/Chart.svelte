<script>
  import { onMount, onDestroy } from 'svelte';
  import {
    Chart,
    LineController,
    LineElement,
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

  onMount(() => {
    if (canvas && config) {
      chart = new Chart(canvas, config);
    }
  });

  onDestroy(() => {
    if (chart) {
      chart.destroy();
    }
  });

  // Watch for config changes and update chart
  $effect(() => {
    if (chart && config) {
      chart.data = config.data;
      chart.options = config.options;
      chart.update();
    }
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
