<script lang="ts">
  import { themeStore, isDarkMode, THEMES } from '../../stores/theme';
  import { showSuccess } from '../../stores.js';
  
  let showMenu = false;
  
  function selectTheme(theme: typeof THEMES[keyof typeof THEMES]) {
    themeStore.setTheme(theme);
    showMenu = false;
    
    const themeLabel = theme === THEMES.SYSTEM ? 'system preference' : `${theme} mode`;
    showSuccess(`Switched to ${themeLabel}`);
  }
  
  function toggleTheme() {
    themeStore.toggle();
    showSuccess(`Switched to ${$isDarkMode ? 'dark' : 'light'} mode`);
  }
  
  // Close menu on click outside
  function handleClickOutside(e: MouseEvent) {
    if (!(e.target as Element).closest('.theme-selector')) {
      showMenu = false;
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="theme-selector relative">
  <button
    class="theme-toggle-btn"
    onclick={() => showMenu = !showMenu}
    aria-label="Theme selector"
    aria-expanded={showMenu}
    aria-haspopup="true"
    title="Select theme"
  >
    <div class="theme-toggle-icon">
      {#if $isDarkMode}
        <!-- Moon icon for dark mode -->
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
        </svg>
      {:else}
        <!-- Sun icon for light mode -->
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>
      {/if}
    </div>
    <svg class="w-3 h-3 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
    </svg>
  </button>
  
  {#if showMenu}
    <div class="theme-menu" role="menu">
      <button
        class="theme-option"
        class:active={$themeStore.theme === THEMES.LIGHT}
        onclick={() => selectTheme(THEMES.LIGHT)}
        role="menuitem"
        tabindex="0"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>
        Light
      </button>
      
      <button
        class="theme-option"
        class:active={$themeStore.theme === THEMES.DARK}
        onclick={() => selectTheme(THEMES.DARK)}
        role="menuitem"
        tabindex="0"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
        </svg>
        Dark
      </button>
      
      <button
        class="theme-option"
        class:active={$themeStore.theme === THEMES.SYSTEM}
        onclick={() => selectTheme(THEMES.SYSTEM)}
        role="menuitem"
        tabindex="0"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
        </svg>
        System
      </button>
    </div>
  {/if}
</div>

<style>
  .theme-selector {
    position: relative;
  }
  
  .theme-toggle-btn {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.5rem 0.75rem;
    border-radius: 0.5rem;
    border: 1px solid var(--border-primary);
    background: var(--surface-card);
    color: var(--text-secondary);
    transition: all var(--transition-fast);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
  }
  
  .theme-toggle-btn:hover {
    background: var(--surface-hover);
    border-color: var(--border-secondary);
    color: var(--text-primary);
    transform: translateY(-1px);
  }
  
  .theme-toggle-btn:focus-visible {
    outline: 2px solid var(--ring-color);
    outline-offset: 2px;
  }
  
  .theme-toggle-icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .theme-menu {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;
    min-width: 10rem;
    background: var(--surface-card);
    border: 1px solid var(--border-primary);
    border-radius: 0.5rem;
    box-shadow: var(--shadow-lg);
    padding: 0.25rem;
    z-index: 50;
    animation: slideDown var(--transition-fast) ease-out;
  }
  
  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-0.5rem);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  
  .theme-option {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 0.875rem;
    font-weight: 500;
    text-align: left;
    border-radius: 0.375rem;
    cursor: pointer;
    transition: all var(--transition-fast);
  }
  
  .theme-option:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }
  
  .theme-option:focus-visible {
    outline: 2px solid var(--ring-color);
    outline-offset: -2px;
  }
  
  .theme-option.active {
    background: var(--bg-accent);
    color: var(--link-color);
  }
  
  @media (max-width: 768px) {
    .theme-menu {
      right: auto;
      left: 0;
    }
  }
</style>