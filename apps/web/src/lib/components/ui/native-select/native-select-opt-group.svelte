<script module>
    /** @typedef {import('svelte/elements').HTMLOptgroupAttributes} NativeSelectOptGroupProps */
</script>

<script>
    import { cn } from "../utils";

    /** @type {NativeSelectOptGroupProps} */
    const { children, ...props } = $props();
</script>

<optgroup
    data-slot="native-select-optgroup"
    {...props}
    class={cn("bg-[Canvas] text-[CanvasText]", props.class)}
>
    {@render children?.()}
</optgroup>
