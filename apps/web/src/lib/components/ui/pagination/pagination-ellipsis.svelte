<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLSpanElement>} */
    const { children, ...props } = $props();
</script>

<span
    aria-hidden="true"
    {...props}
    class={[
        "flex size-8 items-center justify-center text-muted-foreground [&_svg:not([class*='size-'])]:size-4",
        props.class,
    ]}
>
    ...
    <span class="sr-only">More pages</span>
</span>
