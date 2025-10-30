<script lang="ts">
  import { onMount } from 'svelte';
  
  interface Props {
    fallback?: any;
    onError?: (error: Error, errorInfo: any) => void;
    children?: any;
  }
  
  let {
    fallback,
    onError = () => {},
    children
  }: Props = $props();
  
  let hasError = $state(false);
  let error = $state<Error | null>(null);
  let errorInfo = $state<any>(null);
  
  // Reset error state when children change
  $effect(() => {
    if (children) {
      hasError = false;
      error = null;
      errorInfo = null;
    }
  });
  
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
    
    const handleError = (event: ErrorEvent) => {
      handleError(new Error(event.message), { 
        type: 'globalError',
        filename: event.filename,
        lineno: event.lineno,
        colno: event.colno 
      });
    };
    
    window.addEventListener('unhandledrejection', handleUnhandledRejection);
    window.addEventListener('error', handleError);
    
    return () => {
      window.removeEventListener('unhandledrejection', handleUnhandledRejection);
      window.removeEventListener('error', handleError);
    };
  });
  
  function retry(): void {
    hasError = false;
    error = null;
    errorInfo = null;
  }
</script>

{#if hasError}
  {#if fallback}
    {@render fallback({ error, errorInfo, retry })}
  {:else}
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
          <button onclick={retry} class="retry-button">
            Try Again
          </button>
          <button onclick={() => window.location.reload()} class="reload-button">
            Reload Page
          </button>
        </div>
      </div>
    </div>
  {/if}
{:else}
  {@render children?.()}
{/if}

<style>
  .error-boundary {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 400px;
    padding: var(--space-8);
    background-color: var(--bg-secondary);
    border-radius: var(--card-radius);
    border: 2px solid var(--color-error);
  }
  
  .error-boundary-content {
    text-align: center;
    max-width: 500px;
  }
  
  .error-icon {
    font-size: 3rem;
    margin-bottom: var(--space-4);
  }
  
  .error-title {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    color: var(--text-primary);
    margin-bottom: var(--space-2);
  }
  
  .error-message {
    font-size: var(--text-base);
    color: var(--text-secondary);
    margin-bottom: var(--space-6);
  }
  
  .error-details {
    text-align: left;
    margin-bottom: var(--space-6);
    background-color: var(--bg-tertiary);
    border-radius: var(--button-radius);
    padding: var(--space-4);
  }
  
  .error-details summary {
    cursor: pointer;
    font-weight: var(--font-medium);
    color: var(--text-secondary);
    margin-bottom: var(--space-2);
  }
  
  .error-stack,
  .error-info {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--text-tertiary);
    white-space: pre-wrap;
    overflow-x: auto;
    margin: var(--space-2) 0;
  }
  
  .error-actions {
    display: flex;
    gap: var(--space-3);
    justify-content: center;
  }
  
  .retry-button,
  .reload-button {
    padding: var(--space-3) var(--space-6);
    border-radius: var(--button-radius);
    font-weight: var(--font-medium);
    cursor: pointer;
    transition: all var(--transition-fast);
    border: none;
  }
  
  .retry-button {
    background-color: var(--color-brand-primary);
    color: white;
  }
  
  .retry-button:hover {
    background-color: var(--color-brand-primary-hover);
  }
  
  .reload-button {
    background-color: var(--bg-tertiary);
    color: var(--text-secondary);
    border: 1px solid var(--border-primary);
  }
  
  .reload-button:hover {
    background-color: var(--border-primary);
    color: var(--text-primary);
  }
</style>