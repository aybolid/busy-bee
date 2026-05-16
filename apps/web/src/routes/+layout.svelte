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
	<title>Articles</title>
</svelte:head>

<QueryClientProvider client={data.queryClient}>
	<main class="mx-auto max-w-7xl px-4 py-16">
		{@render children()}
	</main>
	<SvelteQueryDevtools />
</QueryClientProvider>
