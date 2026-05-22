<script module>
    import { cva } from "class-variance-authority";

    const variants = cva("group/field flex w-full gap-2 data-[invalid=true]:text-destructive", {
        variants: {
            orientation: {
                vertical: "flex-col *:w-full [&>.sr-only]:w-auto",
                horizontal:
                    "flex-row items-center has-[>[data-slot=field-content]]:items-start *:data-[slot=field-label]:flex-auto has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px",
                responsive:
                    "flex-col *:w-full @md/field-group:flex-row @md/field-group:items-center @md/field-group:*:w-auto @md/field-group:has-[>[data-slot=field-content]]:items-start @md/field-group:*:data-[slot=field-label]:flex-auto [&>.sr-only]:w-auto @md/field-group:has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px",
            },
        },
        defaultVariants: {
            orientation: "vertical",
        },
    });

    /** @typedef {import('class-variance-authority').VariantProps<typeof variants>} FieldVariants */

    /** @typedef {import('svelte/elements').HTMLAttributes<HTMLDivElement> & FieldVariants} FieldProps */
</script>

<script>
    import { cn } from "../utils";

    /** @type {FieldProps} */
    const { children, orientation, ...props } = $props();
</script>

<div
    {...props}
    role="group"
    data-slot="field"
    data-orientation={orientation}
    class={cn(variants({ orientation, class: props.class }))}
>
    {@render children?.()}
</div>
