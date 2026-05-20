<script>
    import EmptyDescription from "./ui/empty/empty-description.svelte";
    import EmptyHeader from "./ui/empty/empty-header.svelte";
    import EmptyMedia from "./ui/empty/empty-media.svelte";
    import EmptyTitle from "./ui/empty/empty-title.svelte";
    import Empty from "./ui/empty/empty.svelte";
    import Spinner from "./ui/spinner.svelte";

    /** @type {Omit<import('./ui/empty/empty.svelte').EmptyProps, 'children'> & { title?: string, description?: string }} */
    const { title = "Loading", description, ...props } = $props();
</script>

<Empty {...props}>
    <EmptyHeader>
        <EmptyMedia>
            <Spinner />
        </EmptyMedia>
        <EmptyTitle>{title}</EmptyTitle>
        {#if description}
            <EmptyDescription>{description}</EmptyDescription>
        {/if}
    </EmptyHeader>
</Empty>
