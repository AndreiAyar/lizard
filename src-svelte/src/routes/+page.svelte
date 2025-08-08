<script lang="ts">
	import { onMount } from 'svelte';
	let settings: { debounce_delay: number } = { debounce_delay: 0.3 };
	onMount(async () => {
		settings = await getSettings();
	});

	async function getSettings() {
		try {
			const response = await fetch('http://localhost:8000/settings', {
				method: 'GET'
			});
			if (!response.ok) {
				throw new Error('Network response was not ok');
			}
			const data = await response.json();
			return data;
		} catch (error) {
			console.error('Failed to fetch settings:', error);
			return { debounce_delay: 0.3 }; // Default value
		}
	}
    console.log('dd', settings.debounce_delay)
	let debounceValue: number = $state(settings.debounce_delay);
	function onSliderChange(event: Event) {
		const value = (event.target as HTMLSelectElement).value;
		console.log('val changed', value);
	}
</script>

<h1>Welcome to SvelteKit</h1>

<input type="range" step={0.1} min={0.1} max="5" value={debounceValue} onchange={onSliderChange} class="range" />
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
