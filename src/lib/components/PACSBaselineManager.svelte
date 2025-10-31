<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  
  // Types for baseline management
  interface ProjectBaseline {
    project_path: string;
    captured_at: string;
    version: string;
    file_inventory: Record<string, FileMetadata>;
    compliance_score: number;
    architecture_hash: string;
    dependencies: string[];
    standards_compliance: Record<string, boolean>;
  }
  
  interface FileMetadata {
    path: string;
    size: number;
    modified: string;
    hash: string;
    file_type: string;
    compliance_relevant: boolean;
  }
  
  interface BaselineComparison {
    baseline_score: number;
    current_score: number;
    score_change: number;
    files_added: string[];
    files_removed: string[];
    files_modified: string[];
    compliance_changes: Record<string, { old: boolean; new: boolean }>;
    recommendations: string[];
  }
  
  // Component props
  let { projectPath = '', currentReport = null } = $props<{ projectPath?: string; currentReport?: any }>();
  
  // Component state
  let baselines: ProjectBaseline[] = $state([]);
  let selectedBaseline: ProjectBaseline | null = $state(null);
  let comparison: BaselineComparison | null = $state(null);
  let isLoading = $state(false);
  let error: string | null = $state(null);
  let showCreateBaseline = $state(false);
  let baselineVersion = $state('');
  let baselineDescription = $state('');
  
  // Load available baselines on mount
  onMount(async () => {
    if (projectPath) {
      await loadBaselines();
    }
  });
  
  // Load all baselines for the project
  async function loadBaselines() {
    if (!projectPath) return;
    
    isLoading = true;
    error = null;
    
    try {
      baselines = await invoke<ProjectBaseline[]>('get_project_baselines', {
        projectPath: projectPath
      });
    } catch (e) {
      error = `Failed to load baselines: ${e}`;
      baselines = [];
    } finally {
      isLoading = false;
    }
  }
  
  // Create new baseline
  async function createBaseline() {
    if (!projectPath || !baselineVersion.trim()) {
      error = 'Please provide a version for the baseline';
      return;
    }
    
    isLoading = true;
    error = null;
    
    try {
      await invoke('create_project_baseline', {
        projectPath: projectPath,
        version: baselineVersion.trim(),
        description: baselineDescription.trim() || undefined
      });
      
      // Reload baselines
      await loadBaselines();
      
      // Reset form
      baselineVersion = '';
      baselineDescription = '';
      showCreateBaseline = false;
      
    } catch (e) {
      error = `Failed to create baseline: ${e}`;
    } finally {
      isLoading = false;
    }
  }
  
  // Compare current state with selected baseline
  async function compareWithBaseline(baseline: ProjectBaseline) {
    if (!currentReport) {
      error = 'No current report available for comparison';
      return;
    }
    
    isLoading = true;
    error = null;
    selectedBaseline = baseline;
    
    try {
      comparison = await invoke<BaselineComparison>('compare_with_baseline', {
        projectPath: projectPath,
        baselineVersion: baseline.version,
        currentReport: currentReport
      });
    } catch (e) {
      error = `Failed to compare with baseline: ${e}`;
      comparison = null;
    } finally {
      isLoading = false;
    }
  }
  
  // Delete baseline
  async function deleteBaseline(baseline: ProjectBaseline) {
    if (!confirm(`Are you sure you want to delete baseline "${baseline.version}"?`)) {
      return;
    }
    
    isLoading = true;
    error = null;
    
    try {
      await invoke('delete_project_baseline', {
        projectPath: projectPath,
        version: baseline.version
      });
      
      // Reload baselines
      await loadBaselines();
      
      // Clear comparison if it was for this baseline
      if (selectedBaseline?.version === baseline.version) {
        selectedBaseline = null;
        comparison = null;
      }
      
    } catch (e) {
      error = `Failed to delete baseline: ${e}`;
    } finally {
      isLoading = false;
    }
  }
  
  // Format date for display
  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleString();
  }
  
  // Get score change color
  function getScoreChangeColor(change: number): string {
    if (change > 0) return 'text-green-600';
    if (change < 0) return 'text-red-600';
    return 'text-gray-600';
  }
  
  // Get score change icon
  function getScoreChangeIcon(change: number): string {
    if (change > 0) return '📈';
    if (change < 0) return '📉';
    return '➡️';
  }
</script>

<div class="bg-white rounded-lg shadow-lg p-6">
  <div class="flex items-center justify-between mb-6">
    <div>
      <h3 class="text-xl font-bold text-gray-900">📊 Baseline Management</h3>
      <p class="text-gray-600">Track compliance changes over time</p>
    </div>
    <button
      onclick={() => showCreateBaseline = !showCreateBaseline}
      class="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 flex items-center gap-2"
    >
      📸 Create Baseline
    </button>
  </div>
  
  <!-- Create Baseline Form -->
  {#if showCreateBaseline}
    <div class="mb-6 p-4 bg-indigo-50 border border-indigo-200 rounded-lg">
      <h4 class="font-semibold text-indigo-900 mb-3">Create New Baseline</h4>
      <div class="space-y-3">
        <div>
          <label for="baseline-version" class="block text-sm font-medium text-gray-700 mb-1">
            Version *
          </label>
          <input
            id="baseline-version"
            type="text"
            bind:value={baselineVersion}
            placeholder="e.g., v1.0.0, release-candidate, pre-production"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
          />
        </div>
        <div>
          <label for="baseline-description" class="block text-sm font-medium text-gray-700 mb-1">
            Description (optional)
          </label>
          <textarea
            id="baseline-description"
            bind:value={baselineDescription}
            placeholder="Brief description of this baseline..."
            rows="2"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
          ></textarea>
        </div>
        <div class="flex gap-3">
          <button
            onclick={createBaseline}
            disabled={isLoading || !baselineVersion.trim()}
            class="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50"
          >
            {isLoading ? 'Creating...' : 'Create Baseline'}
          </button>
          <button
            onclick={() => showCreateBaseline = false}
            class="px-4 py-2 bg-gray-300 text-gray-700 rounded-md hover:bg-gray-400"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  {/if}
  
  <!-- Error Display -->
  {#if error}
    <div class="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg">
      <div class="text-red-700">{error}</div>
    </div>
  {/if}
  
  <!-- Baselines List -->
  {#if isLoading}
    <div class="text-center py-8">
      <div class="w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full animate-spin mx-auto mb-2"></div>
      <div class="text-gray-600">Loading baselines...</div>
    </div>
  {:else if baselines.length === 0}
    <div class="text-center py-8 bg-gray-50 rounded-lg">
      <div class="text-4xl mb-2">📊</div>
      <div class="text-lg font-semibold text-gray-700">No Baselines Found</div>
      <div class="text-gray-600">Create your first baseline to start tracking compliance changes</div>
    </div>
  {:else}
    <div class="space-y-4">
      <h4 class="font-semibold text-gray-900">Available Baselines ({baselines.length})</h4>
      
      <div class="grid gap-4">
        {#each baselines as baseline}
          <div class="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
            <div class="flex items-center justify-between">
              <div class="flex-1">
                <div class="flex items-center gap-3 mb-2">
                  <span class="font-semibold text-gray-900">{baseline.version}</span>
                  <span class="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded-full">
                    Score: {baseline.compliance_score.toFixed(1)}/100
                  </span>
                  <span class="text-sm text-gray-600">
                    {formatDate(baseline.captured_at)}
                  </span>
                </div>
                
                <div class="text-sm text-gray-600 mb-2">
                  <span class="font-medium">{Object.keys(baseline.file_inventory).length}</span> files tracked
                  • <span class="font-medium">{baseline.dependencies.length}</span> dependencies
                </div>
                
                <!-- Standards Compliance Summary -->
                <div class="flex gap-2 flex-wrap">
                  {#each Object.entries(baseline.standards_compliance) as [standard, compliant]}
                    <span class="px-2 py-1 text-xs rounded-full {compliant ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}">
                      {standard}: {compliant ? '✅' : '❌'}
                    </span>
                  {/each}
                </div>
              </div>
              
              <div class="flex gap-2 ml-4">
                <button
                  onclick={() => compareWithBaseline(baseline)}
                  disabled={!currentReport || isLoading}
                  class="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
                >
                  📊 Compare
                </button>
                <button
                  onclick={() => deleteBaseline(baseline)}
                  disabled={isLoading}
                  class="px-3 py-1 text-sm bg-red-600 text-white rounded hover:bg-red-700 disabled:opacity-50"
                >
                  🗑️
                </button>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
  
  <!-- Comparison Results -->
  {#if comparison && selectedBaseline}
    <div class="mt-6 p-6 bg-gradient-to-r from-blue-50 to-indigo-50 rounded-lg border border-blue-200">
      <h4 class="text-lg font-semibold text-blue-900 mb-4">
        📊 Comparison: Current vs {selectedBaseline.version}
      </h4>
      
      <!-- Score Comparison -->
      <div class="grid grid-cols-3 gap-4 mb-6">
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-600">{comparison.baseline_score.toFixed(1)}</div>
          <div class="text-sm text-gray-600">Baseline Score</div>
        </div>
        <div class="text-center">
          <div class="text-3xl">{getScoreChangeIcon(comparison.score_change)}</div>
          <div class="text-sm font-medium {getScoreChangeColor(comparison.score_change)}">
            {comparison.score_change > 0 ? '+' : ''}{comparison.score_change.toFixed(1)}
          </div>
        </div>
        <div class="text-center">
          <div class="text-2xl font-bold text-indigo-600">{comparison.current_score.toFixed(1)}</div>
          <div class="text-sm text-gray-600">Current Score</div>
        </div>
      </div>
      
      <!-- File Changes -->
      <div class="grid grid-cols-3 gap-4 mb-4">
        <div class="text-center p-3 bg-green-50 rounded border border-green-200">
          <div class="text-lg font-bold text-green-600">{comparison.files_added.length}</div>
          <div class="text-sm text-green-700">Files Added</div>
        </div>
        <div class="text-center p-3 bg-yellow-50 rounded border border-yellow-200">
          <div class="text-lg font-bold text-yellow-600">{comparison.files_modified.length}</div>
          <div class="text-sm text-yellow-700">Files Modified</div>
        </div>
        <div class="text-center p-3 bg-red-50 rounded border border-red-200">
          <div class="text-lg font-bold text-red-600">{comparison.files_removed.length}</div>
          <div class="text-sm text-red-700">Files Removed</div>
        </div>
      </div>
      
      <!-- Compliance Changes -->
      {#if Object.keys(comparison.compliance_changes).length > 0}
        <div class="mb-4">
          <h5 class="font-semibold text-blue-900 mb-2">Compliance Changes</h5>
          <div class="space-y-2">
            {#each Object.entries(comparison.compliance_changes) as [standard, change]}
              <div class="flex items-center gap-2 text-sm">
                <span class="font-medium">{standard}:</span>
                <span class="px-2 py-1 rounded text-xs {change.old ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}">
                  {change.old ? '✅' : '❌'}
                </span>
                <span>→</span>
                <span class="px-2 py-1 rounded text-xs {change.new ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}">
                  {change.new ? '✅' : '❌'}
                </span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
      
      <!-- Recommendations -->
      {#if comparison.recommendations.length > 0}
        <div>
          <h5 class="font-semibold text-blue-900 mb-2">Recommendations</h5>
          <ul class="space-y-1">
            {#each comparison.recommendations as recommendation}
              <li class="flex items-start gap-2 text-sm">
                <div class="w-2 h-2 bg-blue-500 rounded-full mt-2 flex-shrink-0"></div>
                <span class="text-blue-800">{recommendation}</span>
              </li>
            {/each}
          </ul>
        </div>
      {/if}
    </div>
  {/if}
</div>