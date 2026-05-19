<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLElement>} */
    const { children, ...props } = $props();
</script>

<nav aria-label="navigation" {...props} class={["mx-auto flex w-full justify-center", props.class]}>
    {@render children?.()}
</nav>
