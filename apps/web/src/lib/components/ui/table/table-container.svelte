<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} class={cn("w-full overflow-x-auto rounded-md border", props.class)}>
    {@render children?.()}
</div>
