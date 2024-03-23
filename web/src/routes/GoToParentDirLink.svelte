<script lang="ts">
	import { calcParentPath } from '$lib/path'
	import { ArrowUpIcon } from 'svelte-feather-icons'

	export let path: string

	const parentPath = calcParentPath(path)
</script>

<div class="my-5 relative h-5">
	<a href={`/?path=` + parentPath} class="absolute left-10 h-5">
		<ArrowUpIcon />
	</a>
</div>
