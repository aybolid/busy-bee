<script module>
    /** @typedef {import('svelte/elements').HTMLLabelAttributes} LabelProps */
</script>

<script>
    import { cn } from "./utils";

    /** @type {LabelProps} */
    const { children, ...props } = $props();
</script>

<label
    data-slot="label"
    {...props}
    class={cn(
        "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50",
        props.class,
    )}
>
    {@render children?.()}
</label>
