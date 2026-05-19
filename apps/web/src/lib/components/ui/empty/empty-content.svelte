<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    class={[
        "flex w-full max-w-sm min-w-0 flex-col items-center gap-2.5 text-sm text-balance",
        props.class,
    ]}
>
    {@render children?.()}
</div>
