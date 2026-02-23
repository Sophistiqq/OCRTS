<script lang="ts">
  import ImageEditor from "../components/Editor/ImageEditor.svelte";
  import SelectionList from "../components/Editor/SelectionList.svelte";
  import {
    imageQueue,
    currentIndex,
    regionMap,
    addRegion,
    removeRegion,
    setRegions,
  } from "../lib/stores/images";
  import { addCard } from "../lib/stores/results";
  import { processRegion } from "../lib/tauri";
  import type { Region, OutputCard } from "../lib/types";
  import { get } from "svelte/store";

  export let onBack: () => void;
  export let onFinish: () => void;

  let highlightedId: string | null = null;
  let processing = false;
  let processingError: string | null = null;

  $: images = $imageQueue;
  $: index = $currentIndex;
  $: image = images[index] ?? null;
  $: regions = image ? ($regionMap[image.id] ?? []) : [];
  $: isLast = index === images.length - 1;

  function handleAddRegion(region: Region) {
    if (!image) return;
    addRegion(image.id, region);
  }

  function handleRemoveRegion(id: string) {
    if (!image) return;
    removeRegion(image.id, id);
  }

  function handleLabelChange(id: string, label: string) {
    if (!image) return;
    const updated = regions.map((r) => (r.id === id ? { ...r, label } : r));
    setRegions(image.id, updated);
  }

  async function processCurrentAndAdvance() {
    if (!image || regions.length === 0) {
      advance();
      return;
    }

    processing = true;
    processingError = null;

    try {
      const results = await Promise.all(
        regions.map((r) => processRegion(image.id, r)),
      );

      const card: OutputCard = {
        imageId: image.id,
        imageName: image.name,
        results,
      };

      addCard(card);
      advance();
    } catch (e) {
      processingError = String(e);
    } finally {
      processing = false;
    }
  }

  function advance() {
    if (isLast) {
      onFinish();
    } else {
      currentIndex.update((i) => i + 1);
    }
  }

  function goBack() {
    if (index > 0) {
      currentIndex.update((i) => i - 1);
    } else {
      onBack();
    }
  }
</script>

<div class="view">
  <!-- Top bar -->
  <header class="topbar">
    <button class="nav-btn" on:click={goBack}>
      ← {index === 0 ? "Upload" : "Previous"}
    </button>

    <div class="progress">
      <span class="image-name">{image?.name ?? ""}</span>
      <span class="counter">{index + 1} / {images.length}</span>
    </div>

    <button
      class="proceed-btn"
      class:loading={processing}
      disabled={processing}
      on:click={processCurrentAndAdvance}
    >
      {#if processing}
        Processing…
      {:else if regions.length === 0}
        Skip →
      {:else if isLast}
        Process & Finish ✓
      {:else}
        Process & Next →
      {/if}
    </button>
  </header>

  <!-- Error banner -->
  {#if processingError}
    <div class="error-banner">
      OCR failed: {processingError}
      <button on:click={() => (processingError = null)}>✕</button>
    </div>
  {/if}

  <!-- Main area -->
  <div class="workspace">
    <div class="canvas-area">
      {#if image}
        {#key image.id}
          <ImageEditor
            imagePath={image.path}
            naturalWidth={image.width}
            naturalHeight={image.height}
            {regions}
            {highlightedId}
            onAddRegion={handleAddRegion}
            onRemoveRegion={handleRemoveRegion}
          />
        {/key}
      {/if}
    </div>

    <aside class="sidebar">
      <SelectionList
        {regions}
        onRemove={handleRemoveRegion}
        onLabelChange={handleLabelChange}
        onHighlight={(id) => (highlightedId = id)}
      />
    </aside>
  </div>

  <!-- Bottom hint -->
  <div class="hint">
    Click and drag on the image to select a region · Backspace removes last
    selection
    {#if regions.length > 0}
      · <strong>{regions.length}</strong> region{regions.length > 1 ? "s" : ""} selected
    {/if}
  </div>
</div>

<style>
  .view {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #0d0d0d;
    color: #ddd;
    font-family: system-ui, sans-serif;
    overflow: hidden;
  }

  /* Top bar */
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.6rem 1rem;
    background: #111;
    border-bottom: 1px solid #222;
    gap: 1rem;
    flex-shrink: 0;
  }
  .nav-btn {
    background: none;
    border: 1px solid #333;
    color: #aaa;
    padding: 0.4rem 0.85rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.85rem;
    white-space: nowrap;
    transition:
      border-color 0.15s,
      color 0.15s;
  }
  .nav-btn:hover {
    border-color: #555;
    color: #ddd;
  }
  .progress {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.1rem;
    flex: 1;
    min-width: 0;
  }
  .image-name {
    font-size: 0.82rem;
    font-weight: 500;
    color: #ccc;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 300px;
  }
  .counter {
    font-size: 0.72rem;
    color: #666;
  }
  .proceed-btn {
    background: #7c9ef8;
    color: #000;
    border: none;
    padding: 0.4rem 1rem;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
    transition:
      background 0.15s,
      opacity 0.15s;
  }
  .proceed-btn:hover:not(:disabled) {
    background: #6389f5;
  }
  .proceed-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .proceed-btn.loading {
    background: #444;
    color: #888;
  }

  /* Error */
  .error-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 1rem;
    background: rgba(248, 124, 124, 0.12);
    border-bottom: 1px solid rgba(248, 124, 124, 0.3);
    color: #f87c7c;
    font-size: 0.82rem;
    flex-shrink: 0;
  }
  .error-banner button {
    background: none;
    border: none;
    color: #f87c7c;
    cursor: pointer;
  }

  /* Workspace */
  .workspace {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
  .canvas-area {
    flex: 1;
    overflow: hidden;
    position: relative;
  }
  .sidebar {
    width: 220px;
    flex-shrink: 0;
    overflow-y: auto;
  }

  /* Bottom hint */
  .hint {
    padding: 0.35rem 1rem;
    font-size: 0.72rem;
    color: #555;
    background: #0d0d0d;
    border-top: 1px solid #1a1a1a;
    flex-shrink: 0;
  }
  .hint strong {
    color: #7c9ef8;
  }
</style>
