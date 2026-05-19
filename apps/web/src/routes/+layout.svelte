<script>
    import "./layout.css";
    import favicon from "$lib/assets/favicon.svg";

    import { QueryClientProvider } from "@tanstack/svelte-query";
    import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";

    import Action from "$lib/components/ui/action.svelte";

    /** @type {import('./$types').LayoutProps} */
    let { children, data } = $props();
</script>

<svelte:head>
    <link rel="icon" href={favicon} />
    <title>Articles</title>
</svelte:head>

<QueryClientProvider client={data.queryClient}>
    <header class="mx-auto max-w-7xl px-4 py-8">
        <nav>
            <Action anchor href="/" variant="link">Articles</Action>
        </nav>
    </header>

    <main class="mx-auto max-w-7xl px-4 py-16">
        {@render children()}
    </main>

    <SvelteQueryDevtools />
</QueryClientProvider>
