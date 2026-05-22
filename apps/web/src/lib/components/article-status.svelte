<script>
    import Badge from "./ui/badge.svelte";
    import Bug from "./ui/icons/bug.svelte";
    import Spinner from "./ui/spinner.svelte";

    /** @type {Omit<import('$lib/components/ui/badge.svelte').BadgeProps, 'children'> & { status: import('$lib/api/articles').ArticleStatus }} */
    const { status, ...props } = $props();

    /** @type {{ label: string, variant: NonNullable<import('$lib/components/ui/badge.svelte').BadgeVariants['variant']> }} */
    const { label, variant } = $derived.by(() => {
        switch (status) {
            case "new":
                return { label: "New", variant: "outline" };
            case "pending":
                return { label: "Pending", variant: "secondary" };
            case "processed":
                return { label: "Processed", variant: "default" };
            case "error":
                return { label: "Error", variant: "destructive" };
        }
    });
</script>

<Badge {...props} variant={props.variant ?? variant}>
    {#if status === "error"}
        <Bug />
    {:else if status === "pending"}
        <Spinner />
    {/if}
    <span>
        {label}
    </span>
</Badge>
