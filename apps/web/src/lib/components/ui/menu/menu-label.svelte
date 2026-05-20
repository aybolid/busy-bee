<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} class={["px-1.5 py-1 text-xs font-medium text-muted-foreground", props.class]}>
    {@render children?.()}
</div>
