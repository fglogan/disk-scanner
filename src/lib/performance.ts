/**
 * Performance Monitoring & Optimization
 * Comprehensive performance tracking and optimization utilities
 */

export interface PerformanceMetrics {
  loadTime: number;
  renderTime: number;
  componentMountTime: number;
  memoryUsage: number;
  bundleSize: number;
  timestamp: number;
}

export interface PerformanceBudget {
  maxLoadTime: number;      // 3000ms
  maxRenderTime: number;    // 100ms
  maxComponentMount: number; // 50ms
  maxMemoryUsage: number;   // 100MB
  maxBundleSize: number;    // 1MB
}

class PerformanceMonitor {
  private metrics: PerformanceMetrics[] = [];
  private budget: PerformanceBudget = {
    maxLoadTime: 3000,
    maxRenderTime: 100,
    maxComponentMount: 50,
    maxMemoryUsage: 100 * 1024 * 1024, // 100MB
    maxBundleSize: 1024 * 1024 // 1MB
  };
  private observers: Map<string, PerformanceObserver> = new Map();

  constructor() {
    this.initializeObservers();
  }

  private initializeObservers() {
    if (typeof window === 'undefined') return;

    // Navigation timing observer
    if ('PerformanceObserver' in window) {
      const navObserver = new PerformanceObserver((list) => {
        const entries = list.getEntries();
        for (const entry of entries) {
          if (entry.entryType === 'navigation') {
            const navEntry = entry as PerformanceNavigationTiming;
            this.recordMetric({
              loadTime: navEntry.loadEventEnd - navEntry.loadEventStart,
              renderTime: navEntry.domContentLoadedEventEnd - navEntry.domContentLoadedEventStart,
              componentMountTime: 0,
              memoryUsage: this.getMemoryUsage(),
              bundleSize: 0,
              timestamp: Date.now()
            });
          }
        }
      });

      try {
        navObserver.observe({ entryTypes: ['navigation'] });
        this.observers.set('navigation', navObserver);
      } catch (e) {
        console.warn('Navigation observer not supported');
      }

      // Measure observer for custom metrics
      const measureObserver = new PerformanceObserver((list) => {
        const entries = list.getEntries();
        for (const entry of entries) {
          if (entry.name.startsWith('component-mount:')) {
            this.recordComponentMount(entry.name.replace('component-mount:', ''), entry.duration);
          }
        }
      });

      try {
        measureObserver.observe({ entryTypes: ['measure'] });
        this.observers.set('measure', measureObserver);
      } catch (e) {
        console.warn('Measure observer not supported');
      }
    }
  }

  private recordMetric(metric: PerformanceMetrics) {
    this.metrics.push(metric);
    this.checkBudgets(metric);
    
    // Keep only last 100 metrics
    if (this.metrics.length > 100) {
      this.metrics = this.metrics.slice(-100);
    }
  }

  private checkBudgets(metric: PerformanceMetrics) {
    const violations: string[] = [];

    if (metric.loadTime > this.budget.maxLoadTime) {
      violations.push(`Load time exceeded: ${metric.loadTime}ms > ${this.budget.maxLoadTime}ms`);
    }

    if (metric.renderTime > this.budget.maxRenderTime) {
      violations.push(`Render time exceeded: ${metric.renderTime}ms > ${this.budget.maxRenderTime}ms`);
    }

    if (metric.componentMountTime > this.budget.maxComponentMount) {
      violations.push(`Component mount exceeded: ${metric.componentMountTime}ms > ${this.budget.maxComponentMount}ms`);
    }

    if (metric.memoryUsage > this.budget.maxMemoryUsage) {
      violations.push(`Memory usage exceeded: ${(metric.memoryUsage / 1024 / 1024).toFixed(2)}MB > ${(this.budget.maxMemoryUsage / 1024 / 1024).toFixed(2)}MB`);
    }

    if (violations.length > 0) {
      console.warn('Performance budget violations:', violations);
      this.dispatchBudgetViolation(violations);
    }
  }

  private dispatchBudgetViolation(violations: string[]) {
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent('performance-budget-violation', {
        detail: { violations }
      }));
    }
  }

  public startComponentMount(componentName: string) {
    if (typeof performance !== 'undefined') {
      performance.mark(`component-mount-start:${componentName}`);
    }
  }

  public endComponentMount(componentName: string) {
    if (typeof performance !== 'undefined') {
      performance.mark(`component-mount-end:${componentName}`);
      performance.measure(
        `component-mount:${componentName}`,
        `component-mount-start:${componentName}`,
        `component-mount-end:${componentName}`
      );
    }
  }

  private recordComponentMount(componentName: string, duration: number) {
    const currentMetric = this.metrics[this.metrics.length - 1];
    if (currentMetric) {
      currentMetric.componentMountTime = Math.max(currentMetric.componentMountTime, duration);
    }
  }

  private getMemoryUsage(): number {
    if (typeof window !== 'undefined' && 'performance' in window && 'memory' in (window.performance as any)) {
      return (window.performance as any).memory.usedJSHeapSize;
    }
    return 0;
  }

  public getMetrics(): PerformanceMetrics[] {
    return [...this.metrics];
  }

  public getAverageMetrics(): Partial<PerformanceMetrics> {
    if (this.metrics.length === 0) return {};

    const sum = this.metrics.reduce((acc, metric) => ({
      loadTime: acc.loadTime + metric.loadTime,
      renderTime: acc.renderTime + metric.renderTime,
      componentMountTime: acc.componentMountTime + metric.componentMountTime,
      memoryUsage: acc.memoryUsage + metric.memoryUsage,
      bundleSize: acc.bundleSize + metric.bundleSize
    }), {
      loadTime: 0,
      renderTime: 0,
      componentMountTime: 0,
      memoryUsage: 0,
      bundleSize: 0
    });

    const count = this.metrics.length;
    return {
      loadTime: sum.loadTime / count,
      renderTime: sum.renderTime / count,
      componentMountTime: sum.componentMountTime / count,
      memoryUsage: sum.memoryUsage / count,
      bundleSize: sum.bundleSize / count
    };
  }

  public setBudgets(budget: Partial<PerformanceBudget>) {
    this.budget = { ...this.budget, ...budget };
  }

  public destroy() {
    this.observers.forEach(observer => observer.disconnect());
    this.observers.clear();
  }
}

// Global performance monitor instance
export const performanceMonitor = new PerformanceMonitor();

// Performance decorator for Svelte components
export function withPerformanceTracking(componentName: string) {
  return function <T extends new (...args: any[]) => any>(constructor: T) {
    return class extends constructor {
      constructor(...args: any[]) {
        performanceMonitor.startComponentMount(componentName);
        super(...args);
        performanceMonitor.endComponentMount(componentName);
      }
    };
  };
}

// Lazy loading utilities
export function createLazyLoader<T>(
  importFn: () => Promise<T>
): () => Promise<T> {
  let cached: T | null = null;
  let loading: Promise<T> | null = null;

  return async (): Promise<T> => {
    if (cached) return cached;
    
    if (loading) return loading;

    loading = importFn().then(module => {
      cached = module;
      loading = null;
      return module;
    });

    return loading;
  };
}

// Memory optimization utilities
export class MemoryOptimizer {
  private cleanupTasks: (() => void)[] = [];
  private intervals: number[] = [];
  private timeouts: number[] = [];
  private observers: (ResizeObserver | IntersectionObserver | MutationObserver)[] = [];

  public addCleanupTask(task: () => void) {
    this.cleanupTasks.push(task);
  }

  public registerInterval(intervalId: number) {
    this.intervals.push(intervalId);
  }

  public registerTimeout(timeoutId: number) {
    this.timeouts.push(timeoutId);
  }

  public registerObserver(observer: ResizeObserver | IntersectionObserver | MutationObserver) {
    this.observers.push(observer);
  }

  public cleanup() {
    // Clear intervals
    this.intervals.forEach(id => clearInterval(id));
    this.intervals = [];

    // Clear timeouts
    this.timeouts.forEach(id => clearTimeout(id));
    this.timeouts = [];

    // Disconnect observers
    this.observers.forEach(observer => observer.disconnect());
    this.observers = [];

    // Run custom cleanup tasks
    this.cleanupTasks.forEach(task => {
      try {
        task();
      } catch (e) {
        console.warn('Cleanup task failed:', e);
      }
    });
    this.cleanupTasks = [];
  }
}

// Bundle size analysis (development only)
export function analyzeBundleSize() {
  if (import.meta.env.DEV) {
    // This would typically be done by a build plugin
    console.log('Bundle size analysis would run here in development');
  }
}

// Performance utility functions
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: number;
  return (...args: Parameters<T>) => {
    clearTimeout(timeout);
    timeout = window.setTimeout(() => func(...args), wait);
  };
}

export function throttle<T extends (...args: any[]) => any>(
  func: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle: boolean;
  return (...args: Parameters<T>) => {
    if (!inThrottle) {
      func(...args);
      inThrottle = true;
      setTimeout(() => inThrottle = false, limit);
    }
  };
}