<script>
	import { QueryClient, QueryClientProvider } from '@sveltestack/svelte-query'
	import Todos from './components/Todos.svelte'

	const queryClient = new QueryClient()
</script>

<main>
	a

	<!-- <QueryClientProvider client={queryClient}>
		<Todos />
	</QueryClientProvider>	 -->
</main>
