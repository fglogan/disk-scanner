<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { writable } from 'svelte/store';
  import { selectedDirectory, isScanning } from '../stores.js';

  let cacheCategories = [];
  let selectedCaches = new Set();
  let totalSize = 0;
  let error = null;
  let scanningComplete = false;

  $: totalSelectedSize = Array.from(selectedCaches).reduce((sum, cacheKey) => {
    const [categoryIndex, entryIndex] = cacheKey.split('-').map(Number);
    const entry = cacheCategories[categoryIndex]?.entries[entryIndex];
    return sum + (entry?.size_mb || 0);
  }, 0);

  $: selectedCount = selectedCaches.size;

  onMount(() => {
    if ($selectedDirectory) {
      scanCaches();
    }
  });

  async function selectDirectory() {
    try {
      const directory = await open({
        directory: true,
        multiple: false,
        title: 'Select Directory to Scan for Caches'
      });
      
      if (directory) {
        selectedDirectory.set(directory);
        await scanCaches();
      }
    } catch (err) {
      error = `Failed to select directory: ${err.message}`;
    }
  }

  async function scanCaches() {
    if (!$selectedDirectory) return;
    
    try {
      isScanning.set(true);
      error = null;
      scanningComplete = false;
      selectedCaches.clear();
      
      const result = await invoke('scan_dev_caches', {
        opts: {
          root: $selectedDirectory,
          followSymlinks: false
        }
      });
      
      cacheCategories = result;
      totalSize = result.reduce((sum, category) => sum + category.total_size_mb, 0);
      scanningComplete = true;
      
    } catch (err) {
      error = `Failed to scan caches: ${err.message}`;
    } finally {
      isScanning.set(false);
    }
  }

  function toggleCache(categoryIndex, entryIndex) {
    const cacheKey = `${categoryIndex}-${entryIndex}`;
    if (selectedCaches.has(cacheKey)) {
      selectedCaches.delete(cacheKey);
    } else {
      selectedCaches.add(cacheKey);
    }
    selectedCaches = selectedCaches; // Trigger reactivity
  }

  function toggleCategory(categoryIndex) {
    const category = cacheCategories[categoryIndex];
    category.entries.forEach((_, entryIndex) => {
      const cacheKey = `${categoryIndex}-${entryIndex}`;
      if (selectedCaches.has(cacheKey)) {
        selectedCaches.delete(cacheKey);
      } else {
        selectedCaches.add(cacheKey);
      }
    });
    selectedCaches = selectedCaches; // Trigger reactivity
  }

  function getSafetyColor(safety) {
    switch (safety) {
      case 'safe': return 'text-green-400';
      case 'caution': return 'text-yellow-400';
      case 'dangerous': return 'text-red-400';
      default: return 'text-gray-400';
    }
  }

  function getSafetyBg(safety) {
    switch (safety) {
      case 'safe': return 'bg-green-900/30 border-green-700';
      case 'caution': return 'bg-yellow-900/30 border-yellow-700';
      case 'dangerous': return 'bg-red-900/30 border-red-700';
      default: return 'bg-gray-900/30 border-gray-700';
    }
  }

  function formatSize(sizeMb) {
    if (sizeMb >= 1024) {
      return `${(sizeMb / 1024).toFixed(1)} GB`;
    }
    return `${sizeMb.toFixed(1)} MB`;
  }

  async function cleanSelectedCaches() {
    if (selectedCount === 0) return;
    
    try {
      const selectedPaths = [];
      selectedCaches.forEach(cacheKey => {
        const [categoryIndex, entryIndex] = cacheKey.split('-').map(Number);
        const entry = cacheCategories[categoryIndex]?.entries[entryIndex];
        if (entry) {
          selectedPaths.push(entry.path);
        }
      });

      const result = await invoke('cleanup_dirs', {
        req: {
          paths: selectedPaths,
          dryRun: false,
          trash: true
        }
      });

      // Refresh the scan after cleanup
      await scanCaches();
      
    } catch (err) {
      error = `Failed to clean caches: ${err.message}`;
    }
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-bold text-white mb-2">Developer Caches</h1>
      <p class="text-slate-300">
        Clearing caches is generally safe and can free up significant space. 
        Packages will be re-downloaded on next use.
      </p>
    </div>
    <button
      on:click={selectDirectory}
      class="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
      disabled={$isScanning}
    >
      {$isScanning ? 'Scanning...' : 'Select Directory'}
    </button>
  </div>

  {#if $selectedDirectory}
    <div class="bg-slate-800 rounded-lg p-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <svg class="w-5 h-5 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
          </svg>
          <span class="text-slate-300">{$selectedDirectory}</span>
        </div>
        <button
          on:click={scanCaches}
          class="bg-slate-700 hover:bg-slate-600 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors"
          disabled={$isScanning}
        >
          {$isScanning ? 'Scanning...' : 'Rescan'}
        </button>
      </div>
    </div>
  {/if}

  {#if error}
    <div class="bg-red-900/50 border border-red-700 rounded-lg p-4">
      <div class="flex items-center space-x-2">
        <svg class="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
        <span class="text-red-200">{error}</span>
      </div>
    </div>
  {/if}

  {#if scanningComplete && cacheCategories.length === 0}
    <div class="bg-slate-800 rounded-xl shadow-lg p-8 text-center">
      <svg class="w-16 h-16 text-slate-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
      <h3 class="text-xl font-semibold text-white mb-2">No Developer Caches Found</h3>
      <p class="text-slate-400">This directory doesn't contain any recognizable developer cache directories.</p>
    </div>
  {/if}

  {#if scanningComplete && cacheCategories.length > 0}
    <div class="bg-slate-800 rounded-xl shadow-lg p-6">
      <div class="flex items-center justify-between mb-6">
        <div>
          <h2 class="text-xl font-semibold text-white">Cache Analysis</h2>
          <p class="text-slate-400">
            Found {cacheCategories.length} cache categories totaling {formatSize(totalSize)}
          </p>
        </div>
        {#if selectedCount > 0}
          <div class="flex items-center space-x-4">
            <div class="text-right">
              <p class="text-sm text-slate-400">Selected for cleanup</p>
              <p class="text-lg font-semibold text-white">{formatSize(totalSelectedSize)}</p>
            </div>
            <button
              on:click={cleanSelectedCaches}
              class="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg font-medium transition-colors"
            >
              Clean Selected ({selectedCount})
            </button>
          </div>
        {/if}
      </div>

      <div class="space-y-4">
        {#each cacheCategories as category, categoryIndex}
          <div class="border border-slate-700 rounded-lg overflow-hidden">
            <div class="bg-slate-700/50 p-4">
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3">
                  <button
                    on:click={() => toggleCategory(categoryIndex)}
                    class="text-slate-300 hover:text-white transition-colors"
                    title="Select all entries in this category"
                  >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                    </svg>
                  </button>
                  <div>
                    <h3 class="text-lg font-semibold text-white">{category.display_name}</h3>
                    <p class="text-sm text-slate-400">{category.entry_count} entries • {formatSize(category.total_size_mb)}</p>
                  </div>
                </div>
                <div class="flex items-center space-x-2">
                  <span class="px-2 py-1 rounded-full text-xs font-medium {getSafetyBg(category.safety)} {getSafetyColor(category.safety)}">
                    {category.safety}
                  </span>
                </div>
              </div>
            </div>
            
            <div class="divide-y divide-slate-700">
              {#each category.entries as entry, entryIndex}
                <div class="p-4 hover:bg-slate-700/30 transition-colors">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-3 flex-1">
                      <input
                        type="checkbox"
                        checked={selectedCaches.has(`${categoryIndex}-${entryIndex}`)}
                        on:change={() => toggleCache(categoryIndex, entryIndex)}
                        class="w-4 h-4 text-blue-600 bg-slate-700 border-slate-600 rounded focus:ring-blue-500"
                      />
                      <div class="flex-1 min-w-0">
                        <p class="text-white font-medium truncate">{entry.path}</p>
                        <p class="text-sm text-slate-400">{entry.description}</p>
                      </div>
                    </div>
                    <div class="flex items-center space-x-3 text-right">
                      <div>
                        <p class="text-white font-medium">{formatSize(entry.size_mb)}</p>
                        <p class="text-xs {getSafetyColor(entry.safety)}">{entry.safety}</p>
                      </div>
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  {#if !$isScanning && !scanningComplete && !$selectedDirectory}
    <div class="bg-slate-800 rounded-xl shadow-lg p-8 text-center">
      <svg class="w-16 h-16 text-slate-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
      </svg>
      <h3 class="text-xl font-semibold text-white mb-2">Select Directory to Scan</h3>
      <p class="text-slate-400 mb-4">Choose a directory to analyze for developer caches and build artifacts.</p>
      <button
        on:click={selectDirectory}
        class="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
      >
        Select Directory
      </button>
    </div>
  {/if}
</div>
