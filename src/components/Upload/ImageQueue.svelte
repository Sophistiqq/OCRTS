<script lang="ts">
  import {
    imageQueue,
    removeImage,
    reorderImages,
  } from "../../lib/stores/images";

  let dragFromIndex: number | null = null;

  function onDragStart(index: number) {
    dragFromIndex = index;
  }

  function onDropItem(toIndex: number) {
    if (dragFromIndex !== null && dragFromIndex !== toIndex) {
      reorderImages(dragFromIndex, toIndex);
    }
    dragFromIndex = null;
  }
</script>

{#if $imageQueue.length > 0}
  <div class="queue">
    <p class="queue-label">
      {$imageQueue.length} image{$imageQueue.length > 1 ? "s" : ""} queued
    </p>
    <ul class="list">
      {#each $imageQueue as img, i (img.id)}
        <li
          class="item"
          draggable="true"
          on:dragstart={() => onDragStart(i)}
          on:dragover={(e) => e.preventDefault()}
          on:drop={() => onDropItem(i)}
        >
          <img class="thumb" src={img.thumbnail} alt={img.name} />
          <div class="meta">
            <span class="name">{img.name}</span>
            <span class="dims">{img.width} × {img.height}</span>
          </div>
          <button
            class="remove"
            on:click={() => removeImage(img.id)}
            aria-label="Remove">✕</button
          >
        </li>
      {/each}
    </ul>
  </div>
{/if}

<style>
  .queue {
    margin-top: 1.5rem;
  }
  .queue-label {
    font-size: 0.8rem;
    color: #888;
    margin-bottom: 0.5rem;
  }
  .list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0.75rem;
    background: #1e1e1e;
    border-radius: 8px;
    cursor: grab;
  }
  .item:active {
    cursor: grabbing;
  }
  .thumb {
    width: 48px;
    height: 48px;
    object-fit: cover;
    border-radius: 4px;
  }
  .meta {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
  }
  .name {
    font-size: 0.875rem;
    font-weight: 500;
  }
  .dims {
    font-size: 0.75rem;
    color: #888;
  }
  .remove {
    background: none;
    border: none;
    color: #888;
    cursor: pointer;
    font-size: 0.9rem;
    padding: 0.25rem 0.4rem;
    border-radius: 4px;
    transition:
      color 0.1s,
      background 0.1s;
  }
  .remove:hover {
    color: #f87c7c;
    background: rgba(248, 124, 124, 0.1);
  }
</style>
