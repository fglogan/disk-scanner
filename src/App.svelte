<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import "./app.css";
  import { currentPage, diskInfo, darkMode, toasts, showShortcutsHelp, showSuccess } from "./lib/stores.js";

  import Sidebar from "./lib/components/Sidebar.svelte";
  import Dashboard from "./lib/components/Dashboard.svelte";
  import LargeFiles from "./lib/components/LargeFiles.svelte";
  import ProjectBloat from "./lib/components/ProjectBloat.svelte";
  import SystemJunk from "./lib/components/SystemJunk.svelte";
  import Duplicates from "./lib/components/Duplicates.svelte";
  // import DevCaches from "./lib/components/DevCaches.svelte"; // Disabled: uses non-existent selectedDirectory store
  // import GitScanner from "./lib/components/GitScanner.svelte"; // Disabled: uses non-existent selectedDirectory store
  import GitAssistance from "./lib/components/GitAssistance.svelte";
  import ProjectScanner from "./lib/components/ProjectScanner.svelte";
  import Settings from "./lib/components/Settings.svelte";
  import Toast from "./lib/components/ui/Toast.svelte";
  import KeyboardShortcuts from "./lib/components/ui/KeyboardShortcuts.svelte";

  // Initialize theme on mount
  onMount(() => {
    // Set initial theme attribute
    if (typeof window !== 'undefined') {
      const isDark = $darkMode;
      document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light');
      document.documentElement.classList.toggle('dark', isDark);
    }

    // Load initial data
    invoke("get_disk_info")
      .then((info) => diskInfo.set(info as any))
      .catch((e) => console.error("Failed to load disk info:", e));

    // Setup keyboard shortcuts
    setupKeyboardShortcuts();
  });

  // Reactive theme updates using $effect
  $effect(() => {
    if (typeof window !== 'undefined') {
      document.documentElement.setAttribute('data-theme', $darkMode ? 'dark' : 'light');
      document.documentElement.classList.toggle('dark', $darkMode);
    }
  });

  function setupKeyboardShortcuts() {
    function handleKeydown(event: KeyboardEvent) {
      // Ignore if user is typing in an input
      const target = event.target as HTMLElement;
      if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) {
        return;
      }

      // Handle modifier combinations first
      if (event.ctrlKey || event.metaKey) {
        switch (event.key.toLowerCase()) {
          case 't':
            event.preventDefault();
            darkMode.toggle();
            showSuccess(`Switched to ${$darkMode ? 'dark' : 'light'} mode`);
            break;
          case 'k':
            event.preventDefault();
            showShortcutsHelp.update(show => !show);
            break;
        }
        return;
      }

      // Handle single key shortcuts
      switch (event.key.toLowerCase()) {
        case 'd':
          event.preventDefault();
          currentPage.set('dashboard');
          showSuccess('📊 Dashboard');
          break;
        case 'p':
          event.preventDefault();
          currentPage.set('project-scanner');
          showSuccess('🔍 Project Scanner');
          break;
        case 's':
          event.preventDefault();
          currentPage.set('settings');
          showSuccess('⚙️ Settings');
          break;
        case 'l':
          event.preventDefault();
          currentPage.set('large-files');
          showSuccess('📁 Large Files');
          break;
        case 'b':
          event.preventDefault();
          currentPage.set('project-bloat');
          showSuccess('🗂️ Project Bloat');
          break;
        case 'j':
          event.preventDefault();
          currentPage.set('system-junk');
          showSuccess('🗑️ System Junk');
          break;
        case 'u':
          event.preventDefault();
          currentPage.set('duplicates');
          showSuccess('📄 Duplicates');
          break;
        case 'c':
          event.preventDefault();
          toasts.clear();
          showSuccess('🧹 Cleared notifications');
          break;
        case 'g':
          event.preventDefault();
          currentPage.set('git-assistance');
          showSuccess('🔧 Git Assistance');
          break;
        case '?':
          event.preventDefault();
          showShortcutsHelp.update(show => !show);
          break;
        case 'escape':
          event.preventDefault();
          showShortcutsHelp.set(false);
          toasts.clear();
          break;
      }
    }

    document.addEventListener('keydown', handleKeydown);
    
    return () => {
      document.removeEventListener('keydown', handleKeydown);
    };
  }
</script>

<div class="flex h-full transition-colors duration-200" class:dark={$darkMode} class:bg-slate-900={$darkMode} class:bg-slate-50={!$darkMode} class:text-slate-200={$darkMode} class:text-slate-800={!$darkMode}>
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
      {:else if $currentPage === "project-scanner"}
        <ProjectScanner />
      {:else if $currentPage === "git-assistance"}
        <GitAssistance />
      {:else if $currentPage === "settings"}
        <Settings />
      {/if}
  </main>

  <!-- Toast Notifications Container -->
  <div class="fixed top-4 right-4 z-50 space-y-2">
    {#each $toasts as toast (toast.id)}
      <Toast {toast} />
    {/each}
  </div>

  <!-- Keyboard Shortcuts Help Overlay -->
  {#if $showShortcutsHelp}
    <KeyboardShortcuts />
  {/if}
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
