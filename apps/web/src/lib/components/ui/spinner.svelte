<script>
    import LoaderCircle from "./icons/loader-circle.svelte";

    /** @type {import('svelte/elements').SVGAttributes<SVGElement>} */
    const props = $props();
</script>

<LoaderCircle
    role="status"
    aria-label="Loading"
    {...props}
    class={["size-4 animate-spin", props.class]}
/>
