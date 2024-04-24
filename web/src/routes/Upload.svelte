<script lang="ts">
	import { addImage } from '$lib/images'
	import { useUpload } from '$lib/upload'

	const upload = useUpload()

	async function handleUpload(e: Event & { currentTarget: HTMLInputElement }) {
		const files = e.currentTarget.files
		if (files === null) {
			return
		}
		const file = files[0]
		const formdata = new FormData()
		formdata.append('file', file)
		const compressedUrl = await $upload.mutateAsync(formdata)
		addImage({ compressedUrl, filename: file.name })
	}

	async function handleDrop(e: DragEvent) {
		const files = e?.dataTransfer?.files ?? null
		if (files === null) {
			return
		}
		const file = files[0]
		const formdata = new FormData()
		formdata.append('file', file)
		const compressedUrl = await $upload.mutateAsync(formdata)
		addImage({ compressedUrl, filename: file.name })
	}
</script>

<section
	class="w-full h-10 bg-white"
	on:drop|preventDefault={handleDrop}
	on:dragover|preventDefault
	on:dragleave|preventDefault
>
	drop here
	<input multiple type="file" on:change|preventDefault|stopPropagation={handleUpload} />
</section>
