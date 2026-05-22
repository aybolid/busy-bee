<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLFieldsetAttributes} */
    const { children, ...props } = $props();
</script>

<fieldset
    {...props}
    data-slot="field-set"
    class={cn(
        "flex flex-col gap-4 has-[>[data-slot=checkbox-group]]:gap-3 has-[>[data-slot=radio-group]]:gap-3",
        props.class,
    )}
>
    {@render children?.()}
</fieldset>
