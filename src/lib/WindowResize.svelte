<script>
  import { getCurrentWindow } from '@tauri-apps/api/window';

  const appWindow = getCurrentWindow();

  async function startResize(direction) {
    await appWindow.startResizeDragging(direction);
  }
</script>

<!-- Invisible resize handles around the window perimeter -->
<div class="resize-n"   onmousedown={() => startResize('North')}></div>
<div class="resize-s"   onmousedown={() => startResize('South')}></div>
<div class="resize-e"   onmousedown={() => startResize('East')}></div>
<div class="resize-w"   onmousedown={() => startResize('West')}></div>
<div class="resize-nw"  onmousedown={() => startResize('NorthWest')}></div>
<div class="resize-ne"  onmousedown={() => startResize('NorthEast')}></div>
<div class="resize-sw"  onmousedown={() => startResize('SouthWest')}></div>
<div class="resize-se"  onmousedown={() => startResize('SouthEast')}></div>

<style>
  /* All handles are fixed-position, zero-size in the non-resize axis,
     and sit on top of everything via z-index. */
  div[class^="resize-"] {
    position: fixed;
    z-index: 9999;
  }

  /* Edges */
  .resize-n  { top: 0; left: 4px; right: 4px; height: 4px; cursor: n-resize; }
  .resize-s  { bottom: 0; left: 4px; right: 4px; height: 4px; cursor: s-resize; }
  .resize-e  { right: 0; top: 4px; bottom: 4px; width: 4px; cursor: e-resize; }
  .resize-w  { left: 0; top: 4px; bottom: 4px; width: 4px; cursor: w-resize; }

  /* Corners */
  .resize-nw { top: 0; left: 0; width: 8px; height: 8px; cursor: nw-resize; }
  .resize-ne { top: 0; right: 0; width: 8px; height: 8px; cursor: ne-resize; }
  .resize-sw { bottom: 0; left: 0; width: 8px; height: 8px; cursor: sw-resize; }
  .resize-se { bottom: 0; right: 0; width: 8px; height: 8px; cursor: se-resize; }
</style>
