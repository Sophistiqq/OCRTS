<script lang="ts">
  import DropZone from "../components/Upload/DropZone.svelte";
  import ImageQueue from "../components/Upload/ImageQueue.svelte";
  import { imageQueue } from "../lib/stores/images";
  import { currentIndex } from "../lib/stores/images";

  export let onProceed: () => void;
</script>

<div class="view">
  <header>
    <h1>OCR Sheet</h1>
    <p class="subtitle">Extract text from printed spreadsheets</p>
  </header>

  <main>
    <DropZone />
    <ImageQueue />
  </main>

  {#if $imageQueue.length > 0}
    <footer>
      <button
        class="proceed"
        on:click={() => {
          currentIndex.set(0);
          onProceed();
        }}
      >
        Start selecting regions →
      </button>
    </footer>
  {/if}
</div>

<style>
  .view {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding: 2rem;
    max-width: 640px;
    margin: 0 auto;
    box-sizing: border-box;
  }
  header {
    margin-bottom: 2rem;
  }
  h1 {
    font-size: 1.5rem;
    font-weight: 700;
    margin: 0;
  }
  .subtitle {
    color: #888;
    margin: 0.25rem 0 0;
    font-size: 0.9rem;
  }
  main {
    flex: 1;
  }
  footer {
    padding-top: 1.5rem;
  }
  .proceed {
    width: 100%;
    padding: 0.85rem;
    background: #7c9ef8;
    color: #000;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s;
  }
  .proceed:hover {
    background: #5f84f5;
  }
</style>
