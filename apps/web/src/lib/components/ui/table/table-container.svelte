<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} class={cn("relative w-full overflow-x-auto", props.class)}>
    {@render children?.()}
</div>
