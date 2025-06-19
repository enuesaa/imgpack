<svelte:options runes={true} />

<script lang="ts">
	type Overlay = {
		img: HTMLImageElement
		x: number
		y: number
		width: number
		height: number
	}
	let canvas = $state<HTMLCanvasElement>()
	let ctx = $state<CanvasRenderingContext2D>()

	let baseImage = new Image()
	baseImage.src = 'example.png'
	baseImage.onload = () => {
		if (canvas === undefined) {
			return
		}
		canvas.width = baseImage.width
		canvas.height = baseImage.height
		ctx = canvas.getContext('2d') ?? undefined
		drawCanvas()
	}
	let overlays = $state<Overlay[]>([])
	let currentOverlay = $state<Overlay>()
	let offsetX = $state(0)
	let offsetY = $state(0)
	let resizing = $state(false)
	let resizeCorner = $state('')
	const handleSize = 10

	function addOverlay(file: File) {
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
				drawCanvas()
			}
			const result = e.target?.result
			if (typeof result !== 'string') {
				return
			}
			img.src = result
		}
		reader.readAsDataURL(file)
	}

	function handleFileUpload(e: Event & { currentTarget: HTMLInputElement }) {
		const files = Array.from(e.currentTarget?.files ?? [])
		files.forEach(addOverlay)
	}

	function drawCanvas() {
		if (canvas === undefined || ctx === undefined) {
			return
		}
		ctx.clearRect(0, 0, canvas.width, canvas.height)
		ctx.drawImage(baseImage, 0, 0)

		overlays.forEach((overlay) => {
			if (ctx === undefined) {
				return
			}
			ctx.drawImage(overlay.img, overlay.x, overlay.y, overlay.width, overlay.height)

			if (overlay === currentOverlay) {
				ctx.strokeStyle = 'red'
				ctx.lineWidth = 2
				ctx.strokeRect(overlay.x, overlay.y, overlay.width, overlay.height)
				drawHandles(overlay)
			}
		})
	}

	function drawHandles(overlay: Overlay) {
		if (ctx === undefined) {
			return
		}
		ctx.fillStyle = 'blue'
		const corners = getCorners(overlay)
		for (const [_, pos] of Object.entries(corners)) {
			ctx.fillRect(pos.x, pos.y, handleSize, handleSize)
		}
	}

	function getCorners(overlay: Overlay) {
		return {
			tl: { x: overlay.x, y: overlay.y },
			tr: { x: overlay.x + overlay.width - handleSize, y: overlay.y },
			bl: { x: overlay.x, y: overlay.y + overlay.height - handleSize },
			br: {
				x: overlay.x + overlay.width - handleSize,
				y: overlay.y + overlay.height - handleSize
			}
		}
	}

	function getMousePos(e: MouseEvent) {
		if (canvas === undefined) {
			return { x: 0, y: 0 }
		}
		const rect = canvas.getBoundingClientRect()
		return {
			x: e.clientX - rect.left,
			y: e.clientY - rect.top
		}
	}

	function hitTestHandles(pos: { x: number; y: number }, overlay: Overlay) {
		const corners = getCorners(overlay)
		for (const [corner, handlePos] of Object.entries(corners)) {
			if (
				pos.x >= handlePos.x &&
				pos.x <= handlePos.x + handleSize &&
				pos.y >= handlePos.y &&
				pos.y <= handlePos.y + handleSize
			) {
				return corner
			}
		}
		return null
	}

	function onMouseDown(e: MouseEvent) {
		const pos = getMousePos(e)
		currentOverlay = undefined

		for (let i = overlays.length - 1; i >= 0; i--) {
			const o = overlays[i]
			const corner = hitTestHandles(pos, o)
			if (corner) {
				currentOverlay = o
				resizing = true
				resizeCorner = corner
				return
			} else if (pos.x >= o.x && pos.x <= o.x + o.width && pos.y >= o.y && pos.y <= o.y + o.height) {
				currentOverlay = o

				overlays.splice(i, 1)
				overlays.push(o)

				offsetX = pos.x - o.x
				offsetY = pos.y - o.y
				drawCanvas()
				return
			}
		}
		drawCanvas()
	}

	function onMouseMove(e: MouseEvent) {
		if (!currentOverlay) return
		const pos = getMousePos(e)

		if (resizing) {
			switch (resizeCorner) {
				case 'br':
					currentOverlay.width = pos.x - currentOverlay.x
					currentOverlay.height = pos.y - currentOverlay.y
					break
				case 'tr':
					currentOverlay.height += currentOverlay.y - pos.y
					currentOverlay.y = pos.y
					currentOverlay.width = pos.x - currentOverlay.x
					break
				case 'bl':
					currentOverlay.width += currentOverlay.x - pos.x
					currentOverlay.x = pos.x
					currentOverlay.height = pos.y - currentOverlay.y
					break
				case 'tl':
					currentOverlay.width += currentOverlay.x - pos.x
					currentOverlay.height += currentOverlay.y - pos.y
					currentOverlay.x = pos.x
					currentOverlay.y = pos.y
					break
			}
		} else {
			currentOverlay.x = pos.x - offsetX
			currentOverlay.y = pos.y - offsetY
		}

		drawCanvas()
	}

	function onMouseUp() {
		resizing = false
		currentOverlay = undefined
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
	onmousedown={onMouseDown}
	onmousemove={onMouseMove}
	onmouseup={onMouseUp}
	class="border-white border my-1"
></canvas>

<input bind:this={inputfile} type="file" accept="image/*" onchange={handleFileUpload} style="display:none" />
<button onclick={handleClick}>選択</button>
