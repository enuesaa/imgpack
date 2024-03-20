<script>
	import Router from 'svelte-spa-router'
	import routes from './routes'

	// import { QueryClient, QueryClientProvider } from '@sveltestack/svelte-query'
	// import Todos from './components/Todos.svelte'
	// const queryClient = new QueryClient()
</script>

<main>
	<Router {routes} />
	<!--
	<QueryClientProvider client={queryClient}>
		<Todos />
	</QueryClientProvider>
	-->
</main>
