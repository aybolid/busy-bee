<script>
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    data-slot="field-content"
    class={cn("group/field-content flex flex-1 flex-col gap-0.5 leading-snug", props.class)}
>
    {@render children?.()}
</div>
