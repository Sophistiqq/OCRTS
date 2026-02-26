<script lang="ts">
  import type { RegionResult, OcrCell } from "../../lib/types";
  import { outputCards } from "../../lib/stores/results";

  export let result: RegionResult;
  export let cardId: string;

  function handleInput(cell: OcrCell) {
    if (!cell.manual) {
      cell.manual = true;
      // Trigger a local re-render to update the background color
      result = result;
    }
  }

  function handleBlur() {
    // Final sync to store when focus is lost. 
    // This allows the browser to keep its own undo/redo history while focused.
    outputCards.update(cards => [...cards]);
  }

  function getCellColor(cell: OcrCell) {
    if (cell.manual) return "rgba(124, 158, 248, 0.25)"; 
    
    const conf = cell.confidence;
    if (conf >= 90) return "transparent";
    if (conf >= 75) return "rgba(248, 201, 124, 0.2)";
    if (conf >= 50) return "rgba(248, 150, 76, 0.25)"; 
    return "rgba(248, 124, 124, 0.3)";
  }
</script>

<div class="region-result">
  <div class="table-container">
    <table>
      <tbody>
        {#each result.columns as row, rowIndex}
          <tr>
            {#each row as cell, colIndex}
              <td style="background: {getCellColor(cell)}">
                <div class="cell-wrapper">
                  <input
                    type="text"
                    bind:value={cell.text}
                    on:input={() => handleInput(cell)}
                    on:blur={handleBlur}
                    class:low-conf={cell.confidence < 75 && !cell.manual}
                    class:edited={cell.manual}
                  />
                  {#if cell.manual && cell.text !== cell.originalText}
                    <div class="original-tooltip">
                      <span class="tooltip-label">Before:</span>
                      <span class="tooltip-value">{cell.originalText || '(empty)'}</span>
                    </div>
                  {/if}
                </div>
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<style>
  .region-result {
    margin-top: 0.5rem;
  }
  .table-container {
    overflow-x: auto;
    border-radius: 4px;
    border: 1px solid #333;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.85rem;
  }
  td {
    border: 1px solid #222;
    padding: 0;
    transition: background 0.2s;
    position: relative;
  }
  .cell-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
  }
  input {
    width: 100%;
    background: transparent;
    border: none;
    color: #ddd;
    padding: 0.4rem 0.6rem;
    font-family: inherit;
    font-size: inherit;
    outline: none;
    min-width: 100px;
    display: block;
    box-sizing: border-box;
  }
  input:focus {
    background: rgba(255, 255, 255, 0.05);
  }

  /* Original value tooltip on hover */
  .original-tooltip {
    position: absolute;
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%) translateY(-4px);
    background: #222;
    color: #fff;
    padding: 0.3rem 0.6rem;
    border-radius: 4px;
    font-size: 0.7rem;
    white-space: nowrap;
    pointer-events: none;
    opacity: 0;
    transition: opacity 0.15s, transform 0.15s;
    box-shadow: 0 4px 12px rgba(0,0,0,0.5);
    z-index: 100;
    border: 1px solid #444;
  }
  .cell-wrapper:hover .original-tooltip {
    opacity: 1;
    transform: translateX(-50%) translateY(-8px);
  }
  .tooltip-label {
    color: #888;
    margin-right: 0.4rem;
    font-weight: 600;
  }
  .tooltip-value {
    color: #f8c97c;
    font-family: monospace;
  }

  input.low-conf {
    color: #fff;
  }
  input.edited {
    color: #7c9ef8;
    font-weight: 600;
  }
</style>
