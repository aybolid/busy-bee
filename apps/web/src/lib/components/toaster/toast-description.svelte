<script>
    import { cn } from "../ui/utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} class={cn("text-xs text-muted-foreground", props.class)}>
    {@render children?.()}
</div>
