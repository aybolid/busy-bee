<script>
    import ArrowLeft from "../icons/arrow-left.svelte";
    import { cn } from "../utils";
    import PaginationAction from "./pagination-action.svelte";

    /** @type {import('./pagination-action.svelte').PaginationActionProps & { text?: string }} */
    let { children, text = "Previous", ref = $bindable(), ...props } = $props();
</script>

<PaginationAction
    bind:ref
    aria-label="Go to previous page"
    size="default"
    {...props}
    class={cn("pl-1.5", props.class)}
>
    <ArrowLeft />
    <span class="hidden sm:block">{text}</span>
</PaginationAction>
