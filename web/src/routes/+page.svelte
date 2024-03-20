<script lang="ts">
	import { count } from './store'

    let currentCount: number
    count.subscribe(val => {
        currentCount = val;
    });
</script>

<h2>{currentCount}</h2>
<button on:click={() => count.update(val => {val++; return val})}>+</button>

<a href="/about">about</a>
