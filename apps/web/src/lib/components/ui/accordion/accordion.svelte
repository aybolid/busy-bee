<script>
    import { cn } from "../utils";
    import { setAccordionContext } from "./context";

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();

    /** @type {import('./context').AccordionContext} */
    const accordion = $state({ openIds: [] });

    setAccordionContext(accordion);
</script>

<div {...props} data-slot="accordion" class={cn("flex w-full flex-col", props.class)}>
    {@render children?.()}
</div>
