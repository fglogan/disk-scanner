/**
 * Enhanced State Management with Performance Optimization
 * Advanced store implementation with persistence, undo/redo, and caching
 */

import { writable, derived, get, type Writable } from 'svelte/store';
import { performanceMonitor } from './performance.js';

// Enhanced store interfaces
export interface StoreState {
  data: any;
  loading: boolean;
  error: string | null;
  lastUpdated: number;
  version: number;
}

export interface UndoRedoState<T> {
  past: T[];
  present: T;
  future: T[];
}

export interface CacheConfig {
  ttl: number; // Time to live in milliseconds
  maxSize: number; // Maximum number of entries
  strategy: 'lru' | 'fifo'; // Eviction strategy
}

export interface PersistenceConfig {
  key: string;
  storage: 'localStorage' | 'sessionStorage';
  serialize?: (value: any) => string;
  deserialize?: (value: string) => any;
}

// Enhanced store creator with performance tracking
export function createEnhancedStore<T>(
  initialValue: T,
  options: {
    persistence?: PersistenceConfig;
    cache?: CacheConfig;
    undoRedo?: boolean;
    validate?: (value: T) => boolean;
  } = {}
) {
  const { persistence, cache: cacheConfig, undoRedo, validate } = options;
  
  // Load persisted value
  let initial = initialValue;
  if (persistence && typeof window !== 'undefined') {
    try {
      const storage = window[persistence.storage];
      const stored = storage.getItem(persistence.key);
      if (stored) {
        const deserialize = persistence.deserialize || JSON.parse;
        initial = deserialize(stored);
      }
    } catch (e) {
      console.warn(`Failed to load persisted state for ${persistence.key}:`, e);
    }
  }

  // Create base store
  const baseStore = writable<StoreState>({
    data: initial,
    loading: false,
    error: null,
    lastUpdated: Date.now(),
    version: 0
  });

  // Undo/Redo functionality
  let undoRedoStore: Writable<UndoRedoState<T>> | null = null;
  if (undoRedo) {
    undoRedoStore = writable<UndoRedoState<T>>({
      past: [],
      present: initial,
      future: []
    });
  }

  // Cache implementation
  const cache = new Map<string, { value: T; timestamp: number }>();
  
  function evictCache() {
    if (!cacheConfig) return;
    
    const now = Date.now();
    const { ttl, maxSize, strategy } = cacheConfig;
    
    // Remove expired entries
    for (const [key, entry] of cache.entries()) {
      if (now - entry.timestamp > ttl) {
        cache.delete(key);
      }
    }
    
    // Enforce size limit
    if (cache.size > maxSize) {
      const entries = Array.from(cache.entries());
      const toRemove = cache.size - maxSize;
      
      if (strategy === 'lru') {
        entries.sort((a, b) => a[1].timestamp - b[1].timestamp);
      }
      
      for (let i = 0; i < toRemove; i++) {
        cache.delete(entries[i][0]);
      }
    }
  }

  // Performance-tracked update function
  function updateWithTracking(updater: (value: StoreState) => StoreState) {
    const start = performance.now();
    
    baseStore.update(current => {
      const updated = updater(current);
      
      // Validation
      if (validate && !validate(updated.data)) {
        console.warn('Store validation failed');
        return { ...current, error: 'Validation failed' };
      }
      
      // Persistence
      if (persistence && typeof window !== 'undefined') {
        try {
          const storage = window[persistence.storage];
          const serialize = persistence.serialize || JSON.stringify;
          storage.setItem(persistence.key, serialize(updated.data));
        } catch (e) {
          console.warn('Failed to persist state:', e);
        }
      }
      
      // Undo/Redo tracking
      if (undoRedo && undoRedoStore) {
        undoRedoStore.update(undoState => ({
          past: [...undoState.past.slice(-50), undoState.present], // Keep last 50 states
          present: updated.data,
          future: []
        }));
      }
      
      return {
        ...updated,
        lastUpdated: Date.now(),
        version: current.version + 1
      };
    });
    
    const duration = performance.now() - start;
    if (duration > 10) { // Warn if update takes more than 10ms
      console.warn(`Slow store update: ${duration.toFixed(2)}ms`);
    }
  }

  // Enhanced store interface
  return {
    subscribe: baseStore.subscribe,
    
    set(value: T) {
      updateWithTracking(current => ({
        ...current,
        data: value,
        error: null
      }));
    },
    
    update(updater: (value: T) => T) {
      updateWithTracking(current => ({
        ...current,
        data: updater(current.data),
        error: null
      }));
    },
    
    setLoading(loading: boolean) {
      baseStore.update(current => ({ ...current, loading }));
    },
    
    setError(error: string | null) {
      baseStore.update(current => ({ ...current, error }));
    },
    
    async load(loader: () => Promise<T>) {
      this.setLoading(true);
      this.setError(null);
      
      try {
        const value = await loader();
        this.set(value);
      } catch (error) {
        this.setError(error instanceof Error ? error.message : 'Load failed');
      } finally {
        this.setLoading(false);
      }
    },
    
    // Cache methods
    cache(key: string, value: T) {
      if (cacheConfig) {
        cache.set(key, { value, timestamp: Date.now() });
        evictCache();
      }
    },
    
    getCached(key: string): T | null {
      if (!cacheConfig) return null;
      
      const entry = cache.get(key);
      if (!entry) return null;
      
      const { ttl } = cacheConfig;
      if (Date.now() - entry.timestamp > ttl) {
        cache.delete(key);
        return null;
      }
      
      return entry.value;
    },
    
    // Undo/Redo methods
    undo() {
      if (!undoRedo || !undoRedoStore) return;
      
      undoRedoStore.update(state => {
        if (state.past.length === 0) return state;
        
        const previous = state.past[state.past.length - 1];
        const newPast = state.past.slice(0, -1);
        
        this.set(previous);
        
        return {
          past: newPast,
          present: previous,
          future: [state.present, ...state.future]
        };
      });
    },
    
    redo() {
      if (!undoRedo || !undoRedoStore) return;
      
      undoRedoStore.update(state => {
        if (state.future.length === 0) return state;
        
        const next = state.future[0];
        const newFuture = state.future.slice(1);
        
        this.set(next);
        
        return {
          past: [...state.past, state.present],
          present: next,
          future: newFuture
        };
      });
    },
    
    canUndo: undoRedo ? derived(undoRedoStore!, state => state.past.length > 0) : null,
    canRedo: undoRedo ? derived(undoRedoStore!, state => state.future.length > 0) : null,
    
    // Performance methods
    getMetrics() {
      const state = get(baseStore);
      return {
        version: state.version,
        lastUpdated: state.lastUpdated,
        cacheSize: cache.size
      };
    },
    
    reset() {
      this.set(initialValue);
      cache.clear();
      if (undoRedo && undoRedoStore) {
        undoRedoStore.set({
          past: [],
          present: initialValue,
          future: []
        });
      }
    }
  };
}

// Specialized stores for the application
export const enhancedDiskInfo = createEnhancedStore(
  {
    total_gb: 0,
    used_gb: 0,
    free_gb: 0,
    usage_pct: 0,
  },
  {
    persistence: {
      key: 'disk-info',
      storage: 'sessionStorage'
    },
    cache: {
      ttl: 60000, // 1 minute
      maxSize: 10,
      strategy: 'lru'
    }
  }
);

export const enhancedScanResults = createEnhancedStore(
  {
    largeFiles: [],
    duplicates: [],
    junkFiles: [],
    bloatCategories: []
  },
  {
    persistence: {
      key: 'scan-results',
      storage: 'sessionStorage'
    },
    undoRedo: true,
    cache: {
      ttl: 300000, // 5 minutes
      maxSize: 5,
      strategy: 'lru'
    }
  }
);

export const enhancedSettings = createEnhancedStore(
  {
    directories: [],
    min_dup_size: 1,
    min_large_file_size: 100,
    ignore_patterns: ["*.log", "*/.git/*"],
    bg_monitor_enabled: true,
    scan_interval: "12h",
  },
  {
    persistence: {
      key: 'app-settings',
      storage: 'localStorage'
    },
    validate: (settings) => {
      return settings.min_dup_size > 0 && settings.min_large_file_size > 0;
    }
  }
);

// Offline capability store - separate simple store for this specific use case
export const offlineStore = writable({
  isOnline: typeof navigator !== 'undefined' ? navigator.onLine : true,
  syncQueue: [] as Array<{ action: string; data: any; timestamp: number }>,
  lastSync: null as number | null
});

// Setup offline event listeners
if (typeof window !== 'undefined') {
  window.addEventListener('online', () => {
    offlineStore.update(state => ({ ...state, isOnline: true }));
  });
  
  window.addEventListener('offline', () => {
    offlineStore.update(state => ({ ...state, isOnline: false }));
  });
}

// Sync queue management
export function addToSyncQueue(action: string, data: any) {
  offlineStore.update(state => ({
    ...state,
    syncQueue: [...state.syncQueue, {
      action,
      data,
      timestamp: Date.now()
    }]
  }));
}

export async function processSyncQueue() {
  const state = get(offlineStore);
  if (!state.isOnline || state.syncQueue.length === 0) return;
  
  const queue = [...state.syncQueue];
  offlineStore.update(state => ({ ...state, syncQueue: [] }));
  
  for (const item of queue) {
    try {
      // Process sync item (implement based on your needs)
      console.log('Processing sync item:', item);
    } catch (error) {
      console.error('Failed to sync item:', error);
      // Re-add failed items to queue
      offlineStore.update(state => ({
        ...state,
        syncQueue: [...state.syncQueue, item]
      }));
    }
  }
  
  offlineStore.update(state => ({ ...state, lastSync: Date.now() }));
}