<script>
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import './app.css';
	import DiskInfo from './ui/DiskInfo.svelte';
	import BloatList from './ui/BloatList.svelte';
	import DuplicateList from './ui/DuplicateList.svelte';

	let root = '/Users/frank/Development/private/projects';
	let diskInfo = [];
	let bloatItems = [];
	let duplicateGroups = [];
	let selectedPaths = new Set();
	let minBytes = 10 * 1024 * 1024;
	let followSymlinks = false;
	let showSettings = false;

	async function scanDisk() {
		try {
			const info = await invoke('get_disk_info');
			diskInfo = info.mounts;
		} catch (e) {
			alert('Error: ' + e);
		}
	}

	async function scanBloat() {
		try {
			const res = await invoke('scan_bloat', { opts: { root, minBytes, followSymlinks } });
			bloatItems = res;
			selectedPaths.clear();
		} catch (e) {
			alert('Error: ' + e);
		}
	}

	async function scanDuplicates() {
		try {
			const res = await invoke('scan_duplicates', { opts: { root, followSymlinks } });
			duplicateGroups = res;
			selectedPaths.clear();
		} catch (e) {
			alert('Error: ' + e);
		}
	}

	async function cleanup() {
		const paths = Array.from(selectedPaths);
		if (paths.length === 0) return alert('Select items to clean');
		try {
			const result = await invoke('cleanup_dirs', { req: { paths, dryRun: false, trash: true } });
			alert(`Deleted: ${result.deleted.length}, Skipped: ${result.skipped.length}, Errors: ${result.errors.length}`);
			// Refresh bloat items
			await scanBloat();
		} catch (e) {
			alert('Error: ' + e);
		}
	}

	function handleSelectionChange(event) {
		selectedPaths = event.detail.selectedPaths;
	}

	listen('scan_progress', (e) => {
		console.log('progress', e.payload);
	});
</script>

<main class="p-6 bg-gray-100 min-h-screen">
	<h1 class="text-3xl font-bold mb-6">Disk Bloat Scanner</h1>

	<div class="mb-4">
		<label for="rootInput" class="block text-sm font-medium mb-2">Root Directory:</label>
		<input id="rootInput" bind:value={root} class="w-full p-2 border rounded" type="text" />
	</div>

	<div class="mb-4">
		<button on:click={() => showSettings = !showSettings} class="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600">
			{showSettings ? 'Hide' : 'Show'} Settings
		</button>
	</div>

	{#if showSettings}
		<div class="mb-6 p-4 bg-white rounded shadow">
			<h3 class="text-lg font-semibold mb-4">Scan Settings</h3>
			<div class="mb-4">
				<label class="block text-sm font-medium mb-2">Minimum Size (MB):</label>
				<input bind:value={minBytes} type="number" class="p-2 border rounded" min="1" step="1" />
			</div>
			<div class="mb-4">
				<label class="flex items-center">
					<input bind:checked={followSymlinks} type="checkbox" class="mr-2" />
					Follow Symlinks
				</label>
			</div>
		</div>
	{/if}

	<div class="mb-6 space-x-2">
		<button on:click={scanDisk} class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">Get Disk Info</button>
		<button on:click={scanBloat} class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600">Scan for Bloat</button>
		<button on:click={scanDuplicates} class="px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600">Scan Duplicates</button>
		<button on:click={cleanup} class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600">Cleanup Selected</button>
	</div>

	<DiskInfo {diskInfo} />

	<BloatList {bloatItems} {selectedPaths} on:selectionChange={handleSelectionChange} />

	<DuplicateList {duplicateGroups} {selectedPaths} on:selectionChange={handleSelectionChange} />
</main>

<style>
	/* TailwindCSS will handle styles */
</style>