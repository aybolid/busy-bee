<script module>
    import { cva } from "class-variance-authority";

    const variants = cva(
        "group/item flex w-full flex-wrap items-center rounded-lg border text-sm transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors [a]:hover:bg-muted",
        {
            variants: {
                variant: {
                    default: "border-transparent",
                    outline: "border-border",
                    muted: "border-transparent bg-muted/50",
                },
                size: {
                    default: "gap-2.5 px-3 py-2.5",
                    sm: "gap-2.5 px-3 py-2.5",
                    xs: "gap-2 px-2.5 py-2 in-data-[slot=dropdown-menu-content]:p-0",
                },
            },
            defaultVariants: {
                variant: "default",
                size: "default",
            },
        },
    );

    /** @typedef {import('class-variance-authority').VariantProps<typeof variants>} ItemVariants */
</script>

<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & ItemVariants} */
    const { children, variant, size, ...props } = $props();
</script>

<div
    {...props}
    data-slot="item"
    data-variant={variant}
    data-size={size}
    class={cn(variants({ variant, size, class: props.class }))}
>
    {@render children?.()}
</div>
