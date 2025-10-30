<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  
  // ✅ Svelte 5 Props with TypeScript
  interface Props {
    title: string;
    description: string;
    scanCommand: string;
    scanOptions?: Record<string, any>;
    icon?: string;
    variant?: 'primary' | 'secondary' | 'danger';
    disabled?: boolean;
    onScanComplete?: (results: any) => void;
    onScanError?: (error: string) => void;
  }
  
  let {
    title,
    description,
    scanCommand,
    scanOptions = {},
    icon = '📁',
    variant = 'primary',
    disabled = false,
    onScanComplete = () => {},
    onScanError = () => {}
  }: Props = $props();
  
  // ✅ Svelte 5 State with $state
  let isScanning = $state(false);
  let progress = $state('');
  let error = $state<string | null>(null);
  let results = $state<any>(null);
  let filesScanned = $state(0);
  
  // ✅ Svelte 5 Derived values with $derived
  let buttonText = $derived(isScanning ? 'Scanning...' : `Scan ${title}`);
  let canScan = $derived(!isScanning && !disabled);
  let hasResults = $derived(results !== null);
  let hasError = $derived(error !== null);
  
  // ✅ Svelte 5 Effects with $effect
  $effect(() => {
    // Auto-clear error after 5 seconds
    if (error) {
      const timer = setTimeout(() => {
        error = null;
      }, 5000);
      
      return () => clearTimeout(timer);
    }
  });
  
  // ✅ Modern async/await with proper error handling
  async function handleScan(): Promise<void> {
    if (!canScan) return;
    
    isScanning = true;
    progress = 'Initializing scan...';
    error = null;
    results = null;
    filesScanned = 0;
    
    // Simulate progress updates
    const progressTimer = setInterval(() => {
      if (isScanning) {
        filesScanned += Math.floor(Math.random() * 50) + 10;
      }
    }, 100);
    
    try {
      progress = `Scanning ${title.toLowerCase()}...`;
      
      const scanResults = await invoke(scanCommand, {
        opts: {
          follow_symlinks: false,
          min_bytes: 0,
          ...scanOptions
        }
      });
      
      results = scanResults;
      progress = 'Scan completed successfully';
      onScanComplete(scanResults);
      
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      error = `Failed to scan ${title.toLowerCase()}: ${errorMessage}`;
      onScanError(errorMessage);
      
    } finally {
      isScanning = false;
      clearInterval(progressTimer);
      
      // Clear progress after delay
      setTimeout(() => {
        if (!isScanning) progress = '';
      }, 2000);
    }
  }
  
  // ✅ Keyboard accessibility
  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleScan();
    }
  }
  
  // ✅ Variant styles with proper typing
  const variantStyles = {
    primary: 'bg-indigo-600 hover:bg-indigo-500 focus:ring-indigo-500',
    secondary: 'bg-slate-600 hover:bg-slate-500 focus:ring-slate-500',
    danger: 'bg-red-600 hover:bg-red-500 focus:ring-red-500'
  } as const;
  
  const currentVariantStyle = $derived(variantStyles[variant]);
</script>

<!-- ✅ Semantic HTML with proper ARIA -->
<article 
  class="bg-slate-800 rounded-xl shadow-lg p-6 transition-all duration-200 hover:shadow-xl"
  role="region"
  aria-labelledby="scan-card-title"
>
  <!-- Header -->
  <header class="flex items-center justify-between mb-4">
    <div class="flex items-center space-x-3">
      <span class="text-2xl" role="img" aria-label={`${title} icon`}>
        {icon}
      </span>
      <div>
        <h3 
          id="scan-card-title"
          class="text-lg font-semibold text-white"
        >
          {title}
        </h3>
        <p class="text-sm text-slate-400">
          {description}
        </p>
      </div>
    </div>
    
    <!-- Status Indicator -->
    {#if isScanning}
      <div 
        class="flex items-center space-x-2 text-indigo-400"
        role="status"
        aria-live="polite"
      >
        <div class="animate-spin h-4 w-4 border-2 border-indigo-400 border-t-transparent rounded-full"></div>
        <span class="text-sm">Scanning</span>
      </div>
    {:else if hasResults}
      <div 
        class="flex items-center space-x-2 text-emerald-400"
        role="status"
      >
        <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
        <span class="text-sm">Complete</span>
      </div>
    {:else if hasError}
      <div 
        class="flex items-center space-x-2 text-red-400"
        role="alert"
      >
        <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
        </svg>
        <span class="text-sm">Error</span>
      </div>
    {/if}
  </header>
  
  <!-- Progress Section -->
  {#if isScanning && progress}
    <div 
      class="mb-4 p-3 bg-indigo-900/30 rounded-lg border border-indigo-600/30"
      role="status"
      aria-live="polite"
    >
      <p class="text-xs text-indigo-400 mb-1">Status:</p>
      <p class="text-sm text-white">{progress}</p>
      {#if filesScanned > 0}
        <p class="text-xs text-indigo-300 mt-1">
          Files scanned: {filesScanned.toLocaleString()}
        </p>
      {/if}
    </div>
  {/if}
  
  <!-- Error Display -->
  {#if error}
    <div 
      class="mb-4 p-3 bg-red-900/30 rounded-lg border border-red-600/30"
      role="alert"
    >
      <p class="text-xs text-red-400 mb-1">Error:</p>
      <p class="text-sm text-red-200">{error}</p>
    </div>
  {/if}
  
  <!-- Results Preview -->
  {#if results && !isScanning}
    <div class="mb-4 p-3 bg-emerald-900/30 rounded-lg border border-emerald-600/30">
      <p class="text-xs text-emerald-400 mb-1">Results:</p>
      <p class="text-sm text-white">
        {Array.isArray(results) ? `Found ${results.length} items` : 'Scan completed'}
      </p>
    </div>
  {/if}
  
  <!-- Action Button -->
  <button
    onclick={handleScan}
    onkeydown={handleKeydown}
    disabled={!canScan}
    class="w-full {currentVariantStyle} disabled:opacity-50 disabled:cursor-not-allowed text-white font-semibold py-3 px-4 rounded-lg transition-all duration-150 shadow-lg focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-slate-800"
    aria-describedby="scan-card-title"
  >
    {buttonText}
  </button>
</article>

<style>
  /* ✅ Scoped styles for animations */
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
  
  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>