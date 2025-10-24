<script>
  import { invoke } from "@tauri-apps/api/core";
  import { junkFiles, selectedPaths } from "../stores.js";

  let expandedCategories = new Set();

  function toggleCategory(categoryId) {
    if (expandedCategories.has(categoryId)) {
      expandedCategories.delete(categoryId);
    } else {
      expandedCategories.add(categoryId);
    }
    expandedCategories = expandedCategories; // Trigger reactivity
  }

  function toggleSelection(path) {
    if ($selectedPaths.has(path)) {
      $selectedPaths.delete(path);
    } else {
      $selectedPaths.add(path);
    }
    selectedPaths.set($selectedPaths); // Trigger reactivity
  }

  function isCriticalPath(path) {
    // Check if path contains source code or important directories
    return (
      path.includes("/src/") ||
      path.includes("/lib/") ||
      path.includes("/.git/") ||
      path.includes("/node_modules/") ||
      /\.(rs|js|ts|py|go|cpp|java|rb|php|swift|kt)$/.test(path)
    );
  }

  async function cleanupSelected() {
    if ($selectedPaths.size === 0) {
      alert("No items selected for cleanup");
      return;
    }

    // Check for critical paths
    const criticalPaths = Array.from($selectedPaths).filter(isCriticalPath);
    if (criticalPaths.length > 0) {
      const extraConfirm = confirm(
        `⚠️ WARNING: You're about to delete ${criticalPaths.length} file(s) from SOURCE CODE or CRITICAL directories!\n\n` +
          `This could break your projects. These files will be moved to trash, but recovery may be difficult.\n\n` +
          `Examples:\n${criticalPaths.slice(0, 3).join("\n")}\n\n` +
          `Are you ABSOLUTELY SURE you want to continue?`,
      );

      if (!extraConfirm) {
        return;
      }
    }

    const confirmDelete = confirm(
      `Delete ${$selectedPaths.size} junk file(s)? They will be moved to trash.`,
    );
    
    if (!confirmDelete) {
      return;
    }

    try {
      const paths = Array.from($selectedPaths);
      console.log("🗑️ Deleting paths:", paths);
      
      const result = await invoke("cleanup_dirs", {
        req: { paths, dry_run: false, trash: true },
      });

      console.log("✅ Cleanup result:", result);

      // Clear selection immediately
      selectedPaths.set(new Set());

      // Build success message
      let message = `✅ Successfully deleted ${result.deleted.length} file(s)`;
      
      if (result.skipped.length > 0) {
        message += `\n⚠️ Skipped ${result.skipped.length} file(s) (not found)`;
      }
      
      if (result.errors.length > 0) {
        message += `\n❌ ${result.errors.length} error(s) occurred`;
      }

      message += `\n\nRefresh the page to update the list.`;

      alert(message);

      // Auto-reload after 1 second
      if (result.deleted.length > 0) {
        setTimeout(() => {
          window.location.reload();
        }, 1000);
      }
    } catch (e) {
      console.error("❌ Cleanup failed:", e);
      alert("Cleanup failed: " + e);
    }
  }

  function getCategoryIcon(categoryId) {
    const icons = {
      system:
        '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-slate-400"><rect width="20" height="14" x="2" y="3" rx="2"/><line x1="8" x2="16" y1="21" y2="21"/><line x1="12" x2="12" y1="17" y2="21"/></svg>',
      build:
        '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-orange-400"><path d="M2 12h20"/><path d="M6 8h12"/><path d="M6 16h12"/></svg>',
      editor:
        '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-blue-400"><path d="M12 20h9"/><path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"/></svg>',
    };
    return icons[categoryId] || icons["system"];
  }

  function getCategoryColor(categoryId) {
    const colors = {
      system: "slate-400",
      build: "orange-400",
      editor: "blue-400",
    };
    return colors[categoryId] || "indigo-400";
  }

  function getSafetyBadge(safety) {
    if (safety === "safe") {
      return "🟢 Safe to Delete";
    } else if (safety === "check") {
      return "🟡 Review First";
    } else {
      return "🔴 Caution";
    }
  }
</script>

<div class="flex justify-between items-center mb-8">
  <h1 class="text-3xl font-bold text-white">System Junk</h1>
  <button
    on:click={cleanupSelected}
    disabled={$selectedPaths.size === 0}
    class="bg-red-600 hover:bg-red-500 text-white font-semibold py-2.5 px-5 rounded-lg transition-colors duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
  >
    Clean Selected ({$selectedPaths.size})
  </button>
</div>

<p class="text-slate-300 mb-6">
  System junk files (.DS_Store, Thumbs.db), build artifacts (*.pyc, *.class),
  and editor temporary files (*.swp, *~). These are safe to delete.
</p>

{#if $junkFiles.length === 0}
  <div class="bg-slate-800 rounded-xl shadow-lg p-8 text-center">
    <p class="text-slate-400 text-lg mb-4">
      ✨ No junk files found! Your system is clean.
    </p>
    <p class="text-sm text-slate-500">
      Run a scan from the Dashboard to detect:<br />
      • System files (.DS_Store, Thumbs.db)<br />
      • Build artifacts (*.pyc, *.class)<br />
      • Editor temp files (*.swp, *~)
    </p>
  </div>
{:else}
  <div class="space-y-4">
    {#each $junkFiles as category}
      <div class="bg-slate-800 rounded-xl shadow-lg overflow-hidden">
        <!-- Category Header -->
        <button
          on:click={() => toggleCategory(category.category_id)}
          class="w-full flex items-center justify-between p-6 hover:bg-slate-700 transition-colors duration-150"
        >
          <div class="flex items-center space-x-4">
            <div class="w-10 h-10 flex items-center justify-center">
              {@html getCategoryIcon(category.category_id)}
            </div>
            <div class="text-left">
              <h3 class="text-lg font-semibold text-white">
                {category.display_name}
              </h3>
              <p class="text-sm text-slate-400">
                {category.file_count} files • {category.total_size_kb.toFixed(
                  1,
                )} KB • {getSafetyBadge(category.safety)}
              </p>
            </div>
          </div>
          <svg
            class="w-6 h-6 text-slate-400 transform transition-transform duration-200 {expandedCategories.has(
              category.category_id,
            )
              ? 'rotate-180'
              : ''}"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </button>

        <!-- Expanded Files List -->
        {#if expandedCategories.has(category.category_id)}
          <div class="border-t border-slate-700 bg-slate-900/50">
            <div class="max-h-96 overflow-y-auto">
              {#each category.files as file}
                <div
                  class="flex items-center justify-between p-4 border-b border-slate-800 hover:bg-slate-800 transition-colors duration-150"
                >
                  <label class="flex items-center space-x-3 cursor-pointer flex-1">
                    <input
                      type="checkbox"
                      checked={$selectedPaths.has(file.path)}
                      on:change={() => toggleSelection(file.path)}
                      class="w-5 h-5 rounded border-slate-600 text-indigo-600 focus:ring-indigo-500"
                    />
                    <div class="flex-1 min-w-0">
                      <p
                        class="text-sm font-mono text-slate-300 truncate"
                        title={file.path}
                      >
                        {file.path}
                      </p>
                      <p class="text-xs text-slate-500">
                        {file.size_kb.toFixed(2)} KB
                      </p>
                    </div>
                  </label>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>
{/if}
