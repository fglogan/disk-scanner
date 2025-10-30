<script lang="ts">
  interface Props {
    variant?: 'default' | 'elevated' | 'outlined' | 'glass';
    padding?: 'none' | 'sm' | 'md' | 'lg';
    class?: string;
    children?: any;
  }
  
  let {
    variant = 'default',
    padding = 'md',
    class: className = '',
    children
  }: Props = $props();
  
  const variantStyles = {
    default: 'bg-slate-800 shadow-lg',
    elevated: 'bg-slate-800 shadow-xl shadow-slate-900/50',
    outlined: 'bg-slate-800/50 border border-slate-700',
    glass: 'bg-slate-800/70 backdrop-blur-sm border border-slate-700/50'
  } as const;
  
  const paddingStyles = {
    none: '',
    sm: 'p-4',
    md: 'p-6',
    lg: 'p-8'
  } as const;
  
  let computedClasses = $derived([
    'rounded-xl transition-all duration-200',
    variantStyles[variant],
    paddingStyles[padding],
    className
  ].filter(Boolean).join(' '));
</script>

<div class={computedClasses}>
  {@render children?.()}
</div>