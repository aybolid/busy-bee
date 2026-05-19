<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} class={["flex max-w-sm flex-col items-center gap-2", props.class]}>
    {@render children?.()}
</div>
