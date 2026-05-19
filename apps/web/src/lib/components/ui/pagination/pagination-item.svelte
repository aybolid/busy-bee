<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLLIElement>} */
    const { children, ...props } = $props();
</script>

<li {...props}>
    {@render children?.()}
</li>
