<script lang="ts">
  import type { OutputCard } from "../../lib/types";
  import RegionResultView from "./RegionResult.svelte";

  export let card: OutputCard;

  function copyCard() {
    const text = card.results
      .map(r => r.columns.map(row => row.map(c => c.text).join("\t")).join("\n"))
      .join("\n\n");
    navigator.clipboard.writeText(text);
  }
</script>

<div class="card">
  <header>
    <div class="info">
      <h3>{card.imageName}</h3>
      <span class="count">{card.results.length} region(s)</span>
    </div>
    <button class="copy-btn" on:click={copyCard}>
      Copy Result
    </button>
  </header>

  <div class="results">
    {#each card.results as result}
      <div class="result-item">
        <p class="label">{result.text.split('\n')[0].substring(0, 30)}...</p>
        <RegionResultView {result} cardId={card.imageId} />
      </div>
    {/each}
  </div>
</div>

<style>
  .card {
    background: #111;
    border: 1px solid #222;
    border-radius: 10px;
    padding: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }
  h3 {
    font-size: 1rem;
    margin: 0;
    color: #fff;
  }
  .count {
    font-size: 0.75rem;
    color: #666;
  }
  .copy-btn {
    background: #2a2a2a;
    color: #7c9ef8;
    border: 1px solid #333;
    padding: 0.4rem 0.8rem;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.1s;
  }
  .copy-btn:hover {
    background: #333;
    border-color: #7c9ef8;
  }
  .results {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  .label {
    font-size: 0.7rem;
    color: #444;
    margin: 0 0 0.25rem;
    font-family: monospace;
  }
</style>
