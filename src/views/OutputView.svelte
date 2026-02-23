<script lang="ts">
  import { outputCards, clearResults } from "../lib/stores/results";
  import { imageQueue, regionMap, currentIndex } from "../lib/stores/images";
  import ResultCard from "../components/Output/ResultCard.svelte";

  export let onBack: () => void;

  function startOver() {
    clearResults();
    imageQueue.set([]);
    regionMap.set({});
    currentIndex.set(0);
    onBack();
  }

  function saveAsText() {
    const lines: string[] = [];
    for (const card of $outputCards) {
      lines.push(`=== ${card.imageName} ===`);
      for (let i = 0; i < card.results.length; i++) {
        const r = card.results[i];
        lines.push(`\n[Region ${i + 1}]`);
        const hasColumns =
          r.columns.length > 0 && r.columns.some((row) => row.length > 1);
        if (hasColumns) {
          lines.push(r.columns.map((row) => row.join("\t")).join("\n"));
        } else {
          lines.push(r.text.trim());
        }
      }
      lines.push("");
    }

    const blob = new Blob([lines.join("\n")], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "ocr-results.txt";
    a.click();
    URL.revokeObjectURL(url);
  }

  function saveAsCsv() {
    const lines: string[] = [];
    for (const card of $outputCards) {
      lines.push(`"${card.imageName}"`);
      for (let i = 0; i < card.results.length; i++) {
        const r = card.results[i];
        lines.push(`Region ${i + 1}`);
        const hasColumns =
          r.columns.length > 0 && r.columns.some((row) => row.length > 1);
        if (hasColumns) {
          for (const row of r.columns) {
            lines.push(
              row.map((cell) => `"${cell.replace(/"/g, '""')}"`).join(","),
            );
          }
        } else {
          lines.push(`"${r.text.trim().replace(/"/g, '""')}"`);
        }
        lines.push("");
      }
    }

    const blob = new Blob([lines.join("\n")], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "ocr-results.csv";
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="view">
  <header class="topbar">
    <button class="nav-btn" on:click={startOver}>← Start Over</button>
    <span class="title">Results</span>
    <div class="actions">
      <button class="action-btn" on:click={saveAsText}>Save .txt</button>
      <button class="action-btn" on:click={saveAsCsv}>Save .csv</button>
    </div>
  </header>

  <main class="content">
    {#if $outputCards.length === 0}
      <div class="empty-state">
        <p>No results yet.</p>
        <button class="nav-btn" on:click={startOver}>← Go back</button>
      </div>
    {:else}
      <div class="cards">
        {#each $outputCards as card (card.imageId)}
          <ResultCard {card} />
        {/each}
      </div>
    {/if}
  </main>
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
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.6rem 1rem;
    background: #111;
    border-bottom: 1px solid #222;
    flex-shrink: 0;
    gap: 1rem;
  }
  .title {
    font-size: 0.95rem;
    font-weight: 600;
    color: #ccc;
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
  .actions {
    display: flex;
    gap: 0.5rem;
  }
  .action-btn {
    background: none;
    border: 1px solid #333;
    color: #aaa;
    padding: 0.4rem 0.75rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.82rem;
    transition:
      border-color 0.15s,
      color 0.15s;
  }
  .action-btn:hover {
    border-color: #7c9ef8;
    color: #7c9ef8;
  }
  .content {
    flex: 1;
    overflow-y: auto;
    padding: 1.25rem;
  }
  .cards {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 860px;
    margin: 0 auto;
  }
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 1rem;
    color: #555;
  }
</style>
