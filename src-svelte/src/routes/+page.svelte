<script lang="ts">
	import { onMount } from 'svelte';
	const default_delay = 0.3;
	let settings: Record<string, number | string>;
	let debounceValue: number = $state(default_delay);
	onMount(async () => {
		settings = await getSettings();
		debounceValue = settings?.debounce_delay as number;
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
			console.log('data este', data);
			return data;
		} catch (error) {
			console.error('Failed to fetch settings:', error);
			return { debounce_delay: 0.69 }; // Default value
		}
	}

	async function updateSettings(value: number) {
		try {
			if (value > 0) {
				const request = await fetch('http://localhost:8000/settings', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({ debounce_delay: debounceValue })
				});

                if(request.status === 200){
                    ///Todo
                }
			}
		} catch (error) {
			console.error('There was an error on updating', error);
		}
	}

	function onSliderChange(event: Event) {
		const value = (event.target as HTMLSelectElement).value;
		updateSettings(+value);
		console.log('val changed', value);
	}
</script>

<h1>Welcome to SvelteKit</h1>

<input
	type="range"
	step={0.1}
	min={0.1}
	max="5"
	value={debounceValue}
	onchange={onSliderChange}
	class="range"
/>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
