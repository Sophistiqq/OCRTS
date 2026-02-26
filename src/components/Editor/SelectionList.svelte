<script lang="ts">
  import type { Region } from "../../lib/types";

  export let regions: Region[] = [];
  export let onRemove: (id: string) => void;
  export let onLabelChange: (id: string, label: string) => void;
  export let onHighlight: (id: string | null) => void;
  export let onNumericToggle: (id: string, val: boolean) => void;
</script>

<div class="panel">
  <p class="panel-title">Selections</p>

  {#if regions.length === 0}
    <p class="empty">Draw rectangles on the image to select regions for OCR.</p>
  {:else}
    <ul class="list">
      {#each regions as region, i (region.id)}
        <li
          class="item"
          on:mouseenter={() => onHighlight(region.id)}
          on:mouseleave={() => onHighlight(null)}
        >
          <span class="index">#{i + 1}</span>
          <input
            class="label-input"
            type="text"
            placeholder="Label (optional)"
            value={region.label ?? ""}
            on:input={(e) =>
              onLabelChange(region.id, (e.target as HTMLInputElement).value)}
          />
          <button
            class="numeric-toggle"
            class:active={region.is_numeric}
            on:click={() => onNumericToggle(region.id, !region.is_numeric)}
            title="Numbers Only Mode"
          >
            123
          </button>
          <button
            class="remove"
            on:click={() => onRemove(region.id)}
            aria-label="Remove region">✕</button
          >
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 1rem;
    box-sizing: border-box;
    border-left: 1px solid #2a2a2a;
    background: #111;
  }
  .panel-title {
    font-size: 0.78rem;
    font-weight: 600;
    color: #888;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    margin: 0 0 0.75rem;
  }
  .empty {
    font-size: 0.82rem;
    color: #555;
    line-height: 1.5;
    margin: 0;
  }
  .list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    overflow-y: auto;
  }
  .item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.45rem 0.5rem;
    background: #1a1a1a;
    border: 1px solid #2a2a2a;
    border-radius: 6px;
    transition: border-color 0.15s;
  }
  .item:hover {
    border-color: #7c9ef8;
  }
  .index {
    font-size: 0.72rem;
    font-weight: 700;
    color: #7c9ef8;
    min-width: 1.5rem;
  }
  .label-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: #ddd;
    font-size: 0.82rem;
    font-family: inherit;
    min-width: 0;
  }
  .label-input::placeholder {
    color: #444;
  }
  .numeric-toggle {
    background: #2a2a2a;
    border: 1px solid #333;
    color: #666;
    font-size: 0.65rem;
    font-weight: 700;
    padding: 0.2rem 0.4rem;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.1s;
  }
  .numeric-toggle.active {
    background: rgba(124, 158, 248, 0.2);
    border-color: #7c9ef8;
    color: #7c9ef8;
  }
  .remove {
    background: none;
    border: none;
    color: #555;
    cursor: pointer;
    font-size: 0.8rem;
    padding: 0.2rem 0.3rem;
    border-radius: 4px;
    flex-shrink: 0;
    transition:
      color 0.1s,
      background 0.1s;
  }
  .remove:hover {
    color: #f87c7c;
    background: rgba(248, 124, 124, 0.1);
  }
</style>
