<script>
    import ChevronDown from "../icons/chevron-down.svelte";
    import { cn } from "../utils";
    import { getAccordionContext } from "./context";
    import { getAccordionItemContext } from "./item-context";

    const { id } = getAccordionItemContext();
    const accordion = getAccordionContext();

    const isOpen = $derived.by(() => accordion.openIds.includes(id));

    /** @type {import('svelte/elements').SVGAttributes<SVGElement>} */
    const props = $props();
</script>

<ChevronDown
    data-slot="accordion-trigger-icon"
    {...props}
    class={cn(
        "pointer-events-none shrink-0 transition-transform",
        isOpen ? "rotate-0" : "rotate-180",
        props.class,
    )}
/>
