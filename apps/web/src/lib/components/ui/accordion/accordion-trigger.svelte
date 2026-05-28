<script>
    import { cn } from "../utils";
    import { getAccordionContext } from "./context";
    import { getAccordionItemContext } from "./item-context";

    /** @type {import('svelte/elements').HTMLButtonAttributes} */
    const { children, ...props } = $props();

    const accordion = getAccordionContext();
    const { id } = getAccordionItemContext();

    function toggleItem() {
        if (accordion.openIds.includes(id)) {
            accordion.openIds = accordion.openIds.filter((openId) => openId !== id);
        } else {
            accordion.openIds.push(id);
        }
    }
</script>

<button
    type="button"
    {...props}
    onclick={(e) => {
        toggleItem();
        props.onclick?.(e);
    }}
    data-slot="accordion-trigger"
    class={cn(
        "group/accordion-trigger relative flex flex-1 items-start justify-between rounded-lg border border-transparent py-2.5 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:after:border-ring aria-disabled:pointer-events-none aria-disabled:opacity-50 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 **:data-[slot=accordion-trigger-icon]:text-muted-foreground",
        props.class,
    )}
>
    {@render children?.()}
</button>
