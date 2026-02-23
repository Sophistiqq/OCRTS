<script lang="ts">
  import type { OutputCard } from "../../lib/types";
  import RegionResultComponent from "./RegionResult.svelte";

  export let card: OutputCard;

  let collapsed = false;

  function copyAll() {
    const text = card.results
      .map((r, i) => {
        const label = `[Region ${i + 1}]`;
        const body =
          r.columns.length > 0 && r.columns.some((row) => row.length > 1)
            ? r.columns.map((row) => row.join("\t")).join("\n")
            : r.text.trim();
        return `${label}\n${body}`;
      })
      .join("\n\n");
    navigator.clipboard.writeText(text);
  }
</script>

<div class="card">
  <div class="card-header">
    <button
      class="toggle"
      on:click={() => (collapsed = !collapsed)}
      aria-label="Toggle card"
    >
      <span class="arrow">{collapsed ? "▶" : "▼"}</span>
      <span class="filename">{card.imageName}</span>
      <span class="count"
        >{card.results.length} region{card.results.length > 1 ? "s" : ""}</span
      >
    </button>
    <button class="copy-all" on:click={copyAll}>Copy All</button>
  </div>

  {#if !collapsed}
    <div class="card-body">
      {#if card.results.length === 0}
        <p class="empty">No regions were processed for this image.</p>
      {:else}
        {#each card.results as result, i (result.regionId)}
          <RegionResultComponent {result} index={i} />
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
  .card {
    background: #111;
    border: 1px solid #2a2a2a;
    border-radius: 10px;
    overflow: hidden;
  }
  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.6rem 0.75rem;
    background: #161616;
    border-bottom: 1px solid #2a2a2a;
    gap: 0.5rem;
  }
  .toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: none;
    border: none;
    color: inherit;
    cursor: pointer;
    flex: 1;
    text-align: left;
    font-family: inherit;
    min-width: 0;
  }
  .arrow {
    font-size: 0.65rem;
    color: #666;
    flex-shrink: 0;
  }
  .filename {
    font-size: 0.875rem;
    font-weight: 600;
    color: #ddd;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .count {
    font-size: 0.72rem;
    color: #666;
    flex-shrink: 0;
  }
  .copy-all {
    background: none;
    border: 1px solid #333;
    color: #888;
    font-size: 0.72rem;
    padding: 0.25rem 0.6rem;
    border-radius: 4px;
    cursor: pointer;
    flex-shrink: 0;
    transition:
      color 0.1s,
      border-color 0.1s;
  }
  .copy-all:hover {
    color: #ddd;
    border-color: #555;
  }
  .card-body {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 0.75rem;
  }
  .empty {
    font-size: 0.82rem;
    color: #555;
    margin: 0;
  }
</style>
