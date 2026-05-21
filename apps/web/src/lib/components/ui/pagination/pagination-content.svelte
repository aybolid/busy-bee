<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLUListElement>} */
    const { children, ...props } = $props();
</script>

<ul {...props} class={cn("flex items-center gap-0.5", props.class)}>
    {@render children?.()}
</ul>
