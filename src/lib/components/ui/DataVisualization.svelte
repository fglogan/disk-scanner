<script lang="ts">
  // Advanced Data Visualization Component - Svelte 5 Runes
  // Interactive charts and graphs for disk usage analysis
  
  import { onMount } from 'svelte';
  
  interface DataPoint {
    label: string;
    value: number;
    color: string;
    category?: string;
  }
  
  interface ChartProps {
    data: DataPoint[];
    type: 'pie' | 'bar';
    title: string;
    interactive?: boolean;
    showLegend?: boolean;
    height?: number;
  }
  
  let { 
    data = [], 
    type = 'pie', 
    title = 'Data Visualization',
    interactive = true,
    showLegend = true,
    height = 400
  }: ChartProps = $props();
  
  let selectedSegment = $state<string | null>(null);
  let hoveredSegment = $state<string | null>(null);
  let animationProgress = $state(0);
  let isAnimating = $state(false);
  
  onMount(() => {
    animateChart();
  });
  
  function animateChart() {
    isAnimating = true;
    animationProgress = 0;
    
    const duration = 1000;
    const startTime = Date.now();
    
    function animate() {
      const elapsed = Date.now() - startTime;
      const progress = Math.min(elapsed / duration, 1);
      
      animationProgress = 1 - Math.pow(1 - progress, 3);
      
      if (progress < 1) {
        requestAnimationFrame(animate);
      } else {
        isAnimating = false;
      }
    }
    
    requestAnimationFrame(animate);
  }
  
  // Calculate chart data with animation
  let chartData = $derived(data.map((item, index) => {
    const total = data.reduce((sum, d) => sum + d.value, 0);
    const percentage = total > 0 ? (item.value / total) * 100 : 0;
    
    return {
      ...item,
      percentage,
      index,
      animatedValue: item.value * animationProgress
    };
  }));
  
  function handleSegmentClick(segment: any) {
    if (!interactive) return;
    selectedSegment = selectedSegment === segment.label ? null : segment.label;
  }
  
  function handleSegmentHover(segment: any | null) {
    if (!interactive) return;
    hoveredSegment = segment?.label || null;
  }
  
  function formatValue(value: number): string {
    if (value >= 1024) {
      return `${(value / 1024).toFixed(1)} GB`;
    }
    return `${value.toFixed(1)} MB`;
  }
  
  function formatPercentage(percentage: number): string {
    return `${percentage.toFixed(1)}%`;
  }
</script>

<div class="data-visualization" style="height: {height + 100}px;">
  <div class="chart-header">
    <h3 class="chart-title">{title}</h3>
    {#if selectedSegment}
      <div class="selection-info">
        Selected: {selectedSegment}
      </div>
    {/if}
  </div>
  
  <div class="chart-container">
    {#if type === 'pie'}
      <svg width="100%" height={height} viewBox="0 0 400 400">
        {#each chartData as segment, i}
          {@const total = data.reduce((sum, d) => sum + d.value, 0)}
          {@const angle = total > 0 ? (segment.animatedValue / total) * 360 : 0}
          {@const startAngle = chartData.slice(0, i).reduce((sum, d) => sum + (d.animatedValue / total) * 360, -90)}
          {@const endAngle = startAngle + angle}
          {@const radius = 150}
          {@const centerX = 200}
          {@const centerY = 200}
          {@const startAngleRad = (startAngle * Math.PI) / 180}
          {@const endAngleRad = (endAngle * Math.PI) / 180}
          {@const x1 = centerX + radius * Math.cos(startAngleRad)}
          {@const y1 = centerY + radius * Math.sin(startAngleRad)}
          {@const x2 = centerX + radius * Math.cos(endAngleRad)}
          {@const y2 = centerY + radius * Math.sin(endAngleRad)}
          {@const largeArcFlag = angle > 180 ? 1 : 0}
          {@const pathData = `M ${centerX} ${centerY} L ${x1} ${y1} A ${radius} ${radius} 0 ${largeArcFlag} 1 ${x2} ${y2} Z`}
          
          <path
            d={pathData}
            fill={segment.color}
            stroke="rgba(255, 255, 255, 0.2)"
            stroke-width="2"
            class="pie-segment"
            class:selected={selectedSegment === segment.label}
            class:hovered={hoveredSegment === segment.label}
            onclick={() => handleSegmentClick(segment)}
            onmouseenter={() => handleSegmentHover(segment)}
            onmouseleave={() => handleSegmentHover(null)}
            role="button"
            tabindex="0"
            aria-label="{segment.label}: {formatValue(segment.value)} ({formatPercentage(segment.percentage)})"
            onkeydown={(e) => e.key === 'Enter' && handleSegmentClick(segment)}
          />
          
          {#if segment.percentage > 5 && !isAnimating}
            {@const labelX = centerX + (radius * 0.7) * Math.cos((startAngle + endAngle) / 2 * Math.PI / 180)}
            {@const labelY = centerY + (radius * 0.7) * Math.sin((startAngle + endAngle) / 2 * Math.PI / 180)}
            <text
              x={labelX}
              y={labelY}
              text-anchor="middle"
              dominant-baseline="middle"
              class="segment-label"
              fill="white"
              font-size="12"
              font-weight="500"
            >
              {formatPercentage(segment.percentage)}
            </text>
          {/if}
        {/each}
      </svg>
      
    {:else if type === 'bar'}
      <svg width="100%" height={height} viewBox="0 0 {50 + data.length * 60} {height}">
        {#each chartData as segment, index}
          {@const maxValue = Math.max(...data.map(d => d.value))}
          {@const barHeight = maxValue > 0 ? (segment.animatedValue / maxValue) * (height - 80) : 0}
          {@const x = 50 + index * 60}
          {@const y = height - 40 - barHeight}
          
          <rect
            x={x}
            y={y}
            width="40"
            height={barHeight}
            fill={segment.color}
            class="bar-segment"
            class:selected={selectedSegment === segment.label}
            class:hovered={hoveredSegment === segment.label}
            onclick={() => handleSegmentClick(segment)}
            onmouseenter={() => handleSegmentHover(segment)}
            onmouseleave={() => handleSegmentHover(null)}
            role="button"
            tabindex="0"
            aria-label="{segment.label}: {formatValue(segment.value)}"
            onkeydown={(e) => e.key === 'Enter' && handleSegmentClick(segment)}
          />
          
          <text
            x={x + 20}
            y={height - 20}
            text-anchor="middle"
            class="bar-label"
            fill="#e2e8f0"
            font-size="11"
          >
            {segment.label.length > 8 ? segment.label.substring(0, 8) + '...' : segment.label}
          </text>
          
          {#if barHeight > 30}
            <text
              x={x + 20}
              y={y + barHeight / 2}
              text-anchor="middle"
              dominant-baseline="middle"
              class="bar-value"
              fill="white"
              font-size="10"
              font-weight="500"
            >
              {formatValue(segment.value)}
            </text>
          {/if}
        {/each}
      </svg>
    {/if}
    
    {#if hoveredSegment}
      <div class="tooltip" role="tooltip">
        <div class="tooltip-content">
          <div class="tooltip-title">{hoveredSegment}</div>
          <div class="tooltip-value">
            {formatValue(chartData.find(d => d.label === hoveredSegment)?.value || 0)}
          </div>
          <div class="tooltip-percentage">
            {formatPercentage(chartData.find(d => d.label === hoveredSegment)?.percentage || 0)}
          </div>
        </div>
      </div>
    {/if}
  </div>
  
  {#if showLegend}
    <div class="chart-legend">
      {#each chartData as item}
        <div 
          class="legend-item"
          class:selected={selectedSegment === item.label}
          onclick={() => handleSegmentClick(item)}
          role="button"
          tabindex="0"
          aria-label="Toggle {item.label}"
          onkeydown={(e) => e.key === 'Enter' && handleSegmentClick(item)}
        >
          <div class="legend-color" style="background-color: {item.color}"></div>
          <div class="legend-label">{item.label}</div>
          <div class="legend-value">{formatValue(item.value)}</div>
          <div class="legend-percentage">({formatPercentage(item.percentage)})</div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .data-visualization {
    display: flex;
    flex-direction: column;
    background: #1e293b;
    border-radius: 0.75rem;
    padding: 1.5rem;
    color: #e2e8f0;
    position: relative;
    animation: fadeIn 0.3s ease-out;
  }
  
  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }
  
  .chart-title {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #f8fafc;
  }
  
  .selection-info {
    font-size: 0.875rem;
    color: #94a3b8;
    background: #334155;
    padding: 0.25rem 0.75rem;
    border-radius: 0.375rem;
  }
  
  .chart-container {
    flex: 1;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  
  .pie-segment, .bar-segment {
    cursor: pointer;
    transition: all 0.2s ease;
    filter: brightness(1);
  }
  
  .pie-segment:hover, .bar-segment:hover,
  .pie-segment.hovered, .bar-segment.hovered {
    filter: brightness(1.2);
    transform: scale(1.02);
    transform-origin: center;
  }
  
  .pie-segment.selected, .bar-segment.selected {
    filter: brightness(1.3);
    stroke: #60a5fa;
    stroke-width: 3;
  }
  
  .segment-label, .bar-label, .bar-value {
    pointer-events: none;
    user-select: none;
  }
  
  .tooltip {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: rgba(0, 0, 0, 0.9);
    border: 1px solid #475569;
    border-radius: 0.5rem;
    padding: 0.75rem;
    pointer-events: none;
    z-index: 10;
    backdrop-filter: blur(8px);
  }
  
  .tooltip-content {
    text-align: center;
  }
  
  .tooltip-title {
    font-weight: 600;
    color: #f8fafc;
    margin-bottom: 0.25rem;
  }
  
  .tooltip-value {
    font-size: 1.125rem;
    font-weight: 700;
    color: #60a5fa;
    margin-bottom: 0.125rem;
  }
  
  .tooltip-percentage {
    font-size: 0.875rem;
    color: #94a3b8;
  }
  
  .chart-legend {
    margin-top: 1.5rem;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 0.75rem;
  }
  
  .legend-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.375rem;
    cursor: pointer;
    transition: all 0.2s ease;
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid transparent;
  }
  
  .legend-item:hover {
    background: rgba(255, 255, 255, 0.05);
    border-color: #475569;
  }
  
  .legend-item.selected {
    background: rgba(96, 165, 250, 0.1);
    border-color: #60a5fa;
  }
  
  .legend-color {
    width: 1rem;
    height: 1rem;
    border-radius: 0.25rem;
    flex-shrink: 0;
  }
  
  .legend-label {
    flex: 1;
    font-weight: 500;
    color: #e2e8f0;
  }
  
  .legend-value {
    font-weight: 600;
    color: #f8fafc;
  }
  
  .legend-percentage {
    font-size: 0.875rem;
    color: #94a3b8;
    margin-left: 0.25rem;
  }
  
  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: scale(0.9);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
  
  @media (max-width: 768px) {
    .data-visualization {
      padding: 1rem;
    }
    
    .chart-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }
    
    .chart-legend {
      grid-template-columns: 1fr;
    }
    
    .legend-item {
      font-size: 0.875rem;
    }
  }
</style>