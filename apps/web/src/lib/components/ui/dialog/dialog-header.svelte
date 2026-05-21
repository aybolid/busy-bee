<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} data-slot="dialog-header" class={cn("flex flex-col gap-2", props.class)}>
    {@render children?.()}
</div>
