<script lang="ts">
	import { melt, createTreeView } from '@melt-ui/svelte'
	import type { ListFilesSchemaItem } from '$lib/api'

	const { elements: { tree, item, group } } = createTreeView()
	const files: ListFilesSchemaItem[] = [{
		name: 'lib',
		isCompressable: false,
		isDir: true,
	}]
</script>

<ul class="overflow-auto px-4 pb-4 pt-2" {...$tree}>
	{#each files as file}
		<li
			class="flex items-center gap-1 p-1"
			use:melt={$item({
				id: file.name,
				hasChildren: file.isDir,
			})}
		>
			<span>{file.name}</span>
		</li>
	{/each}
</ul>
