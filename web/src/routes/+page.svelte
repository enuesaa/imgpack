<script lang="ts">
	import { count } from './store'
	import { listNotes } from '../api/notes'

	// import { type MouseEventHandler } from 'svelte/elements'

	const updateCount = () => {
		count.set(($count += 1))
	}
</script>

<h1 class="text-3xl font-bold underline">
	Hello world!
</h1>
  
<style lang="postcss">
	:global(html) {
		background-color: theme(colors.gray.700);
	}
</style>

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
