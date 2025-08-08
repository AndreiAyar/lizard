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
			if (value >= 0) {
				const request = await fetch('http://localhost:8000/settings', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({ debounce_delay: value })
				});

				if (request.status === 200) {
					///Todo
                    debounceValue = value
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

<div class="flex min-h-screen items-center justify-center bg-white p-8">
	<div class="w-full max-w-lg space-y-8">
		<!-- Header -->
		<div class="text-center">
			<h1 class="mb-2 text-6xl font-light text-gray-900">🦎</h1>
			<h2 class="mb-1 text-3xl font-light text-gray-800">LIZARD</h2>
			<p class="text-sm text-gray-500">Keypress sound settings</p>
		</div>

		<!-- Settings Card -->
		<div class="rounded-2xl border border-gray-100 bg-gray-50 p-8 shadow-sm">
			<div class="space-y-6">
				<!-- Current Value Display -->
				<div class="text-center">
					<div
						class={`${debounceValue === default_delay ? 'text-green-600' : 'text-gray-900'} mb-1 text-4xl  font-light`}
					>
						{debounceValue.toFixed(1)}s {debounceValue === default_delay ? '- best' : null}
					</div>
					<div class="text-sm text-gray-500">Delay between sounds</div>
				</div>

				<!-- Slider -->
				<div class="space-y-4">
					<input
						type="range"
						step={0.1}
						min={0}
						max="5"
						bind:value={debounceValue}
						oninput={onSliderChange}
						class="slider h-2 w-full cursor-pointer appearance-none rounded-lg bg-gray-200"
					/>

					<!-- Range Labels -->
					<div class="flex justify-between text-xs text-gray-400">
						<button
							type="button"
							class="m-0 cursor-pointer border-none bg-transparent p-0 font-medium text-green-500"
							onclick={() => {
								updateSettings(default_delay);
							}}
						>
							Very Fast
						</button>
						<span class="text-gray-500">Fast</span>
						<span class="text-gray-500">1s <span class="text-xs font-normal">(Slow)</span></span>
						<span class="text-gray-500"
							>3s <span class="text-xs font-normal">(Very slow)</span></span
						>
						<span class="text-gray-500">5.0s</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Footer -->
		<div class="text-center">
			<p class="text-xs text-gray-400">Press any key to hear LIZARD 🦎</p>
		</div>
	</div>
</div>

<style>
	.slider::-webkit-slider-thumb {
		appearance: none;
		height: 20px;
		width: 20px;
		border-radius: 50%;
		background: #374151;
		cursor: pointer;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease;
	}

	.slider::-webkit-slider-thumb:hover {
		background: #111827;
		transform: scale(1.1);
	}

	.slider::-moz-range-thumb {
		height: 20px;
		width: 20px;
		border-radius: 50%;
		background: #374151;
		cursor: pointer;
		border: none;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}
</style>
