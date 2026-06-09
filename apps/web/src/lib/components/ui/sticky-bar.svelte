<script>
    import { slide } from "svelte/transition";
    import { cn } from "./utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & { position?: 'top' | 'bottom' }} */
    const { children, position = "bottom", ...props } = $props();
</script>

<div
    {...props}
    transition:slide={{ duration: 100 }}
    class={cn(
        "sticky mx-auto flex w-fit items-center gap-4 rounded-md border bg-card p-2 shadow-md z-40",
        {
            "top-8": position === "top",
            "bottom-8": position === "bottom",
        },
        props.class,
    )}
>
    {@render children?.()}
</div>
