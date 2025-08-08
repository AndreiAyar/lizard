<script lang="ts">
	function onSliderChange(event: Event) {
		const value = (event.target as HTMLSelectElement).value;
		console.log('val changed', value);
	}
</script>

<h1>Welcome to SvelteKit</h1>

<input type="range" min={0} max="100" value="40" on:change={onSliderChange} class="range" />
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
