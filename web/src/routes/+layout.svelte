<script lang="ts">
	import { setContext } from 'svelte'
	import { writable } from 'svelte/store'

	const user = writable()

	$: user.set({ name: 'aaa' })

	setContext('user', user)
</script>

<slot />
