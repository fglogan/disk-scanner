<script lang="ts">
  import { onMount } from 'svelte';
  import * as d3 from 'd3';
  import type { DiskInfo } from '../../stores.js';

  interface Props {
    data: DiskInfo;
    width?: number;
    height?: number;
    animated?: boolean;
    showLabels?: boolean;
    class?: string;
  }

  let {
    data,
    width = 300,
    height = 300,
    animated = true,
    showLabels = true,
    class: className = ''
  }: Props = $props();

  let svgElement: SVGSVGElement;
  let isHovered = $state(false);

  type ChartDataItem = {
    name: string;
    value: number;
    color: string;
    percentage: number;
  };

  const chartData = $derived((): ChartDataItem[] => [
    { name: 'Used', value: data.used_gb, color: '#ef4444', percentage: data.usage_pct },
    { name: 'Free', value: data.free_gb, color: '#22c55e', percentage: 100 - data.usage_pct }
  ]);

  onMount(() => {
    drawChart();
    return () => {
      // Cleanup any D3 timers or animations
      d3.select(svgElement).selectAll('*').interrupt();
    };
  });

  $effect(() => {
    if (svgElement && data) {
      drawChart();
    }
  });

  function drawChart() {
    if (!svgElement) return;

    const svg = d3.select(svgElement);
    svg.selectAll('*').remove();

    const radius = Math.min(width, height) / 2 - 20;
    const centerX = width / 2;
    const centerY = height / 2;

    const g = svg.append('g')
      .attr('transform', `translate(${centerX}, ${centerY})`);

    const pie = d3.pie<ChartDataItem>()
      .value(d => d.value)
      .sort(null);

    const arc = d3.arc<d3.PieArcDatum<ChartDataItem>>()
      .innerRadius(radius * 0.6)
      .outerRadius(radius);

    const arcHover = d3.arc<d3.PieArcDatum<ChartDataItem>>()
      .innerRadius(radius * 0.6)
      .outerRadius(radius * 1.1);

    const pieData = pie(chartData());
    const arcs = g.selectAll('.arc')
      .data(pieData)
      .enter()
      .append('g')
      .attr('class', 'arc cursor-pointer');

    // Draw slices
    const paths = arcs.append('path')
      .attr('fill', d => d.data.color)
      .attr('stroke', '#1e293b')
      .attr('stroke-width', 2)
      .style('opacity', 0.9);

    if (animated) {
      paths
        .transition()
        .duration(1000)
        .attrTween('d', function(d) {
          const i = d3.interpolate({ startAngle: 0, endAngle: 0 }, d);
          return (t: number) => arc(i(t)) || '';
        });
    } else {
      paths.attr('d', d => arc(d) || '');
    }

    // Add hover effects
    arcs
      .on('mouseenter', function() {
        isHovered = true;
        d3.select(this).select('path')
          .transition()
          .duration(200)
          .attr('d', (d: any) => arcHover(d) || '');
      })
      .on('mouseleave', function() {
        isHovered = false;
        d3.select(this).select('path')
          .transition()
          .duration(200)
          .attr('d', (d: any) => arc(d) || '');
      });

    if (showLabels) {
      // Add percentage labels
      arcs.append('text')
        .attr('transform', d => `translate(${arc.centroid(d)})`)
        .attr('text-anchor', 'middle')
        .attr('dominant-baseline', 'middle')
        .style('font-size', '14px')
        .style('font-weight', 'bold')
        .style('fill', 'white')
        .style('text-shadow', '1px 1px 2px rgba(0,0,0,0.8)')
        .text(d => `${d.data.percentage.toFixed(1)}%`);
    }

    // Center text with total size
    const centerText = g.append('g')
      .attr('class', 'center-text');

    centerText.append('text')
      .attr('text-anchor', 'middle')
      .attr('y', -10)
      .style('font-size', '18px')
      .style('font-weight', 'bold')
      .style('fill', '#e2e8f0')
      .text(`${data.total_gb.toFixed(1)} GB`);

    centerText.append('text')
      .attr('text-anchor', 'middle')
      .attr('y', 10)
      .style('font-size', '12px')
      .style('fill', '#94a3b8')
      .text('Total');
  }
</script>

<div class="disk-chart-container {className}" class:hovered={isHovered}>
  <svg
    bind:this={svgElement}
    {width}
    {height}
    class="w-full h-full"
    role="img"
    aria-label="Disk usage chart showing {data.usage_pct.toFixed(1)}% used of {data.total_gb.toFixed(1)} GB total"
  >
  </svg>
  
  {#if showLabels}
    <div class="legend mt-4 flex justify-center gap-6 text-sm">
      {#each chartData() as item}
        <div class="flex items-center gap-2">
          <div 
            class="w-3 h-3 rounded-full" 
            style="background-color: {item.color}"
          ></div>
          <span class="text-slate-300">
            {item.name}: {item.value.toFixed(1)} GB
          </span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .disk-chart-container {
    transition: transform 0.2s ease;
  }
  
  .disk-chart-container.hovered {
    transform: scale(1.02);
  }
  
  :global(.arc) {
    transition: all 0.2s ease;
  }
</style>