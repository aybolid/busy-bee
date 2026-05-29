<script>
    import { cn } from "../utils";

    const id = $props.id();

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & { position?: 'top' | 'bottom' } & { trigger?: import('svelte').Snippet<[{ popovertarget: string }]> }} */
    const { trigger, children, position = "bottom", ...props } = $props();
</script>

<div {...props} class={cn("group contents", props.class)}>
    <span
        class="[anchor-name:var(--anchor)] group-has-[:popover-open]:opacity-50"
        style="--anchor: --anchor-{id}"
    >
        {@render trigger?.({ popovertarget: id })}
    </span>

    <div
        {id}
        popover
        style="--anchor: --anchor-{id}"
        class={cn(
            "bg-transparent p-px pt-1.5 [position-anchor:var(--anchor)] [position-try:flip-block]",
            {
                "[position-area:top_center]": position === "top",
                "[position-area:bottom_center]": position === "bottom",
            },
        )}
    >
        {@render children?.()}
    </div>
</div>
