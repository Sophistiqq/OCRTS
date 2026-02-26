<script lang="ts">
  import { onMount } from "svelte";
  import { imageQueue, currentIndex, updateProcessingSettings, rotateImage } from "../lib/stores/images";
  import { preprocessImage } from "../lib/tauri";

  export let onProceed: () => void;
  export let onBack: () => void;

  // Defaults: Off
  let blur = 0;
  let threshold = -2; // -2 for Disabled
  let originalUrl = "";
  let processedUrl = "";
  let loading = false;
  let error = "";
  
  // Track what was actually processed to detect "stale" preview
  let lastProcessedParams = { blur: -1, threshold: -99, rotation: -1 };

  $: currentImg = $imageQueue[$currentIndex];

  // Only reset when the image ID changes
  let lastId = "";
  $: if (currentImg && currentImg.id !== lastId) {
    lastId = currentImg.id;
    if (currentImg.processing) {
      blur = currentImg.processing.blur;
      threshold = currentImg.processing.threshold;
    } else {
      blur = 0;
      threshold = -2;
    }
    processedUrl = "";
    originalUrl = "";
    error = "";
    lastProcessedParams = { blur: -1, threshold: -99, rotation: -1 };
    
    // Load full resolution original
    import("../lib/tauri").then(m => m.loadImageFull(currentImg.path)).then(url => {
      originalUrl = url;
    });
  }

  // Detect if current preview matches current settings + rotation
  $: isStale = processedUrl !== "" && (
    blur !== lastProcessedParams.blur || 
    threshold !== lastProcessedParams.threshold ||
    currentImg?.rotation !== lastProcessedParams.rotation
  );

  async function runProcessing() {
    if (!currentImg) return;
    loading = true;
    error = "";
    try {
      // Note: preprocessImage in Rust doesn't handle rotation currently, 
      // it works on the raw file. But we want the preview to show it.
      // The OCR command handles rotation. 
      // For the PREVIEW here, we will apply CSS rotation to the images.
      const result = await preprocessImage(currentImg.path, blur, threshold);
      processedUrl = result;
      lastProcessedParams = { blur, threshold, rotation: currentImg.rotation };
      updateProcessingSettings(currentImg.id, blur, threshold);
    } catch (e) {
      error = String(e);
      console.error(e);
    } finally {
      loading = false;
    }
  }

  function handleNext() {
    if (currentImg) {
      updateProcessingSettings(currentImg.id, blur, threshold);
    }
    if ($currentIndex < $imageQueue.length - 1) {
      currentIndex.update(n => n + 1);
    } else {
      onProceed();
    }
  }

  function handlePrev() {
    if ($currentIndex > 0) {
      currentIndex.update(n => n - 1);
    } else {
      onBack();
    }
  }

  function skipAll() {
    $imageQueue.forEach(img => {
      updateProcessingSettings(img.id, 0, -2);
    });
    onProceed();
  }
</script>

<div class="view">
  <header>
    <div class="header-left">
      <button class="nav-btn" on:click={handlePrev}>← Back</button>
      <div class="titles">
        <h1>Image Processing</h1>
        <p class="subtitle">Clean up noise and adjust orientation</p>
      </div>
    </div>

    <div class="header-center">
      <span class="img-info">Image {$currentIndex + 1} of {$imageQueue.length}</span>
      <div class="rotation-controls">
        <button class="icon-btn" title="Rotate Left" on:click={() => rotateImage(currentImg.id, -90)}>↺</button>
        <span class="rot-val">{currentImg?.rotation}°</span>
        <button class="icon-btn" title="Rotate Right" on:click={() => rotateImage(currentImg.id, 90)}>↻</button>
      </div>
    </div>

    <div class="header-right">
      <button class="skip-btn" on:click={skipAll}>Skip All</button>
      <button class="proceed-btn" on:click={handleNext}>
        {$currentIndex === $imageQueue.length - 1 ? "Finish" : "Next"} →
      </button>
    </div>
  </header>

  <main>
    <div class="preview-container">
      <div class="preview-box">
        <span class="label">Original</span>
        {#if originalUrl}
          <div class="img-wrapper" style="transform: rotate({currentImg.rotation}deg)">
            <img src={originalUrl} alt="Original" />
          </div>
        {:else}
          <div class="spinner"></div>
        {/if}
      </div>
      <div class="preview-box" class:stale={isStale}>
        <span class="label">Processed {isStale ? '(Outdated)' : ''}</span>
        
        {#if loading}
          <div class="overlay-msg">
            <div class="spinner"></div>
            <span>Processing...</span>
          </div>
        {:else if error}
          <div class="overlay-msg error">
            <span>Error: {error}</span>
            <button class="retry-btn" on:click={runProcessing}>Retry</button>
          </div>
        {:else if processedUrl}
          <div class="img-wrapper" style="transform: rotate({currentImg.rotation}deg)">
            <img src={processedUrl} alt="Processed" />
          </div>
          {#if isStale}
            <div class="stale-tag">Settings or rotation changed - Update preview</div>
          {/if}
        {:else}
          <div class="placeholder">
            <p>Adjust filters and click "Apply" to see preview</p>
            <button class="apply-btn" on:click={runProcessing}>
              Apply Filters
            </button>
          </div>
        {/if}
      </div>
    </div>

    <div class="footer-controls">
      <div class="control-group">
        <label for="blur-slider">Noise Reduction (Blur)</label>
        <div class="slider-row">
          <input id="blur-slider" type="range" min="0" max="3" step="0.1" bind:value={blur} />
          <span class="val-badge">{blur === 0 ? "Off" : blur.toFixed(1)}</span>
        </div>
      </div>

      <div class="control-group">
        <label for="threshold-select">Threshold Mode</label>
        <select id="threshold-select" bind:value={threshold}>
          <option value={-2}>Disabled (Original Colors)</option>
          <option value={-3}>Adaptive (Best for Dirt/Shadows)</option>
          <option value={-1}>Otsu (Automatic Binarization)</option>
          <option value={128}>Fixed (128 - Mid)</option>
          <option value={160}>Fixed (160 - High Contrast)</option>
          <option value={100}>Fixed (100 - Low Contrast)</option>
        </select>
      </div>
      
      <button class="apply-btn main" on:click={runProcessing} disabled={loading}>
        {processedUrl ? "Update Preview" : "Apply Filters"}
      </button>
    </div>
  </main>
</div>

<style>
  .view {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding: 0;
    width: 100vw;
    background: #0d0d0d;
    color: #ddd;
    overflow: hidden;
  }
  
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1.5rem;
    background: #111;
    border-bottom: 1px solid #222;
    flex-shrink: 0;
  }

  .header-left, .header-right {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    flex: 1;
  }
  .header-right { justify-content: flex-end; }
  
  .titles h1 {
    font-size: 1.1rem;
    margin: 0;
    font-weight: 700;
  }
  .subtitle {
    font-size: 0.75rem;
    color: #666;
    margin: 0;
  }

  .header-center {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
  }
  .img-info {
    font-size: 0.7rem;
    color: #555;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .rotation-controls {
    display: flex;
    align-items: center;
    gap: 1rem;
    background: #1a1a1a;
    padding: 0.25rem 0.75rem;
    border-radius: 20px;
    border: 1px solid #333;
  }
  .rot-val {
    font-size: 0.85rem;
    font-weight: 600;
    min-width: 35px;
    text-align: center;
    color: #7c9ef8;
  }

  .icon-btn {
    background: none;
    border: none;
    color: #aaa;
    font-size: 1.2rem;
    cursor: pointer;
    line-height: 1;
    padding: 0.2rem;
    transition: color 0.15s;
  }
  .icon-btn:hover { color: #7c9ef8; }

  .nav-btn, .skip-btn {
    background: none;
    border: 1px solid #333;
    color: #aaa;
    padding: 0.4rem 0.8rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.85rem;
  }
  .nav-btn:hover, .skip-btn:hover { border-color: #555; color: #fff; }

  .proceed-btn {
    background: #7c9ef8;
    color: #000;
    border: none;
    padding: 0.4rem 1.25rem;
    border-radius: 6px;
    font-weight: 700;
    cursor: pointer;
    font-size: 0.85rem;
  }

  main {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 1rem;
    gap: 1rem;
    overflow: hidden;
  }

  .preview-container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    flex: 1;
    min-height: 0;
  }

  .preview-box {
    background: #080808;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    overflow: hidden;
    border: 1px solid #222;
  }
  .preview-box.stale { border-color: #443; }

  .img-wrapper {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }
  .img-wrapper img {
    max-width: 90%;
    max-height: 90%;
    object-fit: contain;
    box-shadow: 0 4px 20px rgba(0,0,0,0.5);
  }

  .label {
    position: absolute;
    top: 0.75rem;
    left: 0.75rem;
    background: rgba(0,0,0,0.8);
    padding: 0.25rem 0.6rem;
    border-radius: 4px;
    font-size: 0.65rem;
    z-index: 10;
    color: #888;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border: 1px solid #333;
  }

  .footer-controls {
    display: grid;
    grid-template-columns: 1fr 1fr auto;
    align-items: center;
    gap: 2rem;
    background: #111;
    padding: 1rem 1.5rem;
    border-radius: 12px;
    border: 1px solid #222;
  }

  .control-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .control-group label {
    font-size: 0.75rem;
    font-weight: 600;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .slider-row {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  input[type="range"] { flex: 1; cursor: pointer; }
  
  .val-badge {
    background: #1a1a1a;
    padding: 0.2rem 0.6rem;
    border-radius: 4px;
    font-size: 0.8rem;
    color: #7c9ef8;
    font-weight: 700;
    min-width: 30px;
    text-align: center;
    border: 1px solid #333;
  }

  select {
    background: #1a1a1a;
    color: #ddd;
    border: 1px solid #333;
    padding: 0.5rem;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
    outline: none;
  }
  select:focus { border-color: #7c9ef8; }
  /* Dark mode options */
  option { background: #1a1a1a; color: #ddd; }

  .apply-btn {
    padding: 0.6rem 1.5rem;
    background: #7c9ef8;
    color: #000;
    border: none;
    border-radius: 6px;
    font-weight: 700;
    cursor: pointer;
    transition: transform 0.1s;
  }
  .apply-btn:active { transform: scale(0.98); }
  .apply-btn:disabled { opacity: 0.5; cursor: not-allowed; }

  .overlay-msg {
    position: absolute;
    inset: 0;
    background: rgba(0,0,0,0.8);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    z-index: 20;
    backdrop-filter: blur(4px);
  }
  .placeholder { text-align: center; color: #444; }

  .spinner {
    width: 30px;
    height: 30px;
    border: 3px solid #222;
    border-top-color: #7c9ef8;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  .stale-tag {
    position: absolute;
    bottom: 1rem;
    background: #f8c97c;
    color: #000;
    padding: 0.3rem 0.8rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 700;
    box-shadow: 0 4px 12px rgba(0,0,0,0.5);
    z-index: 10;
  }
</style>
