<script lang="ts">
	export let title: string

	type User = {
		name: string
	}

	import { getContext } from 'svelte'
	const user = getContext<User>('user')
</script>

<h1>{title}</h1>
<p>Welcome {$user.name}</p>
