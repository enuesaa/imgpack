<svelte:options runes={true} />

<script lang="ts">
	let canvas = $state<HTMLCanvasElement>()
	let ctx = $state<CanvasRenderingContext2D>()

	let img = new Image()
	img.src = 'example.png'
	img.onload = () => {
		if (canvas === undefined) {
			return
		}
		canvas.width = img.width
		canvas.height = img.height
		ctx = canvas.getContext('2d') ?? undefined
		if (ctx === undefined) {
			return
		}
		ctx.drawImage(img, 0, 0)
		drawLabel(100, 80, 'テキスト')
	}

	function drawLabel(x: number, y: number, text: string) {
		if (ctx === undefined) {
			return
		}
		ctx.font = 'bold 18px sans-serif'
		const textMetrics = ctx.measureText(text)
		const textWidth = textMetrics.width
		const textHeight = 24
		const padding = 8

		ctx.fillStyle = 'white'
		ctx.fillRect(x, y - textHeight, textWidth + padding * 2, textHeight + padding)

		ctx.strokeStyle = 'red'
		ctx.lineWidth = 3
		ctx.strokeRect(x, y - textHeight, textWidth + padding * 2, textHeight + padding)

		ctx.fillStyle = 'red'
		ctx.fillText(text, x + padding, y)
	}
</script>

<canvas
	bind:this={canvas}
	class="border-white border my-1"
></canvas>

