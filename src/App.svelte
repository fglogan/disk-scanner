<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import "./app.css";
  import { currentPage, diskInfo } from "./lib/stores.js";

  import Sidebar from "./lib/components/Sidebar.svelte";
  import Dashboard from "./lib/components/Dashboard.svelte";
  import LargeFiles from "./lib/components/LargeFiles.svelte";
  import ProjectBloat from "./lib/components/ProjectBloat.svelte";
  import SystemJunk from "./lib/components/SystemJunk.svelte";
  import Duplicates from "./lib/components/Duplicates.svelte";
  // import DevCaches from "./lib/components/DevCaches.svelte"; // Disabled: uses non-existent selectedDirectory store
  // import GitScanner from "./lib/components/GitScanner.svelte"; // Disabled: uses non-existent selectedDirectory store
  import GitAssistance from "./lib/components/GitAssistance.svelte";
  import Settings from "./lib/components/Settings.svelte";

  // Load initial data on app startup
  onMount(() => {
    invoke("get_disk_info")
      .then((info) => diskInfo.set(info))
      .catch((e) => console.error("Failed to load disk info:", e));
  });
</script>

<div class="flex h-full bg-slate-900 text-slate-200">
  <!-- Sidebar Navigation -->
  <Sidebar />

  <!-- Main Content Area -->
  <main class="flex-1 overflow-y-auto p-8 md:p-12">
    {#if $currentPage === "dashboard"}
      <Dashboard />
    {:else if $currentPage === "large-files"}
      <LargeFiles />
    {:else if $currentPage === "project-bloat"}
      <ProjectBloat />
    {:else if $currentPage === "system-junk"}
      <SystemJunk />
     {:else if $currentPage === "duplicates"}
       <Duplicates />
     <!-- {:else if $currentPage === "dev-caches"}
       <DevCaches /> -->
      <!-- {:else if $currentPage === "git-scanner"}
        <GitScanner /> -->
      {:else if $currentPage === "git-assistance"}
        <GitAssistance />
      {:else if $currentPage === "settings"}
        <Settings />
      {/if}
  </main>
</div>

<style>
  :global(body) {
    font-family: "Inter", sans-serif;
    overscroll-behavior: none;
    margin: 0;
    padding: 0;
    height: 100vh;
    overflow: hidden;
  }

  :global(#app) {
    height: 100vh;
  }

  /* Webkit scrollbar styling for a more native feel */
  :global(::-webkit-scrollbar) {
    width: 8px;
    height: 8px;
  }
  :global(::-webkit-scrollbar-track) {
    background: #2d3748; /* slate-800 */
  }
  :global(::-webkit-scrollbar-thumb) {
    background: #4a5568; /* slate-600 */
    border-radius: 4px;
  }
  :global(::-webkit-scrollbar-thumb:hover) {
    background: #718096; /* slate-500 */
  }
</style>
