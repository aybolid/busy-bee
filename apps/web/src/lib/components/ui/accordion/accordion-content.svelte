<script>
    import { cn } from "../utils";
    import { getAccordionContext } from "./context";
    import { getAccordionItemContext } from "./item-context";
    import { slide } from "svelte/transition";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();

    const accordion = getAccordionContext();
    const { id } = getAccordionItemContext();

    const isOpen = $derived(accordion.openIds.includes(id));
</script>

{#if isOpen}
    <div
        {...props}
        data-slot="accordion-content"
        transition:slide={{ duration: 100 }}
        class={cn("overflow-hidden pt-0 pb-2.5 text-sm [&_p:not(:last-child)]:mb-4", props.class)}
    >
        {@render children?.()}
    </div>
{/if}
