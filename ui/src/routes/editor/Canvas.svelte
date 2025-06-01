<svelte:options runes={true} />

<script lang="ts">
	import { preventdefault } from '$lib/utils'
	import CanvasBackground from './CanvasBackground.svelte'

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

<CanvasBackground {canvas} {ctx} />
