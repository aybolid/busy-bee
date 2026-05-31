<script>
    import { cn } from "../ui/utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} class={cn("text-sm font-medium text-foreground", props.class)}>
    {@render children?.()}
</div>
