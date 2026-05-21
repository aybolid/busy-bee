<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    data-slot="card-title"
    class={cn("text-base leading-snug font-medium group-data-[size=sm]/card:text-sm", props.class)}
>
    {@render children?.()}
</div>
