<script lang="ts" context="module">
	export type Item = {
		title: string;
		children: Item[];
	};
</script>

<script lang="ts">
	import { melt } from '@melt-ui/svelte'

	export let itemfn;
	export let groupfn;
	export let item: Item;
	export let level = 1;
</script>

<li class={level !== 1 ? 'pl-4' : ''}>
	<button
		class="flex items-center gap-1 rounded-md p-1 focus:bg-magnum-200"
		use:melt={$itemfn({
			id: item.title,
			hasChildren: item.children.length > 0,
		})}
	>
		<span>{item.title}</span>
	</button>

	{#if item.children.length > 0}
		<ul use:melt={$groupfn({ id: item.title })}>
			{#each item.children as child, i}
				<svelte:self item={child} level={level + 1} itemfn={itemfn} groupfn={groupfn} />
			{/each}
		</ul>
	{/if}
</li>
