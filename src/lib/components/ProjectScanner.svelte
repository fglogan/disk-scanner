<script>
  import { invoke } from '@tauri-apps/api/core';
  import { settings } from '../stores.js';

  let repos = [];
  let selectedRepo = null;
  let loading = false;
  let error = '';

  function groupByActivity(items) {
    const now = Date.now() / 1000;
    const groups = { Active: [], Recent: [], Idle: [], Cold: [] };
    for (const r of items) {
      const ts = r.status && r.status.last_commit_ts ? r.status.last_commit_ts : 0;
      const age = ts ? now - ts : Number.POSITIVE_INFINITY;
      if (age < 7 * 86400) groups.Active.push(r);
      else if (age < 30 * 86400) groups.Recent.push(r);
      else if (age < 180 * 86400) groups.Idle.push(r);
      else groups.Cold.push(r);
    }
    return groups;
  }

  function computeHealth(status) {
    if (!status) return { label: 'Unknown', color: 'text-slate-400' };
    if (status.uncommitted > 0 || status.untracked > 5) return { label: 'Warning', color: 'text-red-400' };
    if (!status.has_upstream) return { label: 'Caution', color: 'text-amber-400' };
    if (status.behind > 0) return { label: 'Fair', color: 'text-yellow-300' };
    return { label: 'OK', color: 'text-emerald-400' };
  }

  function getRepoName(path) {
    return path.split('/').filter(Boolean).pop() || path;
  }

  function getRepoDescription(path) {
    // Extract simple description from path structure
    const parts = path.split('/').filter(Boolean);
    if (parts.length > 2) {
      return `${parts[parts.length - 2]}/${parts[parts.length - 1]}`;
    }
    return path;
  }

  function detectProjectType(path, status) {
    // Simple heuristics - could be enhanced by checking actual files
    const name = path.toLowerCase();
    if (name.includes('frontend') || name.includes('ui') || name.includes('web')) {
      return 'Frontend';
    }
    if (name.includes('backend') || name.includes('api') || name.includes('server')) {
      return 'Backend';
    }
    if (name.includes('mobile') || name.includes('ios') || name.includes('android')) {
      return 'Mobile';
    }
    return 'Full Stack';
  }

  function getActivityLabel(ts) {
    if (!ts) return 'Never';
    const now = Date.now() / 1000;
    const age = now - ts;
    if (age < 86400) return 'Today';
    if (age < 7 * 86400) return 'This week';
    if (age < 30 * 86400) return 'This month';
    if (age < 180 * 86400) return `${Math.floor(age / 30 / 86400)} months ago`;
    return `${Math.floor(age / 365 / 86400)} years ago`;
  }

  async function testScan() {
    console.log('=== TEST SCAN ===');
    console.log('Settings:', $settings);
    console.log('Directories:', $settings.directories);
    
    const testPath = '/Volumes/Tempext-Projects/Users/tempext/Projects';
    console.log('Testing with hardcoded path:', testPath);
    
    try {
      const result = await invoke('scan_git_repos', { 
        opts: { root: testPath, follow_symlinks: false, min_bytes: 0 } 
      });
      console.log('Test scan result:', result);
      alert(`Test scan found ${result.length} repositories!`);
    } catch (e) {
      console.error('Test scan failed:', e);
      alert('Test scan failed: ' + e);
    }
  }

  async function scanProjects() {
    error = '';
    loading = true;
    repos = [];
    try {
      const roots = $settings.directories || [];
      console.log('ProjectScanner: scanning roots:', roots);
      console.log('ProjectScanner: roots type:', typeof roots, Array.isArray(roots));
      console.log('ProjectScanner: roots length:', roots.length);
      
      if (roots.length === 0) {
        error = 'No directories configured. Go to Settings and add directories to scan.';
        loading = false;
        return;
      }
      
      const seen = new Set();
      for (const root of roots) {
        console.log('ProjectScanner: scanning root:', root);
        console.log('ProjectScanner: invoking scan_git_repos with opts:', { root, follow_symlinks: false, min_bytes: 0 });
        try {
          const found = await invoke('scan_git_repos', { opts: { root, follow_symlinks: false, min_bytes: 0 } });
          console.log('ProjectScanner: SUCCESS - found', found.length, 'repos in', root);
          console.log('ProjectScanner: found repos:', found);
        
          for (const repo of found) {
            if (!seen.has(repo.repo_path)) {
              seen.add(repo.repo_path);
              let status = null;
              try {
                status = await invoke('get_git_repo_status', { path: repo.repo_path });
              } catch (e) {
                console.warn('Failed to get status for', repo.repo_path, e);
              }
              repos = [...repos, { path: repo.repo_path, size_mb: repo.total_size_mb, entries: repo.entries, status }];
            }
          }
        } catch (err) {
          console.error('ProjectScanner: ERROR scanning root:', root, err);
          error = `Failed to scan ${root}: ${err}`;
        }
      }
      console.log('ProjectScanner: total repos found:', repos.length);
    } catch (e) {
      console.error('ProjectScanner error:', e);
      error = String(e);
    } finally {
      loading = false;
    }
  }

  $: groups = groupByActivity(repos);
</script>

<div class="flex justify-between items-center mb-6">
  <h1 class="text-3xl font-bold text-white">Project Scanner</h1>
  <div class="flex items-center gap-4">
    <span class="text-sm text-slate-400">
      {$settings.directories?.length || 0} director{($settings.directories?.length || 0) === 1 ? 'y' : 'ies'} configured
    </span>
    <button on:click={() => console.log('Settings state:', $settings)}
      class="bg-slate-600 hover:bg-slate-500 text-white px-3 py-2 rounded text-sm">
      Debug
    </button>
    <button on:click={testScan}
      class="bg-amber-600 hover:bg-amber-500 text-white px-3 py-2 rounded text-sm">
      Test
    </button>
    <button on:click={scanProjects} disabled={loading || !($settings.directories && $settings.directories.length)}
      class="bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white px-4 py-2 rounded-lg">
      {#if loading}Scanning...{:else}Scan Projects{/if}
    </button>
  </div>
</div>

{#if error}
  <div class="bg-red-900/40 border border-red-700 rounded-lg p-3 text-red-200 text-sm">{error}</div>
{/if}

{#if repos.length === 0 && !loading}
  <div class="bg-slate-800 rounded-xl p-6 text-slate-300">No repositories found. Select directories in Settings and scan.</div>
{:else}
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Left: grouped list -->
    <div class="bg-slate-800 rounded-xl p-4 space-y-4">
      {#each Object.keys(groups) as key}
        <div>
          <div class="flex items-center justify-between mb-2">
            <h3 class="text-white font-semibold">{key}</h3>
            <span class="text-xs text-slate-400">{groups[key].length}</span>
          </div>
          <div class="space-y-2 max-h-64 overflow-auto pr-1">
            {#each groups[key] as r}
              <button class="w-full text-left bg-slate-700/40 hover:bg-slate-700/60 rounded p-2 transition-colors"
                class:ring-2={selectedRepo === r}
                class:ring-indigo-500={selectedRepo === r}
                on:click={() => (selectedRepo = r)}>
                <div class="flex items-center justify-between">
                  <div class="text-sm text-white font-medium">{getRepoName(r.path)}</div>
                  <div class="text-xs {computeHealth(r.status).color}">{computeHealth(r.status).label}</div>
                </div>
                <div class="text-xs text-slate-400 truncate" title={getRepoDescription(r.path)}>
                  {getRepoDescription(r.path)}
                </div>
                <div class="text-xs text-slate-500 mt-1">
                  {(r.status && r.status.branch) || 'unknown'} • {r.size_mb.toFixed(1)} MB
                </div>
              </button>
            {/each}
          </div>
        </div>
      {/each}
    </div>

    <!-- Right: details -->
    <div class="lg:col-span-2 bg-slate-800 rounded-xl p-6" >
      {#if selectedRepo}
        <!-- Header with full path -->
        <div class="mb-6">
          <h2 class="text-2xl font-bold text-white mb-1">{getRepoName(selectedRepo.path)}</h2>
          <p class="text-slate-400 text-sm font-mono mb-2">{selectedRepo.path}</p>
          <div class="flex items-center gap-4 text-sm">
            <span class="text-slate-300">
              <span class="text-slate-500">Branch:</span> 
              <span class="text-white font-medium">{(selectedRepo.status && selectedRepo.status.branch) || 'unknown'}</span>
            </span>
            <span class="text-slate-300">
              <span class="text-slate-500">↑</span> {(selectedRepo.status && selectedRepo.status.ahead)||0}
              <span class="text-slate-500 ml-1">↓</span> {(selectedRepo.status && selectedRepo.status.behind)||0}
            </span>
            <span class="px-2 py-1 rounded text-xs {computeHealth(selectedRepo.status).color} bg-slate-700/60">
              {computeHealth(selectedRepo.status).label}
            </span>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="flex gap-2 mb-6">
          <button class="bg-slate-700 hover:bg-slate-600 text-white text-sm px-4 py-2 rounded" on:click={scanProjects}>
            Rescan
          </button>
          <button class="bg-emerald-600 hover:bg-emerald-500 text-white text-sm px-4 py-2 rounded">
            Launch Fixer Agent
          </button>
          <button class="bg-blue-600 hover:bg-blue-500 text-white text-sm px-4 py-2 rounded">
            Open in Editor
          </button>
        </div>

        <!-- Status Cards Grid -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-6">
          <!-- Git Status -->
          <div class="bg-slate-700/40 rounded-lg p-4">
            <div class="text-slate-400 text-xs mb-1">Repository Status</div>
            <div class="text-white font-semibold text-lg">
              {((selectedRepo.status && selectedRepo.status.uncommitted)||0) > 0 ? 'Dirty' : 'Clean'}
            </div>
            <div class="text-slate-500 text-xs mt-1">
              {(selectedRepo.status && selectedRepo.status.uncommitted)||0} uncommitted
            </div>
          </div>

          <!-- Untracked Files -->
          <div class="bg-slate-700/40 rounded-lg p-4">
            <div class="text-slate-400 text-xs mb-1">Untracked Files</div>
            <div class="text-white font-semibold text-lg">{(selectedRepo.status && selectedRepo.status.untracked)||0}</div>
            <div class="text-slate-500 text-xs mt-1">
              {(selectedRepo.status && selectedRepo.status.untracked) > 0 ? 'Need review' : 'All tracked'}
            </div>
          </div>

          <!-- Upstream -->
          <div class="bg-slate-700/40 rounded-lg p-4">
            <div class="text-slate-400 text-xs mb-1">Remote Tracking</div>
            <div class="text-white font-semibold text-lg">
              {(selectedRepo.status && selectedRepo.status.has_upstream) ? 'Yes' : 'No'}
            </div>
            <div class="text-slate-500 text-xs mt-1">
              {(selectedRepo.status && selectedRepo.status.has_upstream) ? 'Tracked' : 'Local only'}
            </div>
          </div>

          <!-- Last Activity -->
          <div class="bg-slate-700/40 rounded-lg p-4">
            <div class="text-slate-400 text-xs mb-1">Last Commit</div>
            <div class="text-white font-semibold text-sm">
              {getActivityLabel(selectedRepo.status && selectedRepo.status.last_commit_ts)}
            </div>
            <div class="text-slate-500 text-xs mt-1">
              {(selectedRepo.status && selectedRepo.status.last_commit_ts) 
                ? new Date(selectedRepo.status.last_commit_ts*1000).toLocaleDateString() 
                : '—'}
            </div>
          </div>
        </div>

        <!-- Additional Info Grid -->
        <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
          <!-- Storage -->
          <div class="bg-slate-700/40 rounded-lg p-4">
            <div class="text-slate-400 text-xs mb-1">Storage</div>
            <div class="text-white font-semibold text-lg">{selectedRepo.size_mb.toFixed(1)} MB</div>
            <div class="text-slate-500 text-xs mt-1">.git directory</div>
          </div>

          <!-- Project Type -->
          <div class="bg-slate-700/40 rounded-lg p-4">
            <div class="text-slate-400 text-xs mb-1">Project Type</div>
            <div class="text-white font-semibold text-sm">{detectProjectType(selectedRepo.path, selectedRepo.status)}</div>
            <div class="text-slate-500 text-xs mt-1">Auto-detected</div>
          </div>

          <!-- Sync Status -->
          <div class="bg-slate-700/40 rounded-lg p-4">
            <div class="text-slate-400 text-xs mb-1">Sync Status</div>
            <div class="text-white font-semibold text-sm">
              {#if !selectedRepo.status || !selectedRepo.status.has_upstream}
                Local Only
              {:else if (selectedRepo.status.ahead === 0 && selectedRepo.status.behind === 0)}
                In Sync
              {:else if selectedRepo.status.behind > 0}
                Behind
              {:else}
                Ahead
              {/if}
            </div>
            <div class="text-slate-500 text-xs mt-1">
              {#if selectedRepo.status && selectedRepo.status.has_upstream}
                {selectedRepo.status.ahead}↑ {selectedRepo.status.behind}↓
              {:else}
                No remote
              {/if}
            </div>
          </div>

          <!-- Dependencies (placeholder) -->
          <div class="bg-slate-700/40 rounded-lg p-4">
            <div class="text-slate-400 text-xs mb-1">Dependencies</div>
            <div class="text-white font-semibold text-sm">Unknown</div>
            <div class="text-slate-500 text-xs mt-1">Scan needed</div>
          </div>

          <!-- Compliance (placeholder) -->
          <div class="bg-slate-700/40 rounded-lg p-4">
            <div class="text-slate-400 text-xs mb-1">Compliance</div>
            <div class="text-white font-semibold text-sm">Not Scanned</div>
            <div class="text-slate-500 text-xs mt-1">PACS required</div>
          </div>

          <!-- Code Health (placeholder) -->
          <div class="bg-slate-700/40 rounded-lg p-4">
            <div class="text-slate-400 text-xs mb-1">Code Health</div>
            <div class="text-white font-semibold text-sm">Unknown</div>
            <div class="text-slate-500 text-xs mt-1">Analysis needed</div>
          </div>
        </div>

        <!-- Git Entries Section -->
        {#if selectedRepo.entries && selectedRepo.entries.length > 0}
          <div class="mt-6">
            <h3 class="text-white font-semibold mb-3">Git Storage Details</h3>
            <div class="space-y-2 max-h-64 overflow-auto">
              {#each selectedRepo.entries as entry}
                <div class="bg-slate-700/30 rounded p-3 text-sm">
                  <div class="flex justify-between items-start mb-1">
                    <span class="text-white font-medium">{entry.entry_type}</span>
                    <span class="text-slate-300">{entry.size_mb.toFixed(2)} MB</span>
                  </div>
                  <div class="text-slate-400 text-xs">{entry.description}</div>
                  {#if entry.safety !== 'safe'}
                    <div class="text-amber-400 text-xs mt-1">⚠️ {entry.safety}</div>
                  {/if}
                </div>
              {/each}
            </div>
          </div>
        {/if}
      {:else}
        <div class="text-slate-400">Select a repository from the list.</div>
      {/if}
    </div>
  </div>
{/if}
