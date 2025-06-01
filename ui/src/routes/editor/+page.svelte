<svelte:options runes={true} />

<script lang="ts">
	import ImageSelector from './ImageSelector.svelte'
	import Canvas from './Canvas.svelte'

	function addOverlay(file: File) {

	}
</script>

<ImageSelector {addOverlay} />
<Canvas />
