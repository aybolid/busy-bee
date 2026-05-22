<script>
    import Separator from "../separator.svelte";
    import { cn } from "../utils";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div
    {...props}
    data-slot="field-separator"
    data-content={!!children}
    class={cn(
        "relative -my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2",
        props.class,
    )}
>
    <Separator class="absolute inset-0 top-1/2" />
    {#if children}
        <span
            class="relative mx-auto block w-fit bg-background px-2 text-muted-foreground"
            data-slot="field-separator-content"
        >
            {@render children()}
        </span>
    {/if}
</div>
