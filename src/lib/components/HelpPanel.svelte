<script lang="ts">
  import { t } from '../i18n';
  import { showShortcutsHelp } from '../stores.js';
  import { ariaProps } from '../utils/accessibility';
  import AccessibleButton from './ui/AccessibleButton.svelte';
  
  export let isOpen = false;
  export let context = 'general';
  
  const helpTopics = {
    general: {
      title: 'Getting Started',
      sections: [
        {
          heading: 'Quick Start',
          content: [
            '1. Click "Select Directory" to choose a folder to scan',
            '2. Click "Scan" to analyze the directory',
            '3. Review the results and select files to clean up',
            '4. Click "Clean Up" to remove selected files'
          ]
        },
        {
          heading: 'Scan Categories',
          content: [
            'Large Files: Files over 100MB',
            'Duplicates: Files with identical content',
            'Dev Caches: node_modules, target, build directories',
            'System Junk: Temporary files, logs, caches'
          ]
        }
      ]
    },
    scanning: {
      title: 'Scanning Options',
      sections: [
        {
          heading: 'What gets scanned?',
          content: [
            'All files and subdirectories in the selected folder',
            'Hidden files (starting with .) are included',
            'Symlinks are followed by default',
            'System directories are protected'
          ]
        },
        {
          heading: 'Performance Tips',
          content: [
            'Scan smaller directories first',
            'Close other applications to free up memory',
            'Use exclude patterns for faster scans',
            'Enable background monitoring for automatic scans'
          ]
        }
      ]
    },
    cleanup: {
      title: 'Safe Cleanup',
      sections: [
        {
          heading: 'Deletion Safety',
          content: [
            'Files are moved to trash by default',
            'Permanent deletion requires confirmation',
            'Critical system files are protected',
            'Deletion history is logged'
          ]
        },
        {
          heading: 'Best Practices',
          content: [
            'Review files before deletion',
            'Start with obvious duplicates',
            'Keep one copy of important files',
            'Regular cleanup prevents buildup'
          ]
        }
      ]
    }
  };
  
  $: currentHelp = helpTopics[context] || helpTopics.general;
  
  function openShortcuts() {
    $showShortcutsHelp = true;
    isOpen = false;
  }
</script>

{#if isOpen}
  <div 
    class="help-panel"
    {...ariaProps({
      role: 'dialog',
      label: $t('help.title'),
      describedby: 'help-content'
    })}
  >
    <div class="help-header">
      <h2 class="help-title">{currentHelp.title}</h2>
      <AccessibleButton 
        variant="ghost"
        size="sm"
        on:click={() => isOpen = false}
        aria={{ label: $t('actions.close') }}
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </AccessibleButton>
    </div>
    
    <div id="help-content" class="help-content">
      {#each currentHelp.sections as section}
        <section class="help-section">
          <h3 class="help-section-title">{section.heading}</h3>
          <ul class="help-list">
            {#each section.content as item}
              <li class="help-item">{item}</li>
            {/each}
          </ul>
        </section>
      {/each}
      
      <div class="help-actions">
        <AccessibleButton 
          variant="secondary"
          on:click={openShortcuts}
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
          </svg>
          {$t('help.keyboardShortcuts')}
        </AccessibleButton>
        
        <AccessibleButton 
          variant="secondary"
          on:click={() => window.open('https://github.com/yourusername/disk-bloat-scanner/wiki', '_blank')}
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
          </svg>
          Full Documentation
        </AccessibleButton>
      </div>
    </div>
  </div>
  
  <!-- Backdrop -->
  <div 
    class="help-backdrop" 
    on:click={() => isOpen = false}
    aria-hidden="true"
  />
{/if}

<style>
  .help-panel {
    position: fixed;
    right: 1rem;
    top: 4rem;
    width: 24rem;
    max-width: calc(100vw - 2rem);
    max-height: calc(100vh - 6rem);
    background: var(--surface-card);
    border: 1px solid var(--border-primary);
    border-radius: 0.5rem;
    box-shadow: var(--shadow-lg);
    z-index: 100;
    display: flex;
    flex-direction: column;
    animation: slideIn var(--transition-normal) ease-out;
  }
  
  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateX(1rem);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
  
  .help-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.3);
    z-index: 99;
    animation: fadeIn var(--transition-fast) ease-out;
  }
  
  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
  
  .help-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem;
    border-bottom: 1px solid var(--border-primary);
  }
  
  .help-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }
  
  .help-content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
  }
  
  .help-section {
    margin-bottom: 1.5rem;
  }
  
  .help-section:last-child {
    margin-bottom: 0;
  }
  
  .help-section-title {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 0.5rem;
  }
  
  .help-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  
  .help-item {
    padding: 0.375rem 0;
    color: var(--text-secondary);
    font-size: 0.875rem;
    line-height: 1.5;
  }
  
  .help-item:before {
    content: "• ";
    color: var(--text-muted);
    margin-right: 0.25rem;
  }
  
  .help-actions {
    display: flex;
    gap: 0.5rem;
    margin-top: 1.5rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-primary);
  }
  
  @media (max-width: 640px) {
    .help-panel {
      right: 0;
      top: 0;
      width: 100%;
      height: 100%;
      max-width: none;
      max-height: none;
      border-radius: 0;
    }
    
    .help-actions {
      flex-direction: column;
    }
  }
</style>