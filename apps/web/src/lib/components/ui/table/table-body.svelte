<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLTableSectionElement>} */
    const { children, ...props } = $props();
</script>

<tbody {...props} class={["[&_tr:last-child]:border-0", props.class]}>
    {@render children?.()}
</tbody>
