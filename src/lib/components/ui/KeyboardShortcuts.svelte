<script lang="ts">
  import { fly, fade } from 'svelte/transition';
  import { showShortcutsHelp } from '../../stores.js';

  const shortcuts = [
    { key: 'D', action: 'Dashboard', description: 'Navigate to main dashboard', icon: '📊' },
    { key: 'P', action: 'Project Scanner', description: 'Scan and analyze projects', icon: '🔍' },
    { key: 'L', action: 'Large Files', description: 'View large files analysis', icon: '📁' },
    { key: 'B', action: 'Project Bloat', description: 'View development bloat', icon: '🗂️' },
    { key: 'J', action: 'System Junk', description: 'View system junk files', icon: '🗑️' },
    { key: 'U', action: 'Duplicates', description: 'View duplicate files', icon: '📄' },
    { key: 'G', action: 'Git Assistance', description: 'Git repository tools', icon: '🔧' },
    { key: 'S', action: 'Settings', description: 'Adjust application settings', icon: '⚙️' },
    { key: 'C', action: 'Clear Notifications', description: 'Dismiss all toast messages', icon: '🧹' },
    { key: 'Ctrl/Cmd + T', action: 'Toggle Theme', description: 'Switch between dark and light mode', icon: '🎨' },
    { key: 'Ctrl/Cmd + K', action: 'Command Palette', description: 'Show/hide this help overlay', icon: '⌘' },
    { key: '?', action: 'Show Help', description: 'Toggle keyboard shortcuts overlay', icon: '❓' },
    { key: 'Esc', action: 'Close/Clear', description: 'Close overlays or clear notifications', icon: '✖️' }
  ];

  function closeHelp() {
    showShortcutsHelp.set(false);
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      closeHelp();
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      closeHelp();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- Backdrop -->
<div 
  class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center"
  transition:fade={{ duration: 200 }}
  on:click={handleBackdropClick}
  on:keydown={handleKeydown}
  role="dialog"
  aria-labelledby="shortcuts-title"
  aria-modal="true"
  tabindex="-1"
>
  <!-- Modal Content -->
  <div 
    class="bg-white dark:bg-slate-800 rounded-xl shadow-2xl border border-slate-200 dark:border-slate-700 p-6 max-w-lg w-full mx-4"
    transition:fly={{ y: -20, duration: 300 }}
  >
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <h2 id="shortcuts-title" class="text-xl font-semibold text-slate-900 dark:text-slate-100">
        Keyboard Shortcuts
      </h2>
      <button 
        class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 transition-colors p-1 rounded-md hover:bg-slate-100 dark:hover:bg-slate-700"
        on:click={closeHelp}
        aria-label="Close shortcuts help"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Shortcuts List -->
    <div class="space-y-2 max-h-96 overflow-y-auto">
      {#each shortcuts as shortcut}
        <div class="flex items-center justify-between py-2.5 px-3 rounded-lg bg-slate-50 dark:bg-slate-700/50 hover:bg-slate-100 dark:hover:bg-slate-700/75 transition-colors">
          <div class="flex items-center gap-3 flex-1">
            {#if shortcut.icon}
              <span class="text-base" role="img" aria-hidden="true">{shortcut.icon}</span>
            {/if}
            <div class="flex-1">
              <div class="font-medium text-slate-900 dark:text-slate-100 text-sm">
                {shortcut.action}
              </div>
              <div class="text-xs text-slate-600 dark:text-slate-400 mt-0.5">
                {shortcut.description}
              </div>
            </div>
          </div>
          <div class="ml-4">
            <kbd class="px-2.5 py-1.5 text-xs font-mono bg-slate-200 dark:bg-slate-600 text-slate-700 dark:text-slate-300 rounded-md border border-slate-300 dark:border-slate-500 shadow-sm">
              {shortcut.key}
            </kbd>
          </div>
        </div>
      {/each}
    </div>

    <!-- Footer -->
    <div class="mt-6 pt-4 border-t border-slate-200 dark:border-slate-600">
      <p class="text-xs text-slate-600 dark:text-slate-400 text-center">
        Press <kbd class="text-xs bg-slate-200 dark:bg-slate-600 px-1 py-0.5 rounded">?</kbd> or 
        <kbd class="text-xs bg-slate-200 dark:bg-slate-600 px-1 py-0.5 rounded">Esc</kbd> to close this help
      </p>
    </div>
  </div>
</div>

<style>
  kbd {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Consolas, "Liberation Mono", Menlo, monospace;
  }
</style>