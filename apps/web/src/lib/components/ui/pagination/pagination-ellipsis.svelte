<script>
    import Ellipsis from "../icons/ellipsis.svelte";
    import { cn } from "../utils";

    /** @type {Omit<import('svelte/elements').HTMLAttributes<HTMLSpanElement>, 'children'>} */
    const props = $props();
</script>

<span
    aria-hidden="true"
    {...props}
    class={cn(
        "flex size-8 items-center justify-center text-muted-foreground [&_svg:not([class*='size-'])]:size-4",
        props.class,
    )}
>
    <Ellipsis />
    <span class="sr-only">More pages</span>
</span>
