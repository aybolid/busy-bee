<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    class={cn(
        "min-w-32 overflow-x-hidden overflow-y-auto rounded-lg bg-popover p-2 text-popover-foreground shadow-sm ring-1 ring-foreground/10",
        props.class,
    )}
>
    {@render children?.()}
</div>
