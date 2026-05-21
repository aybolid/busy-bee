<script>
    import { cn } from "./utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} data-slot="skeleton" class={cn("animate-pulse rounded-md bg-muted", props.class)}>
    {@render children?.()}
</div>
