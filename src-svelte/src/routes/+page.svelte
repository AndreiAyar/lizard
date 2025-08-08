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
					body: JSON.stringify({ debounce_delay: debounceValue })
				});

				if (request.status === 200) {
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


<div class="min-h-screen bg-white flex items-center justify-center p-8">
    <div class="w-full max-w-md space-y-8">
        <!-- Header -->
        <div class="text-center">
            <h1 class="text-6xl font-light text-gray-900 mb-2">🦎</h1>
            <h2 class="text-3xl font-light text-gray-800 mb-1">LIZARD</h2>
            <p class="text-gray-500 text-sm">Keypress sound settings</p>
        </div>

        <!-- Settings Card -->
        <div class="bg-gray-50 rounded-2xl p-8 shadow-sm border border-gray-100">
            <div class="space-y-6">
                <!-- Current Value Display -->
                <div class="text-center">
                    <div class="text-4xl font-light text-gray-900 mb-1">
                        {debounceValue.toFixed(1)}s
                    </div>
                    <div class="text-sm text-gray-500">Delay between sounds</div>
                </div>

                <!-- Slider -->
                <div class="space-y-4">
                    <input
                        type="range"
                        step={0.1}
                        min={0.1}
                        max="5"
                        bind:value={debounceValue}
                        oninput={onSliderChange}
                        class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer slider"
                    />
                    
                    <!-- Range Labels -->
                    <div class="flex justify-between text-xs text-gray-400">
                        <span class="text-gray-500">0.1s</span>
                        <span class="text-green-500 font-medium">0.3s<br /><span class="text-xs font-normal">Default</span></span>
                        <span class="text-gray-500">1s<br /><span class="text-xs font-normal">Slow</span></span>
                        <span class="text-gray-500">3s<br /><span class="text-xs font-normal">Very slow</span></span>
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