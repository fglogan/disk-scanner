<script lang="ts">
  import { onMount } from 'svelte';
  
  export let fallback: any = undefined;
  export let onError: (error: Error, errorInfo: any) => void = () => {};
  
  let hasError = false;
  let error: Error | null = null;
  let errorInfo: any = null;
  
  function handleError(err: Error, info?: any): void {
    hasError = true;
    error = err;
    errorInfo = info;
    onError(err, info);
    
    // Log to console for debugging
    console.error('ErrorBoundary caught an error:', err);
    if (info) {
      console.error('Error info:', info);
    }
  }
  
  // Global error handler for unhandled promise rejections
  onMount(() => {
    const handleUnhandledRejection = (event: PromiseRejectionEvent) => {
      handleError(new Error(event.reason), { type: 'unhandledRejection' });
    };
    
    const handleGlobalError = (event: ErrorEvent) => {
      handleError(new Error(event.message), { 
        type: 'globalError',
        filename: event.filename,
        lineno: event.lineno,
        colno: event.colno 
      });
    };
    
    window.addEventListener('unhandledrejection', handleUnhandledRejection);
    window.addEventListener('error', handleGlobalError);
    
    return () => {
      window.removeEventListener('unhandledrejection', handleUnhandledRejection);
      window.removeEventListener('error', handleGlobalError);
    };
  });
  
  function retry(): void {
    hasError = false;
    error = null;
    errorInfo = null;
  }
</script>

{#if hasError}
  <div class="error-boundary">
    <div class="error-boundary-content">
      <div class="error-icon">⚠️</div>
      <h2 class="error-title">Something went wrong</h2>
      <p class="error-message">
        {error?.message || 'An unexpected error occurred'}
      </p>
      
      <details class="error-details">
        <summary>Technical Details</summary>
        <pre class="error-stack">{error?.stack}</pre>
        {#if errorInfo}
          <pre class="error-info">{JSON.stringify(errorInfo, null, 2)}</pre>
        {/if}
      </details>
      
      <div class="error-actions">
        <button on:click={retry} class="retry-button">
          Try Again
        </button>
        <button on:click={() => window.location.reload()} class="reload-button">
          Reload Page
        </button>
      </div>
    </div>
  </div>
{:else}
  <slot />
{/if}

<style>
  .error-boundary {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 400px;
    padding: 2rem;
    background-color: #f8f9fa;
    border-radius: 8px;
    border: 2px solid #dc3545;
    margin: 1rem;
  }
  
  .error-boundary-content {
    text-align: center;
    max-width: 500px;
  }
  
  .error-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }
  
  .error-title {
    font-size: 1.5rem;
    font-weight: bold;
    color: #333;
    margin-bottom: 0.5rem;
  }
  
  .error-message {
    font-size: 1rem;
    color: #666;
    margin-bottom: 1.5rem;
  }
  
  .error-details {
    text-align: left;
    margin-bottom: 1.5rem;
    background-color: #f1f3f4;
    border-radius: 6px;
    padding: 1rem;
  }
  
  .error-details summary {
    cursor: pointer;
    font-weight: 500;
    color: #666;
    margin-bottom: 0.5rem;
  }
  
  .error-stack,
  .error-info {
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 0.75rem;
    color: #555;
    white-space: pre-wrap;
    overflow-x: auto;
    margin: 0.5rem 0;
  }
  
  .error-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: center;
  }
  
  .retry-button,
  .reload-button {
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    border: none;
  }
  
  .retry-button {
    background-color: #007bff;
    color: white;
  }
  
  .retry-button:hover {
    background-color: #0056b3;
  }
  
  .reload-button {
    background-color: #f1f3f4;
    color: #666;
    border: 1px solid #ddd;
  }
  
  .reload-button:hover {
    background-color: #e9ecef;
    color: #333;
  }
  
  /* Dark mode support */
  :global(.dark) .error-boundary {
    background-color: #2d3748;
    border-color: #e53e3e;
  }
  
  :global(.dark) .error-title {
    color: #f7fafc;
  }
  
  :global(.dark) .error-message {
    color: #e2e8f0;
  }
  
  :global(.dark) .error-details {
    background-color: #4a5568;
  }
  
  :global(.dark) .error-details summary {
    color: #e2e8f0;
  }
  
  :global(.dark) .error-stack,
  :global(.dark) .error-info {
    color: #cbd5e0;
  }
  
  :global(.dark) .reload-button {
    background-color: #4a5568;
    color: #e2e8f0;
    border-color: #718096;
  }
  
  :global(.dark) .reload-button:hover {
    background-color: #718096;
    color: #f7fafc;
  }
</style>