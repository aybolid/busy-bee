<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    data-slot="card-description"
    class={cn("text-sm text-muted-foreground", props.class)}
>
    {@render children?.()}
</div>
