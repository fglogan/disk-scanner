<script>
  import { duplicates, selectedPaths } from "../stores.js";

  let expandedSets = new Set();

  function toggleSet(hash) {
    if (expandedSets.has(hash)) {
      expandedSets.delete(hash);
    } else {
      expandedSets.add(hash);
    }
    expandedSets = expandedSets;
  }

  function toggleSelection(path) {
    if ($selectedPaths.has(path)) {
      $selectedPaths.delete(path);
    } else {
      $selectedPaths.add(path);
    }
    selectedPaths.set($selectedPaths);
  }

  function formatDate(timestamp) {
    if (!timestamp) return "Unknown";
    const date = new Date(timestamp * 1000);
    return date.toISOString().split("T")[0];
  }
</script>

<div class="flex justify-between items-center mb-8">
  <h1 class="text-3xl font-bold text-white">Duplicates</h1>
  <button
    class="bg-red-600 hover:bg-red-500 text-white font-semibold py-2.5 px-5 rounded-lg transition-colors duration-150"
  >
    Delete All Selected
  </button>
</div>

{#if $duplicates.length === 0}
  <div class="bg-slate-800 rounded-xl shadow-lg p-8 text-center">
    <p class="text-slate-400">
      No duplicates found. Run a scan from the Dashboard.
    </p>
  </div>
{:else}
  <p class="text-slate-300 mb-6">
    Found {$duplicates.length} sets of duplicate files. Select one file from each
    group to keep.
  </p>

  <div class="space-y-4">
    {#each $duplicates as dupSet}
      <div class="bg-slate-800 rounded-xl overflow-hidden shadow-lg">
        <button
          on:click={() => toggleSet(dupSet.hash)}
          class="w-full flex justify-between items-center p-5 text-left hover:bg-slate-700/50 transition-colors"
        >
          <div class="flex items-center space-x-4 overflow-hidden">
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
              class="text-slate-400 flex-shrink-0"
            >
              <path
                d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"
              /><polyline points="14 2 14 8 20 8" />
            </svg>
            <div class="overflow-hidden">
              <h2 class="text-lg font-medium text-white truncate">
                {dupSet.entries[0]?.path.split("/").pop() || "Unknown file"}
              </h2>
              <p class="text-sm text-slate-400">
                {dupSet.entries.length} duplicates found
              </p>
            </div>
          </div>
          <div class="flex items-center space-x-4 flex-shrink-0">
            <span class="text-lg font-semibold text-amber-400">
              Savable: {dupSet.total_savable_mb >= 1024
                ? (dupSet.total_savable_mb / 1024).toFixed(1) + " GB"
                : dupSet.total_savable_mb.toFixed(1) + " MB"}
            </span>
            <svg
              class="transition-transform duration-300 {expandedSets.has(
                dupSet.hash,
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

        {#if expandedSets.has(dupSet.hash)}
          <div class="p-5 border-t border-slate-700">
            <div class="space-y-3">
              {#each dupSet.entries as entry, idx}
                <label
                  class="flex justify-between items-center bg-slate-700/50 p-3 rounded-lg"
                >
                  <div class="flex items-center flex-grow overflow-hidden">
                    <input
                      type="checkbox"
                      checked={$selectedPaths.has(entry.path)}
                      on:change={() => toggleSelection(entry.path)}
                      class="h-4 w-4 rounded bg-slate-700 border-slate-600 text-indigo-600 focus:ring-indigo-500 mr-4 flex-shrink-0"
                    />
                    <div class="overflow-hidden">
                      <span class="text-sm text-slate-300 truncate block"
                        >{entry.path}</span
                      >
                    </div>
                  </div>
                  <span
                    class="w-24 text-right font-mono text-sm text-slate-400 flex-shrink-0 ml-4"
                  >
                    {entry.size_mb >= 1024
                      ? (entry.size_mb / 1024).toFixed(1) + " GB"
                      : entry.size_mb.toFixed(1) + " MB"}
                  </span>
                  <span
                    class="w-32 text-right text-sm text-slate-400 flex-shrink-0 ml-4"
                    >{formatDate(entry.last_modified)}</span
                  >
                </label>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>
{/if}
