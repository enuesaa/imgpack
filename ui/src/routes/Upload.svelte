<script lang="ts">
	import { useUpload } from '$lib/upload'

	const upload = useUpload()
	let inputfile: HTMLInputElement

	async function handleUpload(e: Event & { currentTarget: HTMLInputElement }) {
		const files = e.currentTarget.files
		if (files === null) {
			return
		}
		await $upload.mutateAsync(files)
	}
	async function handleDrop(e: DragEvent) {
		const files = e?.dataTransfer?.files ?? null
		if (files === null) {
			return
		}
		await $upload.mutateAsync(files)
	}
</script>

<section
	class="w-full rounded-md text-center p-20 cursor-pointer"
	style="border: solid 1px #fafafa;"
	on:drop|preventDefault={handleDrop}
	on:dragover|preventDefault
	on:dragleave|preventDefault
	on:click={() => inputfile.click()}
>
	drop here
	<input
		bind:this={inputfile}
		type="file"
		multiple
		class="opacity-0 block"
		on:change|preventDefault|stopPropagation={handleUpload}
	/>
</section>
