<script>
    import { cn } from "../utils";
    import { getDialogContext } from "./context";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
    const { descriptionId } = getDialogContext();
</script>

<div
    id={descriptionId}
    {...props}
    class={cn(
        "text-sm text-muted-foreground *:[a]:underline *:[a]:underline-offset-3 *:[a]:hover:text-foreground",
        props.class,
    )}
>
    {@render children?.()}
</div>
