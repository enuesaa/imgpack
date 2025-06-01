<svelte:options runes={true} />

<script lang="ts">
	type Props = {
		addOverlay: (file: File) => void
	}
	let { addOverlay }: Props = $props()

	let inputfile = $state<HTMLInputElement>()

	function handleFileUpload(e: Event & {currentTarget: HTMLInputElement}) {
		if (e.currentTarget === undefined || e.currentTarget.files === null) {
			return
		}
		for (const file of e.currentTarget.files) {
			addOverlay(file)
		}
	}

	function handleClick(e: Event) {
		e.preventDefault()
		if (inputfile === undefined) {
			return
		}
		inputfile.click()
	}
</script>

<input
	bind:this={inputfile}
	type="file"
	accept="image/*"
	onchange={handleFileUpload}
	class="hidden"
/>
<button onclick={handleClick}>選択</button>
