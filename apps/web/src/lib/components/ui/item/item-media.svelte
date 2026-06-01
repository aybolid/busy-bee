<script module>
    import { cva } from "class-variance-authority";

    const variants = cva(
        "flex shrink-0 items-center justify-center gap-2 group-has-data-[slot=item-description]/item:translate-y-0.5 group-has-data-[slot=item-description]/item:self-start [&_svg]:pointer-events-none",
        {
            variants: {
                variant: {
                    default: "bg-transparent",
                    icon: "[&_svg:not([class*='size-'])]:size-4",
                    image: "size-10 overflow-hidden rounded-sm group-data-[size=sm]/item:size-8 group-data-[size=xs]/item:size-6 [&_img]:size-full [&_img]:object-cover",
                },
            },
            defaultVariants: {
                variant: "default",
            },
        },
    );

    /** @typedef {import('class-variance-authority').VariantProps<typeof variants>} ItemMediaVariants */
</script>

<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & ItemMediaVariants} */
    const { children, variant, ...props } = $props();
</script>

<div
    {...props}
    data-slot="item-media"
    data-variant={variant}
    class={cn(variants({ variant, class: props.class }))}
>
    {@render children?.()}
</div>
