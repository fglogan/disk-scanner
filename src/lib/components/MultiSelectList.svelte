<script>
  import { onMount, onDestroy } from 'svelte';
  import { fileSelection, selectionUtils } from '$lib/stores/selection.js';
  import Button from './ui/Button.svelte';
  
  export let items = [];
  export let itemKey = 'path'; // Property to use as unique key
  export let context = 'files'; // Context for selection store
  export let selectionStore = fileSelection; // Which selection store to use
  export let onDelete = null; // Delete handler function
  export let renderItem = null; // Custom item renderer
  
  // Subscribe to keyboard events
  onMount(() => {
    window.addEventListener('keydown', selectionUtils.handleKeyDown);
    window.addEventListener('keyup', selectionUtils.handleKeyUp);
  });
  
  onDestroy(() => {
    window.removeEventListener('keydown', selectionUtils.handleKeyDown);
    window.removeEventListener('keyup', selectionUtils.handleKeyUp);
  });
  
  function handleSelectAll() {
    selectionStore.selectAll(items.map(item => item[itemKey]));
  }
  
  function handleClearSelection() {
    selectionStore.clear();
  }
  
  async function handleDeleteSelected() {
    if (!onDelete) return;
    
    const selected = Array.from($selectionStore);
    const selectedItems = items.filter(item => selected.includes(item[itemKey]));
    
    const result = await onDelete(selectedItems);
    if (result?.success) {
      selectionStore.clear();
    }
  }
  
  function isSelected(item) {
    return $selectionStore.has(item[itemKey]);
  }
  
  $: selectedCount = $selectionStore.size;
</script>

<div class="multi-select-container">
  <!-- Selection toolbar -->
  {#if selectedCount > 0}
    <div class="selection-toolbar bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3 mb-4 flex items-center justify-between">
      <div class="text-sm text-blue-700 dark:text-blue-300">
        {selectedCount} item{selectedCount === 1 ? '' : 's'} selected
      </div>
      
      <div class="flex gap-2">
        <Button
          size="sm"
          variant="secondary"
          on:click={handleClearSelection}
        >
          Clear Selection
        </Button>
        
        {#if onDelete}
          <Button
            size="sm"
            variant="danger"
            on:click={handleDeleteSelected}
          >
            Delete Selected
          </Button>
        {/if}
      </div>
    </div>
  {:else if items.length > 0}
    <div class="selection-toolbar text-sm text-gray-600 dark:text-gray-400 mb-4">
      <button
        on:click={handleSelectAll}
        class="text-blue-600 dark:text-blue-400 hover:underline"
      >
        Select All ({items.length})
      </button>
      <span class="mx-2">•</span>
      <span class="text-xs">Hold Ctrl/Cmd to select multiple, Shift to select range</span>
    </div>
  {/if}
  
  <!-- Items list -->
  <div class="space-y-1">
    {#each items as item, index}
      <div
        class="item-row relative rounded p-2 cursor-pointer transition-colors
          {isSelected(item) ? 'bg-blue-100 dark:bg-blue-900/30 ring-2 ring-blue-500' : 'hover:bg-gray-100 dark:hover:bg-gray-800'}"
        on:click={() => selectionUtils.handleItemClick(item[itemKey], index, items, selectionStore, context)}
        role="button"
        tabindex="0"
        on:keydown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            selectionUtils.handleItemClick(item[itemKey], index, items, selectionStore, context);
          }
        }}
      >
        <!-- Checkbox indicator -->
        <div class="absolute left-2 top-1/2 -translate-y-1/2">
          <div class="w-5 h-5 rounded border-2 flex items-center justify-center
            {isSelected(item) ? 'bg-blue-500 border-blue-500' : 'border-gray-300 dark:border-gray-600'}">
            {#if isSelected(item)}
              <svg class="w-3 h-3 text-white" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
              </svg>
            {/if}
          </div>
        </div>
        
        <!-- Item content -->
        <div class="ml-10">
          {#if renderItem}
            {@html renderItem(item)}
          {:else}
            <div class="text-sm font-medium text-gray-900 dark:text-white">
              {item[itemKey]}
            </div>
          {/if}
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .item-row {
    user-select: none;
    -webkit-user-select: none;
  }
  
  .item-row:active {
    transform: scale(0.98);
  }
</style>