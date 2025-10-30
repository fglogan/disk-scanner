<script lang="ts">
  import { onMount } from 'svelte';
  
  interface Props {
    children?: any;
  }
  
  let { children }: Props = $props();
  
  let isMobile = $state(false);
  let isTablet = $state(false);
  let sidebarOpen = $state(false);
  let windowWidth = $state(0);
  
  // Responsive breakpoints
  const MOBILE_BREAKPOINT = 768;
  const TABLET_BREAKPOINT = 1024;
  
  $effect(() => {
    isMobile = windowWidth < MOBILE_BREAKPOINT;
    isTablet = windowWidth >= MOBILE_BREAKPOINT && windowWidth < TABLET_BREAKPOINT;
    
    // Auto-close sidebar on mobile when window resizes
    if (isMobile && sidebarOpen) {
      sidebarOpen = false;
    }
  });
  
  function handleResize(): void {
    windowWidth = window.innerWidth;
  }
  
  function toggleSidebar(): void {
    sidebarOpen = !sidebarOpen;
  }
  
  function closeSidebar(): void {
    sidebarOpen = false;
  }
  
  onMount(() => {
    windowWidth = window.innerWidth;
    window.addEventListener('resize', handleResize);
    
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  });
</script>

<div class="app-layout" class:mobile={isMobile} class:tablet={isTablet}>
  <!-- Mobile Header -->
  {#if isMobile}
    <header class="mobile-header">
      <button 
        class="sidebar-toggle"
        onclick={toggleSidebar}
        aria-label="Toggle navigation menu"
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>
      
      <h1 class="mobile-title">Project Scanner</h1>
      
      <div class="mobile-actions">
        <!-- Quick actions for mobile -->
      </div>
    </header>
  {/if}
  
  <!-- Sidebar Overlay (Mobile) -->
  {#if isMobile && sidebarOpen}
    <div 
      class="sidebar-overlay"
      onclick={closeSidebar}
      role="button"
      tabindex="0"
      aria-label="Close navigation menu"
    ></div>
  {/if}
  
  <!-- Sidebar -->
  <aside 
    class="sidebar"
    class:open={sidebarOpen}
    class:mobile={isMobile}
  >
    <!-- Sidebar content will be injected here -->
    <slot name="sidebar" />
  </aside>
  
  <!-- Main Content -->
  <main 
    class="main-content"
    class:sidebar-open={sidebarOpen && !isMobile}
    class:mobile={isMobile}
  >
    {@render children?.()}
  </main>
</div>

<style>
  .app-layout {
    display: flex;
    height: 100vh;
    background-color: var(--bg-primary);
    color: var(--text-primary);
  }
  
  /* Mobile Header */
  .mobile-header {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: var(--header-height);
    background-color: var(--bg-secondary);
    border-bottom: 1px solid var(--border-primary);
    display: flex;
    align-items: center;
    padding: 0 var(--space-4);
    z-index: var(--z-fixed);
  }
  
  .sidebar-toggle {
    background: none;
    border: none;
    color: var(--text-primary);
    padding: var(--space-2);
    border-radius: var(--button-radius);
    cursor: pointer;
    transition: background-color var(--transition-fast);
  }
  
  .sidebar-toggle:hover {
    background-color: var(--bg-tertiary);
  }
  
  .mobile-title {
    flex: 1;
    text-align: center;
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    margin: 0;
  }
  
  .mobile-actions {
    width: 40px; /* Balance the toggle button */
  }
  
  /* Sidebar */
  .sidebar {
    width: var(--sidebar-width);
    background-color: var(--bg-secondary);
    border-right: 1px solid var(--border-primary);
    transition: transform var(--transition-normal);
    z-index: var(--z-fixed);
  }
  
  .sidebar.mobile {
    position: fixed;
    top: 0;
    left: 0;
    height: 100vh;
    transform: translateX(-100%);
    box-shadow: var(--card-shadow-lg);
  }
  
  .sidebar.mobile.open {
    transform: translateX(0);
  }
  
  /* Sidebar Overlay */
  .sidebar-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: calc(var(--z-fixed) - 1);
    cursor: pointer;
  }
  
  /* Main Content */
  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-8) var(--space-12);
    transition: margin-left var(--transition-normal);
  }
  
  .main-content.mobile {
    padding: var(--space-4);
    padding-top: calc(var(--header-height) + var(--space-4));
  }
  
  /* Tablet adjustments */
  .app-layout.tablet .main-content {
    padding: var(--space-6) var(--space-8);
  }
  
  .app-layout.tablet .sidebar {
    width: calc(var(--sidebar-width) * 0.8);
  }
  
  /* Responsive grid adjustments */
  .app-layout.mobile :global(.grid) {
    grid-template-columns: 1fr !important;
    gap: var(--space-4) !important;
  }
  
  .app-layout.tablet :global(.grid-cols-3) {
    grid-template-columns: repeat(2, 1fr) !important;
  }
  
  .app-layout.tablet :global(.lg\\:col-span-2) {
    grid-column: span 2 !important;
  }
  
  /* Hide desktop-only elements on mobile */
  .app-layout.mobile :global(.desktop-only) {
    display: none !important;
  }
  
  /* Show mobile-only elements */
  .app-layout:not(.mobile) :global(.mobile-only) {
    display: none !important;
  }
  
  /* Responsive text sizing */
  .app-layout.mobile :global(h1) {
    font-size: var(--text-2xl) !important;
  }
  
  .app-layout.mobile :global(h2) {
    font-size: var(--text-xl) !important;
  }
  
  .app-layout.mobile :global(h3) {
    font-size: var(--text-lg) !important;
  }
  
  /* Responsive spacing */
  .app-layout.mobile :global(.space-y-8) {
    gap: var(--space-4) !important;
  }
  
  .app-layout.mobile :global(.space-y-6) {
    gap: var(--space-3) !important;
  }
  
  /* Touch-friendly button sizing */
  .app-layout.mobile :global(button) {
    min-height: 44px; /* iOS recommended touch target */
    padding: var(--space-3) var(--space-4);
  }
  
  /* Responsive cards */
  .app-layout.mobile :global(.card) {
    padding: var(--card-padding-sm);
    border-radius: calc(var(--card-radius) * 0.75);
  }
</style>