<script lang="ts">
  import { currentPage, darkMode, showSuccess } from "../stores.js";
  import tempextLogo from "../../assets/tempext-logo.png";

  function toggleTheme() {
    darkMode.toggle();
    showSuccess(`Switched to ${$darkMode ? 'dark' : 'light'} mode`);
  }

  interface NavItem {
    id: string;
    label: string;
    icon: string;
  }

  const navItems: NavItem[] = [
    { id: "dashboard", label: "Dashboard", icon: "home" },
    { id: "project-scanner", label: "Project Scanner", icon: "git-branch" },
    { id: "pacs-compliance", label: "PACS Compliance", icon: "shield-check" },
    { id: "architecture-viz", label: "Architecture Viz", icon: "network" },
    { id: "large-files", label: "Large Files", icon: "file-stack" },
    { id: "project-bloat", label: "Project Bloat", icon: "package-x" },
    { id: "system-junk", label: "System Junk", icon: "trash" },
    { id: "duplicates", label: "Duplicates", icon: "copy" },
    { id: "dev-caches", label: "Developer Caches", icon: "recycle" },
    { id: "git-assistance", label: "Git Assistance", icon: "git-help" },
  ];

  function navigateTo(pageId: string): void {
    currentPage.set(pageId);
  }
</script>

<nav
  class="sidebar-vibrant w-64 h-full flex-shrink-0 p-4 space-y-2 overflow-y-auto transition-colors duration-200"
  class:dark={$darkMode}
>
  <div class="px-4 py-2 mb-6">
    <!-- Tempext Branding -->
    <div class="flex items-center space-x-2 mb-6">
      <img 
        src={tempextLogo}
        alt="Tempext" 
        class="h-18 w-auto"
      />
    </div>
    <!-- App Title -->
    <h1 class="text-xl font-bold text-white flex items-center space-x-2">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="text-indigo-400"
      >
        <path d="M3 6h18" /><path
          d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"
        /><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" /><line
          x1="10"
          x2="10"
          y1="11"
          y2="17"
        /><line x1="14" x2="14" y1="11" y2="17" />
      </svg>
      <span>Project Scanner</span>
    </h1>
    <p class="text-xs text-slate-400 mt-1 ml-8">by Tempext AI</p>
  </div>

  <ul class="space-y-1">
    {#each navItems as item}
      <li>
        <button
          on:click={() => navigateTo(item.id)}
          class="nav-link w-full text-left {$currentPage === item.id
            ? 'active'
            : ''} flex items-center space-x-3 px-4 py-2.5 rounded-lg font-medium text-slate-300 hover:bg-slate-700 hover:text-white transition-colors duration-150"
        >
          {#if item.icon === "home"}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><path
                d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"
              /><polyline points="9 22 9 12 15 12 15 22" /></svg
            >
          {:else if item.icon === "file-stack"}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><path
                d="M16 22h2c.5 0 1-.2 1.4-.6.4-.4.6-.9.6-1.4V7.5L14.5 2H6c-1.1 0-2 .9-2 2v3"
              /><path d="M14 2v6h6" /><path
                d="M10 20v-1c0-1.1.9-2 2-2h2c1.1 0 2 .9 2 2v1"
              /><path d="M10 16h6" /><path
                d="M4 11.2V20c0 1.1.9 2 2 2h2"
              /></svg
            >
          {:else if item.icon === "package-x"}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><path
                d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v2"
              /><path
                d="M21 14v4a2 2 0 0 1-1 1.73l-7 4a2 2 0 0 1-2 0l-7-4A2 2 0 0 1 3 18v-4"
              /><path d="M3 10V8" /><path d="M21 10V8" /><path
                d="M12 22v-8"
              /><path d="m3.29 7 8.71 5 8.71-5" /><path
                d="m3.29 17 8.71 5 8.71-5"
              /><line x1="9" x2="15" y1="12" y2="12" /><line
                x1="14.8"
                x2="9.2"
                y1="9"
                y2="15"
              /></svg
            >
          {:else if item.icon === "copy"}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><rect width="14" height="14" x="8" y="8" rx="2" ry="2" /><path
                d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"
              /></svg
            >
          {:else if item.icon === "trash"}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg
            >
          {:else if item.icon === "recycle"}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><path d="M7 19a5 5 0 1 0 10 0" /><path d="M11 19V8l-4 4" /><path
                d="m15 13-4-4"
              /><path d="M12 11v8" /><path d="M10.5 13.5 7 10l-4 4" /><path
                d="m13.5 10.5 3.5 3.5 4-4"
              /><path d="M14 7h4a2 2 0 0 1 2 2v0a2 2 0 0 1-2 2h-4" /></svg
            >
           {:else if item.icon === "git-branch"}
             <svg
               xmlns="http://www.w3.org/2000/svg"
               width="20"
               height="20"
               viewBox="0 0 24 24"
               fill="none"
               stroke="currentColor"
               stroke-width="2"
               stroke-linecap="round"
               stroke-linejoin="round"
               ><line x1="6" x2="6" y1="3" y2="15" /><circle
                 cx="18"
                 cy="6"
                 r="3"
               /><circle cx="6" cy="18" r="3" /><path
                 d="M18 9a9 9 0 0 1-9 9"
               /></svg
             >
           {:else if item.icon === "git-help"}
             <svg
               xmlns="http://www.w3.org/2000/svg"
               width="20"
               height="20"
               viewBox="0 0 24 24"
               fill="none"
               stroke="currentColor"
               stroke-width="2"
               stroke-linecap="round"
               stroke-linejoin="round"
               ><circle cx="12" cy="12" r="10" /><path d="M12 16v-4" /><path
                 d="M12 8h.01"
               /></svg
             >
           {/if}
          <span>{item.label}</span>
        </button>
      </li>
    {/each}
  </ul>

  <ul class="pt-4 mt-4 space-y-1 border-t border-slate-700">
    <li>
      <button
        on:click={toggleTheme}
        class="nav-link w-full text-left flex items-center space-x-3 px-4 py-2.5 rounded-lg font-medium text-slate-300 hover:bg-slate-700 hover:text-white transition-colors duration-150"
        title="Toggle theme (Ctrl/Cmd + T)"
      >
        {#if $darkMode}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <circle cx="12" cy="12" r="5"/>
            <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
          </svg>
        {:else}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
          </svg>
        {/if}
        <span>{$darkMode ? 'Light Mode' : 'Dark Mode'}</span>
      </button>
    </li>
    <li>
      <button
        on:click={() => navigateTo("settings")}
        class="nav-link w-full text-left {$currentPage === 'settings'
          ? 'active'
          : ''} flex items-center space-x-3 px-4 py-2.5 rounded-lg font-medium text-slate-300 hover:bg-slate-700 hover:text-white transition-colors duration-150"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          ><path
            d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 0 2l-.15.08a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.38a2 2 0 0 0-.73-2.73l-.15-.1a2 2 0 0 1 0-2l.15-.08a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
          /><circle cx="12" cy="12" r="3" /></svg
        >
        <span>Settings</span>
      </button>
    </li>
  </ul>
</nav>

<style>
  .sidebar-vibrant {
    background-color: rgba(30, 41, 59, 0.7);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border-right: 1px solid rgba(71, 85, 105, 0.5);
  }

  .sidebar-vibrant:not(.dark) {
    background-color: rgba(248, 250, 252, 0.8);
    border-right: 1px solid rgba(226, 232, 240, 0.8);
  }

  .sidebar-vibrant:not(.dark) :global(.text-white) {
    color: #1e293b !important;
  }

  .sidebar-vibrant:not(.dark) :global(.text-slate-300) {
    color: #64748b !important;
  }

  .sidebar-vibrant:not(.dark) :global(.text-slate-400) {
    color: #94a3b8 !important;
  }

  .nav-link.active {
    background-color: #334155;
    color: #ffffff;
  }

  .sidebar-vibrant:not(.dark) .nav-link.active {
    background-color: #e2e8f0;
    color: #1e293b;
  }

  .sidebar-vibrant:not(.dark) .nav-link:hover {
    background-color: #f1f5f9 !important;
    color: #1e293b !important;
  }
</style>
