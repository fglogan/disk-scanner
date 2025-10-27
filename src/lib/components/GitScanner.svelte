<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { writable } from 'svelte/store';
  import { selectedDirectory, isScanning } from '../stores.js';

  let repositories = [];
  let selectedEntries = new Set();
  let totalSize = 0;
  let error = null;
  let scanningComplete = false;

  $: totalSelectedSize = Array.from(selectedEntries).reduce((sum, entryKey) => {
    const [repoIndex, entryIndex] = entryKey.split('-').map(Number);
    const entry = repositories[repoIndex]?.entries[entryIndex];
    return sum + (entry?.size_mb || 0);
  }, 0);

  $: selectedCount = selectedEntries.size;
  $: totalRepos = repositories.length;

  onMount(() => {
    if ($selectedDirectory) {
      scanGitRepos();
    }
  });

  async function selectDirectory() {
    try {
      const directory = await open({
        directory: true,
        multiple: false,
        title: 'Select Directory to Scan for Git Repositories'
      });
      
      if (directory) {
        selectedDirectory.set(directory);
        await scanGitRepos();
      }
    } catch (err) {
      error = `Failed to select directory: ${err.message}`;
    }
  }

  async function scanGitRepos() {
    if (!$selectedDirectory) return;
    
    try {
      isScanning.set(true);
      error = null;
      scanningComplete = false;
      selectedEntries.clear();
      
      const result = await invoke('scan_git_repos', {
        opts: {
          root: $selectedDirectory,
          followSymlinks: false
        }
      });
      
      repositories = result;
      totalSize = result.reduce((sum, repo) => sum + repo.total_size_mb, 0);
      scanningComplete = true;
      
    } catch (err) {
      error = `Failed to scan git repositories: ${err.message}`;
    } finally {
      isScanning.set(false);
    }
  }

  function toggleEntry(repoIndex, entryIndex) {
    const entryKey = `${repoIndex}-${entryIndex}`;
    if (selectedEntries.has(entryKey)) {
      selectedEntries.delete(entryKey);
    } else {
      selectedEntries.add(entryKey);
    }
    selectedEntries = selectedEntries; // Trigger reactivity
  }

  function toggleRepository(repoIndex) {
    const repo = repositories[repoIndex];
    repo.entries.forEach((_, entryIndex) => {
      const entryKey = `${repoIndex}-${entryIndex}`;
      if (repo.entries[entryIndex].actionable) {
        if (selectedEntries.has(entryKey)) {
          selectedEntries.delete(entryKey);
        } else {
          selectedEntries.add(entryKey);
        }
      }
    });
    selectedEntries = selectedEntries; // Trigger reactivity
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

  function getEntryTypeColor(entryType) {
    switch (entryType) {
      case 'large_file': return 'text-orange-400';
      case 'reflog': return 'text-blue-400';
      case 'objects': return 'text-purple-400';
      case 'pack_file': return 'text-cyan-400';
      case 'refs': return 'text-green-400';
      default: return 'text-gray-400';
    }
  }

  function getEntryTypeIcon(entryType) {
    switch (entryType) {
      case 'large_file':
        return 'M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z';
      case 'reflog':
        return 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z';
      case 'objects':
        return 'M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10';
      case 'pack_file':
        return 'M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4';
      case 'refs':
        return 'M13 10V3L4 14h7v7l9-11h-7z';
      default:
        return 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z';
    }
  }

  function formatSize(sizeMb) {
    if (sizeMb >= 1024) {
      return `${(sizeMb / 1024).toFixed(1)} GB`;
    }
    return `${sizeMb.toFixed(1)} MB`;
  }

  function getEntryTypeDescription(entryType) {
    switch (entryType) {
      case 'large_file': return 'Large file in git history';
      case 'reflog': return 'Git reflog - can be expired';
      case 'objects': return 'Git objects - core repository data';
      case 'pack_file': return 'Pack files - compressed objects';
      case 'refs': return 'Git references - branches & tags';
      case 'file': return 'Git metadata file';
      default: return 'Unknown entry type';
    }
  }

  async function cleanSelectedEntries() {
    if (selectedCount === 0) return;
    
    try {
      const selectedPaths = [];
      selectedEntries.forEach(entryKey => {
        const [repoIndex, entryIndex] = entryKey.split('-').map(Number);
        const entry = repositories[repoIndex]?.entries[entryIndex];
        if (entry && entry.actionable) {
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
      await scanGitRepos();
      
    } catch (err) {
      error = `Failed to clean git entries: ${err.message}`;
    }
  }

  function getCleanupSuggestion(entry) {
    if (!entry.actionable) {
      return 'Cannot be safely removed automatically';
    }
    
    switch (entry.entry_type) {
      case 'reflog':
        return 'Can be cleaned with: git reflog expire --expire=now';
      case 'large_file':
        return 'Can be removed with BFG Repo-Cleaner or git filter-branch';
      default:
        return 'Can be safely removed';
    }
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-3xl font-bold text-white mb-2">.git Repository Scanner</h1>
      <p class="text-slate-300">
        Scans `.git` folders for bloat, such as large files committed to history or
        un-packed objects. Some items can be safely cleaned up.
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
          on:click={scanGitRepos}
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

  {#if scanningComplete && repositories.length === 0}
    <div class="bg-slate-800 rounded-xl shadow-lg p-8 text-center">
      <svg class="w-16 h-16 text-slate-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
      <h3 class="text-xl font-semibold text-white mb-2">No Git Repositories Found</h3>
      <p class="text-slate-400">This directory doesn't contain any .git repositories.</p>
    </div>
  {/if}

  {#if scanningComplete && repositories.length > 0}
    <div class="bg-slate-800 rounded-xl shadow-lg p-6">
      <div class="flex items-center justify-between mb-6">
        <div>
          <h2 class="text-xl font-semibold text-white">Git Repository Analysis</h2>
          <p class="text-slate-400">
            Found {totalRepos} git repositories totaling {formatSize(totalSize)}
          </p>
        </div>
        {#if selectedCount > 0}
          <div class="flex items-center space-x-4">
            <div class="text-right">
              <p class="text-sm text-slate-400">Selected for cleanup</p>
              <p class="text-lg font-semibold text-white">{formatSize(totalSelectedSize)}</p>
            </div>
            <button
              on:click={cleanSelectedEntries}
              class="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg font-medium transition-colors"
            >
              Clean Selected ({selectedCount})
            </button>
          </div>
        {/if}
      </div>

      <div class="space-y-6">
        {#each repositories as repo, repoIndex}
          <div class="border border-slate-700 rounded-lg overflow-hidden">
            <div class="bg-slate-700/50 p-4">
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3">
                  <div class="flex items-center space-x-2">
                    <svg class="w-5 h-5 text-orange-400" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M22.65 14.39L12 22.13 1.35 14.39a.84.84 0 01-.3-.94l1.22-3.78 2.44-7.51A.42.42 0 014.82 2h.43a.43.43 0 01.32.14l9.34 10.46a.42.42 0 00.32.14h.43a.42.42 0 00.32-.14l9.34-10.46a.43.43 0 01.32-.14h.43a.42.42 0 01.39.32l2.44 7.51 1.22 3.78a.84.84 0 01-.21.9z"/>
                    </svg>
                    <span class="text-lg font-semibold text-white">Git Repository</span>
                  </div>
                  <div>
                    <p class="text-white font-medium">{repo.repo_path}</p>
                    <p class="text-sm text-slate-400">{repo.entry_count} entries • {formatSize(repo.total_size_mb)}</p>
                  </div>
                </div>
                <button
                  on:click={() => toggleRepository(repoIndex)}
                  class="text-slate-300 hover:text-white transition-colors"
                  title="Select all actionable items"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                  </svg>
                </button>
              </div>
            </div>
            
            <div class="divide-y divide-slate-700">
              {#each repo.entries as entry, entryIndex}
                <div class="p-4 hover:bg-slate-700/30 transition-colors">
                  <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-3 flex-1">
                      {#if entry.actionable}
                        <input
                          type="checkbox"
                          checked={selectedEntries.has(`${repoIndex}-${entryIndex}`)}
                          on:change={() => toggleEntry(repoIndex, entryIndex)}
                          class="w-4 h-4 text-blue-600 bg-slate-700 border-slate-600 rounded focus:ring-blue-500"
                        />
                      {:else}
                        <div class="w-4 h-4"></div>
                      {/if}
                      
                      <div class="flex items-center space-x-2">
                        <svg class="w-5 h-5 {getEntryTypeColor(entry.entry_type)}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={getEntryTypeIcon(entry.entry_type)}></path>
                        </svg>
                        <div class="flex-1 min-w-0">
                          <p class="text-white font-medium truncate">{entry.path}</p>
                          <p class="text-sm text-slate-400">{entry.description}</p>
                          <p class="text-xs {getEntryTypeColor(entry.entry_type)} mt-1">
                            {getEntryTypeDescription(entry.entry_type)}
                          </p>
                        </div>
                      </div>
                    </div>
                    <div class="flex items-center space-x-3 text-right">
                      <div>
                        <p class="text-white font-medium">{formatSize(entry.size_mb)}</p>
                        <p class="text-xs {getSafetyColor(entry.safety)}">{entry.safety}</p>
                        {#if !entry.actionable}
                          <p class="text-xs text-slate-500 mt-1">Not removable</p>
                        {:else}
                          <p class="text-xs text-blue-400 mt-1">Removable</p>
                        {/if}
                      </div>
                    </div>
                  </div>
                  
                  {#if entry.actionable && entry.entry_type === 'large_file'}
                    <div class="mt-3 p-3 bg-yellow-900/30 border border-yellow-700 rounded-lg">
                      <p class="text-xs text-yellow-200">
                        <strong>Cleanup suggestion:</strong> {getCleanupSuggestion(entry)}
                      </p>
                    </div>
                  {/if}
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
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
      </svg>
      <h3 class="text-xl font-semibold text-white mb-2">Select Directory to Scan</h3>
      <p class="text-slate-400 mb-4">Choose a directory to analyze for Git repositories and their storage usage.</p>
      <button
        on:click={selectDirectory}
        class="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
      >
        Select Directory
      </button>
    </div>
  {/if}
</div>
