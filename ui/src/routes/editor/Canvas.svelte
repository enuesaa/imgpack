<svelte:options runes={true} />

<script lang="ts">
	import { preventdefault } from '$lib/utils'
	import CanvasBackground from './CanvasBackground.svelte'
	import type { Overlay } from './+page.svelte'
	import CanvasDraw from './CanvasDraw.svelte'

	type Props = {
		overlays: Overlay[]
	}
	let { overlays = $bindable() }: Props = $props()

	let canvas = $state<HTMLCanvasElement>()
	let ctx = $state<CanvasRenderingContext2D>()

	function handleMouseDown(e: MouseEvent) {}
	function handleMouseMove(e: MouseEvent) {}
	function handleMouseUp(e: MouseEvent) {}

</script>

<canvas
	bind:this={canvas}
	onmousedown={preventdefault(handleMouseDown)}
	onmousemove={preventdefault(handleMouseMove)}
	onmouseup={preventdefault(handleMouseUp)}
	class="border-white border my-1"
></canvas>

<CanvasBackground bind:overlays bind:canvas bind:ctx />
<CanvasDraw bind:overlays bind:canvas bind:ctx />
