<script>
    import { cn } from "../utils";
    import { getAlertDialogContext } from "./context";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
    const { descriptionId } = getAlertDialogContext();
</script>

<div
    id={descriptionId}
    {...props}
    class={cn(
        "text-sm text-balance text-muted-foreground md:text-pretty *:[a]:underline *:[a]:underline-offset-3 *:[a]:hover:text-foreground",
        props.class,
    )}
>
    {@render children?.()}
</div>
