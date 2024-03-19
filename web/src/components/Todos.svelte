<script lang="ts">
	type Data = {
		title: string
	}

	// import { useQuery, useQueryClient } from '@sveltestack/svelte-query'
	// const queryClient = useQueryClient()

	// const datalist = useQuery<Data[]>('todos', async () => {
	// 	return [{ title: 'a' }, { title: 'b' }]
	// })
	const datalist: Data[] = [{ title: 'a' }, { title: 'b' }]
</script>

<div>
	<ul>
		b
		{#each datalist as todo}
			<li>{todo.title}</li>
		{/each}
	</ul>
</div>
