<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  interface Props {
    breakpoints?: {
      sm?: number;
      md?: number;
      lg?: number;
      xl?: number;
      '2xl'?: number;
    };
    class?: string;
    children?: any;
  }

  let {
    breakpoints = {
      sm: 640,
      md: 768,
      lg: 1024,
      xl: 1280,
      '2xl': 1536
    },
    class: className = '',
    children
  }: Props = $props();

  let containerElement: HTMLDivElement;
  let currentBreakpoint = $state('xs');
  let containerWidth = $state(0);
  let containerHeight = $state(0);

  let resizeObserver: ResizeObserver | null = null;

  const breakpointValues = $derived(() => [
    { name: 'xs', min: 0 },
    { name: 'sm', min: breakpoints.sm || 640 },
    { name: 'md', min: breakpoints.md || 768 },
    { name: 'lg', min: breakpoints.lg || 1024 },
    { name: 'xl', min: breakpoints.xl || 1280 },
    { name: '2xl', min: breakpoints['2xl'] || 1536 }
  ]);

  onMount(() => {
    if (!containerElement) return;

    updateDimensions();

    // Use ResizeObserver for better performance than window resize
    if (typeof ResizeObserver !== 'undefined') {
      const observer = new ResizeObserver(entries => {
        for (const entry of entries) {
          const { width, height } = entry.contentRect;
          containerWidth = width;
          containerHeight = height;
          updateBreakpoint(width);
        }
      });

      observer.observe(containerElement);
      // Store observer for cleanup
    } else {
      // Fallback for older browsers
      window.addEventListener('resize', updateDimensions);
    }
  });

  onDestroy(() => {
    // ResizeObserver cleanup is handled automatically
    window.removeEventListener('resize', updateDimensions);
  });

  function updateDimensions() {
    if (!containerElement) return;
    
    const rect = containerElement.getBoundingClientRect();
    containerWidth = rect.width;
    containerHeight = rect.height;
    updateBreakpoint(rect.width);
  }

  function updateBreakpoint(width: number) {
    const activeBreakpoint = [...breakpointValues()]
      .reverse()
      .find((bp: any) => width >= bp.min);
    
    if (activeBreakpoint) {
      currentBreakpoint = activeBreakpoint.name;
    }
  }

  // Provide context to child components
  const containerContext = $derived(() => ({
    breakpoint: currentBreakpoint,
    width: containerWidth,
    height: containerHeight,
    isMobile: currentBreakpoint === 'xs' || currentBreakpoint === 'sm',
    isTablet: currentBreakpoint === 'md',
    isDesktop: currentBreakpoint === 'lg' || currentBreakpoint === 'xl' || currentBreakpoint === '2xl',
    isSmall: currentBreakpoint === 'xs' || currentBreakpoint === 'sm',
    isMedium: currentBreakpoint === 'md' || currentBreakpoint === 'lg',
    isLarge: currentBreakpoint === 'xl' || currentBreakpoint === '2xl'
  }));

  // CSS custom properties for responsive design
  $effect(() => {
    if (containerElement) {
      containerElement.style.setProperty('--container-width', `${containerWidth}px`);
      containerElement.style.setProperty('--container-height', `${containerHeight}px`);
      containerElement.style.setProperty('--breakpoint', currentBreakpoint);
    }
  });
</script>

<div
  bind:this={containerElement}
  class="responsive-container {className}"
  data-breakpoint={currentBreakpoint}
  data-width={containerWidth}
  data-height={containerHeight}
  style="--container-width: {containerWidth}px; --container-height: {containerHeight}px;"
>
  {#if children}
    {@render children(containerContext)}
  {:else}
    <slot {containerContext} />
  {/if}
</div>

<style>
  .responsive-container {
    width: 100%;
    height: 100%;
    position: relative;
  }

  /* Breakpoint-specific styles */
  .responsive-container[data-breakpoint="xs"] {
    --responsive-padding: 1rem;
    --responsive-gap: 0.5rem;
    --responsive-grid-cols: 1;
  }

  .responsive-container[data-breakpoint="sm"] {
    --responsive-padding: 1.5rem;
    --responsive-gap: 0.75rem;
    --responsive-grid-cols: 2;
  }

  .responsive-container[data-breakpoint="md"] {
    --responsive-padding: 2rem;
    --responsive-gap: 1rem;
    --responsive-grid-cols: 3;
  }

  .responsive-container[data-breakpoint="lg"] {
    --responsive-padding: 2.5rem;
    --responsive-gap: 1.5rem;
    --responsive-grid-cols: 4;
  }

  .responsive-container[data-breakpoint="xl"] {
    --responsive-padding: 3rem;
    --responsive-gap: 2rem;
    --responsive-grid-cols: 5;
  }

  .responsive-container[data-breakpoint="2xl"] {
    --responsive-padding: 3rem;
    --responsive-gap: 2rem;
    --responsive-grid-cols: 6;
  }

  /* Utility classes that can be used by child components */
  :global(.responsive-grid) {
    display: grid;
    grid-template-columns: repeat(var(--responsive-grid-cols), 1fr);
    gap: var(--responsive-gap);
    padding: var(--responsive-padding);
  }

  :global(.responsive-padding) {
    padding: var(--responsive-padding);
  }

  :global(.responsive-gap) {
    gap: var(--responsive-gap);
  }
</style>