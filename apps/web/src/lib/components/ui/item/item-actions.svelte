<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} data-slot="item-actions" class={cn("flex items-center gap-2", props.class)}>
    {@render children?.()}
</div>
