<script module>
    /** @typedef {import('svelte/elements').HTMLAttributes<HTMLDivElement>} EmptyProps */
</script>

<script>
    /** @type {EmptyProps} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    class={[
        "flex w-full min-w-0 flex-1 flex-col items-center justify-center gap-4 rounded-xl border-dashed p-6 text-center text-balance",
        props.class,
    ]}
>
    {@render children?.()}
</div>
