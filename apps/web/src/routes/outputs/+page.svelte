<script>
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import PaginationControls from "$lib/components/pagination-controls.svelte";
    import Pending from "$lib/components/pending.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import EmptyDescription from "$lib/components/ui/empty/empty-description.svelte";
    import EmptyHeader from "$lib/components/ui/empty/empty-header.svelte";
    import EmptyTitle from "$lib/components/ui/empty/empty-title.svelte";
    import Empty from "$lib/components/ui/empty/empty.svelte";
    import EllipsisVertical from "$lib/components/ui/icons/ellipsis-vertical.svelte";
    import Refresh from "$lib/components/ui/icons/refresh.svelte";
    import Spinner from "$lib/components/ui/spinner.svelte";
    import TableBody from "$lib/components/ui/table/table-body.svelte";
    import TableCell from "$lib/components/ui/table/table-cell.svelte";
    import TableHead from "$lib/components/ui/table/table-head.svelte";
    import TableHeader from "$lib/components/ui/table/table-header.svelte";
    import TableRow from "$lib/components/ui/table/table-row.svelte";
    import Table from "$lib/components/ui/table/table.svelte";
    import { createQuery } from "@tanstack/svelte-query";
    import Lock from "$lib/components/ui/icons/lock.svelte";
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import TableContainer from "$lib/components/ui/table/table-container.svelte";
    import NativeSelect from "$lib/components/ui/native-select/native-select.svelte";
    import NativeSelectOption from "$lib/components/ui/native-select/native-select-option.svelte";
    import { page } from "$app/state";
    import { goto } from "$app/navigation";
    import { dev } from "$app/environment";
    import NativeSelectOptGroup from "$lib/components/ui/native-select/native-select-opt-group.svelte";
    import { getOutputsQueryOptions, invalidateOutputsQueries } from "$lib/query/outputs";
    import Badge from "$lib/components/ui/badge.svelte";
    import dayjs from "dayjs";
    import relative from "dayjs/plugin/relativeTime";
    import ViewTokenUsageDialog from "$lib/components/view-token-usage-dialog.svelte";
    import OutputActionsMenu from "$lib/components/output-actions-menu.svelte";
    import SvelteMarkdown from "@humanspeak/svelte-markdown";

    dayjs.extend(relative);

    /** @type {import('./$types').PageProps} */
    const props = $props();

    /** @type {import('$lib/api/articles').GetArticlesSearchParams} */
    const getArticleProcessingOutputsSearchParams = $derived({
        page_index: props.data.searchParams.page_index,
        limit: props.data.searchParams.limit,
    });

    const outputsQueryOptions = $derived(
        getOutputsQueryOptions(props.data.ky, {
            searchParams: getArticleProcessingOutputsSearchParams,
        }),
    );

    const outputs = createQuery(() => outputsQueryOptions);

    let lastUpdatedString = $state("Last updated a few seconds ago");

    $effect(() => {
        if (outputs.dataUpdatedAt) {
            lastUpdatedString = `Last updated ${dayjs(outputs.dataUpdatedAt).fromNow()}`;
        } else {
            lastUpdatedString = "Last updated a few seconds ago";
        }

        const interval = setInterval(() => {
            lastUpdatedString = `Last updated ${dayjs(outputs.dataUpdatedAt).fromNow()}`;
        }, 1000 * 60);

        return () => {
            clearInterval(interval);
        };
    });

    /** @type {ReturnType<typeof setTimeout>} */
    let refreshTimeout;
    let canRefresh = $state(true);

    function refresh() {
        if (!canRefresh) return;
        canRefresh = false;

        void invalidateOutputsQueries(props.data.queryClient);

        refreshTimeout = setTimeout(() => (canRefresh = true), 5000);
    }

    $effect(() => {
        return () => {
            clearTimeout(refreshTimeout);
        };
    });

    /**
     * @param {{ limit?: number, pageIndex?: number }} params
     *
     * @returns {URLSearchParams}
     */
    function getUpdatedSearchParams({ limit, pageIndex }) {
        const params = new URLSearchParams(page.url.searchParams);
        params.set("limit", limit?.toString() ?? params.get("limit") ?? "20");
        if (limit) {
            // Reset page on limit change
            params.set("page_index", "0");
        } else {
            params.set("page_index", pageIndex?.toString() ?? params.get("page_index") ?? "0");
        }
        return params;
    }

    /** @type {import('svelte/elements').ChangeEventHandler<HTMLSelectElement>} */
    function handlePageSizeChange(event) {
        const value = event.currentTarget.value;
        const params = getUpdatedSearchParams({ limit: Number(value) });
        void goto(`${page.url.pathname}?${params.toString()}`, { keepFocus: true, noScroll: true });
    }
</script>

<div class="flex items-baseline justify-between gap-8">
    <h1 class="text-4xl font-bold">Outputs</h1>
    <div class="flex flex-col items-end gap-1">
        <Action
            button
            variant="secondary"
            disabled={!canRefresh || outputs.isFetching}
            onclick={refresh}
        >
            {#if outputs.isFetching}
                <Spinner />
            {:else if !canRefresh}
                <Lock />
            {:else}
                <Refresh />
            {/if}
            <span>Refresh</span>
        </Action>
        <div class="text-xs text-muted-foreground">
            {lastUpdatedString}
        </div>
    </div>
</div>

<TableContainer class="my-8">
    <Table>
        <TableHeader>
            <TableRow>
                <TableHead>Output</TableHead>
                <TableHead>Context</TableHead>
                <TableHead>Model</TableHead>
                <TableHead>Tokens</TableHead>
                <TableHead>Created</TableHead>
                <TableHead>Updated</TableHead>
                <TableHead class="sticky right-0 bg-muted/80 backdrop-blur-xs">
                    <!-- Actions -->
                </TableHead>
            </TableRow>
        </TableHeader>
        <TableBody>
            {@const colspan = 7}
            {#if outputs.isPending}
                <TableRow>
                    <TableCell {colspan}>
                        <Pending />
                    </TableCell>
                </TableRow>
            {:else if outputs.isError}
                <TableRow>
                    <TableCell {colspan}>
                        <ErrorAlert error={outputs.error} />
                    </TableCell>
                </TableRow>
            {:else if outputs.isSuccess}
                {#if outputs.data.data.length === 0}
                    <TableRow>
                        <TableCell {colspan}>
                            <Empty>
                                <EmptyHeader>
                                    <EmptyTitle>No outputs</EmptyTitle>
                                    <EmptyDescription>
                                        There are no outputs to display.
                                    </EmptyDescription>
                                </EmptyHeader>
                            </Empty>
                        </TableCell>
                    </TableRow>
                {/if}

                {#each outputs.data.data as output (output.id)}
                    <TableRow>
                        <TableCell>
                            {@const source = output.text.slice(0, 250)}
                            <p class="line-clamp-2 w-96 text-xs text-wrap whitespace-normal">
                                <SvelteMarkdown {source} isInline />
                            </p>
                        </TableCell>
                        <TableCell>
                            <p
                                class="line-clamp-2 w-72 text-xs text-wrap whitespace-normal text-muted-foreground"
                            >
                                {output.user_context || "--"}
                            </p>
                        </TableCell>
                        <TableCell>
                            <Badge variant="secondary">
                                {output.model}
                            </Badge>
                        </TableCell>
                        <TableCell>
                            <ViewTokenUsageDialog usage={output.usage} model={output.model}>
                                {#snippet trigger(props)}
                                    <Action button variant="outline" size="xs" {...props}>
                                        Usage
                                    </Action>
                                {/snippet}
                            </ViewTokenUsageDialog>
                        </TableCell>
                        <TableCell>
                            <Badge variant="secondary">
                                {output.formattedCreatedAt()}
                            </Badge>
                        </TableCell>
                        <TableCell>
                            <Badge
                                variant={output.created_at.toISOString() ===
                                output.updated_at.toISOString()
                                    ? "ghost"
                                    : "secondary"}
                            >
                                {output.formattedUpdatedAt()}
                            </Badge>
                        </TableCell>
                        <TableCell class="sticky right-0 bg-background/80 backdrop-blur-xs">
                            <OutputActionsMenu {output}>
                                {#snippet trigger(props)}
                                    <Action button size="icon-sm" variant="outline" {...props}>
                                        <EllipsisVertical />
                                        <span class="sr-only">Article actions</span>
                                    </Action>
                                {/snippet}
                            </OutputActionsMenu>
                        </TableCell>
                    </TableRow>
                {/each}
            {/if}
        </TableBody>
    </Table>
</TableContainer>

{#if outputs.isSuccess && outputs.data.meta.total > 0}
    <StickyBar>
        <PaginationControls
            class="justify-start"
            url={page.url}
            pageIndex={getArticleProcessingOutputsSearchParams.page_index}
            totalPages={outputs.data.meta.total_pages}
            buildSearchParams={(pageIndex) => getUpdatedSearchParams({ pageIndex })}
        />
        <NativeSelect
            value={getArticleProcessingOutputsSearchParams.limit}
            onchange={handlePageSizeChange}
        >
            <NativeSelectOption value={0} disabled>Page size</NativeSelectOption>
            <NativeSelectOption value={10}>10</NativeSelectOption>
            <NativeSelectOption value={20}>20</NativeSelectOption>
            <NativeSelectOption value={40}>40</NativeSelectOption>
            <NativeSelectOption value={50}>50</NativeSelectOption>
            {#if dev}
                <NativeSelectOptGroup label="Dev only">
                    <NativeSelectOption value={255}>255</NativeSelectOption>
                </NativeSelectOptGroup>
            {/if}
        </NativeSelect>
    </StickyBar>
{/if}
