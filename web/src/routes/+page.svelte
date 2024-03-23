<script lang="ts">
	import { page } from '$app/stores'
	import Files from './Files.svelte'

	const path = $page.url.searchParams.get('path') ?? '/'
</script>

<Files path={path} />
