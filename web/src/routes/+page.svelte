<script lang="ts">
	import { count } from './store'
	import { listNotes } from '../api/notes'

	// import { type MouseEventHandler } from 'svelte/elements'

	const updateCount = () => {
		count.set(($count += 1))
	}
</script>

<h2>{$count}</h2>
<button on:click={updateCount}>+</button>

{#await listNotes()}
	loading..
{:then items}
	{#each items as item}
    	<p>{item.name}</p>
	{/each}
{/await}

<a href="/about">about</a>
