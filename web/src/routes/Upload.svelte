<script lang="ts">
	import { useUpload } from '$lib/upload'

	const upload = useUpload()
	let compressedUrl: string|null = null;

	async function handleUpload(e: Event & { currentTarget: HTMLInputElement }) {
		const files = e.currentTarget.files
		if (files === null) {
			return
		}
		const formdata = new FormData()
		formdata.append('file', files[0])
		compressedUrl = await $upload.mutateAsync(formdata)	
	}
</script>

<input multiple type="file" on:change|preventDefault={handleUpload} />

{#if compressedUrl !== null}
	<img src={compressedUrl} />
	<a href={compressedUrl} download="compressed.png">download</a>
{/if}
