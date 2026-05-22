<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLTableAttributes} */
    const { children, ...props } = $props();
</script>

<table {...props} class={cn("w-full caption-bottom text-sm", props.class)}>
    {@render children?.()}
</table>
