<script lang="ts">
	import type { FileItem } from '$lib/api'
	import { ArrowRightIcon } from 'svelte-feather-icons'

	export let item: FileItem
</script>

<div>
	{item.name}

	{#if item.isDir}
		<a href={`/?path=${item.name}`}><ArrowRightIcon /></a>
	{/if}

	{#if item.isCompressable}
		<button>compress</button>
	{/if}
</div>
