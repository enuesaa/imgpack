<svelte:options runes={true} />

<script lang="ts">
	import { preventdefault } from '$lib/utils'
	import type { Overlay } from './+page.svelte'

	type Props = {
		overlays: Overlay[]
	}
	let { overlays = $bindable() }: Props = $props()

	let input = $state<HTMLInputElement>()

	function handleChange(e: Event & {currentTarget: HTMLInputElement}) {
		if (e.currentTarget === undefined || e.currentTarget.files === null) {
			return
		}
		for (const file of e.currentTarget.files) {
			readfile(file)
		}
	}

	function readfile(file: File) {
		const reader = new FileReader()
		reader.onload = (e) => {
			const img = new Image()
			img.onload = () => {
				overlays.push({
					img,
					x: 50,
					y: 50,
					width: img.width / 2,
					height: img.height / 2
				})
			}
			if (e.target === null || typeof e.target.result !== 'string') {
				return
			}
			img.src = e.target.result
		}
		reader.readAsDataURL(file)
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
