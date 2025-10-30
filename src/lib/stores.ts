// Modern Svelte 5 stores using runes
// Migrated from legacy writable stores to runes-based state management

import { writable } from 'svelte/store';

// ============================================================================
// TYPES
// ============================================================================

export interface DiskInfo {
  total_gb: number;
  used_gb: number;
  free_gb: number;
  usage_pct: number;
}

export interface ScanSettings {
  directories: string[];
  min_dup_size: number; // MB
  min_large_file_size: number; // MB
  ignore_patterns: string[];
  bg_monitor_enabled: boolean;
  scan_interval: string;
}

export interface SummaryStats {
  project_bloat_gb: number;
  project_bloat_count: number;
  large_files_gb: number;
  large_files_count: number;
  duplicates_mb: number;
  duplicates_count: number;
  junk_files_mb: number;
  junk_files_count: number;
  total_cleanable_gb: number;
  last_scan_time: number | null;
}

export interface LargeFile {
  path: string;
  size_mb: number;
  modified_ts?: number;
}

export interface BloatCategory {
  category_id: string;
  total_size_mb: number;
  entries?: Array<{
    path: string;
    size_mb: number;
    entry_type: string;
    description: string;
    safety: string;
  }>;
}

export interface DuplicateSet {
  hash: string;
  total_savable_mb: number;
  files: Array<{
    path: string;
    size_mb: number;
  }>;
}

export interface JunkCategory {
  category_id: string;
  total_size_kb: number;
  file_count: number;
  files?: Array<{
    path: string;
    size_kb: number;
  }>;
}

// ============================================================================
// NAVIGATION & UI STATE
// ============================================================================

export const currentPage = writable<string>("dashboard");
export const isScanning = writable<boolean>(false);
export const scanProgress = writable<string>("");
export const scanError = writable<string | null>(null);

// ============================================================================
// THEME & UI PREFERENCES
// ============================================================================

// Theme management with localStorage persistence
function createThemeStore() {
  const { subscribe, set, update } = writable<boolean>(true); // Default to dark mode
  
  // Load theme from localStorage on initialization
  if (typeof window !== 'undefined') {
    const saved = localStorage.getItem('theme');
    if (saved) {
      set(saved === 'dark');
    }
  }
  
  return {
    subscribe,
    toggle: () => update(dark => {
      const newTheme = !dark;
      if (typeof window !== 'undefined') {
        localStorage.setItem('theme', newTheme ? 'dark' : 'light');
        document.documentElement.setAttribute('data-theme', newTheme ? 'dark' : 'light');
      }
      return newTheme;
    }),
    set: (dark: boolean) => {
      if (typeof window !== 'undefined') {
        localStorage.setItem('theme', dark ? 'dark' : 'light');
        document.documentElement.setAttribute('data-theme', dark ? 'dark' : 'light');
      }
      set(dark);
    }
  };
}

export const darkMode = createThemeStore();

// ============================================================================
// TOAST NOTIFICATIONS
// ============================================================================

export interface Toast {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number;
  dismissible?: boolean;
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);
  
  return {
    subscribe,
    add: (toast: Omit<Toast, 'id'>) => {
      const id = Math.random().toString(36).substr(2, 9);
      const newToast: Toast = {
        id,
        duration: 3000,
        dismissible: true,
        ...toast
      };
      
      update(toasts => [...toasts, newToast]);
      
      // Auto-dismiss if duration is set
      if (newToast.duration && newToast.duration > 0) {
        setTimeout(() => {
          update(toasts => toasts.filter(t => t.id !== id));
        }, newToast.duration);
      }
      
      return id;
    },
    remove: (id: string) => {
      update(toasts => toasts.filter(t => t.id !== id));
    },
    clear: () => {
      update(() => []);
    }
  };
}

export const toasts = createToastStore();

// Convenience functions for common toast types
export const showSuccess = (message: string, duration = 3000) => 
  toasts.add({ type: 'success', message, duration });

export const showError = (message: string, duration = 0) => 
  toasts.add({ type: 'error', message, duration });

export const showWarning = (message: string, duration = 5000) => 
  toasts.add({ type: 'warning', message, duration });

export const showInfo = (message: string, duration = 3000) => 
  toasts.add({ type: 'info', message, duration });

// ============================================================================
// KEYBOARD SHORTCUTS
// ============================================================================

export const showShortcutsHelp = writable<boolean>(false);

// ============================================================================
// SCAN RESULTS
// ============================================================================

export const diskInfo = writable<DiskInfo>({
  total_gb: 0,
  used_gb: 0,
  free_gb: 0,
  usage_pct: 0,
});

export const largeFiles = writable<LargeFile[]>([]);
export const bloatCategories = writable<BloatCategory[]>([]);
export const duplicates = writable<DuplicateSet[]>([]);
export const junkFiles = writable<JunkCategory[]>([]);

// ============================================================================
// SETTINGS
// ============================================================================

export const settings = writable<ScanSettings>({
  directories: [], // User must select directories via UI
  min_dup_size: 1, // 1 MB (lowered from 10)
  min_large_file_size: 100, // 100 MB (lowered from 1024)
  ignore_patterns: ["*.log", "*/.git/*"],
  bg_monitor_enabled: true,
  scan_interval: "12h",
});

// ============================================================================
// SELECTION STATE
// ============================================================================

export const selectedPaths = writable<Set<string>>(new Set());

// ============================================================================
// SUMMARY STATS
// ============================================================================

export const summaryStats = writable<SummaryStats>({
  project_bloat_gb: 0,
  project_bloat_count: 0,
  large_files_gb: 0,
  large_files_count: 0,
  duplicates_mb: 0, // Auto-scales to GB when >= 1024 MB
  duplicates_count: 0,
  junk_files_mb: 0,
  junk_files_count: 0,
  total_cleanable_gb: 0,
  last_scan_time: null,
});

// ============================================================================
// DERIVED STORES (Computed Values)
// ============================================================================

// Calculate total selected file size
export const selectedTotalSize = writable<number>(0);

// Check if any critical paths are selected
export const hasCriticalSelection = writable<boolean>(false);

// Format last scan time as human readable
export const lastScanFormatted = writable<string>("Never");