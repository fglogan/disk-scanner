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
    success: 'bg-green-600 border-green-500',
    error: 'bg-red-600 border-red-500',
    warning: 'bg-amber-600 border-amber-500', 
    info: 'bg-blue-600 border-blue-500'
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
      ×
    </button>
  {/if}
</div>

<style>
  .toast {
    @apply flex items-center justify-between;
    @apply px-4 py-3 rounded-lg border-l-4;
    @apply text-white shadow-lg;
    @apply min-w-80 max-w-md;
    @apply backdrop-blur-sm;
    background: rgba(var(--toast-bg), 0.95);
  }
  
  .toast-content {
    @apply flex items-center gap-3;
    @apply flex-1;
  }
  
  .toast-icon {
    @apply text-lg;
    @apply flex-shrink-0;
  }
  
  .toast-message {
    @apply text-sm font-medium;
    @apply leading-relaxed;
  }
  
  .toast-close {
    @apply ml-4 text-xl font-bold;
    @apply hover:opacity-75 transition-opacity;
    @apply flex-shrink-0;
    @apply w-6 h-6 flex items-center justify-center;
    @apply rounded-full hover:bg-white/20;
  }
</style>