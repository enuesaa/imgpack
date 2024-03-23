<script lang="ts">
	import { createTreeView } from '@melt-ui/svelte'
	import TreeItem, {type Item} from './TreeItem.svelte';

	const { elements: { tree, item: itemfn, group: groupfn } } = createTreeView()
	const treeitem: Item = {
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
	};
</script>

<div class="flex h-[18.75rem] w-[18.75rem] flex-col rounded-x md:h-[350px]">
	<ul class="overflow-auto px-4 pb-4 pt-2" {...$tree}>
		<TreeItem item={treeitem} level={1} itemfn={itemfn} groupfn={groupfn} />
	</ul>
</div>
