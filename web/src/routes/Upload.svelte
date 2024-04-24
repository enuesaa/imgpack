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
		const filename = file.name
		const formdata = new FormData()
		formdata.append('file', file)
		const compressedUrl = await $upload.mutateAsync(formdata)
		addImage({ compressedUrl, filename })
	}
</script>

<input multiple type="file" on:change|preventDefault={handleUpload} />
