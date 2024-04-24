<script lang="ts">
	import { images } from '$lib/images'
	import { removeImage } from '$lib/images'
</script>

{#each $images as image}
	<img src={image.compressedUrl} />
	<a href={image.compressedUrl} download={image.filename}>download</a>
	<button on:click|preventDefault={() => removeImage(image.id)}>remove</button>
{/each}
