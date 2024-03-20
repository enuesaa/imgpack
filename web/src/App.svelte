<script>
  import { Router, createRouter } from '@roxi/routify'
  // @ts-ignore
  import routes from '../.routify/routes.default.js'

  // import { QueryClient, QueryClientProvider } from '@sveltestack/svelte-query'
  // import Todos from './components/Todos.svelte'
  // const queryClient = new QueryClient()
</script>

<Router {routes} />
<!--
<QueryClientProvider client={queryClient}>
	<Todos />
</QueryClientProvider>
-->
