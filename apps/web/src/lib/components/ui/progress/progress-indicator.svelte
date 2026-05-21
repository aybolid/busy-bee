<script>
    import { cn } from "../utils";

    /** @type {Omit<import('svelte/elements').HTMLAttributes<HTMLDivElement>, 'children'> & { value: number }} */
    const { value, ...props } = $props();
</script>

<div
    style="transform: translateX(-{100 - (value || 0)}%)"
    data-slot="progress-indicator"
    {...props}
    class={cn("size-full flex-1 bg-primary transition-all", props.class)}
></div>
