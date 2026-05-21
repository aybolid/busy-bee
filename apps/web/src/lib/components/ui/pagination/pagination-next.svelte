<script>
    import ArrowRight from "../icons/arrow-right.svelte";
    import { cn } from "../utils";
    import PaginationAction from "./pagination-action.svelte";

    /** @type {import('./pagination-action.svelte').PaginationActionProps & { text?: string }} */
    let { children, text = "Next", ref = $bindable(), ...props } = $props();
</script>

<PaginationAction
    bind:ref
    aria-label="Go to next page"
    size="default"
    {...props}
    class={cn("pr-1.5", props.class)}
>
    <span class="hidden sm:block">{text}</span>
    <ArrowRight />
</PaginationAction>
