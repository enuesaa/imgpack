<script lang="ts">
	import { melt, createTreeView } from '@melt-ui/svelte'

	const { elements: { tree, item, group } } = createTreeView({
		defaultExpanded: ['lib-0', 'tree-0'],
	})
	let level = 1;
	type TreeItem = {
		title: string;
		children: TreeItem[];
	};
	const treeItems: TreeItem[] = [
		{
			title: 'lib',
			children: [
				{
					title: 'tree',
					children: [
						{title: 'Tree.svelte',children: []},
					],
				},
				{title: 'index.js',children: []},
			],
		},
		{ title: 'routes',children: [] },
	];
</script>

<div class="flex h-[18.75rem] w-[18.75rem] flex-col rounded-x md:h-[350px]">
	<ul class="overflow-auto px-4 pb-4 pt-2" {...$tree}>

		{#each treeItems as { title, children }, i}
		<li class={level !== 1 ? 'pl-4' : ''}>
			<button
				class="flex items-center gap-1 rounded-md p-1 focus:bg-magnum-200"
				use:melt={$item({
					id: `${title}-${i}`,
					hasChildren: !!children.length,
				})}
			>
				<span>{title}</span>
			</button>

			{#if children}
				<ul use:melt={$group({ id: `${title}-${i}` })}>
					<!-- <svelte:self treeItems={children} level={level + 1} /> -->
				</ul>
			{/if}
		</li>
		{/each}

	</ul>
</div>
