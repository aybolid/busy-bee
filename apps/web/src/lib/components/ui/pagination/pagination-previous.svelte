<script>
    import PaginationAction from "./pagination-action.svelte";

    /** @type {import('./pagination-action.svelte').PaginationActionProps & { text?: string }} */
    const { children, text = "Previous", ...props } = $props();
</script>

<PaginationAction
    aria-label="Go to previous page"
    size="default"
    {...props}
    class={["pl-1.5!", props.class]}
>
    <span>{"<"}</span>
    <span class="hidden sm:block">{text}</span>
</PaginationAction>
