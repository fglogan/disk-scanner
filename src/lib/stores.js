import { writable } from "svelte/store";

// ============================================================================
// TYPES
// ============================================================================

// Navigation & UI State
export const currentPage = writable("dashboard");
export const isScanning = writable(false);
export const scanProgress = writable("");
export const scanError = writable(null);

// ============================================================================
// THEME & UI PREFERENCES
// ============================================================================

// Theme management with localStorage persistence
function createThemeStore() {
  const { subscribe, set, update } = writable(true); // Default to dark mode
  
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
    set: (dark) => {
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

function createToastStore() {
  const { subscribe, update } = writable([]);
  
  return {
    subscribe,
    add: (toast) => {
      const id = Math.random().toString(36).substring(2, 11);
      const newToast = {
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
    remove: (id) => {
      update(toasts => toasts.filter(t => t.id !== id));
    },
    clear: () => {
      update(() => []);
    }
  };
}

export const toasts = createToastStore();

// Convenience functions for common toast types
export const showSuccess = (message, duration = 3000) => 
  toasts.add({ type: 'success', message, duration });

export const showError = (message, duration = 0) => 
  toasts.add({ type: 'error', message, duration });

export const showWarning = (message, duration = 5000) => 
  toasts.add({ type: 'warning', message, duration });

export const showInfo = (message, duration = 3000) => 
  toasts.add({ type: 'info', message, duration });

// ============================================================================
// KEYBOARD SHORTCUTS
// ============================================================================

export const showShortcutsHelp = writable(false);

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

// ============================================================================
// DERIVED STORES (Computed Values)
// ============================================================================

// Calculate total selected file size
export const selectedTotalSize = writable(0);

// Check if any critical paths are selected
export const hasCriticalSelection = writable(false);

// Format last scan time as human readable
export const lastScanFormatted = writable("Never");
