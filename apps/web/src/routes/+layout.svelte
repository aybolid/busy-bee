<script>
    import "./layout.css";
    import favicon from "$lib/assets/favicon.svg";

    import { QueryClientProvider } from "@tanstack/svelte-query";
    import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";

    import Action from "$lib/components/ui/action.svelte";
    import { setGlobalContext } from "$lib/global-context";
    import Toaster from "$lib/components/toaster/toaster.svelte";
    import { sseListener } from "$lib/api/sse";

    /** @type {import('./$types').LayoutProps} */
    let { children, data } = $props();

    setGlobalContext({
        get ky() {
            return data.ky;
        },
        get queryClient() {
            return data.queryClient;
        },
    });

    $effect(() => {
        return sseListener(data.queryClient);
    });
</script>

<svelte:head>
    <link rel="icon" href={favicon} />
    <title>Busy Bee</title>
    <meta name="description" content="Busy Bee web UI" />
</svelte:head>

<QueryClientProvider client={data.queryClient}>
    <Toaster>
        <header class="mx-auto max-w-7xl px-4 pt-16">
            <nav class="flex gap-2">
                <Action anchor href="/" variant="link">RSS feeds</Action>
                <Action anchor href="/articles" variant="link">Articles</Action>
                <Action anchor href="/prompts" variant="link">Prompts</Action>
                <Action anchor href="/outputs" variant="link">Outputs</Action>
            </nav>
        </header>

        <main class="mx-auto max-w-7xl px-4 py-8">
            {@render children()}
        </main>

        <footer class="mx-auto max-w-7xl px-4 pb-16"></footer>
    </Toaster>
    <SvelteQueryDevtools />
</QueryClientProvider>
