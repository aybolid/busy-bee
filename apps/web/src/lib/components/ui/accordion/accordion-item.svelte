<script>
    import { cn } from "../utils";
    import { setAccordionItemContext } from "./item-context";

    const id = $props.id();
    setAccordionItemContext({ id });

    /** @type {import('svelte/elements').HTMLAttributes<HTMLDivElement>} */
    const { children, ...props } = $props();
</script>

<div {...props} data-slot="accordion-item" class={cn("not-last:border-b", props.class)}>
    {@render children?.()}
</div>
