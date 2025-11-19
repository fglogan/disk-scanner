<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { writable } from 'svelte/store';
  import { t, formatters } from '../i18n';
  
  export let isVisible = false;
  
  interface PerformanceMetrics {
    scanDuration: number;
    filesScanned: number;
    memoryUsage: number;
    cpuUsage: number;
    diskSpeed: number;
    timestamp: number;
  }
  
  // Performance metrics store
  const metrics = writable<PerformanceMetrics>({
    scanDuration: 0,
    filesScanned: 0,
    memoryUsage: 0,
    cpuUsage: 0,
    diskSpeed: 0,
    timestamp: Date.now()
  });
  
  // Historical data for charts
  const history = writable<Array<{ memoryUsage: number; cpuUsage: number }>>([]);
  const maxHistoryPoints = 30; // Last 30 data points
  
  let updateInterval: number | null = null;
  let performanceObserver: PerformanceObserver | null = null;
  
  // Get memory usage
  async function getMemoryUsage(): Promise<number> {
    if ('memory' in performance) {
      const memory = (performance as any).memory;
      return Math.round(memory.usedJSHeapSize / 1048576); // Convert to MB
    }
    return 0;
  }
  
  // Estimate CPU usage (simplified)
  function estimateCPUUsage(): number {
    // This is a simplified estimation based on main thread blocking
    const start = performance.now();
    let count = 0;
    
    // Run a simple loop for 10ms
    while (performance.now() - start < 10) {
      count++;
    }
    
    // Normalize to percentage (this is very rough)
    return Math.min(100, Math.round(count / 10000));
  }
  
  // Update metrics
  async function updateMetrics() {
    const memoryUsage = await getMemoryUsage();
    const cpuUsage = estimateCPUUsage();
    
    metrics.update(m => ({
      ...m,
      memoryUsage,
      cpuUsage,
      timestamp: Date.now()
    }));
    
    // Update history
    history.update(h => {
      const newHistory = [...h, { memoryUsage, cpuUsage }];
      if (newHistory.length > maxHistoryPoints) {
        newHistory.shift();
      }
      return newHistory;
    });
  }
  
  // Track scan performance
  export function trackScanPerformance(duration: number, filesCount: number) {
    metrics.update(m => ({
      ...m,
      scanDuration: duration,
      filesScanned: filesCount,
      diskSpeed: filesCount > 0 ? Math.round(filesCount / (duration / 1000)) : 0
    }));
  }
  
  // Setup performance observer
  function setupPerformanceObserver() {
    if ('PerformanceObserver' in window) {
      performanceObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (entry.entryType === 'measure' && entry.name.startsWith('scan-')) {
            trackScanPerformance(entry.duration, 0);
          }
        }
      });
      
      performanceObserver.observe({ entryTypes: ['measure'] });
    }
  }
  
  // Generate sparkline path
  function generateSparkline(data: number[], max: number): string {
    if (data.length < 2) return '';
    
    const width = 100;
    const height = 30;
    const step = width / (data.length - 1);
    
    return data
      .map((value, i) => {
        const x = i * step;
        const y = height - (value / max) * height;
        return `${i === 0 ? 'M' : 'L'} ${x} ${y}`;
      })
      .join(' ');
  }
  
  $: memorySparkline = generateSparkline(
    $history.map(h => h.memoryUsage),
    Math.max(...$history.map(h => h.memoryUsage), 100)
  );
  
  $: cpuSparkline = generateSparkline(
    $history.map(h => h.cpuUsage),
    100
  );
  
  onMount(() => {
    if (isVisible) {
      setupPerformanceObserver();
      updateMetrics();
      updateInterval = window.setInterval(updateMetrics, 1000);
    }
  });
  
  onDestroy(() => {
    if (updateInterval) {
      clearInterval(updateInterval);
    }
    if (performanceObserver) {
      performanceObserver.disconnect();
    }
  });
  
  $: if (isVisible && !updateInterval) {
    updateMetrics();
    updateInterval = window.setInterval(updateMetrics, 1000);
  } else if (!isVisible && updateInterval) {
    clearInterval(updateInterval);
    updateInterval = null;
  }
</script>

{#if isVisible}
  <div class="performance-monitor">
    <div class="monitor-header">
      <h3 class="monitor-title">{$t('performance.title')}</h3>
      <button
        class="monitor-close"
        on:click={() => isVisible = false}
        aria-label="Close performance monitor"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
    
    <div class="monitor-stats">
      <div class="stat-card">
        <div class="stat-label">{$t('performance.scanDuration')}</div>
        <div class="stat-value">{($metrics.scanDuration / 1000).toFixed(2)}s</div>
      </div>
      
      <div class="stat-card">
        <div class="stat-label">{$t('performance.filesScanned')}</div>
        <div class="stat-value">{formatters.number($metrics.filesScanned)}</div>
      </div>
      
      <div class="stat-card">
        <div class="stat-label">{$t('performance.memoryUsage')}</div>
        <div class="stat-value">{$metrics.memoryUsage} MB</div>
        <svg class="sparkline" viewBox="0 0 100 30">
          <path d={memorySparkline} fill="none" stroke="var(--color-info)" stroke-width="2" />
        </svg>
      </div>
      
      <div class="stat-card">
        <div class="stat-label">{$t('performance.cpuUsage')}</div>
        <div class="stat-value">{$metrics.cpuUsage}%</div>
        <svg class="sparkline" viewBox="0 0 100 30">
          <path d={cpuSparkline} fill="none" stroke="var(--color-warning)" stroke-width="2" />
        </svg>
      </div>
      
      <div class="stat-card">
        <div class="stat-label">{$t('performance.diskSpeed')}</div>
        <div class="stat-value">{formatters.number($metrics.diskSpeed)} files/s</div>
      </div>
    </div>
  </div>
{/if}

<style>
  .performance-monitor {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    width: 24rem;
    max-width: calc(100vw - 2rem);
    background: var(--surface-card);
    border: 1px solid var(--border-primary);
    border-radius: 0.5rem;
    box-shadow: var(--shadow-lg);
    padding: 1rem;
    z-index: 50;
    animation: slideUp var(--transition-normal) ease-out;
  }
  
  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(1rem);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  
  .monitor-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1rem;
  }
  
  .monitor-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }
  
  .monitor-close {
    padding: 0.25rem;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 0.25rem;
    transition: all var(--transition-fast);
  }
  
  .monitor-close:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }
  
  .monitor-close:focus-visible {
    outline: 2px solid var(--ring-color);
    outline-offset: 2px;
  }
  
  .monitor-stats {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
  }
  
  .stat-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    border-radius: 0.375rem;
    padding: 0.75rem;
    position: relative;
    overflow: hidden;
  }
  
  .stat-label {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-bottom: 0.25rem;
  }
  
  .stat-value {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
  }
  
  .sparkline {
    position: absolute;
    bottom: 0;
    right: 0;
    width: 100%;
    height: 30px;
    opacity: 0.3;
  }
  
  @media (max-width: 640px) {
    .performance-monitor {
      width: 100%;
      bottom: 0;
      right: 0;
      border-radius: 0.5rem 0.5rem 0 0;
    }
    
    .monitor-stats {
      grid-template-columns: 1fr;
    }
  }
</style>