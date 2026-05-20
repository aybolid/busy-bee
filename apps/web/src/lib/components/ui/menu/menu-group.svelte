<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} class={["flex flex-col", props.class]}>
    {@render children?.()}
</div>
