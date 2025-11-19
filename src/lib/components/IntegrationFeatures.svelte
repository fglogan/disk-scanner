<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { writable } from 'svelte/store';
  
  // Feature states
  let activeTab = 'updates';
  let loading = false;
  let error = null;
  
  // Update notifications (BEAD-028)
  let updateInfo = null;
  let updateConfig = {
    auto_check: true,
    check_interval_hours: 24,
    auto_download: false,
    include_prereleases: false
  };
  let checkingUpdates = false;
  
  // Crash reporting (BEAD-029)
  let crashSettings = {
    enabled: false,
    include_stack_traces: false,
    include_system_info: true,
    include_context: true,
    anonymize_paths: true
  };
  let crashReports = [];
  
  // Analytics (BEAD-030)
  let analyticsConfig = {
    local_enabled: true,
    sharing_enabled: false,
    max_local_events: 10000
  };
  let analyticsMetrics = null;
  let featureUsage = {};
  
  // Backup detection (BEAD-031)
  let detectedBackups = [];
  let scanningBackups = false;
  
  // Cloud storage (BEAD-032)
  let cloudLocations = [];
  let cloudFiles = [];
  let scanningCloud = false;
  
  // External drives (BEAD-033)
  let externalDrives = [];
  let allDrives = [];
  let refreshingDrives = false;
  
  // Compression analysis (BEAD-034)
  let compressionStats = null;
  let analyzingCompression = false;
  let selectedPath = '';
  
  // File statistics (BEAD-035)
  let fileStats = null;
  let analyzingStats = false;
  let chartType = 'pie';
  
  // Check for updates
  async function checkForUpdates() {
    checkingUpdates = true;
    error = null;
    try {
      updateInfo = await invoke('check_for_updates');
    } catch (e) {
      error = `Failed to check for updates: ${e}`;
    } finally {
      checkingUpdates = false;
    }
  }
  
  // Update crash reporting settings
  async function updateCrashSettings() {
    loading = true;
    error = null;
    try {
      await invoke('update_crash_settings', { settings: crashSettings });
    } catch (e) {
      error = `Failed to update crash settings: ${e}`;
    } finally {
      loading = false;
    }
  }
  
  // Get analytics metrics
  async function getAnalyticsMetrics() {
    loading = true;
    error = null;
    try {
      analyticsMetrics = await invoke('get_analytics_metrics');
      featureUsage = await invoke('get_feature_usage');
    } catch (e) {
      error = `Failed to get analytics: ${e}`;
    } finally {
      loading = false;
    }
  }
  
  // Scan for backups
  async function scanForBackups() {
    scanningBackups = true;
    error = null;
    try {
      detectedBackups = await invoke('scan_for_backups', { path: selectedPath || '/' });
    } catch (e) {
      error = `Failed to scan for backups: ${e}`;
    } finally {
      scanningBackups = false;
    }
  }
  
  // Scan cloud storage
  async function scanCloudStorage() {
    scanningCloud = true;
    error = null;
    try {
      cloudLocations = await invoke('get_cloud_locations');
      if (selectedPath) {
        cloudFiles = await invoke('scan_cloud_files', { path: selectedPath });
      }
    } catch (e) {
      error = `Failed to scan cloud storage: ${e}`;
    } finally {
      scanningCloud = false;
    }
  }
  
  // List drives
  async function listDrives() {
    refreshingDrives = true;
    error = null;
    try {
      allDrives = await invoke('list_all_drives');
      externalDrives = await invoke('list_external_drives');
    } catch (e) {
      error = `Failed to list drives: ${e}`;
    } finally {
      refreshingDrives = false;
    }
  }
  
  // Eject drive
  async function ejectDrive(driveInfo) {
    loading = true;
    error = null;
    try {
      await invoke('eject_drive', { driveInfo });
      await listDrives(); // Refresh list
    } catch (e) {
      error = `Failed to eject drive: ${e}`;
    } finally {
      loading = false;
    }
  }
  
  // Analyze compression
  async function analyzeCompression() {
    if (!selectedPath) {
      error = 'Please select a directory first';
      return;
    }
    analyzingCompression = true;
    error = null;
    try {
      compressionStats = await invoke('analyze_compression', { path: selectedPath });
    } catch (e) {
      error = `Failed to analyze compression: ${e}`;
    } finally {
      analyzingCompression = false;
    }
  }
  
  // Analyze file statistics
  async function analyzeFileStats() {
    if (!selectedPath) {
      error = 'Please select a directory first';
      return;
    }
    analyzingStats = true;
    error = null;
    try {
      fileStats = await invoke('analyze_file_statistics', { path: selectedPath });
    } catch (e) {
      error = `Failed to analyze file statistics: ${e}`;
    } finally {
      analyzingStats = false;
    }
  }
  
  // Format bytes
  function formatBytes(bytes) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }
  
  // Select directory
  async function selectDirectory() {
    try {
      const selected = await invoke('select_directory');
      if (selected) {
        selectedPath = selected;
      }
    } catch (e) {
      error = `Failed to select directory: ${e}`;
    }
  }
  
  onMount(() => {
    // Initialize features
    if (activeTab === 'updates') checkForUpdates();
    if (activeTab === 'analytics') getAnalyticsMetrics();
    if (activeTab === 'drives') listDrives();
  });
</script>

<div class="integration-features">
  <div class="header">
    <h2 class="text-2xl font-bold text-gray-800 dark:text-gray-200">Integration Features</h2>
    <p class="text-gray-600 dark:text-gray-400">Advanced system integration and analysis tools</p>
  </div>
  
  {#if error}
    <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 mb-4">
      <p class="text-red-600 dark:text-red-400">{error}</p>
    </div>
  {/if}
  
  <!-- Tab Navigation -->
  <div class="tabs">
    <button 
      class="tab {activeTab === 'updates' ? 'active' : ''}"
      on:click={() => { activeTab = 'updates'; checkForUpdates(); }}
    >
      Updates
    </button>
    <button 
      class="tab {activeTab === 'crash' ? 'active' : ''}"
      on:click={() => activeTab = 'crash'}
    >
      Crash Reporting
    </button>
    <button 
      class="tab {activeTab === 'analytics' ? 'active' : ''}"
      on:click={() => { activeTab = 'analytics'; getAnalyticsMetrics(); }}
    >
      Analytics
    </button>
    <button 
      class="tab {activeTab === 'backups' ? 'active' : ''}"
      on:click={() => activeTab = 'backups'}
    >
      Backups
    </button>
    <button 
      class="tab {activeTab === 'cloud' ? 'active' : ''}"
      on:click={() => activeTab = 'cloud'}
    >
      Cloud Storage
    </button>
    <button 
      class="tab {activeTab === 'drives' ? 'active' : ''}"
      on:click={() => { activeTab = 'drives'; listDrives(); }}
    >
      External Drives
    </button>
    <button 
      class="tab {activeTab === 'compression' ? 'active' : ''}"
      on:click={() => activeTab = 'compression'}
    >
      Compression
    </button>
    <button 
      class="tab {activeTab === 'statistics' ? 'active' : ''}"
      on:click={() => activeTab = 'statistics'}
    >
      File Statistics
    </button>
  </div>
  
  <!-- Tab Content -->
  <div class="tab-content">
    {#if activeTab === 'updates'}
      <!-- BEAD-028: Update notifications -->
      <div class="feature-panel">
        <h3 class="text-lg font-semibold mb-4">Update Notifications</h3>
        
        <div class="mb-6">
          <label class="flex items-center space-x-2 mb-2">
            <input type="checkbox" bind:checked={updateConfig.auto_check} class="rounded">
            <span>Automatically check for updates</span>
          </label>
          <label class="flex items-center space-x-2">
            <input type="checkbox" bind:checked={updateConfig.auto_download} class="rounded">
            <span>Automatically download updates</span>
          </label>
        </div>
        
        <button 
          class="btn btn-primary mb-4"
          on:click={checkForUpdates}
          disabled={checkingUpdates}
        >
          {checkingUpdates ? 'Checking...' : 'Check for Updates'}
        </button>
        
        {#if updateInfo}
          <div class="update-info">
            <div class="grid grid-cols-2 gap-4">
              <div>
                <p class="text-sm text-gray-600 dark:text-gray-400">Current Version</p>
                <p class="font-mono">{updateInfo.current_version}</p>
              </div>
              <div>
                <p class="text-sm text-gray-600 dark:text-gray-400">Latest Version</p>
                <p class="font-mono">{updateInfo.latest_version}</p>
              </div>
            </div>
            
            {#if updateInfo.update_available}
              <div class="mt-4 p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
                <p class="font-semibold text-blue-700 dark:text-blue-300">Update Available!</p>
                {#if updateInfo.release_notes}
                  <p class="mt-2 text-sm">{updateInfo.release_notes}</p>
                {/if}
                {#if updateInfo.update_size}
                  <p class="mt-2 text-sm">Size: {formatBytes(updateInfo.update_size)}</p>
                {/if}
                <button class="btn btn-primary mt-3">Download Update</button>
              </div>
            {:else}
              <p class="mt-4 text-green-600 dark:text-green-400">✓ You're up to date!</p>
            {/if}
          </div>
        {/if}
      </div>
      
    {:else if activeTab === 'crash'}
      <!-- BEAD-029: Crash reporting -->
      <div class="feature-panel">
        <h3 class="text-lg font-semibold mb-4">Crash Reporting (Privacy-First)</h3>
        
        <div class="privacy-settings">
          <label class="flex items-center space-x-2 mb-2">
            <input type="checkbox" bind:checked={crashSettings.enabled} on:change={updateCrashSettings} class="rounded">
            <span>Enable crash reporting (opt-in)</span>
          </label>
          
          {#if crashSettings.enabled}
            <div class="ml-6 space-y-2">
              <label class="flex items-center space-x-2">
                <input type="checkbox" bind:checked={crashSettings.include_stack_traces} on:change={updateCrashSettings} class="rounded">
                <span>Include stack traces</span>
              </label>
              <label class="flex items-center space-x-2">
                <input type="checkbox" bind:checked={crashSettings.include_system_info} on:change={updateCrashSettings} class="rounded">
                <span>Include system information</span>
              </label>
              <label class="flex items-center space-x-2">
                <input type="checkbox" bind:checked={crashSettings.anonymize_paths} on:change={updateCrashSettings} class="rounded">
                <span>Anonymize file paths</span>
              </label>
            </div>
          {/if}
        </div>
        
        <div class="mt-6">
          <p class="text-sm text-gray-600 dark:text-gray-400">
            We respect your privacy. Crash reports are only sent with your explicit consent and can be anonymized.
          </p>
        </div>
      </div>
      
    {:else if activeTab === 'analytics'}
      <!-- BEAD-030: Analytics -->
      <div class="feature-panel">
        <h3 class="text-lg font-semibold mb-4">Privacy-Respecting Analytics</h3>
        
        <div class="privacy-settings mb-6">
          <label class="flex items-center space-x-2 mb-2">
            <input type="checkbox" bind:checked={analyticsConfig.local_enabled} class="rounded">
            <span>Enable local analytics</span>
          </label>
          <label class="flex items-center space-x-2">
            <input type="checkbox" bind:checked={analyticsConfig.sharing_enabled} class="rounded">
            <span>Share anonymous usage data (opt-in)</span>
          </label>
        </div>
        
        {#if analyticsMetrics}
          <div class="metrics-display">
            <h4 class="font-semibold mb-3">Performance Metrics</h4>
            
            <div class="grid grid-cols-2 gap-4">
              <div class="metric-card">
                <p class="text-sm text-gray-600 dark:text-gray-400">Scans Performed</p>
                <p class="text-2xl font-bold">{analyticsMetrics.scan_metrics.scan_count}</p>
                <p class="text-sm">Avg: {analyticsMetrics.scan_metrics.avg_scan_time_sec.toFixed(1)}s</p>
              </div>
              
              <div class="metric-card">
                <p class="text-sm text-gray-600 dark:text-gray-400">Files Cleaned</p>
                <p class="text-2xl font-bold">{analyticsMetrics.cleanup_metrics.total_files_deleted}</p>
                <p class="text-sm">Space: {formatBytes(analyticsMetrics.cleanup_metrics.total_space_cleaned)}</p>
              </div>
            </div>
            
            {#if Object.keys(featureUsage).length > 0}
              <h4 class="font-semibold mt-6 mb-3">Feature Usage</h4>
              <div class="space-y-2">
                {#each Object.entries(featureUsage) as [feature, count]}
                  <div class="flex justify-between">
                    <span>{feature}</span>
                    <span class="font-mono">{count}</span>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {/if}
      </div>
      
    {:else if activeTab === 'backups'}
      <!-- BEAD-031: Backup detection -->
      <div class="feature-panel">
        <h3 class="text-lg font-semibold mb-4">Backup Detection</h3>
        
        <div class="flex gap-2 mb-4">
          <button class="btn btn-secondary" on:click={selectDirectory}>
            Select Directory
          </button>
          <input 
            type="text" 
            bind:value={selectedPath} 
            placeholder="Path to scan"
            class="flex-1 px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
          >
          <button 
            class="btn btn-primary" 
            on:click={scanForBackups}
            disabled={scanningBackups || !selectedPath}
          >
            {scanningBackups ? 'Scanning...' : 'Scan'}
          </button>
        </div>
        
        {#if detectedBackups.length > 0}
          <div class="backups-list">
            {#each detectedBackups as backup}
              <div class="backup-item {backup.is_critical ? 'critical' : ''}">
                <div class="flex justify-between items-start">
                  <div>
                    <p class="font-semibold">{backup.backup_type}</p>
                    <p class="text-sm text-gray-600 dark:text-gray-400">{backup.path}</p>
                    <p class="text-sm mt-1">{backup.warning_message}</p>
                  </div>
                  <span class="text-sm">{formatBytes(backup.size)}</span>
                </div>
              </div>
            {/each}
          </div>
        {:else if scanningBackups}
          <p class="text-gray-600 dark:text-gray-400">Scanning for backups...</p>
        {/if}
      </div>
      
    {:else if activeTab === 'cloud'}
      <!-- BEAD-032: Cloud storage -->
      <div class="feature-panel">
        <h3 class="text-lg font-semibold mb-4">Cloud Storage Detection</h3>
        
        <button 
          class="btn btn-primary mb-4" 
          on:click={scanCloudStorage}
          disabled={scanningCloud}
        >
          {scanningCloud ? 'Scanning...' : 'Detect Cloud Storage'}
        </button>
        
        {#if cloudLocations.length > 0}
          <div class="cloud-locations">
            <h4 class="font-semibold mb-2">Detected Cloud Storage Locations</h4>
            {#each cloudLocations as [path, provider]}
              <div class="cloud-location">
                <span class="provider-badge {provider.toLowerCase()}">{provider}</span>
                <span class="text-sm">{path}</span>
              </div>
            {/each}
          </div>
        {/if}
        
        {#if cloudFiles.length > 0}
          <div class="cloud-files mt-6">
            <h4 class="font-semibold mb-2">Cloud Files</h4>
            {#each cloudFiles as file}
              <div class="cloud-file-item">
                <div class="flex justify-between">
                  <span class="text-sm">{file.path}</span>
                  <span class="sync-status {file.sync_status.toLowerCase()}">{file.sync_status}</span>
                </div>
                {#if file.sync_status === 'CloudOnly'}
                  <p class="text-xs text-orange-600 dark:text-orange-400 mt-1">⚠️ {file.warning_message}</p>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>
      
    {:else if activeTab === 'drives'}
      <!-- BEAD-033: External drives -->
      <div class="feature-panel">
        <h3 class="text-lg font-semibold mb-4">Drive Management</h3>
        
        <button 
          class="btn btn-primary mb-4" 
          on:click={listDrives}
          disabled={refreshingDrives}
        >
          {refreshingDrives ? 'Refreshing...' : 'Refresh Drives'}
        </button>
        
        {#if externalDrives.length > 0}
          <div class="drives-section mb-6">
            <h4 class="font-semibold mb-2">External Drives</h4>
            {#each externalDrives as drive}
              <div class="drive-item external">
                <div class="flex justify-between items-center">
                  <div>
                    <p class="font-semibold">{drive.name}</p>
                    <p class="text-sm text-gray-600 dark:text-gray-400">
                      {drive.file_system} · {formatBytes(drive.available_space)} free of {formatBytes(drive.total_space)}
                    </p>
                  </div>
                  {#if drive.supports_eject}
                    <button 
                      class="btn btn-sm btn-danger" 
                      on:click={() => ejectDrive(drive)}
                    >
                      Eject
                    </button>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        {/if}
        
        {#if allDrives.length > 0}
          <div class="drives-section">
            <h4 class="font-semibold mb-2">All Drives</h4>
            {#each allDrives.filter(d => !d.is_removable) as drive}
              <div class="drive-item">
                <div>
                  <p class="font-semibold">{drive.name}</p>
                  <p class="text-sm text-gray-600 dark:text-gray-400">
                    {drive.drive_type} · {drive.file_system} · {formatBytes(drive.available_space)} free
                  </p>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
      
    {:else if activeTab === 'compression'}
      <!-- BEAD-034: Compression analysis -->
      <div class="feature-panel">
        <h3 class="text-lg font-semibold mb-4">Compression Analysis</h3>
        
        <div class="flex gap-2 mb-4">
          <button class="btn btn-secondary" on:click={selectDirectory}>
            Select Directory
          </button>
          <input 
            type="text" 
            bind:value={selectedPath} 
            placeholder="Directory to analyze"
            class="flex-1 px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
          >
          <button 
            class="btn btn-primary" 
            on:click={analyzeCompression}
            disabled={analyzingCompression || !selectedPath}
          >
            {analyzingCompression ? 'Analyzing...' : 'Analyze'}
          </button>
        </div>
        
        {#if compressionStats}
          <div class="compression-results">
            <div class="summary-card">
              <h4 class="font-semibold mb-2">Compression Opportunity</h4>
              <p class="text-2xl font-bold text-green-600 dark:text-green-400">
                {formatBytes(compressionStats.total_potential_savings)}
              </p>
              <p class="text-sm text-gray-600 dark:text-gray-400">
                potential savings from {compressionStats.total_files} files
              </p>
            </div>
            
            {#if compressionStats.top_compressible_files.length > 0}
              <div class="mt-6">
                <h4 class="font-semibold mb-2">Top Compressible Files</h4>
                {#each compressionStats.top_compressible_files.slice(0, 5) as file}
                  <div class="compression-file">
                    <p class="text-sm font-mono">{file.path}</p>
                    <div class="flex justify-between text-sm">
                      <span>{formatBytes(file.original_size)} → {formatBytes(file.estimated_compressed_size)}</span>
                      <span class="text-green-600 dark:text-green-400">Save {formatBytes(file.potential_savings)}</span>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {/if}
      </div>
      
    {:else if activeTab === 'statistics'}
      <!-- BEAD-035: File statistics -->
      <div class="feature-panel">
        <h3 class="text-lg font-semibold mb-4">File Type Statistics</h3>
        
        <div class="flex gap-2 mb-4">
          <button class="btn btn-secondary" on:click={selectDirectory}>
            Select Directory
          </button>
          <input 
            type="text" 
            bind:value={selectedPath} 
            placeholder="Directory to analyze"
            class="flex-1 px-3 py-2 border rounded-lg dark:bg-gray-700 dark:border-gray-600"
          >
          <button 
            class="btn btn-primary" 
            on:click={analyzeFileStats}
            disabled={analyzingStats || !selectedPath}
          >
            {analyzingStats ? 'Analyzing...' : 'Analyze'}
          </button>
        </div>
        
        {#if fileStats}
          <div class="file-stats">
            <div class="stats-summary">
              <div class="stat-card">
                <p class="text-sm text-gray-600 dark:text-gray-400">Total Files</p>
                <p class="text-2xl font-bold">{fileStats.total_files.toLocaleString()}</p>
              </div>
              <div class="stat-card">
                <p class="text-sm text-gray-600 dark:text-gray-400">Total Size</p>
                <p class="text-2xl font-bold">{formatBytes(fileStats.total_size)}</p>
              </div>
            </div>
            
            <div class="chart-selector mt-4">
              <button 
                class="chart-btn {chartType === 'pie' ? 'active' : ''}" 
                on:click={() => chartType = 'pie'}
              >
                Pie Chart
              </button>
              <button 
                class="chart-btn {chartType === 'bar' ? 'active' : ''}" 
                on:click={() => chartType = 'bar'}
              >
                Bar Chart
              </button>
            </div>
            
            {#if chartType === 'pie' && fileStats.chart_data.size_by_category.length > 0}
              <div class="chart-container">
                <h4 class="font-semibold mb-2">Size by Category</h4>
                <!-- In a real implementation, would use a charting library -->
                <div class="category-list">
                  {#each fileStats.chart_data.size_by_category as segment}
                    <div class="category-item">
                      <div class="color-box" style="background-color: {segment.color}"></div>
                      <span>{segment.label}</span>
                      <span class="ml-auto">{segment.value.toFixed(1)}%</span>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}
            
            {#if fileStats.largest_files.length > 0}
              <div class="mt-6">
                <h4 class="font-semibold mb-2">Largest Files</h4>
                {#each fileStats.largest_files.slice(0, 5) as file}
                  <div class="large-file-item">
                    <p class="text-sm font-mono">{file.path}</p>
                    <p class="text-sm text-gray-600 dark:text-gray-400">{formatBytes(file.size)}</p>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .integration-features {
    @apply p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg;
  }
  
  .header {
    @apply mb-6;
  }
  
  .tabs {
    @apply flex gap-1 border-b border-gray-200 dark:border-gray-700 mb-6 overflow-x-auto;
  }
  
  .tab {
    @apply px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 
           hover:text-gray-900 dark:hover:text-gray-100 
           border-b-2 border-transparent transition-colors whitespace-nowrap;
  }
  
  .tab.active {
    @apply text-blue-600 dark:text-blue-400 border-blue-600 dark:border-blue-400;
  }
  
  .tab-content {
    @apply min-h-[400px];
  }
  
  .feature-panel {
    @apply space-y-4;
  }
  
  .btn {
    @apply px-4 py-2 rounded-lg font-medium transition-colors;
  }
  
  .btn-primary {
    @apply bg-blue-600 text-white hover:bg-blue-700 disabled:bg-gray-400;
  }
  
  .btn-secondary {
    @apply bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 
           hover:bg-gray-300 dark:hover:bg-gray-600;
  }
  
  .btn-danger {
    @apply bg-red-600 text-white hover:bg-red-700;
  }
  
  .btn-sm {
    @apply px-3 py-1 text-sm;
  }
  
  .update-info {
    @apply bg-gray-50 dark:bg-gray-700 rounded-lg p-4;
  }
  
  .privacy-settings {
    @apply space-y-3 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg;
  }
  
  .metric-card {
    @apply p-4 bg-gray-50 dark:bg-gray-700 rounded-lg;
  }
  
  .backup-item {
    @apply p-4 bg-gray-50 dark:bg-gray-700 rounded-lg mb-2;
  }
  
  .backup-item.critical {
    @apply border-2 border-red-300 dark:border-red-700 bg-red-50 dark:bg-red-900/20;
  }
  
  .cloud-location {
    @apply flex items-center gap-2 p-2 bg-gray-50 dark:bg-gray-700 rounded mb-2;
  }
  
  .provider-badge {
    @apply px-2 py-1 text-xs font-semibold rounded;
  }
  
  .provider-badge.icloud {
    @apply bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200;
  }
  
  .provider-badge.dropbox {
    @apply bg-indigo-100 text-indigo-800 dark:bg-indigo-900 dark:text-indigo-200;
  }
  
  .provider-badge.onedrive {
    @apply bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200;
  }
  
  .cloud-file-item {
    @apply p-3 bg-gray-50 dark:bg-gray-700 rounded mb-2;
  }
  
  .sync-status {
    @apply px-2 py-1 text-xs font-medium rounded;
  }
  
  .sync-status.synced {
    @apply bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200;
  }
  
  .sync-status.cloudonly {
    @apply bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200;
  }
  
  .drive-item {
    @apply p-4 bg-gray-50 dark:bg-gray-700 rounded-lg mb-2;
  }
  
  .drive-item.external {
    @apply border-2 border-blue-200 dark:border-blue-700;
  }
  
  .compression-file {
    @apply p-3 bg-gray-50 dark:bg-gray-700 rounded mb-2;
  }
  
  .summary-card {
    @apply p-6 bg-gradient-to-br from-green-50 to-emerald-50 
           dark:from-green-900/20 dark:to-emerald-900/20 
           rounded-lg text-center;
  }
  
  .stats-summary {
    @apply grid grid-cols-2 gap-4 mb-6;
  }
  
  .stat-card {
    @apply p-4 bg-gray-50 dark:bg-gray-700 rounded-lg text-center;
  }
  
  .chart-selector {
    @apply flex gap-2 mb-4;
  }
  
  .chart-btn {
    @apply px-3 py-1 text-sm rounded bg-gray-200 dark:bg-gray-700 
           hover:bg-gray-300 dark:hover:bg-gray-600;
  }
  
  .chart-btn.active {
    @apply bg-blue-600 text-white;
  }
  
  .category-list {
    @apply space-y-2 mt-4;
  }
  
  .category-item {
    @apply flex items-center gap-3;
  }
  
  .color-box {
    @apply w-4 h-4 rounded;
  }
  
  .large-file-item {
    @apply p-3 bg-gray-50 dark:bg-gray-700 rounded mb-2 
           flex justify-between items-center;
  }
</style>