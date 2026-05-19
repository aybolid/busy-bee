<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLUListElement>} */
    const { children, ...props } = $props();
</script>

<ul {...props} class={["flex items-center gap-0.5", props.class]}>
    {@render children?.()}
</ul>
