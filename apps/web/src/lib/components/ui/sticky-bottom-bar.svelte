<script>
    import { cn } from "./utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    class={cn(
        "sticky bottom-8 mx-auto flex w-fit items-center gap-4 rounded-md border bg-background p-2 shadow-md",
        props.class,
    )}
>
    {@render children?.()}
</div>
