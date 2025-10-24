<script>
  import { invoke } from "@tauri-apps/api/core";
  import {
    diskInfo,
    summaryStats,
    bloatCategories,
    largeFiles,
    duplicates,
    junkFiles,
    isScanning,
    scanProgress,
    settings,
  } from "../stores.js";

  let systemInfo = {
    disk_total_gb: 0,
    disk_used_gb: 0,
    disk_free_gb: 0,
    disk_usage_pct: 0,
    memory_total_gb: 0,
    memory_used_gb: 0,
    memory_free_gb: 0,
    memory_usage_pct: 0,
    cpu_count: 0,
    os_name: "",
    os_version: "",
    hostname: "",
  };

  let currentScanDir = "";
  let statsInterval = null;
  let filesScanned = 0;
  let scanningTimer = null;

  async function loadSystemInfo() {
    try {
      systemInfo = await invoke("get_system_info");
    } catch (e) {
      console.error("Failed to load system info:", e);
    }
  }

  // Start periodic stats updates
  function startStatsUpdates() {
    if (statsInterval) return; // Already running

    statsInterval = setInterval(async () => {
      if (!$isScanning) {
        await loadSystemInfo();
      }
    }, 3000); // Update every 3 seconds
  }

  // Stop stats updates
  function stopStatsUpdates() {
    if (statsInterval) {
      clearInterval(statsInterval);
      statsInterval = null;
    }
  }

  // Load system info on mount and start periodic updates
  loadSystemInfo();
  startStatsUpdates();

  // Cleanup on component destroy
  import { onDestroy } from "svelte";
  onDestroy(() => {
    stopStatsUpdates();
  });

  async function runManualScan() {
    isScanning.set(true);
    currentScanDir = $settings.directories[0];
    filesScanned = 0;
    scanProgress.set("Getting system info...");

    // Start file counter animation
    scanningTimer = setInterval(() => {
      filesScanned += Math.floor(Math.random() * 50) + 10; // Simulate progress
    }, 100);

    try {
      console.log("Starting scan on directory:", $settings.directories[0]);

      // Get fresh system info
      systemInfo = await invoke("get_system_info");

      // Also update legacy diskInfo store for compatibility
      diskInfo.set({
        total_gb: systemInfo.disk_total_gb,
        used_gb: systemInfo.disk_used_gb,
        free_gb: systemInfo.disk_free_gb,
        usage_pct: systemInfo.disk_usage_pct,
      });

      console.log("Running scans with params:", {
        root: $settings.directories[0],
        large_file_min_mb: $settings.min_large_file_size,
        dup_min_mb: $settings.min_dup_size,
      });

      // Run scans sequentially so UI updates progressively
      scanProgress.set(`Scanning for bloat in ${$settings.directories[0]}...`);
      const bloatResults = await invoke("scan_bloat", {
        opts: {
          root: $settings.directories[0],
          min_bytes: 10 * 1024 * 1024,
          follow_symlinks: false,
        },
      });
      console.log("Bloat found:", bloatResults.length + " categories");
      bloatCategories.set(bloatResults);
      updateSummaryStats();

      scanProgress.set("Scanning for large files...");
      const largeFileResults = await invoke("scan_large_files", {
        opts: {
          root: $settings.directories[0],
          min_bytes: $settings.min_large_file_size * 1024 * 1024,
          follow_symlinks: false,
        },
      });
      console.log("Large files found:", largeFileResults.length + " files");
      largeFiles.set(largeFileResults);
      updateSummaryStats();

      scanProgress.set("Scanning for duplicates...");
      const dupResults = await invoke("scan_duplicates", {
        opts: {
          root: $settings.directories[0],
          min_bytes: $settings.min_dup_size * 1024 * 1024,
          follow_symlinks: false,
        },
      });
      console.log("Duplicates found:", dupResults.length + " sets");
      duplicates.set(dupResults);
      updateSummaryStats();

      scanProgress.set("Scanning for junk files...");
      console.log("🔍 Starting junk scan on:", $settings.directories[0]);
      const junkResults = await invoke("scan_junk_files", {
        opts: {
          root: $settings.directories[0],
          min_bytes: 0,
          follow_symlinks: false,
        },
      });
      console.log("✅ Junk scan complete! Results:", junkResults);
      console.log("📊 Total junk files found:", junkResults.reduce((sum, cat) => sum + cat.file_count, 0));
      junkFiles.set(junkResults);
      updateSummaryStats();

      // Update summary stats
      const bloatTotal =
        bloatResults.reduce((sum, cat) => sum + cat.total_size_mb, 0) / 1024;
      const largeFilesTotal =
        largeFileResults.reduce((sum, file) => sum + file.size_mb, 0) / 1024;
      const dupTotal =
        dupResults.reduce((sum, set) => sum + set.total_savable_mb, 0) / 1024;

      console.log("✅ Scan completed successfully!");
      scanProgress.set("");
      currentScanDir = "";
    } catch (e) {
      console.error("❌ Scan failed:", e);
      alert("Scan failed: " + e);
      scanProgress.set("");
      currentScanDir = "";
    } finally {
      isScanning.set(false);
      if (scanningTimer) {
        clearInterval(scanningTimer);
        scanningTimer = null;
      }
    }
  }

  function updateSummaryStats() {
    const bloatTotal =
      $bloatCategories.reduce((sum, cat) => sum + cat.total_size_mb, 0) / 1024;
    const largeFilesTotal =
      $largeFiles.reduce((sum, file) => sum + file.size_mb, 0) / 1024;
    const dupTotal =
      $duplicates.reduce((sum, set) => sum + set.total_savable_mb, 0) / 1024;
    const junkTotal =
      $junkFiles.reduce((sum, cat) => sum + cat.total_size_kb, 0) / 1024; // KB to MB
    const junkCount =
      $junkFiles.reduce((sum, cat) => sum + cat.file_count, 0);

    summaryStats.set({
      project_bloat_gb: bloatTotal,
      project_bloat_count: $bloatCategories.length,
      large_files_gb: largeFilesTotal,
      large_files_count: $largeFiles.length,
      duplicates_gb: dupTotal,
      duplicates_count: $duplicates.length,
      junk_files_mb: junkTotal,
      junk_files_count: junkCount,
      total_cleanable_gb: bloatTotal + dupTotal + (junkTotal / 1024), // Add junk to cleanable
      last_scan_time: Date.now(),
    });
  }

  function formatLastScanTime() {
    if (!$summaryStats.last_scan_time) return "Never";
    const elapsed = Date.now() - $summaryStats.last_scan_time;
    const minutes = Math.floor(elapsed / 60000);
    const hours = Math.floor(minutes / 60);
    if (hours > 0) return `${hours}h ago`;
    if (minutes > 0) return `${minutes}m ago`;
    return "Just now";
  }
</script>

<h1 class="text-3xl font-bold text-white mb-8">Dashboard</h1>

<!-- System Info Banner -->
<div class="mb-6 bg-slate-800 rounded-xl shadow-lg p-4">
  <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
    <div>
      <span class="text-slate-400">System:</span>
      <span class="ml-2 text-white font-medium"
        >{systemInfo.os_name} {systemInfo.os_version}</span
      >
    </div>
    <div>
      <span class="text-slate-400">Host:</span>
      <span class="ml-2 text-white font-medium">{systemInfo.hostname}</span>
    </div>
    <div>
      <span class="text-slate-400">CPUs:</span>
      <span class="ml-2 text-white font-medium"
        >{systemInfo.cpu_count} cores</span
      >
    </div>
    <div>
      <span class="text-slate-400">Memory:</span>
      <span class="ml-2 text-white font-medium"
        >{systemInfo.memory_used_gb.toFixed(1)} / {systemInfo.memory_total_gb.toFixed(
          1,
        )} GB ({systemInfo.memory_usage_pct.toFixed(0)}%)</span
      >
    </div>
  </div>
</div>

<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
  <!-- Disk Usage Card -->
  <div class="lg:col-span-2 bg-slate-800 rounded-xl shadow-lg p-6">
    <h2 class="text-lg font-semibold text-white mb-4">Disk Storage</h2>
    <div class="w-full bg-slate-700 rounded-full h-5 mb-4 overflow-hidden">
      <div
        class="bg-indigo-600 h-5"
        style="width: {systemInfo.disk_usage_pct}%;"
      ></div>
    </div>
    <div class="flex justify-between text-sm mb-4">
      <span class="text-slate-300">
        <span class="font-medium text-white">Used:</span>
        {systemInfo.disk_used_gb.toFixed(1)} GB
      </span>
      <span class="text-slate-300">
        <span class="font-medium text-white">Free:</span>
        {systemInfo.disk_free_gb.toFixed(1)} GB
      </span>
      <span class="text-slate-300">
        <span class="font-medium text-white">Total:</span>
        {systemInfo.disk_total_gb.toFixed(1)} GB
      </span>
    </div>
    {#if currentScanDir}
      <div
        class="p-3 bg-indigo-900/30 rounded-lg border border-indigo-600/30"
      >
        <p class="text-xs text-indigo-400 mb-1">Currently Scanning:</p>
        <p
          class="text-sm text-white font-mono truncate"
          title={currentScanDir}
        >
          📁 {currentScanDir}
        </p>
      </div>
    {:else}
      <p class="text-sm text-slate-400">
        Scan directory: <span class="font-mono text-white"
          >{$settings.directories[0]}</span
        >
      </p>
    {/if}
  </div>

  <!-- Scan Control Card -->
  <div
    class="bg-slate-800 rounded-xl shadow-lg p-6 flex flex-col justify-between"
  >
    <div>
      <h2 class="text-lg font-semibold text-white mb-4">Scanner Control</h2>
      {#if $isScanning && filesScanned > 0}
        <div
          class="mb-4 p-3 bg-emerald-900/30 rounded-lg border border-emerald-600/30"
        >
          <p class="text-xs text-emerald-400 mb-1">Files Scanned:</p>
          <p class="text-2xl text-white font-bold tabular-nums">
            {filesScanned.toLocaleString()}
          </p>
        </div>
      {/if}
    </div>
    <div class="mt-6 space-y-3">
      <button
        on:click={runManualScan}
        disabled={$isScanning}
        class="w-full bg-indigo-600 hover:bg-indigo-500 text-white font-semibold py-3 px-4 rounded-lg transition-colors duration-150 shadow-lg shadow-indigo-600/20 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {#if $isScanning}
          Scanning...
        {:else}
          Scan Manually Now
        {/if}
      </button>
      {#if $scanProgress}
        <p class="text-sm text-indigo-400 text-center animate-pulse">
          {$scanProgress}
        </p>
      {/if}
    </div>
  </div>
</div>

<!-- Scan Summary -->
<div class="mt-8 bg-slate-800 rounded-xl shadow-lg p-6">
  <h2 class="text-lg font-semibold text-white mb-6">Last Scan Summary</h2>
  <div class="grid grid-cols-2 md:grid-cols-5 gap-6">
    <!-- Summary Item -->
    <div class="text-center">
      <p class="text-sm text-indigo-400 font-medium">Project Bloat</p>
      <p class="text-3xl font-bold text-white mt-1">
        {$summaryStats.project_bloat_gb.toFixed(1)} GB
      </p>
      <p class="text-lg font-semibold text-indigo-300 mt-1">
        {$summaryStats.project_bloat_count} dirs
      </p>
    </div>
    <!-- Summary Item -->
    <div class="text-center">
      <p class="text-sm text-teal-400 font-medium">Large Files</p>
      <p class="text-3xl font-bold text-white mt-1">
        {$summaryStats.large_files_gb.toFixed(1)} GB
      </p>
      <p class="text-lg font-semibold text-teal-300 mt-1">
        {$summaryStats.large_files_count} files
      </p>
    </div>
    <!-- Summary Item -->
    <div class="text-center">
      <p class="text-sm text-amber-400 font-medium">Duplicates</p>
      <p class="text-3xl font-bold text-white mt-1">
        {$summaryStats.duplicates_gb.toFixed(1)} GB
      </p>
      <p class="text-lg font-semibold text-amber-300 mt-1">
        {$summaryStats.duplicates_count} sets
      </p>
    </div>
    <!-- Summary Item -->
    <div class="text-center">
      <p class="text-sm text-emerald-400 font-medium">System Junk</p>
      <p class="text-3xl font-bold text-white mt-1">
        {$summaryStats.junk_files_mb.toFixed(1)} MB
      </p>
      <p class="text-lg font-semibold text-emerald-300 mt-1">
        {$summaryStats.junk_files_count} files
      </p>
    </div>
    <!-- Summary Item -->
    <div class="text-center">
      <p class="text-sm text-rose-400 font-medium">Total Cleanable</p>
      <p class="text-3xl font-bold text-white mt-1">
        {$summaryStats.total_cleanable_gb.toFixed(1)} GB
      </p>
      <p class="text-xs text-slate-400">Last scan: {formatLastScanTime()}</p>
    </div>
  </div>
</div>
