<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLElement>} */
    const { children, ...props } = $props();
</script>

<caption {...props} class={["mt-4 text-sm text-muted-foreground", props.class]}>
    {@render children?.()}
</caption>
