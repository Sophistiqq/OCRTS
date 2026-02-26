<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { loadImage } from "../../lib/tauri";
  import { addImages } from "../../lib/stores/images";

  let dragging = false;

  async function handleFiles(paths: string[]) {
    const images = await Promise.all(paths.map((p) => loadImage(p)));
    addImages(images);
  }

  async function openFilePicker() {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Images",
          extensions: ["png", "jpg", "jpeg", "webp", "bmp", "tiff"],
        },
      ],
    });
    if (!selected) return;
    const paths = Array.isArray(selected) ? selected : [selected];
    await handleFiles(paths);
  }

  function onDragOver(e: DragEvent) {
    e.preventDefault();
    dragging = true;
  }

  function onDragLeave() {
    dragging = false;
  }

  async function onDrop(e: DragEvent) {
    e.preventDefault();
    dragging = false;
    const files = Array.from(e.dataTransfer?.files ?? []);
    // @ts-ignore - Tauri adds path to File objects in some contexts, but not always via standard types
    const paths = files.map((f: any) => f.path).filter(Boolean);
    if (paths.length) await handleFiles(paths);
  }
</script>

<button
  class="dropzone"
  class:dragging
  on:dragover={onDragOver}
  on:dragleave={onDragLeave}
  on:drop={onDrop}
  on:click={openFilePicker}
  aria-label="Upload images"
>
  <div class="icon">🖼️</div>
  <p class="primary">Drop images here or click to browse</p>
  <p class="secondary">PNG, JPG, BMP, TIFF supported</p>
</button>

<style>
  .dropzone {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    width: 100%;
    min-height: 220px;
    border: 2px dashed #555;
    border-radius: 12px;
    background: transparent;
    cursor: pointer;
    transition:
      border-color 0.15s,
      background 0.15s;
    color: inherit;
    font-family: inherit;
  }
  .dropzone:hover,
  .dropzone.dragging {
    border-color: #7c9ef8;
    background: rgba(124, 158, 248, 0.06);
  }
  .icon {
    font-size: 2.5rem;
  }
  .primary {
    font-size: 1rem;
    font-weight: 500;
    margin: 0;
  }
  .secondary {
    font-size: 0.8rem;
    color: #888;
    margin: 0;
  }
</style>
