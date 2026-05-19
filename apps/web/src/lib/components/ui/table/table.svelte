<script>
    /** @type {import('svelte/elements').HTMLTableAttributes} */
    const { children, ...props } = $props();
</script>

<div class="relative w-full overflow-x-auto">
    <table {...props} class={["w-full caption-bottom text-sm", props.class]}>
        {@render children?.()}
    </table>
</div>
