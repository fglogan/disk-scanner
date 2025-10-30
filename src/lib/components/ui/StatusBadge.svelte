<script lang="ts">
  interface Props {
    status: 'success' | 'warning' | 'error' | 'info' | 'neutral';
    size?: 'sm' | 'md' | 'lg';
    variant?: 'solid' | 'soft' | 'outline';
    icon?: string;
    class?: string;
    children?: any;
  }
  
  let {
    status,
    size = 'md',
    variant = 'soft',
    icon,
    class: className = '',
    children
  }: Props = $props();
  
  const statusConfig = {
    success: {
      solid: 'bg-emerald-600 text-white',
      soft: 'bg-emerald-900/30 text-emerald-400 border border-emerald-600/30',
      outline: 'border border-emerald-600 text-emerald-400 bg-transparent',
      defaultIcon: '✓'
    },
    warning: {
      solid: 'bg-amber-600 text-white',
      soft: 'bg-amber-900/30 text-amber-400 border border-amber-600/30',
      outline: 'border border-amber-600 text-amber-400 bg-transparent',
      defaultIcon: '⚠'
    },
    error: {
      solid: 'bg-red-600 text-white',
      soft: 'bg-red-900/30 text-red-400 border border-red-600/30',
      outline: 'border border-red-600 text-red-400 bg-transparent',
      defaultIcon: '⛔'
    },
    info: {
      solid: 'bg-blue-600 text-white',
      soft: 'bg-blue-900/30 text-blue-400 border border-blue-600/30',
      outline: 'border border-blue-600 text-blue-400 bg-transparent',
      defaultIcon: 'ℹ'
    },
    neutral: {
      solid: 'bg-slate-600 text-white',
      soft: 'bg-slate-700/40 text-slate-300 border border-slate-600/30',
      outline: 'border border-slate-600 text-slate-300 bg-transparent',
      defaultIcon: '●'
    }
  } as const;
  
  const sizeStyles = {
    sm: 'px-2 py-1 text-xs',
    md: 'px-3 py-1.5 text-sm',
    lg: 'px-4 py-2 text-base'
  } as const;
  
  let displayIcon = $derived(icon || statusConfig[status].defaultIcon);
  let statusStyles = $derived(statusConfig[status][variant]);
  
  let computedClasses = $derived([
    'inline-flex items-center space-x-1.5 rounded-full font-medium',
    statusStyles,
    sizeStyles[size],
    className
  ].filter(Boolean).join(' '));
</script>

<span class={computedClasses} role="status">
  {#if displayIcon}
    <span role="img" aria-hidden="true">{displayIcon}</span>
  {/if}
  <span>{@render children?.()}</span>
</span>