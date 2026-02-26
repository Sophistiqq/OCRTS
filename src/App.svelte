<script lang="ts">
  import UploadView from "./views/UploadView.svelte";
  import ProcessingView from "./views/ProcessingView.svelte";
  import EditorView from "./views/EditorView.svelte";
  import OutputView from "./views/OutputView.svelte";

  type View = "upload" | "processing" | "editor" | "output";
  let view: View = "upload";
</script>

{#if view === "upload"}
  <UploadView onProceed={() => (view = "processing")} />
{:else if view === "processing"}
  <ProcessingView
    onBack={() => (view = "upload")}
    onProceed={() => (view = "editor")}
  />
{:else if view === "editor"}
  <EditorView
    onBack={() => (view = "processing")}
    onFinish={() => (view = "output")}
  />
{:else if view === "output"}
  <OutputView onBack={() => (view = "upload")} />
{/if}
