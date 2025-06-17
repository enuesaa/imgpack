<svelte:options runes={true} />

<script lang="ts">
	import type { Overlay } from './+page.svelte'
	type Props = {
		canvas?: HTMLCanvasElement
		ctx?: CanvasRenderingContext2D
		overlays: Overlay[]
	}
	let { canvas = $bindable(), ctx = $bindable(), overlays = $bindable() }: Props = $props()

	$effect(() => {
		if (canvas === undefined || ctx === undefined) {
			return
		}
		overlays.forEach((overlay) => {
			if (ctx === undefined) {
				return
			}
			ctx.drawImage(overlay.img, overlay.x, overlay.y, overlay.width, overlay.height)
		})
	})
</script>
