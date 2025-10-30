<script lang="ts">
  // Advanced Responsive Grid System
  // Provides flexible, responsive layout with automatic sizing
  
  interface GridProps {
    columns?: number | 'auto' | string;
    gap?: string;
    minItemWidth?: string;
    maxItemWidth?: string;
    alignItems?: 'start' | 'center' | 'end' | 'stretch';
    justifyContent?: 'start' | 'center' | 'end' | 'space-between' | 'space-around' | 'space-evenly';
    responsive?: boolean;
    breakpoints?: {
      sm?: number;
      md?: number;
      lg?: number;
      xl?: number;
    };
  }
  
  let {
    columns = 'auto',
    gap = '1rem',
    minItemWidth = '250px',
    maxItemWidth = '1fr',
    alignItems = 'stretch',
    justifyContent = 'start',
    responsive = true,
    breakpoints = {
      sm: 1,
      md: 2,
      lg: 3,
      xl: 4
    },
    children
  }: GridProps & { children: any } = $props();
  
  let gridContainer: HTMLDivElement;
  let currentBreakpoint = $state('xl');
  let containerWidth = $state(0);
  
  // Calculate grid template columns based on current state
  $: gridTemplateColumns = (() => {
    if (typeof columns === 'number') {
      return `repeat(${columns}, 1fr)`;
    }
    
    if (columns === 'auto') {
      if (responsive) {
        return `repeat(auto-fit, minmax(${minItemWidth}, ${maxItemWidth}))`;
      }
      return `repeat(auto-fit, minmax(${minItemWidth}, 1fr))`;
    }
    
    if (responsive && breakpoints) {
      const bp = breakpoints[currentBreakpoint as keyof typeof breakpoints];
      if (bp) {
        return `repeat(${bp}, 1fr)`;
      }
    }
    
    return columns;
  })();
  
  // Responsive breakpoint detection
  function updateBreakpoint() {
    if (!responsive || !gridContainer) return;
    
    const width = gridContainer.offsetWidth;
    containerWidth = width;
    
    if (width >= 1280) {
      currentBreakpoint = 'xl';
    } else if (width >= 1024) {
      currentBreakpoint = 'lg';
    } else if (width >= 768) {
      currentBreakpoint = 'md';
    } else {
      currentBreakpoint = 'sm';
    }
  }
  
  // Resize observer for responsive behavior
  let resizeObserver: ResizeObserver | null = null;
  
  $effect(() => {
    if (gridContainer && responsive) {
      resizeObserver = new ResizeObserver(() => {
        updateBreakpoint();
      });
      
      resizeObserver.observe(gridContainer);
      updateBreakpoint();
      
      return () => {
        if (resizeObserver) {
          resizeObserver.disconnect();
        }
      };
    }
  });
  
  // Grid item auto-sizing
  function getItemStyle(index: number) {
    const baseStyle = {
      'grid-column': 'span 1',
      'grid-row': 'span 1'
    };
    
    // Add any special positioning logic here
    return Object.entries(baseStyle)
      .map(([key, value]) => `${key}: ${value}`)
      .join('; ');
  }
</script>

<div 
  class="responsive-grid"
  bind:this={gridContainer}
  style:grid-template-columns={gridTemplateColumns}
  style:gap={gap}
  style:align-items={alignItems}
  style:justify-content={justifyContent}
  data-breakpoint={currentBreakpoint}
  data-width={containerWidth}
>
  {@render children()}
</div>

<style>
  .responsive-grid {
    display: grid;
    width: 100%;
    transition: all 0.3s ease;
  }
  
  /* Responsive utilities */
  .responsive-grid[data-breakpoint="sm"] {
    --grid-columns: 1;
  }
  
  .responsive-grid[data-breakpoint="md"] {
    --grid-columns: 2;
  }
  
  .responsive-grid[data-breakpoint="lg"] {
    --grid-columns: 3;
  }
  
  .responsive-grid[data-breakpoint="xl"] {
    --grid-columns: 4;
  }
  
  /* Grid item animations */
  .responsive-grid > :global(*) {
    animation: gridItemFadeIn 0.3s ease-out;
    animation-fill-mode: both;
  }
  
  .responsive-grid > :global(*:nth-child(1)) { animation-delay: 0ms; }
  .responsive-grid > :global(*:nth-child(2)) { animation-delay: 50ms; }
  .responsive-grid > :global(*:nth-child(3)) { animation-delay: 100ms; }
  .responsive-grid > :global(*:nth-child(4)) { animation-delay: 150ms; }
  .responsive-grid > :global(*:nth-child(5)) { animation-delay: 200ms; }
  .responsive-grid > :global(*:nth-child(6)) { animation-delay: 250ms; }
  
  @keyframes gridItemFadeIn {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  
  /* Responsive design patterns */
  @media (max-width: 640px) {
    .responsive-grid {
      gap: 0.75rem;
    }
  }
  
  @media (max-width: 480px) {
    .responsive-grid {
      gap: 0.5rem;
    }
  }
</style>