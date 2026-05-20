<script>
    const id = $props.id();

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & { trigger?: import('svelte').Snippet<[{ popovertarget: string }]> }} */
    const { trigger, children, ...props } = $props();
</script>

<div {...props} class={["group contents", props.class]}>
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
        class="bg-transparent p-px pt-1.5 [position-anchor:var(--anchor)] [position-area:bottom_span-right] [position-try:flip-block]"
    >
        {@render children?.()}
    </div>
</div>
