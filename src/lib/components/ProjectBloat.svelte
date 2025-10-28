<script>
  import { invoke } from "@tauri-apps/api/core";
  import { bloatCategories, selectedPaths } from "../stores.js";

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
    selectedPaths.update((s) => {
      const next = new Set(s);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });
  }

  function isCriticalPath(path) {
    // Check for paths that might contain important source code
    return (
      path.includes("/src/") ||
      path.includes("/lib/") ||
      path.includes("/.git/") ||
      path.endsWith("/src") ||
      path.endsWith("/lib")
    );
  }

  async function cleanupSelected() {
    if ($selectedPaths.size === 0) {
      alert("No directories selected for cleanup");
      return;
    }

    // Check for critical paths
    const criticalPaths = Array.from($selectedPaths).filter(isCriticalPath);
    if (criticalPaths.length > 0) {
      const extraConfirm = confirm(
        `⚠️ CRITICAL WARNING: ${criticalPaths.length} directory(ies) may contain SOURCE CODE!\n\n` +
          `Examples:\n${criticalPaths.slice(0, 3).join("\n")}\n\n` +
          `This could delete your entire project source!\n\n` +
          `Are you ABSOLUTELY SURE?`,
      );

      if (!extraConfirm) {
        return;
      }
    }

    // Calculate total size from selected directories
    const totalSizeMB = Array.from($selectedPaths).reduce((sum, path) => {
      const category = $bloatCategories.find((cat) =>
        cat.entries.some((ex) => ex.path === path),
      );
      if (category) {
        const example = category.entries.find((ex) => ex.path === path);
        return sum + (example ? example.size_mb : 0);
      }
      return sum;
    }, 0);

    const totalSizeGB = totalSizeMB / 1024;
    const sizeDisplay = totalSizeGB < 1
      ? `${totalSizeMB.toFixed(1)} MB`
      : `${totalSizeGB.toFixed(2)} GB`;

    if (
      !confirm(
        `Delete ${$selectedPaths.size} directories?\n\n` +
        `Total size: ${sizeDisplay}\n\n` +
        `This will remove all contents. Files will be moved to trash.`,
      )
    ) {
      return;
    }

    if (
      !confirm(
        `Delete ${$selectedPaths.size} items? They will be moved to trash.`,
      )
    ) {
      return;
    }

    try {
      const paths = Array.from($selectedPaths);
      const result = await invoke("cleanup_dirs", {
        req: { paths, dry_run: false, trash: true },
      });

      let message = `✅ Deleted: ${result.deleted.length}`;
      if (result.skipped.length > 0) {
        message += `\n⚠️ Skipped: ${result.skipped.length} (already deleted)`;
      }
      if (result.errors.length > 0) {
        message += `\n❌ Errors: ${result.errors.length}`;
      }
      alert(message);

      // Prune deleted entries from categories and clear selection
      bloatCategories.update((cats) =>
        cats
          .map((c) => {
            const newEntries = c.entries.filter(
              (e) => !result.deleted.includes(e.path),
            );
            return {
              ...c,
              entries: newEntries,
              total_size_mb: newEntries.reduce((sum, e) => sum + e.size_mb, 0),
            };
          })
          .filter((c) => c.entries.length > 0),
      );
      selectedPaths.set(new Set());
    } catch (e) {
      alert("Cleanup failed: " + e);
    }
  }

  function getCategoryIcon(categoryId) {
    const icons = {
      node_modules:
        '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-green-400"><path d="M16.5 9.4 7.5 4.6"/><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.29 7 12 12 20.71 7"/><line x1="12" x2="12" y1="22" y2="12"/></svg>',
      rust_target:
        '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-orange-400"><path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/><circle cx="12" cy="13" r="2"/><path d="M12 15v-2"/><path d="m14.5 15.5-2.5-2.5"/><path d="m9.5 15.5 2.5-2.5"/></svg>',
      python_venv:
        '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-blue-400"><path d="M21 10V6.5A2.5 2.5 0 0 0 18.5 4H10A2.5 2.5 0 0 0 7.5 6.5V10"/><path d="M21 10v10.5a2.5 2.5 0 0 1-2.5 2.5H10A2.5 2.5 0 0 1 7.5 20.5V10"/><path d="M7.5 10h13.5"/><circle cx="11" cy="15" r="2"/><path d="m12.5 16.5-1.9-1.9"/></svg>',
    };
    return icons[categoryId] || icons["node_modules"];
  }

  function getCategoryColor(categoryId) {
    const colors = {
      node_modules: "green-400",
      rust_target: "orange-400",
      python_venv: "blue-400",
      git: "purple-400",
      build_artifacts: "yellow-400",
      vendor: "pink-400",
      java_gradle: "red-400",
    };
    return colors[categoryId] || "indigo-400";
  }
</script>

<div class="flex justify-between items-center mb-8">
  <h1 class="text-3xl font-bold text-white">Project Bloat</h1>
  <button
    on:click={cleanupSelected}
    disabled={$selectedPaths.size === 0}
    class="bg-red-600 hover:bg-red-500 text-white font-semibold py-2.5 px-5 rounded-lg transition-colors duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
  >
    Clean Selected ({$selectedPaths.size})
  </button>
</div>

<p class="text-slate-300 mb-6">
  Finds common build artifacts and dependencies that are safe to delete. Your
  code will not be touched.
</p>

{#if $bloatCategories.length === 0}
  <div class="bg-slate-800 rounded-xl shadow-lg p-8 text-center">
    <p class="text-slate-400">
      No bloat found. Run a scan from the Dashboard to detect project bloat.
    </p>
  </div>
{:else}
  <div class="space-y-4">
    {#each $bloatCategories as category}
      <div class="bg-slate-800 rounded-xl overflow-hidden shadow-lg">
        <button
          on:click={() => toggleCategory(category.category_id)}
          class="collapsible-trigger w-full flex justify-between items-center p-5 text-left hover:bg-slate-700/50 transition-colors"
        >
          <div class="flex items-center space-x-4">
            {@html getCategoryIcon(category.category_id)}
            <div>
              <h2 class="text-lg font-semibold text-white">
                {category.display_name}
                <span
                  class="font-mono text-sm bg-slate-700 px-2 py-0.5 rounded ml-2"
                  >{category.category_id}</span
                >
              </h2>
              <p class="text-sm text-slate-400">
                Found {category.entries.length} directories
              </p>
            </div>
          </div>
          <div class="flex items-center space-x-4">
            <span
              class="text-xl font-semibold text-{getCategoryColor(
                category.category_id,
              )}"
            >
              {category.total_size_mb.toFixed(1)} MB
            </span>
            <!-- Chevron Icon -->
            <svg
              class="chevron-icon transition-transform duration-300 {expandedCategories.has(
                category.category_id,
              )
                ? 'rotate-180'
                : ''}"
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
              <polyline points="6 9 12 15 18 9"></polyline>
            </svg>
          </div>
        </button>

        <!-- Collapsible Content -->
        {#if expandedCategories.has(category.category_id)}
          <div class="p-5 border-t border-slate-700">
            <ul class="space-y-3">
              {#each category.entries as entry}
                <li
                  class="flex justify-between items-center bg-slate-700/50 p-3 rounded-lg"
                >
                  <div
                    class="flex items-center space-x-3 overflow-hidden flex-1"
                  >
                    <input
                      type="checkbox"
                      checked={$selectedPaths.has(entry.path)}
                      on:change={() => toggleSelection(entry.path)}
                      class="h-4 w-4 rounded bg-slate-700 border-slate-600 text-indigo-600 focus:ring-indigo-500"
                    />
                    <span class="text-sm text-slate-300 truncate"
                      >{entry.path}</span
                    >
                  </div>
                  <span class="font-mono text-sm text-slate-400 ml-4"
                    >{entry.size_mb.toFixed(1)} MB</span
                  >
                </li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>
    {/each}
  </div>
{/if}
