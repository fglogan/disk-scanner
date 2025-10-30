<script lang="ts">
  import { onMount } from 'svelte';
  import * as d3 from 'd3';
  
  interface TreemapData {
    name: string;
    value: number;
    color?: string;
    children?: TreemapData[];
  }

  interface Props {
    data: TreemapData;
    width?: number;
    height?: number;
    colorScheme?: string[];
    animated?: boolean;
    class?: string;
  }

  let {
    data,
    width = 600,
    height = 400,
    colorScheme = ['#ef4444', '#f97316', '#eab308', '#22c55e', '#3b82f6', '#8b5cf6'],
    animated = true,
    class: className = ''
  }: Props = $props();

  let svgElement: SVGSVGElement;
  let tooltipVisible = $state(false);
  let tooltipContent = $state({ name: '', value: '', x: 0, y: 0 });

  onMount(() => {
    drawTreemap();
    return () => {
      d3.select(svgElement).selectAll('*').interrupt();
    };
  });

  $effect(() => {
    if (svgElement && data) {
      drawTreemap();
    }
  });

  function formatSize(bytes: number): string {
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    if (bytes === 0) return '0 B';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${sizes[i]}`;
  }

  function drawTreemap() {
    if (!svgElement || !data) return;

    const svg = d3.select(svgElement);
    svg.selectAll('*').remove();

    // Create hierarchy
    const root = d3.hierarchy(data)
      .sum(d => d.value)
      .sort((a, b) => (b.value || 0) - (a.value || 0));

    // Create treemap layout
    const treemap = d3.treemap<TreemapData>()
      .size([width, height])
      .paddingInner(2)
      .paddingOuter(4)
      .round(true);

    treemap(root);

    // Color scale
    const color = d3.scaleOrdinal(colorScheme);

    // Create groups for each node
    const cell = svg.selectAll('.cell')
      .data(root.leaves())
      .enter()
      .append('g')
      .attr('class', 'cell cursor-pointer')
      .attr('transform', (d: any) => `translate(${d.x0},${d.y0})`);

    // Add rectangles
    const rects = cell.append('rect')
      .attr('width', (d: any) => Math.max(0, d.x1 - d.x0))
      .attr('height', (d: any) => Math.max(0, d.y1 - d.y0))
      .attr('fill', (d: any, i: number) => d.data.color || color(i.toString()))
      .attr('stroke', '#1e293b')
      .attr('stroke-width', 1)
      .style('opacity', 0.8);

    if (animated) {
      rects
        .style('opacity', 0)
        .transition()
        .duration(800)
        .delay((d: any, i: number) => i * 50)
        .style('opacity', 0.8);
    }

    // Add text labels
    cell.append('text')
      .attr('x', 4)
      .attr('y', 16)
      .style('font-size', (d: any) => {
        const area = (d.x1 - d.x0) * (d.y1 - d.y0);
        return Math.min(14, Math.max(8, Math.sqrt(area) / 8)) + 'px';
      })
      .style('font-weight', 'bold')
      .style('fill', 'white')
      .style('text-shadow', '1px 1px 2px rgba(0,0,0,0.8)')
      .style('pointer-events', 'none')
      .text((d: any) => {
        const rectWidth = d.x1 - d.x0;
        const name = d.data.name;
        if (rectWidth < 60) return '';
        return name.length > 15 ? name.slice(0, 12) + '...' : name;
      });

    // Add size labels
    cell.append('text')
      .attr('x', 4)
      .attr('y', 32)
      .style('font-size', (d: any) => {
        const area = (d.x1 - d.x0) * (d.y1 - d.y0);
        return Math.min(12, Math.max(8, Math.sqrt(area) / 10)) + 'px';
      })
      .style('fill', '#e2e8f0')
      .style('text-shadow', '1px 1px 2px rgba(0,0,0,0.8)')
      .style('pointer-events', 'none')
      .text((d: any) => {
        const rectWidth = d.x1 - d.x0;
        if (rectWidth < 80) return '';
        return formatSize(d.data.value);
      });

    // Add hover effects and tooltips
    cell
      .on('mouseenter', function(event, d) {
        d3.select(this).select('rect')
          .transition()
          .duration(200)
          .style('opacity', 1)
          .attr('stroke-width', 2);

        tooltipContent = {
          name: d.data.name,
          value: formatSize(d.data.value),
          x: event.pageX + 10,
          y: event.pageY - 10
        };
        tooltipVisible = true;
      })
      .on('mousemove', function(event) {
        tooltipContent.x = event.pageX + 10;
        tooltipContent.y = event.pageY - 10;
      })
      .on('mouseleave', function() {
        d3.select(this).select('rect')
          .transition()
          .duration(200)
          .style('opacity', 0.8)
          .attr('stroke-width', 1);

        tooltipVisible = false;
      });
  }
</script>

<div class="treemap-container {className} relative">
  <svg
    bind:this={svgElement}
    {width}
    {height}
    class="w-full h-full"
    role="img"
    aria-label="Treemap visualization of file sizes"
  >
  </svg>

  {#if tooltipVisible}
    <div
      class="tooltip absolute z-50 bg-slate-800 text-white p-2 rounded-lg shadow-lg border border-slate-600 pointer-events-none"
      style="left: {tooltipContent.x}px; top: {tooltipContent.y}px;"
    >
      <div class="font-semibold">{tooltipContent.name}</div>
      <div class="text-sm text-slate-300">{tooltipContent.value}</div>
    </div>
  {/if}
</div>

<style>
  .treemap-container {
    overflow: visible;
  }
  
  :global(.cell) {
    transition: all 0.2s ease;
  }
  
  .tooltip {
    transform: translate(-50%, -100%);
  }
</style>