<svelte:options runes={true} />

<script lang="ts">
	let canvas = $state<HTMLCanvasElement>()
	let ctx = $state<CanvasRenderingContext2D>()
	let ocanvas = $state<HTMLCanvasElement>()
	let octx = $state<CanvasRenderingContext2D>()

	let img = new Image()
	img.src = 'example.png'
	img.onload = () => {
		if (canvas === undefined || ocanvas === undefined) {
			return
		}
		ctx = canvas.getContext('2d') ?? undefined
		octx = ocanvas.getContext('2d') ?? undefined

		canvas.width = img.width
		canvas.height = img.height
		if (ctx === undefined) {
			return
		}
		ctx.drawImage(img, 0, 0)
	}

	let isDragging = $state(false)
	let startX = $state(0)
	let startY = $state(0)
	let currentX = $state(0)
	let currentY = $state(0)

	function handleMouseDown(event: MouseEvent) {
		if (canvas === undefined || ocanvas === undefined) {
			return
		}
		const rect = canvas.getBoundingClientRect()
		startX = event.clientX - rect.left
		startY = event.clientY - rect.top
		isDragging = true
	}

	function handleMouseMove(event: MouseEvent) {
		if (canvas === undefined || ocanvas === undefined || !isDragging) {
			return
		}
		const rect = canvas.getBoundingClientRect()
		currentX = event.clientX - rect.left
		currentY = event.clientY - rect.top

		if (ctx === undefined) {
			return
		}
		ctx.clearRect(0, 0, canvas.width, canvas.height)
		ctx.drawImage(img, 0, 0)
		ctx.strokeStyle = 'red'
		ctx.lineWidth = 2
		ctx.strokeRect(startX, startY, currentX - startX, currentY - startY)
	}

	function handleMouseUp() {
		if (canvas === undefined || ocanvas === undefined) {
			return
		}
		isDragging = false

		const x = Math.min(startX, currentX)
		const y = Math.min(startY, currentY)
		const width = Math.abs(currentX - startX)
		const height = Math.abs(currentY - startY)

		ocanvas.width = width
		ocanvas.height = height

		if (octx === undefined) {
			return
		}
		octx.drawImage(img, x, y, width, height, 0, 0, width, height)
	}
</script>

<canvas
	bind:this={canvas}
	onmousedown={handleMouseDown}
	onmousemove={handleMouseMove}
	onmouseup={handleMouseUp}
	class="border-white border my-1"
></canvas>
<canvas bind:this={ocanvas} class="border-white border my-1"></canvas>
