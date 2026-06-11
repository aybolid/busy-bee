<script>
    import Accordion from "$lib/components/ui/accordion/accordion.svelte";
    import AccordionItem from "$lib/components/ui/accordion/accordion-item.svelte";
    import AccordionHeader from "$lib/components/ui/accordion/accordion-header.svelte";
    import AccordionTrigger from "$lib/components/ui/accordion/accordion-trigger.svelte";
    import AccordionContent from "$lib/components/ui/accordion/accordion-content.svelte";
    import AccordionChevron from "$lib/components/ui/accordion/accordion-chevron.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import Plus from "$lib/components/ui/icons/plus.svelte";
    import { createQuery } from "@tanstack/svelte-query";
    import { getSystemPromptsQueryOptions } from "$lib/query/prompts";
    import Pending from "$lib/components/pending.svelte";
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import TableContainer from "$lib/components/ui/table/table-container.svelte";
    import Table from "$lib/components/ui/table/table.svelte";
    import TableHeader from "$lib/components/ui/table/table-header.svelte";
    import TableRow from "$lib/components/ui/table/table-row.svelte";
    import TableHead from "$lib/components/ui/table/table-head.svelte";
    import TableBody from "$lib/components/ui/table/table-body.svelte";
    import TableCell from "$lib/components/ui/table/table-cell.svelte";
    import Empty from "$lib/components/ui/empty/empty.svelte";
    import EmptyHeader from "$lib/components/ui/empty/empty-header.svelte";
    import EmptyTitle from "$lib/components/ui/empty/empty-title.svelte";
    import EmptyDescription from "$lib/components/ui/empty/empty-description.svelte";
    import SvelteMarkdown from "@humanspeak/svelte-markdown";
    import Badge from "$lib/components/ui/badge.svelte";
    import dayjs from "dayjs";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const systemPrompts = createQuery(() => getSystemPromptsQueryOptions(props.data.ky));
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
        <AccordionContent class="p-4 space-y-4">
            <div class="flex justify-end">
                <Action anchor href="/prompts/new/system">
                    <Plus />
                    <span>System prompt</span>
                </Action>
            </div>

            <TableContainer>
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>Title</TableHead>
                            <TableHead>Text</TableHead>
                            <TableHead>Created</TableHead>
                            <TableHead>Updated</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {@const colspan = 4}
                        {#if systemPrompts.isPending}
                            <TableRow>
                                <TableCell {colspan}>
                                    <Pending />
                                </TableCell>
                            </TableRow>
                        {:else if systemPrompts.isError}
                            <TableRow>
                                <TableCell {colspan}>
                                    <ErrorAlert error={systemPrompts.error} />
                                </TableCell>
                            </TableRow>
                        {:else if systemPrompts.isSuccess}
                            {#if systemPrompts.data.length === 0}
                                <TableRow>
                                    <TableCell {colspan}>
                                        <Empty>
                                            <EmptyHeader>
                                                <EmptyTitle>No system prompts</EmptyTitle>
                                                <EmptyDescription>
                                                    There are no outputs to display.
                                                </EmptyDescription>
                                            </EmptyHeader>
                                        </Empty>
                                    </TableCell>
                                </TableRow>
                            {/if}

                            {#each systemPrompts.data as prompt (prompt.id)}
                                <TableRow>
                                    <TableCell>
                                        <p class="line-clamp-2 w-72 text-wrap whitespace-normal">
                                            {prompt.text}
                                        </p>
                                    </TableCell>
                                    <TableCell>
                                        {@const source = prompt.text.slice(0, 250)}
                                        <p
                                            class="line-clamp-2 w-96 text-xs text-wrap whitespace-normal"
                                        >
                                            <SvelteMarkdown {source} isInline />
                                        </p>
                                    </TableCell>
                                    <TableCell>
                                        <Badge variant="secondary">
                                            {dayjs(prompt.created_at).format("MMM DD, YYYY, HH:mm")}
                                        </Badge>
                                    </TableCell>
                                    <TableCell>
                                        <Badge
                                            variant={prompt.created_at.toISOString() ===
                                            prompt.updated_at.toISOString()
                                                ? "ghost"
                                                : "secondary"}
                                        >
                                            {dayjs(prompt.updated_at).format("MMM DD, YYYY, HH:mm")}
                                        </Badge>
                                    </TableCell>
                                </TableRow>
                            {/each}
                        {/if}
                    </TableBody>
                </Table>
            </TableContainer>
        </AccordionContent>
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
