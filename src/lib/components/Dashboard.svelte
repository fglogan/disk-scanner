<script>
  import { invoke } from "@tauri-apps/api/core";
  import {
    diskInfo,
    summaryStats,
    bloatCategories,
    largeFiles,
    duplicates,
    isScanning,
    scanProgress,
    settings,
  } from "../stores.js";

  async function runManualScan() {
    isScanning.set(true);
    scanProgress.set("Getting disk info...");

    try {
      console.log("Starting scan on directory:", $settings.directories[0]);

      // Get fresh disk info
      const info = await invoke("get_disk_info");
      diskInfo.set(info);

      console.log("Running scans with params:", {
        root: $settings.directories[0],
        large_file_min_mb: $settings.min_large_file_size,
        dup_min_mb: $settings.min_dup_size,
      });

      // Run scans sequentially so UI updates progressively
      scanProgress.set("Scanning for bloat...");
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

      // Update summary stats
      const bloatTotal =
        bloatResults.reduce((sum, cat) => sum + cat.total_size_mb, 0) / 1024;
      const largeFilesTotal =
        largeFileResults.reduce((sum, file) => sum + file.size_mb, 0) / 1024;
      const dupTotal =
        dupResults.reduce((sum, set) => sum + set.total_savable_mb, 0) / 1024;

      console.log("✅ Scan completed successfully!");
      scanProgress.set("");
    } catch (e) {
      console.error("❌ Scan failed:", e);
      alert("Scan failed: " + e);
      scanProgress.set("");
    } finally {
      isScanning.set(false);
    }
  }

  function updateSummaryStats() {
    const bloatTotal =
      $bloatCategories.reduce((sum, cat) => sum + cat.total_size_mb, 0) / 1024;
    const largeFilesTotal =
      $largeFiles.reduce((sum, file) => sum + file.size_mb, 0) / 1024;
    const dupTotal =
      $duplicates.reduce((sum, set) => sum + set.total_savable_mb, 0) / 1024;

    summaryStats.set({
      project_bloat_gb: bloatTotal,
      project_bloat_count: $bloatCategories.length,
      large_files_gb: largeFilesTotal,
      large_files_count: $largeFiles.length,
      duplicates_gb: dupTotal,
      duplicates_count: $duplicates.length,
      total_cleanable_gb: bloatTotal + dupTotal,
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

<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
  <!-- Disk Usage Card -->
  <div class="lg:col-span-2 bg-slate-800 rounded-xl shadow-lg p-6">
    <h2 class="text-lg font-semibold text-white mb-4">Macintosh HD</h2>
    <div class="w-full bg-slate-700 rounded-full h-5 mb-4 overflow-hidden">
      <div
        class="bg-indigo-600 h-5"
        style="width: {$diskInfo.usage_pct}%;"
      ></div>
    </div>
    <div class="flex justify-between text-sm">
      <span class="text-slate-300">
        <span class="font-medium text-white">Used:</span>
        {$diskInfo.used_gb.toFixed(1)} GB
      </span>
      <span class="text-slate-300">
        <span class="font-medium text-white">Free:</span>
        {$diskInfo.free_gb.toFixed(1)} GB
      </span>
      <span class="text-slate-300">
        <span class="font-medium text-white">Total:</span>
        {$diskInfo.total_gb.toFixed(1)} GB
      </span>
    </div>
  </div>

  <!-- Scan Control Card -->
  <div
    class="bg-slate-800 rounded-xl shadow-lg p-6 flex flex-col justify-between"
  >
    <div>
      <h2 class="text-lg font-semibold text-white mb-4">Scanner Control</h2>
      <p class="text-sm text-slate-400 mb-2">
        Background monitoring is <span class="font-semibold text-green-400"
          >Active</span
        >.
      </p>
      <p class="text-sm text-slate-400">Next scan: 10:00 PM</p>
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
  <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
    <!-- Summary Item -->
    <div class="text-center">
      <p class="text-sm text-indigo-400 font-medium">Project Bloat</p>
      <p class="text-3xl font-bold text-white mt-1">
        {$summaryStats.project_bloat_gb.toFixed(1)} GB
      </p>
      <p class="text-xs text-slate-400">
        in {$summaryStats.project_bloat_count} projects
      </p>
    </div>
    <!-- Summary Item -->
    <div class="text-center">
      <p class="text-sm text-teal-400 font-medium">Large Files</p>
      <p class="text-3xl font-bold text-white mt-1">
        {$summaryStats.large_files_gb.toFixed(1)} GB
      </p>
      <p class="text-xs text-slate-400">
        {$summaryStats.large_files_count} files > 1GB
      </p>
    </div>
    <!-- Summary Item -->
    <div class="text-center">
      <p class="text-sm text-amber-400 font-medium">Duplicates</p>
      <p class="text-3xl font-bold text-white mt-1">
        {$summaryStats.duplicates_gb.toFixed(1)} GB
      </p>
      <p class="text-xs text-slate-400">
        {$summaryStats.duplicates_count} file sets
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
