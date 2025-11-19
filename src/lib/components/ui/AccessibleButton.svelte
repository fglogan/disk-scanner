<script lang="ts">
  import { ariaProps, isKey, KEYS, type AriaProps } from '../../utils/accessibility';
  import { createEventDispatcher } from 'svelte';
  
  export let variant: 'primary' | 'secondary' | 'danger' | 'ghost' = 'secondary';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let disabled = false;
  export let loading = false;
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let aria: AriaProps = {};
  
  const dispatch = createEventDispatcher();
  
  $: ariaAttributes = {
    ...ariaProps(aria),
    'aria-busy': loading || undefined,
    'aria-disabled': disabled || loading || undefined
  };
  
  function handleClick(e: MouseEvent) {
    if (!disabled && !loading) {
      dispatch('click', e);
    }
  }
  
  function handleKeyDown(e: KeyboardEvent) {
    if (isKey(e, KEYS.ENTER) || isKey(e, KEYS.SPACE)) {
      e.preventDefault();
      if (!disabled && !loading) {
        dispatch('click', e);
      }
    }
  }
</script>

<button
  {type}
  {disabled}
  class="accessible-button {variant} {size}"
  class:loading
  class:disabled={disabled || loading}
  on:click={handleClick}
  on:keydown={handleKeyDown}
  {...ariaAttributes}
>
  {#if loading}
    <span class="spinner" aria-hidden="true">
      <svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
    </span>
    <span class="sr-only">Loading...</span>
  {/if}
  <slot />
</button>

<style>
  .accessible-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    font-weight: 500;
    border-radius: 0.375rem;
    transition: all var(--transition-fast);
    cursor: pointer;
    position: relative;
    white-space: nowrap;
    border: 1px solid transparent;
  }
  
  /* Size variants */
  .sm {
    padding: 0.375rem 0.75rem;
    font-size: 0.875rem;
  }
  
  .md {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
  }
  
  .lg {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
  }
  
  /* Style variants */
  .primary {
    background-color: var(--button-bg);
    color: var(--button-text);
  }
  
  .primary:hover:not(.disabled) {
    background-color: var(--button-bg-hover);
    transform: translateY(-1px);
  }
  
  .secondary {
    background-color: var(--surface-card);
    color: var(--text-primary);
    border-color: var(--border-primary);
  }
  
  .secondary:hover:not(.disabled) {
    background-color: var(--surface-hover);
    border-color: var(--border-secondary);
  }
  
  .danger {
    background-color: var(--color-error);
    color: white;
  }
  
  .danger:hover:not(.disabled) {
    opacity: 0.9;
  }
  
  .ghost {
    background-color: transparent;
    color: var(--text-secondary);
  }
  
  .ghost:hover:not(.disabled) {
    background-color: var(--surface-hover);
    color: var(--text-primary);
  }
  
  /* States */
  .disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .loading {
    cursor: wait;
  }
  
  /* Focus styles */
  .accessible-button:focus-visible {
    outline: 2px solid var(--ring-color);
    outline-offset: 2px;
  }
  
  /* Active state */
  .accessible-button:active:not(.disabled) {
    transform: scale(0.98);
  }
  
  /* Spinner */
  .spinner {
    display: inline-flex;
  }
  
  /* Screen reader only text */
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }
  
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
  
  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>