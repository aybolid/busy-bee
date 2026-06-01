<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    data-slot="item-footer"
    class={cn("flex basis-full items-center justify-between gap-2", props.class)}
>
    {@render children?.()}
</div>
