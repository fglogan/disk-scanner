<script lang="ts">
  // Virtual scrolling component for large file lists
  // Improves performance when displaying thousands of files
  
  interface Props<T> {
    items: T[];
    itemHeight: number;
    containerHeight: number;
    renderItem: (item: T, index: number) => any;
    keyExtractor: (item: T, index: number) => string | number;
    overscan?: number;
    class?: string;
  }
  
  let {
    items,
    itemHeight,
    containerHeight,
    renderItem,
    keyExtractor,
    overscan = 5,
    class: className = ''
  }: Props<any> = $props();
  
  let scrollTop = $state(0);
  let containerElement: HTMLDivElement;
  
  // Calculate visible range
  let visibleRange = $derived(() => {
    const start = Math.floor(scrollTop / itemHeight);
    const visibleCount = Math.ceil(containerHeight / itemHeight);
    const end = start + visibleCount;
    
    return {
      start: Math.max(0, start - overscan),
      end: Math.min(items.length, end + overscan)
    };
  });
  
  let visibleItems = $derived(() => {
    const { start, end } = visibleRange;
    return items.slice(start, end).map((item, index) => ({
      item,
      index: start + index,
      key: keyExtractor(item, start + index)
    }));
  });
  
  let totalHeight = $derived(items.length * itemHeight);
  let offsetY = $derived(visibleRange.start * itemHeight);
  
  function handleScroll(event: Event): void {
    const target = event.target as HTMLDivElement;
    scrollTop = target.scrollTop;
  }
</script>

<div
  bind:this={containerElement}
  class="virtual-list-container {className}"
  style="height: {containerHeight}px; overflow-y: auto;"
  onscroll={handleScroll}
>
  <div style="height: {totalHeight}px; position: relative;">
    <div style="transform: translateY({offsetY}px);">
      {#each visibleItems as { item, index, key } (key)}
        <div style="height: {itemHeight}px;">
          {@render renderItem(item, index)}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .virtual-list-container {
    position: relative;
  }
  
  .virtual-list-container::-webkit-scrollbar {
    width: 8px;
  }
  
  .virtual-list-container::-webkit-scrollbar-track {
    background: var(--color-slate-800);
  }
  
  .virtual-list-container::-webkit-scrollbar-thumb {
    background: var(--color-slate-600);
    border-radius: 4px;
  }
  
  .virtual-list-container::-webkit-scrollbar-thumb:hover {
    background: var(--color-slate-500);
  }
</style>