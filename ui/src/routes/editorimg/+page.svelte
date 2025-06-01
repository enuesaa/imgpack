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
	}

	function handleOverlayUpload(event: Event & { currentTarget: HTMLInputElement }) {
		const files = event.currentTarget?.files ?? []
		if (files.length === 0) {
			return
		}
		const reader = new FileReader()
		reader.onload = (e) => {
			let overlay = new Image()
			overlay.onload = () => {
				if (canvas === undefined || ctx === undefined) {
					return
				}
				ctx.clearRect(0, 0, canvas.width, canvas.height)
				ctx.drawImage(img, 0, 0)

				if (overlay) {
					const x = 50
					const y = 50
					const width = overlay.width
					const height = overlay.height

					ctx.drawImage(overlay, x, y, width, height)
				}
			}
			const result = e.target?.result
			if (typeof result !== 'string') {
				return
			}
			overlay.src = result
		}
		reader.readAsDataURL(files[0])
	}

	let inputfile = $state<HTMLInputElement>()

	function handleClick(e: Event) {
		e.preventDefault()
		if (inputfile === undefined) {
			return
		}
		inputfile.click()
	}
</script>

<canvas
	bind:this={canvas}
	class="border-white border my-1"
></canvas>

<input
	bind:this={inputfile}
	type="file"
	accept="image/*"
	onchange={handleOverlayUpload}
	style="display:none"
/>
<button onclick={handleClick}>選択</button>
