import { writable } from "svelte/store";

// Navigation & UI State
export const currentPage = writable("dashboard");
export const isScanning = writable(false);
export const scanProgress = writable("");
export const scanError = writable(null);

// Scan Results
export const diskInfo = writable({
  total_gb: 0,
  used_gb: 0,
  free_gb: 0,
  usage_pct: 0,
});

export const largeFiles = writable([]);
export const bloatCategories = writable([]);
export const duplicates = writable([]);
export const junkFiles = writable([]); // System junk, build artifacts, editor files

// Settings
export const settings = writable({
  directories: [], // User must select directories via UI (was hardcoded to /Users/frank/...)
  min_dup_size: 1, // 1 MB (lowered from 10)
  min_large_file_size: 100, // 100 MB (lowered from 1024)
  ignore_patterns: ["*.log", "*/.git/*"],
  bg_monitor_enabled: true,
  scan_interval: "12h",
});

// Selection State
export const selectedPaths = writable(new Set());

// Summary Stats
export const summaryStats = writable({
  project_bloat_gb: 0,
  project_bloat_count: 0,
  large_files_gb: 0,
  large_files_count: 0,
  duplicates_mb: 0, // Changed from duplicates_gb to duplicates_mb (auto-scales to GB when >= 1024 MB)
  duplicates_count: 0,
  junk_files_mb: 0,
  junk_files_count: 0,
  total_cleanable_gb: 0,
  last_scan_time: null,
});
