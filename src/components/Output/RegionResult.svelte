<script lang="ts">
  import type { RegionResult } from "../../lib/types";

  export let result: RegionResult;
  export let index: number;

  $: hasColumns =
    result.columns.length > 0 && result.columns.some((row) => row.length > 1);
  $: columnCount = hasColumns
    ? Math.max(...result.columns.map((r) => r.length))
    : 0;

  let copied = false;

  function copyText() {
    navigator.clipboard.writeText(result.text);
    copied = true;
    setTimeout(() => (copied = false), 1500);
  }
</script>

<div class="region">
  <div class="region-header">
    <span class="region-label">
      #{index + 1}{result.regionId ? "" : ""}
    </span>
    <button class="copy-btn" on:click={copyText}>
      {copied ? "✓ Copied" : "Copy"}
    </button>
  </div>

  {#if hasColumns}
    <div class="table-wrap">
      <table>
        {#each result.columns as row}
          <tr>
            {#each Array(columnCount) as _, ci}
              <td>{row[ci] ?? ""}</td>
            {/each}
          </tr>
        {/each}
      </table>
    </div>
  {:else if result.text.trim()}
    <pre class="plain-text">{result.text.trim()}</pre>
  {:else}
    <p class="empty">No text detected in this region.</p>
  {/if}
</div>

<style>
  .region {
    border: 1px solid #2a2a2a;
    border-radius: 8px;
    overflow: hidden;
  }
  .region-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.4rem 0.75rem;
    background: #1a1a1a;
    border-bottom: 1px solid #2a2a2a;
  }
  .region-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: #7c9ef8;
  }
  .copy-btn {
    background: none;
    border: 1px solid #333;
    color: #888;
    font-size: 0.72rem;
    padding: 0.2rem 0.5rem;
    border-radius: 4px;
    cursor: pointer;
    transition:
      color 0.1s,
      border-color 0.1s;
  }
  .copy-btn:hover {
    color: #ddd;
    border-color: #555;
  }

  /* Table */
  .table-wrap {
    overflow-x: auto;
    padding: 0.5rem;
  }
  table {
    border-collapse: collapse;
    width: 100%;
    font-size: 0.82rem;
  }
  td {
    padding: 0.3rem 0.6rem;
    border: 1px solid #2a2a2a;
    color: #ccc;
    white-space: nowrap;
    vertical-align: top;
  }
  tr:nth-child(even) td {
    background: #111;
  }

  /* Plain text */
  .plain-text {
    margin: 0;
    padding: 0.6rem 0.75rem;
    font-size: 0.82rem;
    color: #ccc;
    white-space: pre-wrap;
    word-break: break-word;
    font-family: "Courier New", monospace;
    line-height: 1.5;
  }

  .empty {
    margin: 0;
    padding: 0.6rem 0.75rem;
    font-size: 0.8rem;
    color: #555;
  }
</style>
