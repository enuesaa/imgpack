<script lang="ts">
	import Header from './Header.svelte'
	import Tree from './Tree.svelte'
	import { page } from '$app/stores'

	const path = $page.url.searchParams.get('path') ?? '/'
</script>

<div>{$page.url}</div>
<div>{path}</div>
<Header />
<Tree />
