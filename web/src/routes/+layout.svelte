<script lang="ts">
	import '../app.css'
	import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query'
	import Container from './Container.svelte'

	const queryClient = new QueryClient()
</script>

<QueryClientProvider client={queryClient}>
	<Container>
		<slot />
	</Container>
</QueryClientProvider>

<style lang="postcss">
	:global(html) {
		background-color: theme(colors.gray.900);
		color: theme(colors.gray.300);
	}
</style>
