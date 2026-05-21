<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    data-slot="card-content"
    class={["px-4 group-data-[size=sm]/card:px-3", props.class]}
>
    {@render children?.()}
</div>
