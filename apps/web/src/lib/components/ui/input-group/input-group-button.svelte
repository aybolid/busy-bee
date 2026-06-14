<script module>
    import { cva } from "class-variance-authority";

    const variants = cva("flex items-center gap-2 text-sm shadow-none", {
        variants: {
            size: {
                xs: "h-6 gap-1 rounded-[calc(var(--radius)-3px)] px-1.5 [&>svg:not([class*='size-'])]:size-3.5",
                sm: "",
                "icon-xs": "size-6 rounded-[calc(var(--radius)-3px)] p-0 has-[>svg]:p-0",
                "icon-sm": "size-8 p-0 has-[>svg]:p-0",
            },
        },
        defaultVariants: {
            size: "xs",
        },
    });

    /** @typedef {import('class-variance-authority').VariantProps<typeof variants>} InputGroupButtonVariants */
</script>

<script>
    import { cn } from "../utils";
    import Action from "../action.svelte";

    /** @type {import('../action.svelte').ButtonProps & InputGroupButtonVariants} */
    const { children, variant = "ghost", size, ...props } = $props();
</script>

<Action button {...props} data-size={size} {variant} class={cn(variants({ size }), props.class)}>
    {@render children?.()}
</Action>
