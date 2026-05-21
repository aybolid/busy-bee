<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} class={cn("px-1.5 py-1 text-xs font-medium text-muted-foreground", props.class)}>
    {@render children?.()}
</div>
