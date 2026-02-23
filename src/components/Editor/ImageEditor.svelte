<script lang="ts">
  import { onMount } from "svelte";
  import type { Region } from "../../lib/types";
  import { loadImageFull } from "../../lib/tauri"; // add this import

  export let imagePath: string; // real path, for coord scaling
  export let naturalWidth: number; // actual image pixel dimensions
  export let naturalHeight: number;
  export let regions: Region[];
  export let highlightedId: string | null = null;

  export let onAddRegion: (r: Region) => void;
  export let onRemoveRegion: (id: string) => void;

  let canvas: HTMLCanvasElement;
  let container: HTMLDivElement;
  let ctx: CanvasRenderingContext2D;
  let img: HTMLImageElement;
  let imgLoaded = false;

  // Drawing state
  type DrawState = "idle" | "drawing";
  let drawState: DrawState = "idle";
  let startX = 0,
    startY = 0; // canvas coords
  let currentX = 0,
    currentY = 0;

  // Scale factor: canvas display size vs real image size
  let scaleX = 1;
  let scaleY = 1;

  // Canvas display size (fitted to container)
  let canvasW = 0;
  let canvasH = 0;

  function computeLayout() {
    if (!container || !imgLoaded) return;
    const containerW = container.clientWidth;
    const containerH = container.clientHeight;
    const imgRatio = naturalWidth / naturalHeight;
    const containerRatio = containerW / containerH;

    if (imgRatio > containerRatio) {
      canvasW = containerW;
      canvasH = containerW / imgRatio;
    } else {
      canvasH = containerH;
      canvasW = containerH * imgRatio;
    }

    canvas.width = canvasW;
    canvas.height = canvasH;

    scaleX = naturalWidth / canvasW;
    scaleY = naturalHeight / canvasH;

    redraw();
  }

  function redraw() {
    if (!ctx || !imgLoaded) return;
    ctx.clearRect(0, 0, canvasW, canvasH);
    ctx.drawImage(img, 0, 0, canvasW, canvasH);
    drawRegions();
    if (drawState === "drawing") drawActiveRect();
  }

  function drawRegions() {
    for (const region of regions) {
      const isHighlighted = region.id === highlightedId;
      // Convert real coords back to canvas coords
      const rx = region.x / scaleX;
      const ry = region.y / scaleY;
      const rw = region.width / scaleX;
      const rh = region.height / scaleY;

      ctx.save();
      ctx.strokeStyle = isHighlighted ? "#f8c97c" : "#7c9ef8";
      ctx.lineWidth = isHighlighted ? 2.5 : 1.5;
      ctx.setLineDash(isHighlighted ? [] : [5, 3]);
      ctx.strokeRect(rx, ry, rw, rh);
      ctx.fillStyle = isHighlighted
        ? "rgba(248,201,124,0.12)"
        : "rgba(124,158,248,0.08)";
      ctx.fillRect(rx, ry, rw, rh);

      // Label badge
      const label = region.label?.trim() || `#${regions.indexOf(region) + 1}`;
      ctx.font = "11px system-ui, sans-serif";
      ctx.fillStyle = isHighlighted ? "#f8c97c" : "#7c9ef8";
      ctx.fillText(label, rx + 4, ry + 13);
      ctx.restore();
    }
  }

  function drawActiveRect() {
    const x = Math.min(startX, currentX);
    const y = Math.min(startY, currentY);
    const w = Math.abs(currentX - startX);
    const h = Math.abs(currentY - startY);
    ctx.save();
    ctx.strokeStyle = "#7c9ef8";
    ctx.lineWidth = 1.5;
    ctx.setLineDash([5, 3]);
    ctx.strokeRect(x, y, w, h);
    ctx.fillStyle = "rgba(124,158,248,0.1)";
    ctx.fillRect(x, y, w, h);
    ctx.restore();
  }

  function getCanvasPos(e: MouseEvent) {
    const rect = canvas.getBoundingClientRect();
    return {
      x: e.clientX - rect.left,
      y: e.clientY - rect.top,
    };
  }

  function onMouseDown(e: MouseEvent) {
    if (e.button !== 0) return;
    const pos = getCanvasPos(e);
    startX = pos.x;
    startY = pos.y;
    currentX = pos.x;
    currentY = pos.y;
    drawState = "drawing";
  }

  function onMouseMove(e: MouseEvent) {
    if (drawState !== "drawing") return;
    const pos = getCanvasPos(e);
    currentX = pos.x;
    currentY = pos.y;
    redraw();
  }

  function onMouseUp(e: MouseEvent) {
    if (drawState !== "drawing") return;
    drawState = "idle";

    const x = Math.min(startX, currentX);
    const y = Math.min(startY, currentY);
    const w = Math.abs(currentX - startX);
    const h = Math.abs(currentY - startY);

    // Ignore tiny accidental clicks
    if (w < 8 || h < 8) {
      redraw();
      return;
    }

    // Convert canvas coords → real image coords
    const region: Region = {
      id: crypto.randomUUID(),
      x: Math.round(x * scaleX),
      y: Math.round(y * scaleY),
      width: Math.round(w * scaleX),
      height: Math.round(h * scaleY),
    };

    onAddRegion(region);
    redraw();
  }

  // Keyboard: delete last region with Backspace/Delete
  function onKeyDown(e: KeyboardEvent) {
    if ((e.key === "Backspace" || e.key === "Delete") && regions.length > 0) {
      onRemoveRegion(regions[regions.length - 1].id);
    }
  }

  // Reactive: redraw whenever regions or highlight changes
  $: if (ctx && imgLoaded) {
    redraw();
  }
  $: highlightedId, imgLoaded && redraw();
  let loadError: string | null = null;
  // Change onMount to fetch full image:
  onMount(() => {
    ctx = canvas.getContext("2d")!;
    img = new Image();
    img.onload = () => {
      imgLoaded = true;
      computeLayout();
    };
    img.onerror = () => {
      loadError = "Failed to load image";
    };

    // Fetch full resolution image from Rust
    loadImageFull(imagePath)
      .then((src) => {
        img.src = src;
      })
      .catch((e) => {
        loadError = String(e);
      });

    const ro = new ResizeObserver(() => computeLayout());
    ro.observe(container);
    return () => ro.disconnect();
  });
</script>

<svelte:window on:keydown={onKeyDown} />

<div class="container" bind:this={container}>
  {#if loadError}
    <p class="error">{loadError}</p>
  {:else if !imgLoaded}
    <p class="loading">Loading image…</p>
  {/if}
  <canvas
    bind:this={canvas}
    style="cursor: crosshair; display: {imgLoaded ? 'block' : 'none'};"
    on:mousedown={onMouseDown}
    on:mousemove={onMouseMove}
    on:mouseup={onMouseUp}
  ></canvas>
</div>

<style>
  .container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #0d0d0d;
    overflow: hidden;
  }
  .loading {
    color: #555;
    font-size: 0.85rem;
  }
  canvas {
    display: block;
    /* shadow so image edges are visible on dark bg */
    box-shadow: 0 0 0 1px #2a2a2a;
  }
  .error {
    color: #f87c7c;
    font-size: 0.85rem;
  }
</style>
