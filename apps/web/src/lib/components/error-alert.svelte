<script>
    import { dev } from "$app/environment";
    import AlertDescription from "./ui/alert/alert-description.svelte";
    import AlertTitle from "./ui/alert/alert-title.svelte";
    import Alert from "./ui/alert/alert.svelte";
    import Bug from "./ui/icons/bug.svelte";

    /** @type {Omit<import('./ui/alert/alert.svelte').AlertProps, 'children'> & { error?: Error | null, title?: string, description?: string, stack?: string }} */
    const props = $props();

    const { title, description, stack } = $derived({
        title: props.title ?? props.error?.name ?? "Error",
        description: props.description ?? props.error?.message ?? "Something went wrong",
        stack: props.stack ?? props.error?.stack,
    });
</script>

<Alert variant="destructive" {...props}>
    <Bug />
    <AlertTitle>{title}</AlertTitle>
    <AlertDescription>
        <p>{description}</p>
        {@render verbose(stack)}
    </AlertDescription>
</Alert>

{#snippet verbose(/** @type {string | undefined} */ stack)}
    {#if dev && stack}
        {@const lines = stack.split("\n")}
        <div class="rounded-md font-mono text-xs">
            {#each lines as line}
                <div
                    class={line.includes("node_modules")
                        ? "text-muted-foreground"
                        : "text-destructive"}
                >
                    {line}
                </div>
            {/each}
        </div>
    {/if}
{/snippet}
