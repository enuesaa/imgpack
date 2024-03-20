<script lang="ts">
	// see https://zenn.dev/miruoon_892/articles/2c9efd2c302c56bd41a6#actions
	let name: string = 'aaa'

	const focus = (node: HTMLInputElement) => {
		console.log(node)
	}
	const handleClick = () => {
		console.log('name', name)
	}
</script>

<a href="/">top</a>
<input type="text" use:focus bind:value={name} />
<button on:click={handleClick}>check</button>
