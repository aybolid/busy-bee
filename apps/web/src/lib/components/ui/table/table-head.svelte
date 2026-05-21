<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLThAttributes} */
    const { children, ...props } = $props();
</script>

<th
    {...props}
    class={cn(
        "h-10 px-2 text-left align-middle font-medium whitespace-nowrap text-foreground",
        props.class,
    )}
>
    {@render children?.()}
</th>
