<script lang="ts">
  import { onMount } from 'svelte';

  interface Props {
    progress: number; // 0-100
    label?: string;
    size?: 'sm' | 'md' | 'lg';
    variant?: 'linear' | 'circular' | 'stepped';
    animated?: boolean;
    showPercentage?: boolean;
    color?: string;
    steps?: string[];
    class?: string;
  }

  let {
    progress,
    label = '',
    size = 'md',
    variant = 'linear',
    animated = true,
    showPercentage = true,
    color = '#3b82f6',
    steps = [],
    class: className = ''
  }: Props = $props();

  let animatedProgress = $state(0);

  const sizeClasses = {
    sm: { height: 'h-2', text: 'text-sm', width: 'w-24', stroke: '4' },
    md: { height: 'h-4', text: 'text-base', width: 'w-32', stroke: '6' },
    lg: { height: 'h-6', text: 'text-lg', width: 'w-40', stroke: '8' }
  };

  const currentSize = $derived(sizeClasses[size]);

  onMount(() => {
    if (animated) {
      animateProgress();
    } else {
      animatedProgress = progress;
    }
  });

  $effect(() => {
    if (animated) {
      animateProgress();
    } else {
      animatedProgress = progress;
    }
  });

  function animateProgress() {
    const duration = 800;
    const startTime = performance.now();
    const startValue = animatedProgress;
    const targetValue = Math.max(0, Math.min(100, progress));

    function animate(currentTime: number) {
      const elapsed = currentTime - startTime;
      const progressRatio = Math.min(elapsed / duration, 1);
      
      // Easing function (ease-out)
      const eased = 1 - Math.pow(1 - progressRatio, 3);
      animatedProgress = startValue + (targetValue - startValue) * eased;

      if (progressRatio < 1) {
        requestAnimationFrame(animate);
      }
    }

    requestAnimationFrame(animate);
  }

  function getStepStatus(index: number): 'completed' | 'current' | 'pending' {
    const stepProgress = (100 / steps.length) * (index + 1);
    if (animatedProgress >= stepProgress) return 'completed';
    if (animatedProgress >= stepProgress - (100 / steps.length)) return 'current';
    return 'pending';
  }
</script>

<div class="progress-indicator {className}">
  {#if label}
    <div class="flex justify-between items-center mb-2">
      <span class="font-medium text-slate-300 {currentSize.text}">
        {label}
      </span>
      {#if showPercentage && variant !== 'stepped'}
        <span class="text-slate-400 {currentSize.text}">
          {Math.round(animatedProgress)}%
        </span>
      {/if}
    </div>
  {/if}

  {#if variant === 'linear'}
    <div 
      class="progress-track bg-slate-700 rounded-full overflow-hidden {currentSize.height}"
      role="progressbar"
      aria-valuenow={animatedProgress}
      aria-valuemin="0"
      aria-valuemax="100"
      aria-label={label || `Progress: ${Math.round(animatedProgress)}%`}
    >
      <div 
        class="progress-fill h-full rounded-full transition-all duration-300 ease-out"
        style="width: {animatedProgress}%; background-color: {color}; box-shadow: 0 0 10px {color}40;"
      ></div>
    </div>

  {:else if variant === 'circular'}
    <div class="flex items-center justify-center {currentSize.width} aspect-square">
      <svg 
        class="transform -rotate-90" 
        width="100%" 
        height="100%" 
        viewBox="0 0 100 100"
        role="progressbar"
        aria-valuenow={animatedProgress}
        aria-valuemin="0"
        aria-valuemax="100"
        aria-label={label || `Progress: ${Math.round(animatedProgress)}%`}
      >
        <!-- Background circle -->
        <circle
          cx="50"
          cy="50"
          r="45"
          fill="none"
          stroke="#374151"
          stroke-width={currentSize.stroke}
        />
        <!-- Progress circle -->
        <circle
          cx="50"
          cy="50"
          r="45"
          fill="none"
          stroke={color}
          stroke-width={currentSize.stroke}
          stroke-linecap="round"
          stroke-dasharray="282.74"
          stroke-dashoffset={282.74 - (282.74 * animatedProgress) / 100}
          class="transition-all duration-500 ease-out"
          style="filter: drop-shadow(0 0 5px {color}40)"
        />
      </svg>
      {#if showPercentage}
        <div class="absolute inset-0 flex items-center justify-center">
          <span class="font-bold text-slate-200 {currentSize.text}">
            {Math.round(animatedProgress)}%
          </span>
        </div>
      {/if}
    </div>

  {:else if variant === 'stepped'}
    <div class="space-y-3">
      {#each steps as step, index}
        {@const status = getStepStatus(index)}
        <div class="flex items-center gap-3">
          <div 
            class="flex-shrink-0 w-6 h-6 rounded-full border-2 flex items-center justify-center transition-all duration-300"
            class:bg-green-500={status === 'completed'}
            class:border-green-500={status === 'completed'}
            class:bg-blue-500={status === 'current'}
            class:border-blue-500={status === 'current'}
            class:animate-pulse={status === 'current'}
            class:border-slate-500={status === 'pending'}
            class:bg-slate-700={status === 'pending'}
          >
            {#if status === 'completed'}
              <svg class="w-3 h-3 text-white" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
              </svg>
            {:else if status === 'current'}
              <div class="w-2 h-2 bg-white rounded-full"></div>
            {:else}
              <div class="w-2 h-2 bg-slate-500 rounded-full"></div>
            {/if}
          </div>
          <span 
            class="flex-grow transition-colors duration-300"
            class:text-slate-200={status === 'completed' || status === 'current'}
            class:font-medium={status === 'current'}
            class:text-slate-400={status === 'pending'}
          >
            {step}
          </span>
          {#if status === 'current' && showPercentage}
            <span class="text-blue-400 text-sm">
              {Math.round(animatedProgress)}%
            </span>
          {/if}
        </div>
        {#if index < steps.length - 1}
          <div class="ml-3 w-0.5 h-4 bg-slate-600"></div>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .progress-indicator {
    @apply select-none;
  }
  
  .progress-fill {
    background: linear-gradient(90deg, 
      var(--progress-color, #3b82f6) 0%, 
      var(--progress-color-light, #60a5fa) 100%
    );
  }
  
  @keyframes shimmer {
    0% { background-position: -200px 0; }
    100% { background-position: calc(200px + 100%) 0; }
  }
  
  .progress-fill.animated {
    background: linear-gradient(
      90deg,
      var(--progress-color, #3b82f6) 25%,
      var(--progress-color-light, #60a5fa) 50%,
      var(--progress-color, #3b82f6) 75%
    );
    background-size: 200px 100%;
    animation: shimmer 2s infinite linear;
  }
</style>