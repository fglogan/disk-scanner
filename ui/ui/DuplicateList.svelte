<script>
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let duplicateGroups = [];
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

{#if duplicateGroups.length > 0}
	<div>
		<h2 class="text-2xl font-semibold mb-4">Duplicate Files</h2>
		{#each duplicateGroups as group}
			<div class="mb-4 p-4 bg-yellow-100 rounded shadow">
				<p class="font-medium">Hash: {group.hash} - Size: {formatBytes(group.size)} - Files: {group.files.length}</p>
				{#each group.files as file}
					<div class="ml-4 mt-2 flex items-center">
						<input
							type="checkbox"
							checked={selectedPaths.has(file)}
							on:change={() => toggleSelect(file)}
							class="mr-2"
						/>
						<span>{file}</span>
					</div>
				{/each}
			</div>
		{/each}
	</div>
{/if}