// @ts-nocheck
import { describe, it, expect, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import {
  diskInfo,
  summaryStats,
  settings,
  bloatCategories,
  largeFiles,
  duplicates,
  isScanning,
  scanProgress,
  selectedPaths,
  toasts,
  darkMode,
  showShortcutsHelp,
  showSuccess,
  showError,
  showWarning,
  showInfo,
} from './stores';

describe('Store Tests', () => {
  beforeEach(() => {
    // Reset stores to initial state
    diskInfo.set({ total_gb: 0, used_gb: 0, free_gb: 0, usage_pct: 0 });
    bloatCategories.set([]);
    largeFiles.set([]);
    duplicates.set([]);
    isScanning.set(false);
    scanProgress.set('');
    selectedPaths.set(new Set());
    toasts.clear();
    showShortcutsHelp.set(false);
  });

  describe('diskInfo store', () => {
    it('should initialize with default values', () => {
      const value = get(diskInfo);
      expect(value).toEqual({
        total_gb: 0,
        used_gb: 0,
        free_gb: 0,
        usage_pct: 0,
      });
    });

    it('should update disk info', () => {
      diskInfo.set({
        total_gb: 500,
        used_gb: 300,
        free_gb: 200,
        usage_pct: 60,
      });
      const value = get(diskInfo);
      expect(value.total_gb).toBe(500);
      expect(value.usage_pct).toBe(60);
    });
  });

  describe('summaryStats store', () => {
    it('should initialize with zeros', () => {
      const value = get(summaryStats);
      expect(value.project_bloat_gb).toBe(0);
      expect(value.large_files_count).toBe(0);
      expect(value.duplicates_count).toBe(0);
    });

    it('should calculate total cleanable space', () => {
      summaryStats.set({
        project_bloat_gb: 5.5,
        project_bloat_count: 3,
        large_files_gb: 2.5,
        large_files_count: 10,
        duplicates_gb: 1.0,
        duplicates_count: 5,
        total_cleanable_gb: 6.5, // bloat + duplicates
        last_scan_time: null,
      });
      const value = get(summaryStats);
      expect(value.total_cleanable_gb).toBe(6.5);
    });
  });

  describe('settings store', () => {
    it('should have default settings', () => {
      const value = get(settings);
      expect(value.directories).toBeInstanceOf(Array);
      expect(value.directories.length).toBe(0);
      expect(value.min_large_file_size).toBeGreaterThan(0);
      expect(value.scan_interval).toBeDefined();
    });

    it('should update settings', () => {
      settings.update((s) => ({
        ...s,
        min_large_file_size: 500,
        bg_monitor_enabled: true,
      }));
      const value = get(settings);
      expect(value.min_large_file_size).toBe(500);
      expect(value.bg_monitor_enabled).toBe(true);
    });

    it('should add directories', () => {
      const initialDirs = get(settings).directories.length;
      settings.update((s) => ({
        ...s,
        directories: [...s.directories, '/new/test/dir'],
      }));
      const value = get(settings);
      expect(value.directories.length).toBe(initialDirs + 1);
      expect(value.directories).toContain('/new/test/dir');
    });
  });

  describe('bloatCategories store', () => {
    it('should start empty', () => {
      const value = get(bloatCategories);
      expect(value).toEqual([]);
    });

    it('should store bloat categories', () => {
      const mockCategories = [
        {
          category_id: 'node_modules',
          display_name: 'Node.js',
          total_size_mb: 1500,
          entries: [{ path: '/test/node_modules', size_mb: 1500 }],
        },
      ];
      bloatCategories.set(mockCategories);
      const value = get(bloatCategories);
      expect(value.length).toBe(1);
      expect(value[0].category_id).toBe('node_modules');
    });
  });

  describe('largeFiles store', () => {
    it('should store large files', () => {
      const mockFiles = [
        { path: '/test/video.mp4', size_mb: 2048, last_modified: 1234567890 },
        { path: '/test/archive.zip', size_mb: 1024, last_modified: 1234567890 },
      ];
      largeFiles.set(mockFiles);
      const value = get(largeFiles);
      expect(value.length).toBe(2);
      expect(value[0].size_mb).toBe(2048);
    });
  });

  describe('isScanning store', () => {
    it('should track scanning state', () => {
      expect(get(isScanning)).toBe(false);
      isScanning.set(true);
      expect(get(isScanning)).toBe(true);
      isScanning.set(false);
      expect(get(isScanning)).toBe(false);
    });
  });

  describe('scanProgress store', () => {
    it('should store progress messages', () => {
      expect(get(scanProgress)).toBe('');
      scanProgress.set('Scanning for bloat...');
      expect(get(scanProgress)).toBe('Scanning for bloat...');
    });
  });

  describe('selectedPaths store', () => {
    it('should manage selected paths', () => {
      const paths = get(selectedPaths);
      expect(paths.size).toBe(0);

      selectedPaths.update((s) => {
        s.add('/path/to/file1.txt');
        s.add('/path/to/file2.txt');
        return s;
      });

      const updated = get(selectedPaths);
      expect(updated.size).toBe(2);
      expect(updated.has('/path/to/file1.txt')).toBe(true);
    });

    it('should prevent duplicate paths', () => {
      selectedPaths.update((s) => {
        s.add('/path/to/file.txt');
        s.add('/path/to/file.txt'); // Duplicate
        return s;
      });

      const value = get(selectedPaths);
      expect(value.size).toBe(1);
    });
  });

  describe('toast notification system', () => {
    it('should add toast notifications', () => {
      const toastList = get(toasts);
      expect(toastList.length).toBe(0);

      const id1 = showSuccess('Test success message');
      const id2 = showError('Test error message');
      
      const updated = get(toasts);
      expect(updated.length).toBe(2);
      expect(updated[0].type).toBe('success');
      expect(updated[1].type).toBe('error');
      expect(typeof id1).toBe('string');
      expect(typeof id2).toBe('string');
    });

    it('should remove specific toasts', () => {
      const id = showInfo('Test message');
      expect(get(toasts).length).toBe(1);
      
      toasts.remove(id);
      expect(get(toasts).length).toBe(0);
    });

    it('should clear all toasts', () => {
      showSuccess('Message 1');
      showError('Message 2');
      showWarning('Message 3');
      expect(get(toasts).length).toBe(3);
      
      toasts.clear();
      expect(get(toasts).length).toBe(0);
    });

    it('should create toast with default values', () => {
      showInfo('Test message');
      const toast = get(toasts)[0];
      
      expect(toast.type).toBe('info');
      expect(toast.message).toBe('Test message');
      expect(toast.duration).toBe(3000);
      expect(toast.dismissible).toBe(true);
      expect(typeof toast.id).toBe('string');
    });

    it('should handle different toast types with custom durations', () => {
      showSuccess('Success', 5000);
      showError('Error', 0); // Persistent
      showWarning('Warning', 2000);
      
      const toastList = get(toasts);
      expect(toastList[0].duration).toBe(5000);
      expect(toastList[1].duration).toBe(0);
      expect(toastList[2].duration).toBe(2000);
    });
  });

  describe('theme management', () => {
    it('should initialize with dark mode', () => {
      // Note: Initial state depends on localStorage, but defaults to true
      const isDark = get(darkMode);
      expect(typeof isDark).toBe('boolean');
    });

    it('should toggle theme', () => {
      const initial = get(darkMode);
      darkMode.toggle();
      const toggled = get(darkMode);
      expect(toggled).toBe(!initial);
      
      darkMode.toggle();
      const toggledBack = get(darkMode);
      expect(toggledBack).toBe(initial);
    });

    it('should set theme explicitly', () => {
      darkMode.set(true);
      expect(get(darkMode)).toBe(true);
      
      darkMode.set(false);
      expect(get(darkMode)).toBe(false);
    });
  });

  describe('keyboard shortcuts help', () => {
    it('should toggle shortcuts help visibility', () => {
      expect(get(showShortcutsHelp)).toBe(false);
      
      showShortcutsHelp.set(true);
      expect(get(showShortcutsHelp)).toBe(true);
      
      showShortcutsHelp.update(show => !show);
      expect(get(showShortcutsHelp)).toBe(false);
    });
  });
});
