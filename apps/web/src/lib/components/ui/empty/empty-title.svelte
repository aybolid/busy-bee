<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} class={["text-sm font-medium tracking-tight", props.class]}>
    {@render children?.()}
</div>
