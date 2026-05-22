<script>
    import { cn } from "./utils";

    /** @type {Omit<import('svelte/elements').HTMLAttributes<HTMLDivElement>, 'children'> & { orientation?: 'horizontal' | 'vertical' }} */
    const { orientation, ...props } = $props();
</script>

<div
    {...props}
    data-slot="separator"
    aria-orientation={orientation}
    data-orientation={orientation}
    class={cn(
        "shrink-0 bg-border data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:w-px data-[orientation=vertical]:self-stretch",
        props.class,
    )}
></div>
