<script module>
    /** @typedef {import('svelte/elements').HTMLOptionAttributes} NativeSelectOptionProps */
</script>

<script>
    import { cn } from "../utils";

    /** @type {NativeSelectOptionProps} */
    const { children, ...props } = $props();
</script>

<option
    data-slot="native-select-option"
    {...props}
    class={cn("bg-[Canvas] text-[CanvasText]", props.class)}
>
    {@render children?.()}
</option>
