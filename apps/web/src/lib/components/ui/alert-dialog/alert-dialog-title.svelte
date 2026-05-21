<script>
    import { cn } from "../utils";
    import { getAlertDialogContext } from "./context";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
    const { labelId } = getAlertDialogContext();
</script>

<div
    id={labelId}
    {...props}
    class={cn(
        "text-base font-medium sm:group-data-[size=default]/alert-dialog-content:group-has-data-[slot=alert-dialog-media]/alert-dialog-content:col-start-2",
        props.class,
    )}
>
    {@render children?.()}
</div>
