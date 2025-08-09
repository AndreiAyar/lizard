<script lang="ts">
	import { onMount } from 'svelte';
	const default_delay = 0.3;
	let settings: Record<string, number | string>;
	let debounceValue: number = $state(default_delay);
	let isAppOn: boolean = $state(true);
	let backendReady: boolean = $state(false);
	let loadingDots: string = $state('');

	onMount(async () => {
		await waitForBackend();
		
		settings = await getSettings();
		debounceValue = settings?.debounce_delay as number;
	});

	async function waitForBackend() {
		const maxAttempts = 30; // that should be 15 seconds max at 500ms
		let attempts = 0;
		
		const dotsInterval = setInterval(() => {
			loadingDots = loadingDots.length >= 3 ? '' : loadingDots + '.';
		}, 500);

		while (attempts < maxAttempts && !backendReady) {
			try {
				const response = await fetch('http://localhost:8000/health');
				if (response.ok) {
					const data = await response.json();
					if (data.ready === true) {
						backendReady = true;
						clearInterval(dotsInterval);
						return;
					}
				}
			} catch (error) {
				// Backend not yet available, continue polling
			}
			
			attempts++;
			await new Promise(resolve => setTimeout(resolve, 500));
		}
		
		clearInterval(dotsInterval);
		if (!backendReady) {
			console.error('Backend failed to initialize within timeout');
		}
	}

	async function toggleApp() {
		try {
			const request = await fetch('http://localhost:8000/toggle', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				}
			});
			const response = await request.json();

			if (request.status === 200) {
				isAppOn = response?.app_status === 'on';
			}
		} catch (error) {
			console.error('There was an error on updating', error);
		}
	}

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
					debounceValue = value;
				}
			}
		} catch (error) {
			console.error('There was an error on updating', error);
		}
	}

	function onSliderChange(event: Event) {
		const value = (event.target as HTMLSelectElement).value;
		updateSettings(+value);
	}
</script>

{#if !backendReady}
	<!-- Loading State -->
	<div class="flex min-h-screen items-center justify-center bg-white p-8">
		<div class="text-center space-y-6">
			<div class="text-8xl animate-bounce">🦎</div>
			<h2 class="text-2xl font-light text-gray-800">LIZARD</h2>
			<p class="text-gray-600">Initializing{loadingDots}</p>
			<div class="w-48 h-2 bg-gray-200 rounded-full mx-auto overflow-hidden">
				<div class="h-full bg-green-500 rounded-full animate-pulse w-full"></div>
			</div>
		</div>
	</div>
{:else}
	<!-- Main App UI -->
	<div class="flex min-h-screen items-center justify-center bg-white p-8">
		<div class="w-full max-w-lg space-y-8">
			<!-- Header -->
			<div class="text-center">
				<h1 class="mb-2 text-6xl font-light text-gray-900">🦎</h1>
				<h2 class="mb-1 text-3xl font-light text-gray-800">LIZARD</h2>
				<label class=" inline-flex cursor-pointer items-center">
					<input type="checkbox" class="peer sr-only" onchange={toggleApp} bind:checked={isAppOn} />
					<div
						class="peer relative h-6 w-11 rounded-full bg-gray-200 after:absolute after:start-[2px] after:top-0.5 after:h-5 after:w-5 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-green-600 peer-checked:after:translate-x-full peer-checked:after:border-white peer-focus:ring-4 peer-focus:ring-green-300 rtl:peer-checked:after:-translate-x-full dark:border-gray-600 dark:bg-gray-700 dark:peer-checked:bg-green-600 dark:peer-focus:ring-green-800"
					></div>
				</label>
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
{/if}

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

	@keyframes bounce {
		0%, 20%, 53%, 80%, 100% {
			transform: translateY(0);
		}
		40%, 43% {
			transform: translateY(-20px);
		}
		70% {
			transform: translateY(-10px);
		}
		90% {
			transform: translateY(-4px);
		}
	}
	
	.animate-bounce {
		animation: bounce 2s infinite;
	}
	
	.animate-pulse {
		animation: pulse 1.5s infinite;
	}
	
	@keyframes pulse {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}
</style>
