<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import "./app.css";
  import { currentPage, diskInfo, toasts, showShortcutsHelp, showSuccess } from "./lib/stores.js";
  import { themeStore, isDarkMode } from "./lib/stores/theme";
  import { t } from "./lib/i18n";

  import Sidebar from "./lib/components/Sidebar.svelte";
  import Dashboard from "./lib/components/Dashboard.svelte";
  import LargeFiles from "./lib/components/LargeFiles.svelte";
  import ProjectBloat from "./lib/components/ProjectBloat.svelte";
  import SystemJunk from "./lib/components/SystemJunk.svelte";
  import Duplicates from "./lib/components/Duplicates.svelte";
  import GitAssistance from "./lib/components/GitAssistance.svelte";
  import ProjectScanner from "./lib/components/ProjectScanner.svelte";
  import PACSCompliance from "./lib/components/PACSCompliance.svelte";
  import ArchitectureVisualization from "./lib/components/ArchitectureVisualization.svelte";
  import Settings from "./lib/components/Settings.svelte";
  import Toast from "./lib/components/ui/Toast.svelte";
  import KeyboardShortcuts from "./lib/components/ui/KeyboardShortcuts.svelte";
  import HelpPanel from "./lib/components/HelpPanel.svelte";
  import OnboardingTutorial from "./lib/components/OnboardingTutorial.svelte";
  import PerformanceMonitor from "./lib/components/PerformanceMonitor.svelte";
  import ErrorBoundary from "./lib/components/common/ErrorBoundary.svelte";
  import { announce } from "./lib/utils/accessibility";

  let showHelp = false;
  let showPerformance = false;
  let onboardingTutorial: OnboardingTutorial;

  // Initialize on mount
  onMount(() => {
    // Initialize theme
    themeStore.init();

    // Load initial data
    invoke("get_disk_info")
      .then((info) => diskInfo.set(info as any))
      .catch((e) => console.error("Failed to load disk info:", e));

    // Setup keyboard shortcuts
    setupKeyboardShortcuts();
    
    // Add skip link for accessibility
    const skipLink = document.createElement('a');
    skipLink.href = '#main-content';
    skipLink.className = 'skip-link';
    skipLink.textContent = $t('accessibility.skipToMain');
    skipLink.addEventListener('click', (e) => {
      e.preventDefault();
      const main = document.getElementById('main-content');
      if (main) {
        main.focus();
        main.scrollIntoView();
      }
    });
    document.body.insertBefore(skipLink, document.body.firstChild);
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
            themeStore.toggle();
            showSuccess($t('messages.themeChanged', { theme: $isDarkMode ? 'dark' : 'light' }));
            break;
          case 'k':
            event.preventDefault();
            showShortcutsHelp.update(show => !show);
            break;
          case 'h':
            event.preventDefault();
            showHelp = !showHelp;
            break;
          case 'p':
            event.preventDefault();
            showPerformance = !showPerformance;
            break;
        }
        return;
      }

      // Handle single key shortcuts
      switch (event.key.toLowerCase()) {
        case 'd':
          event.preventDefault();
          currentPage.set('dashboard');
          announce($t('nav.dashboard'));
          break;
        case 'p':
          event.preventDefault();
          currentPage.set('project-scanner');
          announce('Project Scanner');
          break;
        case 's':
          event.preventDefault();
          currentPage.set('settings');
          announce($t('nav.settings'));
          break;
        case 'l':
          event.preventDefault();
          currentPage.set('large-files');
          announce($t('categories.largeFiles'));
          break;
        case 'b':
          event.preventDefault();
          currentPage.set('project-bloat');
          announce($t('categories.projectBloat'));
          break;
        case 'j':
          event.preventDefault();
          currentPage.set('system-junk');
          announce($t('categories.systemJunk'));
          break;
        case 'u':
          event.preventDefault();
          currentPage.set('duplicates');
          announce($t('categories.duplicates'));
          break;
        case 'c':
          event.preventDefault();
          toasts.clear();
          showSuccess('Cleared notifications');
          break;
        case 'g':
          event.preventDefault();
          currentPage.set('git-assistance');
          announce('Git Assistance');
          break;
        case 'a':
          event.preventDefault();
          currentPage.set('pacs-compliance');
          announce('PACS Compliance');
          break;
        case 'v':
          event.preventDefault();
          currentPage.set('architecture-viz');
          announce('Architecture Visualization');
          break;
        case '?':
          event.preventDefault();
          showShortcutsHelp.update(show => !show);
          break;
        case 'escape':
          event.preventDefault();
          showShortcutsHelp.set(false);
          showHelp = false;
          showPerformance = false;
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

<div class="app-container" class:dark={$isDarkMode}>
  <!-- Skip link for accessibility -->
  <a href="#main-content" class="skip-link">
    {$t('accessibility.skipToMain')}
  </a>

  <!-- Sidebar Navigation -->
  <ErrorBoundary>
    <Sidebar />
  </ErrorBoundary>

  <!-- Main Content Area -->
  <main id="main-content" class="main-content" tabindex="-1">
    {#if $currentPage === "dashboard"}
      <ErrorBoundary>
        <Dashboard />
      </ErrorBoundary>
    {:else if $currentPage === "large-files"}
      <ErrorBoundary>
        <LargeFiles />
      </ErrorBoundary>
    {:else if $currentPage === "project-bloat"}
      <ErrorBoundary>
        <ProjectBloat />
      </ErrorBoundary>
    {:else if $currentPage === "system-junk"}
      <ErrorBoundary>
        <SystemJunk />
      </ErrorBoundary>
    {:else if $currentPage === "duplicates"}
      <ErrorBoundary>
        <Duplicates />
      </ErrorBoundary>
    {:else if $currentPage === "project-scanner"}
      <ErrorBoundary>
        <ProjectScanner />
      </ErrorBoundary>
    {:else if $currentPage === "git-assistance"}
      <ErrorBoundary>
        <GitAssistance />
      </ErrorBoundary>
    {:else if $currentPage === "pacs-compliance"}
      <ErrorBoundary>
        <PACSCompliance />
      </ErrorBoundary>
    {:else if $currentPage === "architecture-viz"}
      <ErrorBoundary>
        <ArchitectureVisualization />
      </ErrorBoundary>
    {:else if $currentPage === "settings"}
      <ErrorBoundary>
        <Settings />
      </ErrorBoundary>
    {/if}
  </main>

  <!-- Toast Notifications Container -->
  <div class="toast-container" role="region" aria-label="Notifications" aria-live="polite">
    {#each $toasts as toast (toast.id)}
      <Toast {toast} />
    {/each}
  </div>

  <!-- Keyboard Shortcuts Help Overlay -->
  {#if $showShortcutsHelp}
    <KeyboardShortcuts />
  {/if}
  
  <!-- Help Panel -->
  <HelpPanel bind:isOpen={showHelp} context={$currentPage} />
  
  <!-- Onboarding Tutorial -->
  <OnboardingTutorial bind:this={onboardingTutorial} />
  
  <!-- Performance Monitor -->
  <PerformanceMonitor bind:isVisible={showPerformance} />
</div>

<style>
  :global(body) {
    font-family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    overscroll-behavior: none;
    margin: 0;
    padding: 0;
    height: 100vh;
    overflow: hidden;
    background-color: var(--bg-primary);
    color: var(--text-primary);
  }

  :global(#app) {
    height: 100vh;
  }
  
  .app-container {
    display: flex;
    height: 100vh;
    background-color: var(--bg-primary);
    color: var(--text-primary);
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }
  
  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: 2rem 3rem;
    outline: none;
  }
  
  .toast-container {
    position: fixed;
    top: 1rem;
    right: 1rem;
    z-index: 1000;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    pointer-events: none;
  }
  
  .toast-container > :global(*) {
    pointer-events: auto;
  }
  
  /* Skip link for accessibility */
  :global(.skip-link) {
    position: absolute;
    top: -40px;
    left: 0;
    background: var(--button-bg);
    color: var(--button-text);
    padding: 0.5rem 1rem;
    text-decoration: none;
    border-radius: 0 0 0.25rem 0;
    z-index: 10000;
    font-size: 0.875rem;
    font-weight: 500;
  }
  
  :global(.skip-link:focus) {
    top: 0;
  }
  
  @media (max-width: 768px) {
    .main-content {
      padding: 1rem;
    }
  }
</style>