<script lang="ts">
	import { count } from './store'
	// import { type MouseEventHandler } from 'svelte/elements'
	// import { type PageData } from './$types'

	export let data

	const updateCount = (e) => {
		e.preventDefault()
		count.set(($count += 1))
	}
</script>

<h2>{$count}</h2>
<button on:click={updateCount}>+</button>

<div>{data.post.title}</div>

<a href="/about">about</a>
