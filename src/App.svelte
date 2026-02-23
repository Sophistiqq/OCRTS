<script lang="ts">
  import UploadView from "./views/UploadView.svelte";
  import EditorView from "./views/EditorView.svelte";
  import OutputView from "./views/OutputView.svelte";

  type View = "upload" | "editor" | "output";
  let view: View = "upload";
</script>

{#if view === "upload"}
  <UploadView onProceed={() => (view = "editor")} />
{:else if view === "editor"}
  <EditorView
    onBack={() => (view = "upload")}
    onFinish={() => (view = "output")}
  />
{:else if view === "output"}
  <OutputView onBack={() => (view = "upload")} />
{/if}
