<svelte:options runes={true} />

<script lang="ts">
	type Props = {
		canvas?: HTMLCanvasElement
		ctx?: CanvasRenderingContext2D
	}
	let { canvas = $bindable(), ctx = $bindable() }: Props = $props()

	const width = 500
	const height = 500

	$effect(() => {
		const whiteCanvas = document.createElement('canvas')
		whiteCanvas.width = 1
		whiteCanvas.height = 1

		const whiteCtx = whiteCanvas.getContext('2d') ?? undefined
		if (whiteCtx === undefined) {
			return
		}
		whiteCtx.fillStyle = '#fafafa'
		whiteCtx.fillRect(0, 0, 1, 1)

		const whiteImage = new Image()
		whiteImage.src = whiteCanvas.toDataURL()

		whiteImage.onload = () => {
			if (canvas === undefined) {
				return
			}
			canvas.width = width
			canvas.height = height
			ctx = canvas.getContext('2d') ?? undefined
			if (ctx === undefined) {
				return
			}
			ctx.drawImage(whiteImage, 0, 0, width, height)
		}
	})
</script>
