<script lang="ts">
	import { count } from './store'

	function updateCount() {
		count.set(($count += 1))
	}
</script>

<h2>{$count}</h2>
<button on:click={updateCount}>+</button>
