<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    class={[
        "text-sm/relaxed text-muted-foreground [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary",
        props.class,
    ]}
>
    {@render children?.()}
</div>
