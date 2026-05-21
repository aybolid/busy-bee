<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} data-slot="skeleton" class={["animate-pulse rounded-md bg-muted", props.class]}>
    {@render children?.()}
</div>
