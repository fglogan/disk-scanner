import { writable, derived } from 'svelte/store';

// @ts-check

// Multi-selection store for managing selected items
function createSelectionStore() {
  const { subscribe, set, update } = writable(new Set());
  
  return {
    subscribe,
    
    // Add a single item to selection
    add: (/** @type {string} */ item) => update(selected => {
      selected.add(item);
      return new Set(selected);
    }),
    
    // Remove a single item from selection
    remove: (/** @type {string} */ item) => update(selected => {
      selected.delete(item);
      return new Set(selected);
    }),
    
    // Toggle a single item
    toggle: (/** @type {string} */ item) => update(selected => {
      if (selected.has(item)) {
        selected.delete(item);
      } else {
        selected.add(item);
      }
      return new Set(selected);
    }),
    
    // Select multiple items
    addMultiple: (/** @type {string[]} */ items) => update(selected => {
      items.forEach(item => selected.add(item));
      return new Set(selected);
    }),
    
    // Select all items
    selectAll: (/** @type {string[]} */ items) => set(new Set(items)),
    
    // Clear all selections
    clear: () => set(new Set()),
    
    // Check if item is selected
    isSelected: (/** @type {string} */ item) => {
      let selected = false;
      const unsubscribe = subscribe(set => {
        selected = set.has(item);
      });
      unsubscribe();
      return selected;
    },
    
    // Get selected items as array
    getSelected: () => {
      /** @type {string[]} */
      let items = [];
      const unsubscribe = subscribe(set => {
        items = Array.from(set);
      });
      unsubscribe();
      return items;
    }
  };
}

// Create selection stores for different data types
export const fileSelection = createSelectionStore();
export const duplicateSelection = createSelectionStore();
export const cacheSelection = createSelectionStore();
export const repoSelection = createSelectionStore();

// Derived store for total selection count
export const totalSelectedCount = derived(
  [fileSelection, duplicateSelection, cacheSelection, repoSelection],
  ([$files, $duplicates, $caches, $repos]) => 
    $files.size + $duplicates.size + $caches.size + $repos.size
);

// Selection mode store (for keyboard shortcuts)
export const selectionMode = writable({
  shiftPressed: false,
  ctrlPressed: false,
  /** @type {number | null} */
  lastSelectedIndex: null,
  /** @type {string | null} */
  currentContext: null // 'files', 'duplicates', 'caches', 'repos'
});

// Handle keyboard events for multi-select
export function handleKeyDown(/** @type {KeyboardEvent} */ event) {
  selectionMode.update(mode => ({
    ...mode,
    shiftPressed: event.shiftKey,
    ctrlPressed: event.ctrlKey || event.metaKey
  }));
}

export function handleKeyUp(/** @type {KeyboardEvent} */ event) {
  selectionMode.update(mode => ({
    ...mode,
    shiftPressed: event.shiftKey,
    ctrlPressed: event.ctrlKey || event.metaKey
  }));
}

// Handle item click with modifiers
export function handleItemClick(
  /** @type {string} */ item,
  /** @type {number} */ index,
  /** @type {any[]} */ items,
  /** @type {any} */ selectionStore,
  /** @type {string} */ context
) {
  const unsubscribe = selectionMode.subscribe(mode => {
    if (mode.ctrlPressed) {
      // Ctrl/Cmd click: toggle single item
      selectionStore.toggle(item);
      selectionMode.update(m => ({ ...m, lastSelectedIndex: index, currentContext: context }));
    } else if (mode.shiftPressed && mode.lastSelectedIndex !== null && mode.currentContext === context) {
      // Shift click: select range
      const start = Math.min(mode.lastSelectedIndex, index);
      const end = Math.max(mode.lastSelectedIndex, index);
      const rangeItems = items.slice(start, end + 1);
      selectionStore.addMultiple(rangeItems);
    } else {
      // Normal click: select only this item
      selectionStore.clear();
      selectionStore.add(item);
      selectionMode.update(m => ({ ...m, lastSelectedIndex: index, currentContext: context }));
    }
  });
  unsubscribe();
}

// Batch operations on selected items
export async function deleteSelected(
  /** @type {any} */ selectionStore,
  /** @type {(items: string[]) => Promise<{success: boolean}>} */ deleteFunction
) {
  const selected = selectionStore.getSelected();
  if (selected.length === 0) return;
  
  const result = await deleteFunction(selected);
  if (result.success) {
    selectionStore.clear();
  }
  return result;
}

// Export selection utilities
export const selectionUtils = {
  handleKeyDown,
  handleKeyUp,
  handleItemClick,
  deleteSelected
};