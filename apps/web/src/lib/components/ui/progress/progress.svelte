<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    data-slot="progress"
    {...props}
    class={cn(
        "relative flex h-1 w-full items-center overflow-x-hidden rounded-full bg-muted",
        props.class,
    )}
>
    {@render children?.()}
</div>
