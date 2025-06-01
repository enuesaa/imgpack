<svelte:options runes={true} />

<script lang="ts">
	import { preventdefault } from '$lib/utils'

	type Props = {
		addOverlay: (file: File) => void
	}
	let { addOverlay }: Props = $props()

	let input = $state<HTMLInputElement>()

	function handleChange(e: Event & {currentTarget: HTMLInputElement}) {
		if (e.currentTarget === undefined || e.currentTarget.files === null) {
			return
		}
		for (const file of e.currentTarget.files) {
			addOverlay(file)
		}
	}

	function handleClick() {
		if (input === undefined) {
			return
		}
		input.click()
	}
</script>

<input
	bind:this={input}
	type="file"
	accept="image/*"
	onchange={preventdefault(handleChange)}
	class="hidden"
/>
<button onclick={preventdefault(handleClick)}>選択</button>
