<script>
    import Accordion from "$lib/components/ui/accordion/accordion.svelte";
    import AccordionItem from "$lib/components/ui/accordion/accordion-item.svelte";
    import AccordionHeader from "$lib/components/ui/accordion/accordion-header.svelte";
    import AccordionTrigger from "$lib/components/ui/accordion/accordion-trigger.svelte";
    import AccordionContent from "$lib/components/ui/accordion/accordion-content.svelte";
    import AccordionChevron from "$lib/components/ui/accordion/accordion-chevron.svelte";

    // /** @type {import('./$types').PageProps} */
    // const props = $props();
</script>

<div class="flex items-baseline justify-between gap-8 pb-8">
    <h1 class="text-4xl font-bold">Prompts</h1>
</div>

<Accordion class="pt-8">
    <AccordionItem>
        <AccordionHeader>
            <AccordionTrigger class="px-4">
                <span>System prompts</span>
                <AccordionChevron />
            </AccordionTrigger>
        </AccordionHeader>
        <AccordionContent class="p-4"></AccordionContent>
    </AccordionItem>
    <AccordionItem>
        <AccordionHeader>
            <AccordionTrigger class="px-4">
                <span>Instruction prompts</span>
                <AccordionChevron />
            </AccordionTrigger>
        </AccordionHeader>
        <AccordionContent class="p-4"></AccordionContent>
    </AccordionItem>
</Accordion>
