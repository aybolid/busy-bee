<script module>
    import { cva } from "class-variance-authority";

    const variants = cva(
        "mb-2 flex shrink-0 items-center justify-center [&_svg]:pointer-events-none [&_svg]:shrink-0",
        {
            variants: {
                variant: {
                    icon: "flex size-8 shrink-0 items-center justify-center rounded-lg bg-muted text-foreground [&_svg:not([class*='size-'])]:size-4",
                    image: "bg-transparent",
                },
            },
            defaultVariants: {
                variant: "icon",
            },
        },
    );

    /** @typedef {import('class-variance-authority').VariantProps<typeof variants>} EmptyMediaVariants */
</script>

<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & EmptyMediaVariants} */
    const { children, ...props } = $props();
</script>

<div {...props} class={variants({ class: props.class, variant: props.variant })}>
    {@render children?.()}
</div>
