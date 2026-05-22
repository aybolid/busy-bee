<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLLegendElement> & { variant?: 'legend' | 'label' }} */
    const { children, variant = "legend", ...props } = $props();
</script>

<legend
    {...props}
    data-slot="field-legend"
    data-variant={variant}
    class={cn(
        "mb-1.5 font-medium data-[variant=label]:text-sm data-[variant=legend]:text-base",
        props.class,
    )}
>
    {@render children?.()}
</legend>
