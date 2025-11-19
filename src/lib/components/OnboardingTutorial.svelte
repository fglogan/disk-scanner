<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { writable } from 'svelte/store';
  import { t } from '../i18n';
  import { ariaProps, trapFocus, announce } from '../utils/accessibility';
  import AccessibleButton from './ui/AccessibleButton.svelte';
  
  // Tutorial state
  const tutorialState = writable({
    active: false,
    currentStep: 0,
    completed: false
  });
  
  // Check if user has seen tutorial
  function hasSeenTutorial() {
    return localStorage.getItem('tutorial-completed') === 'true';
  }
  
  // Mark tutorial as completed
  function completeTutorial() {
    localStorage.setItem('tutorial-completed', 'true');
    tutorialState.update(s => ({ ...s, active: false, completed: true }));
  }
  
  // Skip tutorial
  function skipTutorial() {
    completeTutorial();
  }
  
  // Tutorial steps
  const steps = [
    {
      target: '[data-tutorial="select-directory"]',
      title: $t('tutorial.step1Title'),
      content: $t('tutorial.step1Text'),
      position: 'bottom'
    },
    {
      target: '[data-tutorial="scan-button"]',
      title: 'Start Scanning',
      content: 'Click this button to begin scanning the selected directory',
      position: 'bottom'
    },
    {
      target: '[data-tutorial="results-panel"]',
      title: $t('tutorial.step2Title'),
      content: $t('tutorial.step2Text'),
      position: 'left'
    },
    {
      target: '[data-tutorial="cleanup-button"]',
      title: $t('tutorial.step3Title'),
      content: $t('tutorial.step3Text'),
      position: 'top'
    }
  ];
  
  let spotlight: HTMLElement | null = null;
  let tooltip: HTMLElement | null = null;
  let focusTrap: { destroy: () => void } | null = null;
  
  $: currentStep = $tutorialState.currentStep;
  $: isActive = $tutorialState.active;
  $: step = steps[currentStep];
  
  // Start tutorial
  export function startTutorial() {
    tutorialState.set({
      active: true,
      currentStep: 0,
      completed: false
    });
    announce($t('tutorial.welcome'));
  }
  
  // Navigate steps
  function nextStep() {
    tutorialState.update(s => {
      if (s.currentStep < steps.length - 1) {
        announce(`Step ${s.currentStep + 2} of ${steps.length}`);
        return { ...s, currentStep: s.currentStep + 1 };
      } else {
        completeTutorial();
        announce('Tutorial completed');
        return s;
      }
    });
  }
  
  function prevStep() {
    tutorialState.update(s => {
      if (s.currentStep > 0) {
        announce(`Step ${s.currentStep} of ${steps.length}`);
        return { ...s, currentStep: s.currentStep - 1 };
      }
      return s;
    });
  }
  
  // Position spotlight and tooltip
  function positionElements() {
    if (!isActive || !step) return;
    
    const target = document.querySelector(step.target);
    if (!target) return;
    
    const rect = target.getBoundingClientRect();
    
    // Position spotlight
    if (spotlight) {
      spotlight.style.top = `${rect.top - 10}px`;
      spotlight.style.left = `${rect.left - 10}px`;
      spotlight.style.width = `${rect.width + 20}px`;
      spotlight.style.height = `${rect.height + 20}px`;
    }
    
    // Position tooltip
    if (tooltip) {
      const tooltipRect = tooltip.getBoundingClientRect();
      let top = 0;
      let left = 0;
      
      switch (step.position) {
        case 'top':
          top = rect.top - tooltipRect.height - 20;
          left = rect.left + (rect.width - tooltipRect.width) / 2;
          break;
        case 'bottom':
          top = rect.bottom + 20;
          left = rect.left + (rect.width - tooltipRect.width) / 2;
          break;
        case 'left':
          top = rect.top + (rect.height - tooltipRect.height) / 2;
          left = rect.left - tooltipRect.width - 20;
          break;
        case 'right':
          top = rect.top + (rect.height - tooltipRect.height) / 2;
          left = rect.right + 20;
          break;
      }
      
      // Keep tooltip on screen
      top = Math.max(10, Math.min(top, window.innerHeight - tooltipRect.height - 10));
      left = Math.max(10, Math.min(left, window.innerWidth - tooltipRect.width - 10));
      
      tooltip.style.top = `${top}px`;
      tooltip.style.left = `${left}px`;
    }
  }
  
  // Initialize tutorial on mount
  onMount(() => {
    if (!hasSeenTutorial()) {
      // Auto-start tutorial for new users
      setTimeout(() => {
        startTutorial();
      }, 1000);
    }
    
    // Listen for window resize
    window.addEventListener('resize', positionElements);
    window.addEventListener('scroll', positionElements);
  });
  
  onDestroy(() => {
    window.removeEventListener('resize', positionElements);
    window.removeEventListener('scroll', positionElements);
    focusTrap?.destroy();
  });
  
  // Watch for step changes
  $: if (isActive) {
    setTimeout(positionElements, 50);
    
    // Set up focus trap on tooltip
    if (tooltip && !focusTrap) {
      focusTrap = trapFocus(tooltip);
    }
  } else if (focusTrap) {
    focusTrap.destroy();
    focusTrap = null;
  }
</script>

{#if isActive}
  <!-- Backdrop -->
  <div class="tutorial-backdrop" aria-hidden="true">
    <!-- Spotlight hole -->
    <div 
      bind:this={spotlight}
      class="tutorial-spotlight"
    />
  </div>
  
  <!-- Tooltip -->
  <div 
    bind:this={tooltip}
    class="tutorial-tooltip"
    {...ariaProps({
      role: 'dialog',
      label: 'Tutorial',
      describedby: 'tutorial-content'
    })}
  >
    <div class="tutorial-header">
      <h3 class="tutorial-title">{step.title}</h3>
      <span class="tutorial-progress">
        {currentStep + 1} / {steps.length}
      </span>
    </div>
    
    <div id="tutorial-content" class="tutorial-content">
      {step.content}
    </div>
    
    <div class="tutorial-actions">
      <AccessibleButton
        variant="ghost"
        size="sm"
        on:click={skipTutorial}
      >
        {$t('tutorial.skipTutorial')}
      </AccessibleButton>
      
      <div class="tutorial-nav">
        <AccessibleButton
          variant="secondary"
          size="sm"
          disabled={currentStep === 0}
          on:click={prevStep}
          aria={{ label: $t('actions.back') }}
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
          </svg>
        </AccessibleButton>
        
        <AccessibleButton
          variant="primary"
          size="sm"
          on:click={nextStep}
        >
          {currentStep === steps.length - 1 ? $t('actions.finish') : $t('actions.next')}
          {#if currentStep < steps.length - 1}
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          {/if}
        </AccessibleButton>
      </div>
    </div>
  </div>
{/if}

<style>
  .tutorial-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    z-index: 9998;
    pointer-events: none;
  }
  
  .tutorial-spotlight {
    position: absolute;
    border-radius: 0.5rem;
    box-shadow: 
      0 0 0 9999px rgba(0, 0, 0, 0.7),
      0 0 20px rgba(99, 102, 241, 0.5);
    pointer-events: auto;
    transition: all var(--transition-normal) ease-out;
  }
  
  .tutorial-tooltip {
    position: fixed;
    background: var(--surface-card);
    border: 1px solid var(--border-primary);
    border-radius: 0.5rem;
    box-shadow: var(--shadow-lg);
    padding: 1.5rem;
    max-width: 20rem;
    z-index: 9999;
    animation: fadeInScale var(--transition-normal) ease-out;
  }
  
  @keyframes fadeInScale {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
  
  .tutorial-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1rem;
  }
  
  .tutorial-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }
  
  .tutorial-progress {
    font-size: 0.75rem;
    color: var(--text-muted);
    background: var(--bg-tertiary);
    padding: 0.125rem 0.5rem;
    border-radius: 9999px;
  }
  
  .tutorial-content {
    color: var(--text-secondary);
    line-height: 1.5;
    margin-bottom: 1.5rem;
  }
  
  .tutorial-actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  
  .tutorial-nav {
    display: flex;
    gap: 0.5rem;
  }
  
  @media (max-width: 640px) {
    .tutorial-tooltip {
      max-width: calc(100vw - 2rem);
      padding: 1rem;
    }
  }
</style>