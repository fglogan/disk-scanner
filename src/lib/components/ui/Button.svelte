<script lang="ts">
  import type { ComponentProps } from 'svelte';
  
  // ✅ Comprehensive props interface
  interface Props {
    variant?: 'primary' | 'secondary' | 'danger' | 'ghost' | 'outline';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    loading?: boolean;
    fullWidth?: boolean;
    icon?: string;
    iconPosition?: 'left' | 'right';
    type?: 'button' | 'submit' | 'reset';
    class?: string;
    onclick?: (event: MouseEvent) => void;
    children?: any;
  }
  
  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    loading = false,
    fullWidth = false,
    icon,
    iconPosition = 'left',
    type = 'button',
    class: className = '',
    onclick,
    children
  }: Props = $props();
  
  // ✅ Computed styles with proper typing
  const baseStyles = 'inline-flex items-center justify-center font-semibold rounded-lg transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-slate-800 disabled:opacity-50 disabled:cursor-not-allowed';
  
  const variantStyles = {
    primary: 'bg-indigo-600 hover:bg-indigo-500 text-white shadow-lg shadow-indigo-600/20 focus:ring-indigo-500',
    secondary: 'bg-slate-600 hover:bg-slate-500 text-white shadow-lg focus:ring-slate-500',
    danger: 'bg-red-600 hover:bg-red-500 text-white shadow-lg shadow-red-600/20 focus:ring-red-500',
    ghost: 'bg-transparent hover:bg-slate-700 text-slate-300 hover:text-white focus:ring-slate-500',
    outline: 'border-2 border-slate-600 hover:border-slate-500 bg-transparent hover:bg-slate-700 text-slate-300 hover:text-white focus:ring-slate-500'
  } as const;
  
  const sizeStyles = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2.5 text-sm',
    lg: 'px-6 py-3 text-base'
  } as const;
  
  let computedClasses = $derived([
    baseStyles,
    variantStyles[variant],
    sizeStyles[size],
    fullWidth ? 'w-full' : '',
    className
  ].filter(Boolean).join(' '));
  
  let isDisabled = $derived(disabled || loading);
  
  function handleClick(event: MouseEvent): void {
    if (isDisabled) {
      event.preventDefault();
      return;
    }
    onclick?.(event);
  }
  
  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      if (!isDisabled) {
        onclick?.(new MouseEvent('click'));
      }
    }
  }
</script>

<button
  {type}
  class={computedClasses}
  disabled={isDisabled}
  onclick={handleClick}
  onkeydown={handleKeydown}
  aria-busy={loading}
>
  {#if loading}
    <svg 
      class="animate-spin -ml-1 mr-2 h-4 w-4 text-current" 
      fill="none" 
      viewBox="0 0 24 24"
      aria-hidden="true"
    >
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
    </svg>
    Loading...
  {:else}
    {#if icon && iconPosition === 'left'}
      <span class="mr-2" role="img" aria-hidden="true">{icon}</span>
    {/if}
    
    {@render children?.()}
    
    {#if icon && iconPosition === 'right'}
      <span class="ml-2" role="img" aria-hidden="true">{icon}</span>
    {/if}
  {/if}
</button>

<style>
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
  
  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>