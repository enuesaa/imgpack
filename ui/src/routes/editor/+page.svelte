<svelte:options runes={true} />

<script lang="ts" module>
	export type Overlay = {
		img: HTMLImageElement
		x: number
		y: number
		width: number
		height: number
	}
</script>

<script lang="ts">
	import ImageSelector from './ImageSelector.svelte'
	import Canvas from './Canvas.svelte'

	let overlays = $state<Overlay[]>([])
</script>

<ImageSelector bind:overlays />
<Canvas bind:overlays />
