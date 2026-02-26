<script lang="ts">
  import { outputCards, clearResults } from "../lib/stores/results";
  import { imageQueue, regionMap, currentIndex } from "../lib/stores/images";
  import ResultCard from "../components/Output/ResultCard.svelte";
  import { save } from "@tauri-apps/plugin-dialog";
  import { writeTextFile } from "@tauri-apps/plugin-fs";
  export let onBack: () => void;

  function startOver() {
    if (confirm("Clear all results and start over?")) {
      clearResults();
      imageQueue.set([]);
      regionMap.set({});
      currentIndex.set(0);
      onBack();
    }
  }

  function copyAll() {
    const text = $outputCards
      .map(card => 
        card.imageName + "\n" +
        card.results.map(r => 
          r.columns.map(row => row.map(c => c.text).join("\t")).join("\n")
        ).join("\n")
      ).join("\n\n---\n\n");
    navigator.clipboard.writeText(text);
  }

  async function saveAsText() {
    const path = await save({
      filters: [{ name: "Text", extensions: ["txt"] }],
      defaultPath: "ocr-results.txt",
    });
    if (!path) return;

    const lines: string[] = [];
    for (const card of $outputCards) {
      lines.push(`=== ${card.imageName} ===`);
      for (const r of card.results) {
        lines.push(r.columns.map(row => row.map(c => c.text).join("\t")).join("\n"));
        lines.push("");
      }
    }
    await writeTextFile(path, lines.join("\n"));
  }

  async function saveAsCsv() {
    const path = await save({
      filters: [{ name: "CSV", extensions: ["csv"] }],
      defaultPath: "ocr-results.csv",
    });
    if (!path) return;

    const lines: string[] = [];
    for (const card of $outputCards) {
      for (const r of card.results) {
        for (const row of r.columns) {
          lines.push(row.map(cell => `"${cell.text.replace(/"/g, '""')}"`).join(","));
        }
      }
    }
    await writeTextFile(path, lines.join("\n"));
  }
</script>

<div class="view">
  <header class="topbar">
    <button class="nav-btn" on:click={onBack}>← Back to Editor</button>
    <span class="title">Extraction Results</span>
    <button class="copy-all-btn" on:click={copyAll}>Copy All Results</button>
  </header>

  <main class="content">
    {#if $outputCards.length === 0}
      <div class="empty-state">
        <p>No results yet.</p>
        <button class="nav-btn" on:click={onBack}>← Go back</button>
      </div>
    {:else}
      <div class="cards">
        {#each $outputCards as card (card.imageId)}
          <ResultCard {card} />
        {/each}
        
        <div class="footer-actions">
          <button class="secondary-btn" on:click={startOver}>Start Over</button>
          <div class="export-group">
            <button class="secondary-btn" on:click={saveAsText}>Export .txt</button>
            <button class="secondary-btn" on:click={saveAsCsv}>Export .csv</button>
          </div>
        </div>
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
  }
  .copy-all-btn {
    background: #7c9ef8;
    color: #000;
    border: none;
    padding: 0.4rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 600;
  }
  .content {
    flex: 1;
    overflow-y: auto;
    padding: 1.25rem;
  }
  .cards {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 860px;
    margin: 0 auto;
    padding-bottom: 3rem;
  }
  .footer-actions {
    margin-top: 2rem;
    padding-top: 2rem;
    border-top: 1px solid #222;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .export-group {
    display: flex;
    gap: 0.5rem;
  }
  .secondary-btn {
    background: #1a1a1a;
    border: 1px solid #333;
    color: #888;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.85rem;
  }
  .secondary-btn:hover {
    border-color: #444;
    color: #ddd;
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
