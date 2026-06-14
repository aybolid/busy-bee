<script>
    import Input from "../input.svelte";
    import { cn } from "../utils";

    /** @type {import('svelte').ComponentProps<typeof Input>} */
    const props = $props();
</script>

<Input
    {...props}
    data-slot="input-group-control"
    class={cn(
        "flex-1 rounded-none border-0 bg-transparent shadow-none ring-0 focus-visible:ring-0 disabled:bg-transparent aria-invalid:ring-0 dark:bg-transparent dark:disabled:bg-transparent",
        props.class,
    )}
/>
