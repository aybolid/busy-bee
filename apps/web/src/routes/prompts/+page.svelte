<script>
    import Action from "$lib/components/ui/action.svelte";
    import Plus from "$lib/components/ui/icons/plus.svelte";
    import { createQuery } from "@tanstack/svelte-query";
    import {
        getInstructionPromptsQueryOptions,
        getSystemPromptsQueryOptions,
    } from "$lib/query/prompts";
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
    import SystemPromptActionsMenu from "$lib/components/system-prompt-actions-menu.svelte";
    import EllipsisVertical from "$lib/components/ui/icons/ellipsis-vertical.svelte";
    import EmptyContent from "$lib/components/ui/empty/empty-content.svelte";
    import CreateInstructionPromptFormDialog from "$lib/components/create-instruction-prompt-form-dialog.svelte";
    import InstructionPromptActionsMenu from "$lib/components/instruction-prompt-actions-menu.svelte";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const systemPrompts = createQuery(() => getSystemPromptsQueryOptions(props.data.ky));
    const instructionPrompts = createQuery(() => getInstructionPromptsQueryOptions(props.data.ky));
</script>

<div class="flex items-baseline justify-between gap-8 pb-8">
    <h1 class="text-4xl font-bold">Prompts</h1>
</div>

<div class="flex justify-between items-baseline gap-4 pb-8">
    <h2 class="text-2xl font-semibold">System</h2>
    {#if systemPrompts.isSuccess && systemPrompts.data.length > 0}
        <Action anchor href="/prompts/system/new">
            <Plus />
            <span>System prompt</span>
        </Action>
    {/if}
</div>

<TableContainer class="mb-8">
    <Table>
        <TableHeader>
            <TableRow>
                <TableHead>Title</TableHead>
                <TableHead>Text</TableHead>
                <TableHead>Created</TableHead>
                <TableHead>Updated</TableHead>
                <TableHead>Version</TableHead>
                <TableHead class="sticky right-0 bg-muted/80 backdrop-blur-xs">
                    <!-- Actions -->
                </TableHead>
            </TableRow>
        </TableHeader>
        <TableBody>
            {@const colspan = 6}
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
                                        There are no prompts to display.
                                    </EmptyDescription>
                                </EmptyHeader>
                                <EmptyContent>
                                    <Action anchor href="/prompts/system/new">
                                        <Plus />
                                        <span>System prompt</span>
                                    </Action>
                                </EmptyContent>
                            </Empty>
                        </TableCell>
                    </TableRow>
                {/if}

                {#each systemPrompts.data as prompt (prompt.id)}
                    <TableRow>
                        <TableCell>
                            <Action anchor href="/prompts/system/{prompt.id}" variant="link">
                                <span class="max-w-48 truncate">
                                    {prompt.title}
                                </span>
                            </Action>
                        </TableCell>
                        <TableCell>
                            {@const source = prompt.text.slice(0, 250)}
                            <p class="line-clamp-2 w-96 text-xs text-wrap whitespace-normal">
                                <SvelteMarkdown {source} isInline />
                            </p>
                        </TableCell>
                        <TableCell>
                            <Badge variant="secondary">
                                {prompt.formattedCreatedAt()}
                            </Badge>
                        </TableCell>
                        <TableCell>
                            <Badge
                                variant={prompt.created_at.toISOString() ===
                                prompt.updated_at.toISOString()
                                    ? "ghost"
                                    : "secondary"}
                            >
                                {prompt.formattedUpdatedAt()}
                            </Badge>
                        </TableCell>
                        <TableCell>
                            <Badge>
                                {prompt.formattedVersion()}
                            </Badge>
                        </TableCell>
                        <TableCell class="sticky right-0 bg-background/80 backdrop-blur-xs">
                            <SystemPromptActionsMenu systemPrompt={prompt}>
                                {#snippet trigger(props)}
                                    <Action button size="icon-sm" variant="outline" {...props}>
                                        <EllipsisVertical />
                                        <span class="sr-only">Prompt actions</span>
                                    </Action>
                                {/snippet}
                            </SystemPromptActionsMenu>
                        </TableCell>
                    </TableRow>
                {/each}
            {/if}
        </TableBody>
    </Table>
</TableContainer>

<div class="flex justify-between items-baseline gap-4 pb-8">
    <h2 class="text-2xl font-semibold">Instruction</h2>
    {#if instructionPrompts.isSuccess && instructionPrompts.data.length > 0}
        <CreateInstructionPromptFormDialog>
            {#snippet trigger(props)}
                <Action button {...props}>
                    <Plus />
                    <span>Instruction prompt</span>
                </Action>
            {/snippet}
        </CreateInstructionPromptFormDialog>
    {/if}
</div>

<TableContainer class="mb-8">
    <Table>
        <TableHeader>
            <TableRow>
                <TableHead>Title</TableHead>
                <TableHead>Text</TableHead>
                <TableHead>Created</TableHead>
                <TableHead>Updated</TableHead>
                <TableHead>Version</TableHead>
                <TableHead class="sticky right-0 bg-muted/80 backdrop-blur-xs">
                    <!-- Actions -->
                </TableHead>
            </TableRow>
        </TableHeader>
        <TableBody>
            {@const colspan = 6}
            {#if instructionPrompts.isPending}
                <TableRow>
                    <TableCell {colspan}>
                        <Pending />
                    </TableCell>
                </TableRow>
            {:else if instructionPrompts.isError}
                <TableRow>
                    <TableCell {colspan}>
                        <ErrorAlert error={systemPrompts.error} />
                    </TableCell>
                </TableRow>
            {:else if instructionPrompts.isSuccess}
                {#if instructionPrompts.data.length === 0}
                    <TableRow>
                        <TableCell {colspan}>
                            <Empty>
                                <EmptyHeader>
                                    <EmptyTitle>No instruction prompts</EmptyTitle>
                                    <EmptyDescription>
                                        There are no prompts to display.
                                    </EmptyDescription>
                                </EmptyHeader>
                                <EmptyContent>
                                    <CreateInstructionPromptFormDialog>
                                        {#snippet trigger(props)}
                                            <Action button {...props}>
                                                <Plus />
                                                <span>Instruction prompt</span>
                                            </Action>
                                        {/snippet}
                                    </CreateInstructionPromptFormDialog>
                                </EmptyContent>
                            </Empty>
                        </TableCell>
                    </TableRow>
                {/if}

                {#each instructionPrompts.data as prompt (prompt.id)}
                    <TableRow>
                        <TableCell>
                            <span class="max-w-48 truncate font-medium">
                                {prompt.title}
                            </span>
                        </TableCell>
                        <TableCell>
                            <p class="line-clamp-2 w-96 text-xs text-wrap whitespace-normal">
                                {prompt.text}
                            </p>
                        </TableCell>
                        <TableCell>
                            <Badge variant="secondary">
                                {prompt.formattedCreatedAt()}
                            </Badge>
                        </TableCell>
                        <TableCell>
                            <Badge
                                variant={prompt.created_at.toISOString() ===
                                prompt.updated_at.toISOString()
                                    ? "ghost"
                                    : "secondary"}
                            >
                                {prompt.formattedUpdatedAt()}
                            </Badge>
                        </TableCell>
                        <TableCell>
                            <Badge>
                                {prompt.formattedVersion()}
                            </Badge>
                        </TableCell>
                        <TableCell class="sticky right-0 bg-background/80 backdrop-blur-xs">
                            <InstructionPromptActionsMenu instructionPrompt={prompt}>
                                {#snippet trigger(props)}
                                    <Action button size="icon-sm" variant="outline" {...props}>
                                        <EllipsisVertical />
                                        <span class="sr-only">Prompt actions</span>
                                    </Action>
                                {/snippet}
                            </InstructionPromptActionsMenu>
                        </TableCell>
                    </TableRow>
                {/each}
            {/if}
        </TableBody>
    </Table>
</TableContainer>
