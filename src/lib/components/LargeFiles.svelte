<script>
  import { invoke } from "@tauri-apps/api/core";
  import { ask } from "@tauri-apps/plugin-dialog";
  import { largeFiles, selectedPaths } from "../stores.js";

  let selectionCount = 0;

  function toggleSelection(path) {
    console.log("Toggle selection for:", path);
    selectedPaths.update((paths) => {
      const newPaths = new Set(paths);
      if (newPaths.has(path)) {
        console.log("Removing from selection");
        newPaths.delete(path);
      } else {
        console.log("Adding to selection");
        newPaths.add(path);
      }
      selectionCount = newPaths.size;
      console.log("New selection count:", selectionCount);
      return newPaths;
    });
  }

  function formatDate(timestamp) {
    if (!timestamp) return "Unknown";
    const date = new Date(timestamp * 1000);
    return date.toISOString().split("T")[0];
  }

  // Categorize file safety based on path patterns
  function getSafetyLevel(path) {
    const lower = path.toLowerCase();

    // SAFE - Build artifacts, caches, logs
    if (
      lower.includes("node_modules") ||
      lower.includes("/target/") ||
      lower.includes("/build/") ||
      lower.includes("/dist/") ||
      lower.includes("/.next/") ||
      lower.includes(".log") ||
      lower.includes("/cache/") ||
      lower.includes("/.cache/")
    ) {
      return {
        level: "safe",
        label: "✓ Safe to Delete",
        color: "text-green-400",
        bg: "bg-green-900/30",
      };
    }

    // CHECK - Media, archives, downloads
    if (lower.match(/\.(mp4|mov|avi|mkv|zip|tar|gz|dmg|iso)$/)) {
      return {
        level: "check",
        label: "⚠ Check First",
        color: "text-amber-400",
        bg: "bg-amber-900/30",
      };
    }

    // DANGER - Source code, documents, databases
    if (lower.match(/\.(rs|js|ts|py|java|cpp|go|sql|db|sqlite|json|md|txt)$/)) {
      return {
        level: "danger",
        label: "⛔ Danger Zone",
        color: "text-red-400",
        bg: "bg-red-900/30",
      };
    }

    // Default to CHECK
    return {
      level: "check",
      label: "⚠ Check First",
      color: "text-amber-400",
      bg: "bg-amber-900/30",
    };
  }

  // Extract filename and parent directory for better display
  function getDisplayPath(fullPath) {
    const parts = fullPath.split("/");
    const filename = parts[parts.length - 1];
    const parentDir = parts.length > 2 ? parts[parts.length - 2] : "";
    const projectRoot =
      parts.length > 5
        ? ".../" + parts.slice(-4, -1).join("/")
        : parts.slice(0, -1).join("/");

    return { filename, parentDir, projectRoot, fullPath };
  }

  function isCriticalPath(path) {
    // Check if path contains source code or important directories
    return (
      path.includes("/src/") ||
      path.includes("/lib/") ||
      path.includes("/.git/") ||
      /\.(rs|js|ts|py|go|cpp|java|rb|php|swift|kt|h|c|hpp)$/.test(path)
    );
  }

  async function deleteSelected() {
    console.log(
      "deleteSelected() called, selected count:",
      $selectedPaths.size,
    );
    console.log("Selected paths:", Array.from($selectedPaths));

    if ($selectedPaths.size === 0) {
      await ask("No files selected", {
        title: "Delete Files",
        kind: "warning",
      });
      return;
    }

    // Check for critical paths
    const criticalPaths = Array.from($selectedPaths).filter(isCriticalPath);
    if (criticalPaths.length > 0) {
      const criticalConfirm = await ask(
        `⚠️ DANGER: ${criticalPaths.length} SOURCE CODE FILE(S) SELECTED!\n\n` +
          `These appear to be source code or critical system files:\n` +
          `${criticalPaths.slice(0, 3).join("\n")}\n\n` +
          `Deleting these could BREAK YOUR PROJECTS!\n\n` +
          `Are you ABSOLUTELY CERTAIN?`,
        {
          title: "⚠️ CRITICAL FILE WARNING",
          kind: "error",
        },
      );

      if (!criticalConfirm) {
        console.log("User cancelled critical file deletion");
        return;
      }
    }

    const count = $selectedPaths.size;
    const totalSizeMB = Array.from($selectedPaths).reduce((sum, path) => {
      const file = $largeFiles.find((f) => f.path === path);
      return sum + (file ? file.size_mb : 0);
    }, 0);

    console.log("Showing confirmation for", count, "files");
    const confirmed = await ask(
      `Move ${count} file(s) (${(totalSizeMB / 1024).toFixed(2)} GB) to trash?\n\nThis action can be undone by restoring from Trash.`,
      {
        title: "Confirm Delete",
        kind: "warning",
      },
    );

    if (!confirmed) {
      console.log("User cancelled");
      return;
    }

    try {
      const paths = Array.from($selectedPaths);
      console.log("Deleting paths:", paths);

      const result = await invoke("cleanup_dirs", {
        req: { paths, dry_run: false, trash: true },
      });

      console.log("Delete result:", result);

      // Remove deleted files from the list
      largeFiles.update((files) =>
        files.filter((f) => !result.deleted.includes(f.path)),
      );

      // Clear selection
      selectedPaths.set(new Set());
      selectionCount = 0;

      if (result.errors.length > 0) {
        alert(
          `Completed with errors:\nDeleted: ${result.deleted.length}\nErrors: ${result.errors.length}\n\n${result.errors.join("\n")}`,
        );
      } else {
        alert(`Successfully moved ${result.deleted.length} file(s) to trash`);
      }
    } catch (e) {
      console.error("Delete failed:", e);
      alert("Delete failed: " + e);
    }
  }
</script>

<div class="flex justify-between items-center mb-8">
  <h1 class="text-3xl font-bold text-white">Large Files</h1>
  <button
    on:click={deleteSelected}
    disabled={selectionCount === 0}
    class="bg-red-600 hover:bg-red-500 text-white font-semibold py-2.5 px-5 rounded-lg transition-colors duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
  >
    Delete Selected ({selectionCount})
  </button>
</div>

{#if $largeFiles.length === 0}
  <div class="bg-slate-800 rounded-xl shadow-lg p-8 text-center">
    <p class="text-slate-400">
      No large files found. Run a scan from the Dashboard.
    </p>
  </div>
{:else}
  <div class="bg-slate-800 rounded-xl shadow-lg overflow-hidden">
    <table class="w-full text-left">
      <thead class="border-b border-slate-700">
        <tr>
          <th class="p-4 w-12"></th>
          <th class="p-4 text-sm font-semibold text-slate-300">File</th>
          <th class="p-4 text-sm font-semibold text-slate-300 w-40">Safety</th>
          <th class="p-4 text-sm font-semibold text-slate-300 w-32 text-right"
            >Size</th
          >
          <th class="p-4 text-sm font-semibold text-slate-300 w-32 text-right"
            >Modified</th
          >
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-700">
        {#each $largeFiles as file}
          {@const pathInfo = getDisplayPath(file.path)}
          {@const safety = getSafetyLevel(file.path)}
          <tr class="hover:bg-slate-700/50 {safety.bg}">
            <td class="p-4">
              <input
                type="checkbox"
                checked={$selectedPaths.has(file.path)}
                on:change={() => toggleSelection(file.path)}
                class="h-5 w-5 rounded bg-slate-700 border-slate-600 text-indigo-600 focus:ring-indigo-500"
              />
            </td>
            <td class="p-4">
              <div class="flex flex-col">
                <span class="text-sm font-semibold text-white"
                  >{pathInfo.filename}</span
                >
                <span
                  class="text-xs text-slate-400 font-mono truncate"
                  title={file.path}
                >
                  {pathInfo.projectRoot}
                </span>
              </div>
            </td>
            <td class="p-4">
              <span class="text-sm font-semibold {safety.color}">
                {safety.label}
              </span>
            </td>
            <td
              class="p-4 text-sm text-slate-200 font-mono text-right font-bold"
            >
              {file.size_mb >= 1024
                ? (file.size_mb / 1024).toFixed(1) + " GB"
                : file.size_mb.toFixed(1) + " MB"}
            </td>
            <td class="p-4 text-sm text-slate-400 text-right">
              {formatDate(file.last_modified)}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
{/if}
