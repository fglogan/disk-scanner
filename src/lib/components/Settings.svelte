<script>
  import { settings } from "../stores.js";
  import { open } from "@tauri-apps/plugin-dialog";

  let bgMonitorEnabled = $settings.bg_monitor_enabled;
  let scanInterval = $settings.scan_interval;
  let minDupSize = $settings.min_dup_size;

  function toggleBgMonitor() {
    bgMonitorEnabled = !bgMonitorEnabled;
    settings.update((s) => ({ ...s, bg_monitor_enabled: bgMonitorEnabled }));
  }

  async function addDirectory() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Directory to Scan",
      });

      if (selected) {
        const newDir = typeof selected === "string" ? selected : selected.path;
        console.log("Selected directory:", newDir);

        settings.update((s) => {
          if (!s.directories.includes(newDir)) {
            return {
              ...s,
              directories: [...s.directories, newDir],
            };
          }
          return s;
        });
      }
    } catch (error) {
      console.error("Failed to select directory:", error);
    }
  }

  function removeDirectory(dir) {
    settings.update((s) => ({
      ...s,
      directories: s.directories.filter((d) => d !== dir),
    }));
  }
</script>

<h1 class="text-3xl font-bold text-white mb-8">Settings</h1>

<div class="max-w-2xl space-y-8">
  <!-- Background Scanning -->
  <div class="bg-slate-800 rounded-xl shadow-lg p-6">
    <h2 class="text-lg font-semibold text-white mb-4">Background Scanning</h2>
    <div class="flex items-center justify-between mb-4">
      <label for="bg-monitor" class="text-slate-300"
        >Enable background monitor</label
      >
      <button
        id="bg-monitor"
        role="switch"
        aria-checked={bgMonitorEnabled}
        on:click={toggleBgMonitor}
        class="{bgMonitorEnabled
          ? 'bg-indigo-600'
          : 'bg-slate-700'} relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-800"
      >
        <span
          class="{bgMonitorEnabled
            ? 'translate-x-5'
            : 'translate-x-0'} pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"
        ></span>
      </button>
    </div>
    <div class="flex items-center justify-between">
      <label for="scan-interval" class="text-slate-300">Scan interval</label>
      <select
        id="scan-interval"
        bind:value={scanInterval}
        on:change={() =>
          settings.update((s) => ({ ...s, scan_interval: scanInterval }))}
        class="bg-slate-700 border border-slate-600 text-white rounded-md p-2 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
      >
        <option value="6h">Every 6 hours</option>
        <option value="12h">Every 12 hours</option>
        <option value="24h">Once a day</option>
        <option value="7d">Once a week</option>
      </select>
    </div>
  </div>

  <!-- Scan Configuration -->
  <div class="bg-slate-800 rounded-xl shadow-lg p-6">
    <h2 class="text-lg font-semibold text-white mb-4">Scan Configuration</h2>
    <div class="flex items-center justify-between">
      <label for="min-dup-size" class="text-slate-300"
        >Minimum size for duplicates</label
      >
      <div class="flex items-center space-x-2">
        <input
          type="number"
          id="min-dup-size"
          bind:value={minDupSize}
          on:change={() =>
            settings.update((s) => ({ ...s, min_dup_size: minDupSize }))}
          class="w-20 bg-slate-700 border border-slate-600 text-white text-lg rounded-md p-2 text-right focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
          min="1"
          step="1"
        />
        <span class="text-slate-400 text-lg">MB</span>
      </div>
    </div>
  </div>

  <!-- Monitored Directories -->
  <div class="bg-slate-800 rounded-xl shadow-lg p-6">
    <h2 class="text-lg font-semibold text-white mb-4">Monitored Directories</h2>
    <ul class="space-y-3 mb-4">
      {#each $settings.directories as dir}
        <li
          class="flex justify-between items-center bg-slate-700/50 p-3 rounded-lg"
        >
          <span class="font-mono text-sm text-slate-300">{dir}</span>
          <button
            on:click={() => removeDirectory(dir)}
            class="text-slate-400 hover:text-red-400 text-xl font-bold transition-colors"
            title="Remove directory">&times;</button
          >
        </li>
      {/each}
    </ul>
    <button
      on:click={addDirectory}
      class="w-full bg-slate-700 hover:bg-slate-600 text-white font-semibold py-2.5 px-4 rounded-lg transition-colors duration-150"
    >
      Add Directory
    </button>
  </div>

  <!-- Ignore Patterns -->
  <div class="bg-slate-800 rounded-xl shadow-lg p-6">
    <h2 class="text-lg font-semibold text-white mb-4">Ignore Patterns</h2>
    <p class="text-sm text-slate-400 mb-3">
      Uses glob patterns. One pattern per line.
    </p>
    <textarea
      value={$settings.ignore_patterns.join("\n")}
      class="w-full h-32 bg-slate-700 border border-slate-600 text-white font-mono text-sm rounded-lg p-3 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none"
      placeholder="*.tmp&#10;**/.cache/&#10;*.log"
    ></textarea>
  </div>
</div>
