<script lang="ts">
	import { onMount } from 'svelte'

	let canvas: HTMLCanvasElement
	let ctx: CanvasRenderingContext2D|null
	let outputCanvas: HTMLCanvasElement
	let outputCtx: CanvasRenderingContext2D|null

	let img = new Image()
	img.src = 'example.png'

	let isDragging = false
	let startX = 0
	let startY = 0
	let currentX = 0
	let currentY = 0

	onMount(() => {
		img.onload = () => {
			ctx = canvas.getContext('2d')
			outputCtx = outputCanvas.getContext('2d')
			canvas.width = img.width
			canvas.height = img.height

			if (ctx === null) {
				return
			}
			ctx.drawImage(img, 0, 0)
		}
	})

	function startSelection(event: MouseEvent) {
		const rect = canvas.getBoundingClientRect()
		startX = event.clientX - rect.left
		startY = event.clientY - rect.top
		isDragging = true
	}

	function updateSelection(event: MouseEvent) {
		if (!isDragging) {
			return
		}
		const rect = canvas.getBoundingClientRect()
		currentX = event.clientX - rect.left
		currentY = event.clientY - rect.top

		if (ctx === null) {
			return
		}
		ctx.clearRect(0, 0, canvas.width, canvas.height)
		ctx.drawImage(img, 0, 0)
		ctx.strokeStyle = 'red'
		ctx.lineWidth = 2
		ctx.strokeRect(startX, startY, currentX - startX, currentY - startY)
	}

	function finishSelection() {
		isDragging = false

		const x = Math.min(startX, currentX)
		const y = Math.min(startY, currentY)
		const width = Math.abs(currentX - startX)
		const height = Math.abs(currentY - startY)

		outputCanvas.width = width
		outputCanvas.height = height

		if (outputCtx === null) {
			return
		}
		outputCtx.drawImage(img, x, y, width, height, 0, 0, width, height)
	}
</script>

<canvas bind:this={canvas} on:mousedown={startSelection} on:mousemove={updateSelection} on:mouseup={finishSelection} />
<canvas bind:this={outputCanvas} />

<style>
	canvas {
		border: 1px solid #ccc;
		margin: 10px;
	}
</style>
