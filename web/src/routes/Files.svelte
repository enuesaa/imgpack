<script lang="ts">
	import { useListFiles } from '$lib/api'

	export let path: string
	const files = useListFiles(path)
</script>

<ul>
	{#if $files.isLoading}
		Loading...
	{:else if $files.isError}
		error: {$files.error.message}
	{:else if $files.isSuccess}
		<p>{$files.data.path}</p>
		{#each $files.data.items as item}
			<li>{item.name}</li>
		{/each}
	{/if}
</ul>
