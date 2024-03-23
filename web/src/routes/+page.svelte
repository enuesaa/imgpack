<script lang="ts">
	import { page } from '$app/stores'
	import Files from './Files.svelte'
	import GoToParentDirLink from './GoToParentDirLink.svelte'

	const path = $page.url.searchParams.get('path') ?? './'
</script>

<GoToParentDirLink path={path} />
<Files path={path} />
