<script lang="ts">
	import { page } from '$app/stores'
	import { normalizePath } from '$lib/path'
	import Files from './Files.svelte'
	import GoToParentDirLink from './GoToParentDirLink.svelte'

	let path = normalizePath($page.url.searchParams.get('path'))
</script>

<GoToParentDirLink path={path} />
<Files path={path} />
