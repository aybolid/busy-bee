<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLSpanElement>} */
    const { children, ...props } = $props();
</script>

<span
    {...props}
    class={cn(
        "flex items-center gap-2 text-sm text-muted-foreground [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4",
        props.class,
    )}
>
    {@render children?.()}
</span>
