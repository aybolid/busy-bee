<script>
    import { cn } from "../utils";
    import { getDialogContext } from "./context";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
    const { labelId } = getDialogContext();
</script>

<div id={labelId} {...props} class={cn("text-base leading-none font-medium", props.class)}>
    {@render children?.()}
</div>
