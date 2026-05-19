<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLTableRowElement>} */
    const { children, ...props } = $props();
</script>

<tr {...props} class={["border-b transition-colors hover:bg-muted/50", props.class]}>
    {@render children?.()}
</tr>
