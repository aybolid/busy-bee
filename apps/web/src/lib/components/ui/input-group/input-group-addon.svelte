<script module>
    import { cva } from "class-variance-authority";

    const variants = cva(
        "flex h-auto cursor-text items-center justify-center gap-2 py-1.5 text-sm font-medium text-muted-foreground select-none group-data-[disabled=true]/input-group:opacity-50 [&>kbd]:rounded-[calc(var(--radius)-5px)] [&>svg:not([class*='size-'])]:size-4",
        {
            variants: {
                align: {
                    "inline-start":
                        "order-first pl-2 has-[>button]:ml-[-0.3rem] has-[>kbd]:ml-[-0.15rem]",
                    "inline-end":
                        "order-last pr-2 has-[>button]:mr-[-0.3rem] has-[>kbd]:mr-[-0.15rem]",
                    "block-start":
                        "order-first w-full justify-start px-2.5 pt-2 group-has-[>input]/input-group:pt-2 [.border-b]:pb-2",
                    "block-end":
                        "order-last w-full justify-start px-2.5 pb-2 group-has-[>input]/input-group:pb-2 [.border-t]:pt-2",
                },
            },
            defaultVariants: {
                align: "inline-start",
            },
        },
    );

    /** @typedef {import('class-variance-authority').VariantProps<typeof variants>} AddonVariants */
</script>

<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & AddonVariants} */
    const { children, align, ...props } = $props();
</script>

<div
    {...props}
    role="group"
    data-slot="input-group-addon"
    data-align={align}
    class={cn(variants({ align, class: props.class }))}
    onclick={(e) => {
        if (!e.target || /** @type {HTMLElement} */ (e.target).closest("button")) {
            return;
        }
        e.currentTarget.parentElement?.querySelector("input")?.focus();
    }}
>
    {@render children?.()}
</div>
