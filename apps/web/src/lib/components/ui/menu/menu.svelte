<script>
    import { cn } from "../utils";
    import { setMenuContext } from "./context";

    const id = $props.id();
    const context = setMenuContext({ menuId: id });

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & { trigger?: import('svelte').Snippet<[{ popovertarget: string }]> }} */
    const { trigger, children, ...props } = $props();
</script>

<div {...props} class={cn("group contents", props.class)}>
    <span
        class="[anchor-name:var(--anchor)] group-has-[:popover-open]:opacity-50"
        style="--anchor: --anchor-{context.menuId}"
    >
        {@render trigger?.({ popovertarget: context.menuId })}
    </span>

    <div
        id={context.menuId}
        popover
        style="--anchor: --anchor-{context.menuId}"
        class="bg-transparent p-px pt-1.5 [position-anchor:var(--anchor)] [position-area:bottom_span-right] [position-try:flip-block]"
    >
        {@render children?.()}
    </div>
</div>
