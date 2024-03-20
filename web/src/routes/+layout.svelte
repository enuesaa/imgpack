<script lang="ts">
	import '../app.css'
	import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query'

	const queryClient = new QueryClient()
</script>

<QueryClientProvider client={queryClient}>
	<slot />
</QueryClientProvider>

<style lang="postcss">
	:global(html) {
		background-color: theme(colors.gray.700);
	}
</style>
