<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & { size?: 'sm' | 'default' }} */
    const { children, size = "default", ...props } = $props();
</script>

<div
    data-size={size}
    {...props}
    class={[
        "group/alert-dialog-content fixed top-1/2 left-1/2 z-50 grid w-full -translate-x-1/2 -translate-y-1/2 gap-4",
        "rounded-xl bg-popover p-4 text-popover-foreground shadow-md ring-1 ring-foreground/10 duration-100 outline-none",
        "data-[size=default]:max-w-xs data-[size=sm]:max-w-xs data-[size=default]:sm:max-w-sm",
        props.class,
    ]}
>
    {@render children?.()}
</div>
