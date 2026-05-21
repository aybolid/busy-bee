<script>
    import LoaderCircle from "./icons/loader-circle.svelte";
    import { cn } from "./utils";

    /** @type {import('svelte/elements').SVGAttributes<SVGElement>} */
    const props = $props();
</script>

<LoaderCircle
    role="status"
    aria-label="Loading"
    {...props}
    class={cn("size-4 animate-spin", props.class)}
/>
