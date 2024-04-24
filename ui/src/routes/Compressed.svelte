<script lang="ts">
	import { images } from '$lib/images'
	import { removeImage } from '$lib/images'
</script>

<div class="flex gap-5 p-5">
	{#each $images as image}
		<div class="w-20 h-20 inline-block">
			<a href={image.compressedUrl} download={image.filename.replace('.', '.out.')} class="w-full">
				<img src={image.compressedUrl} />			
			</a>
			<button on:click|preventDefault={() => removeImage(image.id)}>remove</button>
		</div>
	{/each}
</div>
