<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { currentPage, isScanning } from '$lib/stores.js';
  import { fileSelection, duplicateSelection, cacheSelection, repoSelection } from '$lib/stores/selection.js';
  
  export let showHelp = false;
  
  // Keyboard shortcut definitions
  const shortcuts = [
    // Navigation
    { key: '1', ctrl: true, description: 'Go to Dashboard', action: () => currentPage.set('dashboard') },
    { key: '2', ctrl: true, description: 'Go to Results', action: () => currentPage.set('results') },
    { key: '3', ctrl: true, description: 'Go to Settings', action: () => currentPage.set('settings') },
    
    // Actions
    { key: 'r', ctrl: true, description: 'Refresh/Rescan', action: handleRefresh },
    { key: 'Delete', description: 'Delete selected items', action: handleDelete },
    { key: 'a', ctrl: true, description: 'Select all', action: handleSelectAll },
    { key: 'Escape', description: 'Clear selection', action: handleClearSelection },
    { key: 'e', ctrl: true, description: 'Export results', action: handleExport },
    { key: 'z', ctrl: true, description: 'Undo last deletion', action: handleUndo },
    
    // View
    { key: '/', ctrl: true, description: 'Show keyboard shortcuts', action: () => showHelp = !showHelp },
    { key: 'f', ctrl: true, description: 'Focus search', action: focusSearch },
    
    // Multi-select helpers
    { key: 'Shift', description: 'Select range (hold)', action: null },
    { key: 'Ctrl/Cmd', description: 'Toggle selection (hold)', action: null },
  ];
  
  // Custom keyboard shortcut bindings
  let customShortcuts = {};
  
  async function loadCustomShortcuts() {
    try {
      // TODO: Load from backend config
      customShortcuts = {};
    } catch (error) {
      console.error('Failed to load custom shortcuts:', error);
    }
  }
  
  function handleKeyDown(event) {
    // Ignore if typing in input/textarea
    if (event.target.matches('input, textarea, select')) {
      return;
    }
    
    // Find matching shortcut
    const shortcut = shortcuts.find(s => {
      const keyMatch = s.key.toLowerCase() === event.key.toLowerCase();
      const ctrlMatch = s.ctrl ? (event.ctrlKey || event.metaKey) : !event.ctrlKey && !event.metaKey;
      return keyMatch && ctrlMatch;
    });
    
    if (shortcut && shortcut.action) {
      event.preventDefault();
      shortcut.action();
    }
  }
  
  async function handleRefresh() {
    if ($isScanning) return;
    
    // Trigger rescan based on current page
    const page = $currentPage;
    if (page === 'dashboard') {
      // Refresh dashboard data
      await invoke('get_system_info');
    } else if (page === 'results') {
      // Re-run last scan
      // TODO: Track and re-run last scan type
    }
  }
  
  async function handleDelete() {
    // Get all selected items across stores
    const selectedFiles = Array.from($fileSelection);
    const selectedDuplicates = Array.from($duplicateSelection);
    const selectedCaches = Array.from($cacheSelection);
    const selectedRepos = Array.from($repoSelection);
    
    const allSelected = [
      ...selectedFiles,
      ...selectedDuplicates,
      ...selectedCaches,
      ...selectedRepos
    ];
    
    if (allSelected.length === 0) return;
    
    // TODO: Show confirmation dialog
    // TODO: Invoke delete command
    console.log('Delete selected:', allSelected);
  }
  
  function handleSelectAll() {
    // Select all based on current context
    const page = $currentPage;
    // TODO: Implement based on what's currently visible
  }
  
  function handleClearSelection() {
    fileSelection.clear();
    duplicateSelection.clear();
    cacheSelection.clear();
    repoSelection.clear();
  }
  
  function handleExport() {
    // TODO: Open export dialog
    console.log('Export triggered');
  }
  
  async function handleUndo() {
    try {
      const history = await invoke('get_undo_history');
      if (history.length > 0) {
        const latest = history[0];
        await invoke('restore_deletion', { operationId: latest.id });
      }
    } catch (error) {
      console.error('Undo failed:', error);
    }
  }
  
  function focusSearch() {
    const searchInput = document.querySelector('input[type="search"], input[placeholder*="Search"]');
    if (searchInput) {
      searchInput.focus();
    }
  }
  
  onMount(() => {
    window.addEventListener('keydown', handleKeyDown);
    loadCustomShortcuts();
  });
  
  onDestroy(() => {
    window.removeEventListener('keydown', handleKeyDown);
  });
</script>

<!-- Keyboard shortcuts help overlay -->
{#if showHelp}
<div 
  class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
  on:click={() => showHelp = false}
>
  <div 
    class="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto"
    on:click|stopPropagation
  >
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white">Keyboard Shortcuts</h2>
      <button
        on:click={() => showHelp = false}
        class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
    
    <div class="space-y-6">
      <!-- Navigation shortcuts -->
      <div>
        <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-2">
          Navigation
        </h3>
        <div class="space-y-1">
          {#each shortcuts.filter(s => ['1', '2', '3'].includes(s.key)) as shortcut}
            <div class="flex justify-between items-center py-1">
              <span class="text-gray-600 dark:text-gray-400">{shortcut.description}</span>
              <kbd class="shortcut-key">
                {#if shortcut.ctrl}Ctrl+{/if}{shortcut.key}
              </kbd>
            </div>
          {/each}
        </div>
      </div>
      
      <!-- Action shortcuts -->
      <div>
        <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-2">
          Actions
        </h3>
        <div class="space-y-1">
          {#each shortcuts.filter(s => ['r', 'Delete', 'a', 'Escape', 'e', 'z'].includes(s.key)) as shortcut}
            <div class="flex justify-between items-center py-1">
              <span class="text-gray-600 dark:text-gray-400">{shortcut.description}</span>
              <kbd class="shortcut-key">
                {#if shortcut.ctrl}Ctrl+{/if}{shortcut.key}
              </kbd>
            </div>
          {/each}
        </div>
      </div>
      
      <!-- Selection shortcuts -->
      <div>
        <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-2">
          Selection
        </h3>
        <div class="space-y-1">
          {#each shortcuts.filter(s => ['Shift', 'Ctrl/Cmd'].includes(s.key)) as shortcut}
            <div class="flex justify-between items-center py-1">
              <span class="text-gray-600 dark:text-gray-400">{shortcut.description}</span>
              <kbd class="shortcut-key">{shortcut.key}</kbd>
            </div>
          {/each}
        </div>
      </div>
    </div>
    
    <div class="mt-6 pt-4 border-t border-gray-200 dark:border-gray-700">
      <p class="text-sm text-gray-500 dark:text-gray-400">
        Press <kbd class="shortcut-key">Ctrl+/</kbd> to toggle this help
      </p>
    </div>
  </div>
</div>
{/if}

<style>
  .shortcut-key {
    @apply px-2 py-1 text-xs font-mono bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded border border-gray-300 dark:border-gray-600;
  }
</style>