<script lang="ts">
	import '../app.css';
	export const ssr = false;
	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children?.()}
