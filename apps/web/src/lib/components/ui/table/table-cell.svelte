<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLTdAttributes} */
    const { children, ...props } = $props();
</script>

<td {...props} class={cn("p-2 align-middle whitespace-nowrap", props.class)}>
    {@render children?.()}
</td>
