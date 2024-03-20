<script lang="ts">
	import { fetchListNotes } from '$lib/notes'
	let listNotes = fetchListNotes()
</script>

{#if $listNotes.isLoading}
	<p>Loading...</p>
{:else if $listNotes.isError}
	<p>Error: {$listNotes.error.message}</p>
{:else if $listNotes.isSuccess}
	{#each $listNotes.data as item}
		<p>{item.name}</p>
	{/each}
{/if}
