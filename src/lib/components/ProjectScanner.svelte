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
        const found = await invoke('scan_git_repos', { opts: { root, follow_symlinks: false, min_bytes: 0 } });
        console.log('ProjectScanner: found', found.length, 'repos in', root);
        
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
              <button class="w-full text-left bg-slate-700/40 hover:bg-slate-700/60 rounded p-2"
                on:click={() => (selectedRepo = r)}>
                <div class="flex items-center justify-between">
                  <div class="truncate text-sm text-white" title={r.path}>{r.path}</div>
                  <div class="text-xs {computeHealth(r.status).color}">{computeHealth(r.status).label}</div>
                </div>
                <div class="text-xs text-slate-400">
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
        <div class="flex items-center justify-between mb-4">
          <div>
            <h2 class="text-white font-semibold">{selectedRepo.path}</h2>
            <p class="text-slate-400 text-sm">Branch: {(selectedRepo.status && selectedRepo.status.branch) || 'unknown'} • Ahead {(selectedRepo.status && selectedRepo.status.ahead)||0} / Behind {(selectedRepo.status && selectedRepo.status.behind)||0}</p>
          </div>
          <div class="space-x-2">
            <button class="bg-slate-700 hover:bg-slate-600 text-white text-sm px-3 py-2 rounded" on:click={scanProjects}>Rescan</button>
            <button class="bg-emerald-600 hover:bg-emerald-500 text-white text-sm px-3 py-2 rounded">Launch Fixer Agent</button>
          </div>
        </div>
        <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
          <div class="bg-slate-700/40 rounded p-3 text-sm">
            <div class="text-slate-300">Dirty</div>
            <div class="text-white font-semibold">{((selectedRepo.status && selectedRepo.status.uncommitted)||0) > 0 ? 'Yes' : 'No'}</div>
          </div>
          <div class="bg-slate-700/40 rounded p-3 text-sm">
            <div class="text-slate-300">Untracked</div>
            <div class="text-white font-semibold">{(selectedRepo.status && selectedRepo.status.untracked)||0}</div>
          </div>
          <div class="bg-slate-700/40 rounded p-3 text-sm">
            <div class="text-slate-300">Upstream</div>
            <div class="text-white font-semibold">{(selectedRepo.status && selectedRepo.status.has_upstream) ? 'Yes' : 'No'}</div>
          </div>
          <div class="bg-slate-700/40 rounded p-3 text-sm">
            <div class="text-slate-300">Last Commit</div>
            <div class="text-white font-semibold">{(selectedRepo.status && selectedRepo.status.last_commit_ts) ? new Date(selectedRepo.status.last_commit_ts*1000).toISOString().slice(0,10) : '—'}</div>
          </div>
          <div class="bg-slate-700/40 rounded p-3 text-sm">
            <div class="text-slate-300">.git Size</div>
            <div class="text-white font-semibold">{selectedRepo.size_mb.toFixed(1)} MB</div>
          </div>
          <div class="bg-slate-700/40 rounded p-3 text-sm">
            <div class="text-slate-300">AI Drift</div>
            <div class="text-white font-semibold">—</div>
          </div>
        </div>
      {:else}
        <div class="text-slate-400">Select a repository from the list.</div>
      {/if}
    </div>
  </div>
{/if}
