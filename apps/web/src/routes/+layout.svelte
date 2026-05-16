<script>
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	import { QueryClientProvider } from '@tanstack/svelte-query';
	import { SvelteQueryDevtools } from '@tanstack/svelte-query-devtools';

	/** @type {import('./$types').LayoutProps} */
	let { children, data } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Web</title>
</svelte:head>

<QueryClientProvider client={data.queryClient}>
	<main class="mx-auto max-w-7xl p-4">
		{@render children()}
	</main>
	<SvelteQueryDevtools />
</QueryClientProvider>
