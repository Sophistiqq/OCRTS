<script lang="ts">
  import { onMount } from "svelte";
  import type { Region, ProcessingSettings } from "../../lib/types";
  import { loadImageFull, preprocessImage } from "../../lib/tauri"; // add this import

  export let imagePath: string; // real path, for coord scaling
  export let naturalWidth: number; // actual image pixel dimensions
  export let naturalHeight: number;
  export let rotation: number = 0;
  export let regions: Region[];
  export let highlightedId: string | null = null;
  export let processing: ProcessingSettings | undefined = undefined;

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

    // Use visual dimensions for aspect ratio
    const isVertical = rotation === 90 || rotation === 270;
    const visualW = isVertical ? naturalHeight : naturalWidth;
    const visualH = isVertical ? naturalWidth : naturalHeight;

    const imgRatio = visualW / visualH;
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

    scaleX = visualW / canvasW;
    scaleY = visualH / canvasH;

    redraw();
  }

  function redraw() {
    if (!ctx || !imgLoaded) return;
    ctx.clearRect(0, 0, canvasW, canvasH);

    ctx.save();
    ctx.translate(canvasW / 2, canvasH / 2);
    ctx.rotate((rotation * Math.PI) / 180);

    const isVertical = rotation === 90 || rotation === 270;
    const drawW = isVertical ? canvasH : canvasW;
    const drawH = isVertical ? canvasW : canvasH;

    ctx.drawImage(img, -drawW / 2, -drawH / 2, drawW, drawH);
    ctx.restore();

    drawRegions();
    if (drawState === "drawing") drawActiveRect();
  }

  function fromNatural(region: Region) {
    let x, y, w, h;
    if (rotation === 0) {
      ({ x, y, width: w, height: h } = region);
    } else if (rotation === 90) {
      x = naturalHeight - region.y - region.height;
      y = region.x;
      w = region.height;
      h = region.width;
    } else if (rotation === 180) {
      x = naturalWidth - region.x - region.width;
      y = naturalHeight - region.y - region.height;
      w = region.width;
      h = region.height;
    } else {
      // 270
      x = region.y;
      y = naturalWidth - region.x - region.width;
      w = region.height;
      h = region.width;
    }
    return { x, y, w, h };
  }

  function toNatural(rect: {
    x: number;
    y: number;
    w: number;
    h: number;
  }): Region {
    let x, y, w, h;
    if (rotation === 0) {
      ({ x, y, w, h } = rect);
    } else if (rotation === 90) {
      x = rect.y;
      y = naturalHeight - rect.x - rect.w;
      w = rect.h;
      h = rect.w;
    } else if (rotation === 180) {
      x = naturalWidth - rect.x - rect.w;
      y = naturalHeight - rect.y - rect.h;
      w = rect.w;
      h = rect.h;
    } else {
      // 270
      x = naturalWidth - rect.y - rect.h;
      y = rect.x;
      w = rect.h;
      h = rect.w;
    }
    return { id: crypto.randomUUID(), x, y, width: w, height: h };
  }

  function drawRegions() {
    for (const region of regions) {
      const isHighlighted = region.id === highlightedId;
      // Convert real coords back to canvas coords
      const visual = fromNatural(region);
      const rx = visual.x / scaleX;
      const ry = visual.y / scaleY;
      const rw = visual.w / scaleX;
      const rh = visual.h / scaleY;

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
    const region = toNatural({
      x: Math.round(x * scaleX),
      y: Math.round(y * scaleY),
      w: Math.round(w * scaleX),
      h: Math.round(h * scaleY),
    });

    onAddRegion(region);
    redraw();
  }

  // Keyboard: delete last region with Backspace/Delete
  function onKeyDown(e: KeyboardEvent) {
    if ((e.key === "Backspace" || e.key === "Delete") && regions.length > 0) {
      onRemoveRegion(regions[regions.length - 1].id);
    }
  }

  async function loadSource() {
    if (!imagePath) return;
    try {
      let src;
      if (processing) {
        src = await preprocessImage(imagePath, processing.blur, processing.threshold);
      } else {
        src = await loadImageFull(imagePath);
      }
      img.src = src;
    } catch (e) {
      loadError = String(e);
    }
  }

  $: if (img && imagePath) {
    loadSource();
  }

  // Reactive: redraw whenever regions, rotation or highlight changes
  $: if (ctx && imgLoaded) {
    rotation; // dummy access to ensure dependency
    computeLayout(); // rotation changes layout
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
