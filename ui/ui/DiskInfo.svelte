<script>
	export let diskInfo = [];

	function formatBytes(n) {
		const u = ["B", "KB", "MB", "GB", "TB"];
		let i = 0, v = n;
		while (v >= 1024 && i < u.length - 1) {
			v /= 1024;
			i++;
		}
		return `${v.toFixed(1)} ${u[i]}`;
	}
</script>

{#if diskInfo.length > 0}
	<div class="mb-6">
		<h2 class="text-2xl font-semibold mb-4">Disk Information</h2>
		{#each diskInfo as d}
			<div class="p-4 bg-white rounded shadow mb-2">
				Mount: {d.mount_point} - Used: {formatBytes(d.used)} ({d.usage_pct.toFixed(1)}%)
			</div>
		{/each}
	</div>
{/if}