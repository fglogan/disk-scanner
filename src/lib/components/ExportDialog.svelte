<script>
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import Button from './ui/Button.svelte';
  
  export let show = false;
  export let data = null;
  export let dataType = '';
  
  let selectedFormat = 'json';
  let isExporting = false;
  let exportMessage = '';
  
  const formatOptions = [
    { value: 'json', label: 'JSON (Structured Data)' },
    { value: 'csv', label: 'CSV (Spreadsheet Compatible)' }
  ];
  
  async function handleExport() {
    if (!data || !dataType) return;
    
    isExporting = true;
    exportMessage = '';
    
    try {
      // Open save dialog
      const filePath = await save({
        filters: [{
          name: selectedFormat === 'json' ? 'JSON Files' : 'CSV Files',
          extensions: [selectedFormat]
        }],
        defaultPath: `disk-scan-${dataType}-${new Date().toISOString().split('T')[0]}.${selectedFormat}`
      });
      
      if (!filePath) {
        isExporting = false;
        return;
      }
      
      // Prepare export request
      const exportRequest = {
        format: selectedFormat,
        data_type: {
          [dataType]: data
        },
        output_path: filePath
      };
      
      // Export data
      const result = await invoke('export_scan_results', { request: exportRequest });
      exportMessage = result;
      
      // Auto-close after successful export
      setTimeout(() => {
        show = false;
        exportMessage = '';
      }, 2000);
      
    } catch (error) {
      console.error('Export failed:', error);
      exportMessage = `Export failed: ${error}`;
    } finally {
      isExporting = false;
    }
  }
  
  function handleClose() {
    show = false;
    exportMessage = '';
  }
</script>

{#if show}
<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" on:click={handleClose}>
  <div class="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4" on:click|stopPropagation>
    <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-white">Export Scan Results</h2>
    
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Export Format
        </label>
        <select
          bind:value={selectedFormat}
          class="w-full px-3 py-2 border rounded-lg bg-white dark:bg-gray-700 border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
          disabled={isExporting}
        >
          {#each formatOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
      
      <div class="text-sm text-gray-600 dark:text-gray-400">
        <p class="mb-2">Data to export: <strong>{dataType}</strong></p>
        {#if selectedFormat === 'json'}
          <p>JSON format preserves full data structure and is ideal for programmatic processing.</p>
        {:else}
          <p>CSV format is compatible with Excel and other spreadsheet applications.</p>
        {/if}
      </div>
      
      {#if exportMessage}
        <div class={`p-3 rounded text-sm ${exportMessage.includes('failed') ? 'bg-red-100 text-red-700 dark:bg-red-900/20 dark:text-red-400' : 'bg-green-100 text-green-700 dark:bg-green-900/20 dark:text-green-400'}`}>
          {exportMessage}
        </div>
      {/if}
    </div>
    
    <div class="flex gap-3 mt-6">
      <Button
        on:click={handleExport}
        disabled={isExporting || !data}
        class="flex-1"
      >
        {isExporting ? 'Exporting...' : 'Export'}
      </Button>
      <Button
        on:click={handleClose}
        variant="secondary"
        disabled={isExporting}
        class="flex-1"
      >
        Cancel
      </Button>
    </div>
  </div>
</div>
{/if}

<style>
  select:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>