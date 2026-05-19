<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLTableSectionElement>} */
    const { children, ...props } = $props();
</script>

<tfoot {...props} class={["border-t bg-muted/50 font-medium [&>tr]:last:border-b-0", props.class]}>
    {@render children?.()}
</tfoot>
