<script>
    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement> & { size?: 'default' | 'sm' }} */
    const { children, size = "default", ...props } = $props();
</script>

<div
    {...props}
    data-size={size}
    data-slot="card"
    class={[
        "group/card flex flex-col gap-4 overflow-hidden rounded-xl bg-card py-4 text-sm text-card-foreground ring-1 ring-foreground/10",
        "has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:gap-3 data-[size=sm]:py-3",
        "data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl",
        props.class,
    ]}
>
    {@render children?.()}
</div>
