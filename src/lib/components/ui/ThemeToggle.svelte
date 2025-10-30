<script lang="ts">
  import { darkMode, showSuccess } from '../../stores.js';
  
  function toggleTheme() {
    darkMode.toggle();
    showSuccess(`Switched to ${$darkMode ? 'dark' : 'light'} mode`);
  }
</script>

<button
  class="theme-toggle-btn"
  onclick={toggleTheme}
  aria-label="Toggle theme between dark and light mode"
  title="Toggle theme (Ctrl/Cmd + T)"
>
  <div class="theme-toggle-icon">
    {#if $darkMode}
      <!-- Sun icon for dark mode (to switch to light) -->
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
      </svg>
    {:else}
      <!-- Moon icon for light mode (to switch to dark) -->
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
      </svg>
    {/if}
  </div>
  <span class="theme-toggle-text">
    {$darkMode ? 'Light' : 'Dark'}
  </span>
</button>

<style>
  .theme-toggle-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border-radius: 0.5rem;
    border: 1px solid;
    background: transparent;
    transition: all 0.2s ease;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    
    /* Light mode styles */
    color: rgb(71 85 105); /* slate-600 */
    border-color: rgb(203 213 225); /* slate-300 */
  }
  
  .theme-toggle-btn:hover {
    /* Light mode hover */
    background: rgb(248 250 252); /* slate-50 */
    border-color: rgb(148 163 184); /* slate-400 */
    transform: translateY(-1px);
  }
  
  .theme-toggle-btn:active {
    transform: translateY(0);
  }
  
  /* Dark mode styles */
  :global(.dark) .theme-toggle-btn {
    color: rgb(148 163 184); /* slate-400 */
    border-color: rgb(71 85 105); /* slate-600 */
  }
  
  :global(.dark) .theme-toggle-btn:hover {
    background: rgb(30 41 59); /* slate-800 */
    border-color: rgb(100 116 139); /* slate-500 */
    color: rgb(203 213 225); /* slate-300 */
  }
  
  .theme-toggle-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.3s ease;
  }
  
  .theme-toggle-btn:hover .theme-toggle-icon {
    transform: rotate(180deg);
  }
  
  .theme-toggle-text {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.025em;
  }
  
  @media (max-width: 768px) {
    .theme-toggle-text {
      display: none;
    }
    
    .theme-toggle-btn {
      padding: 0.5rem;
    }
  }
</style>