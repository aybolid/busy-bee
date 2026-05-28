<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} data-slot="accordion-header" class={cn("flex", props.class)}>
    {@render children?.()}
</div>
