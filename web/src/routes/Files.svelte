<script lang="ts">
	import { listFiles } from '$lib/api'
	import File from './File.svelte'

	export let path: string
	const files = listFiles(path)
</script>

<ul>
	{#if $files.isLoading}
		Loading...
	{:else if $files.isError}
		error: {$files.error.message}
	{:else if $files.isSuccess}
		{#each $files.data.items as item}
			<li><File item={item} /></li>
		{/each}
	{/if}
</ul>
