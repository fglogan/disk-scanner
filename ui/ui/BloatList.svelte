<script>
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let bloatItems = [];
	export let selectedPaths = new Set();

	function formatBytes(n) {
		const u = ["B", "KB", "MB", "GB", "TB"];
		let i = 0, v = n;
		while (v >= 1024 && i < u.length - 1) {
			v /= 1024;
			i++;
		}
		return `${v.toFixed(1)} ${u[i]}`;
	}

	function toggleSelect(path) {
		if (selectedPaths.has(path)) {
			selectedPaths.delete(path);
		} else {
			selectedPaths.add(path);
		}
		dispatch('selectionChange', { selectedPaths });
	}
</script>

{#if bloatItems.length > 0}
	<div>
		<h2 class="text-2xl font-semibold mb-4">Bloat Found</h2>
		{#each bloatItems as item}
			<div class="p-4 bg-white rounded shadow mb-2 flex items-center">
				<input
					type="checkbox"
					checked={selectedPaths.has(item.path)}
					on:change={() => toggleSelect(item.path)}
					class="mr-4"
				/>
				<span>{item.path} - {formatBytes(item.bytes)}</span>
			</div>
		{/each}
	</div>
{/if}