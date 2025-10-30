<script lang="ts">
  import { fly } from 'svelte/transition';
  import { toasts, type Toast } from '../../stores.js';
  
  let { toast }: { toast: Toast } = $props();
  
  const icons = {
    success: '✅',
    error: '❌', 
    warning: '⚠️',
    info: 'ℹ️'
  };
  
  const colors = {
    success: 'bg-green-600 border-green-500 shadow-green-500/25',
    error: 'bg-red-600 border-red-500 shadow-red-500/25',
    warning: 'bg-amber-600 border-amber-500 shadow-amber-500/25', 
    info: 'bg-blue-600 border-blue-500 shadow-blue-500/25'
  };
  
  function dismiss() {
    toasts.remove(toast.id);
  }
  

</script>

<div 
  class="toast {colors[toast.type]}"
  transition:fly={{ x: 300, duration: 300 }}
  role="alert"
  aria-live="polite"
>
  <div class="toast-content">
    <span class="toast-icon" role="img" aria-hidden="true">
      {icons[toast.type]}
    </span>
    <span class="toast-message">
      {toast.message}
    </span>
  </div>
  
  {#if toast.dismissible}
    <button 
      class="toast-close"
      onclick={dismiss}
      aria-label="Dismiss notification"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  {/if}
</div>

<style>
  .toast {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    border-radius: 0.75rem;
    border-left: 4px solid;
    color: white;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
    min-width: 20rem;
    max-width: 28rem;
    backdrop-filter: blur(8px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    position: relative;
    overflow: hidden;
  }
  
  .toast::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%);
    pointer-events: none;
  }
  
  .toast-content {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
  }
  
  .toast-icon {
    font-size: 1.125rem;
    flex-shrink: 0;
  }
  
  .toast-message {
    font-size: 0.875rem;
    font-weight: 500;
    line-height: 1.625;
  }
  
  .toast-close {
    margin-left: 1rem;
    transition: all 0.2s ease;
    flex-shrink: 0;
    width: 1.75rem;
    height: 1.75rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.375rem;
    cursor: pointer;
    border: none;
    background: transparent;
    color: rgba(255, 255, 255, 0.8);
  }
  
  .toast-close:hover {
    color: white;
    background-color: rgba(255, 255, 255, 0.15);
    transform: scale(1.05);
  }
  
  .toast-close:focus {
    outline: 2px solid rgba(255, 255, 255, 0.5);
    outline-offset: 2px;
  }
</style>