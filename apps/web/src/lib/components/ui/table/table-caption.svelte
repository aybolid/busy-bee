<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLElement>} */
    const { children, ...props } = $props();
</script>

<caption {...props} class={cn("mt-4 text-sm text-muted-foreground", props.class)}>
    {@render children?.()}
</caption>
