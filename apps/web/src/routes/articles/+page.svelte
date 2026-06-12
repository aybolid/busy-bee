<script>
    import ArticleStatus from "$lib/components/article-status.svelte";
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import PaginationControls from "$lib/components/pagination-controls.svelte";
    import Pending from "$lib/components/pending.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import Badge from "$lib/components/ui/badge.svelte";
    import EmptyDescription from "$lib/components/ui/empty/empty-description.svelte";
    import EmptyHeader from "$lib/components/ui/empty/empty-header.svelte";
    import EmptyTitle from "$lib/components/ui/empty/empty-title.svelte";
    import Empty from "$lib/components/ui/empty/empty.svelte";
    import EllipsisVertical from "$lib/components/ui/icons/ellipsis-vertical.svelte";
    import ExternalLink from "$lib/components/ui/icons/external-link.svelte";
    import Refresh from "$lib/components/ui/icons/refresh.svelte";
    import ProgressIndicator from "$lib/components/ui/progress/progress-indicator.svelte";
    import Progress from "$lib/components/ui/progress/progress.svelte";
    import Skeleton from "$lib/components/ui/skeleton.svelte";
    import Spinner from "$lib/components/ui/spinner.svelte";
    import TableBody from "$lib/components/ui/table/table-body.svelte";
    import TableCell from "$lib/components/ui/table/table-cell.svelte";
    import TableHead from "$lib/components/ui/table/table-head.svelte";
    import TableHeader from "$lib/components/ui/table/table-header.svelte";
    import TableRow from "$lib/components/ui/table/table-row.svelte";
    import Table from "$lib/components/ui/table/table.svelte";
    import {
        getArticlesQueryOptions,
        getArticleStatsQueryOptions,
        invalidateArticlesQueries,
    } from "$lib/query/articles";
    import { createQuery } from "@tanstack/svelte-query";
    import dayjs from "dayjs";
    import relative from "dayjs/plugin/relativeTime";
    import Lock from "$lib/components/ui/icons/lock.svelte";
    import StickyBar from "$lib/components/ui/sticky-bar.svelte";
    import TableContainer from "$lib/components/ui/table/table-container.svelte";
    import NativeSelect from "$lib/components/ui/native-select/native-select.svelte";
    import NativeSelectOption from "$lib/components/ui/native-select/native-select-option.svelte";
    import { page } from "$app/state";
    import { goto } from "$app/navigation";
    import { dev } from "$app/environment";
    import NativeSelectOptGroup from "$lib/components/ui/native-select/native-select-opt-group.svelte";
    import Popover from "$lib/components/ui/popover/popover.svelte";
    import PopoverContent from "$lib/components/ui/popover/popover-content.svelte";
    import { getRssFeedsQueryOptions } from "$lib/query/rss-feeds";
    import { toaster } from "$lib/components/toaster/store";
    import ArticleActionsMenu from "$lib/components/article-actions-menu.svelte";
    import Trash from "$lib/components/ui/icons/trash.svelte";
    import BulkDeleteArticlesDialog from "$lib/components/bulk-delete-articles-dialog.svelte";

    dayjs.extend(relative);

    /** @type {import('./$types').PageProps} */
    const props = $props();

    /** @type {import('$lib/api/articles').GetArticlesSearchParams} */
    const getArticlesSearchParams = $derived({
        page_index: props.data.searchParams.page_index,
        limit: props.data.searchParams.limit,
    });

    const articlesQueryOptions = $derived(
        getArticlesQueryOptions(props.data.ky, { searchParams: getArticlesSearchParams }),
    );

    const articles = createQuery(() => articlesQueryOptions);
    const articleStats = createQuery(() => getArticleStatsQueryOptions(props.data.ky));

    const feeds = createQuery(() => getRssFeedsQueryOptions(props.data.ky));

    $effect(() => {
        if (feeds.error) {
            toaster.push("Failed to fetch RSS feeds", {
                description: feeds.error.message,
                props: { variant: "destructive" },
            });
        }
    });

    const feedsMap = $derived.by(() => {
        const data = feeds.data ?? [];

        return data.reduce(
            (acc, feed) => {
                acc[feed.id] = feed;
                return acc;
            },
            /** @type {Record<import('$lib/api/rss-feeds').RssFeedId, import('$lib/api/rss-feeds').RssFeed>} */
            ({}),
        );
    });

    let lastUpdatedString = $state("Last updated a few seconds ago");

    $effect(() => {
        if (articles.dataUpdatedAt) {
            lastUpdatedString = `Last updated ${dayjs(articles.dataUpdatedAt).fromNow()}`;
        } else {
            lastUpdatedString = "Last updated a few seconds ago";
        }

        const interval = setInterval(() => {
            lastUpdatedString = `Last updated ${dayjs(articles.dataUpdatedAt).fromNow()}`;
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

        void invalidateArticlesQueries(props.data.queryClient);
        void articleStats.refetch();

        refreshTimeout = setTimeout(() => (canRefresh = true), 5000);
    }

    $effect(() => {
        return () => {
            clearTimeout(refreshTimeout);
        };
    });

    let selection = /** @type {Array<import('$lib/api/articles').ArticleId>} */ ($state([]));

    const allSelected = $derived.by(() => {
        if (!articles.data?.data.length || articles.data.data.length === 0) {
            return false;
        }
        return articles.data.data.length === selection.length;
    });

    /** @type {import('svelte/elements').ChangeEventHandler<HTMLInputElement>} */
    function toggleAllSelection(e) {
        if (e.currentTarget.checked) {
            // Actions are not allowed for pending articles
            selection =
                articles.data?.data.filter((a) => a.status !== "pending").map((a) => a.id) ?? [];
        } else {
            selection = [];
        }
    }

    $effect(() => {
        articles.dataUpdatedAt;
        selection = [];
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
    <h1 class="text-4xl font-bold">Articles</h1>
    <div class="flex flex-col items-end gap-1">
        <Action
            button
            variant="secondary"
            disabled={!canRefresh || articles.isFetching || articleStats.isFetching}
            onclick={refresh}
        >
            {#if articles.isFetching || articleStats.isFetching}
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

<div class="py-8">
    {#if articleStats.isPending}
        <div class="grid grid-cols-2 gap-4 sm:grid-cols-4">
            <Skeleton class="h-15" />
            <Skeleton class="h-15" />
            <Skeleton class="h-15" />
            <Skeleton class="h-15" />
        </div>
    {:else if articleStats.isError}
        <ErrorAlert error={articleStats.error} />
    {:else if articleStats.isSuccess}
        {@const total = articleStats.data.total}
        {#snippet li(
            /** @type {import('$lib/api/articles').ArticleStatus} */ status,
            /** @type {number} */ count,
            /** @type {string} */ formattedCount,
            /** @type {string} */ description,
        )}
            {@const value = (count / total) * 100}
            <li>
                <div class="flex items-center justify-between gap-2 pb-2">
                    <ArticleStatus {status} />
                    <div class="font-semibold text-nowrap tabular-nums">
                        <span>
                            {formattedCount}
                        </span>
                        <span class="text-xs font-normal text-muted-foreground">/{total}</span>
                    </div>
                </div>
                <Progress>
                    <ProgressIndicator {value} class={[status === "error" && "bg-destructive"]} />
                </Progress>
                <div class="pt-2 text-xs text-muted-foreground">
                    {description}
                </div>
            </li>
        {/snippet}

        <ul class="grid grid-cols-2 gap-4 sm:grid-cols-4">
            {@render li(
                "new",
                articleStats.data.new,
                articleStats.data.formattedNew(),
                "Newly added articles",
            )}
            {@render li(
                "pending",
                articleStats.data.pending,
                articleStats.data.formattedPending(),
                "Articles that are being processed",
            )}
            {@render li(
                "processed",
                articleStats.data.processed,
                articleStats.data.formattedProcessed(),
                "Articles that were processed at least once",
            )}
            {@render li(
                "error",
                articleStats.data.error,
                articleStats.data.formattedError(),
                "Something went wrong during latest processing",
            )}
        </ul>
    {/if}
</div>

{#if selection.length !== 0}
    <StickyBar position="top" class="mb-4 w-full justify-between" transition>
        <Badge variant="ghost">
            {selection.length} selected
        </Badge>
        <div class="flex items-center gap-2">
            <BulkDeleteArticlesDialog
                articleIds={selection}
                onSuccess={() => {
                    selection = [];
                }}
            >
                {#snippet trigger(props)}
                    <Action button size="xs" variant="destructive" {...props}>
                        <Trash />
                        <span>Delete</span>
                    </Action>
                {/snippet}
            </BulkDeleteArticlesDialog>
            <Action button size="xs">
                <span>Process</span>
            </Action>
        </div>
    </StickyBar>
{/if}

<TableContainer class="mb-8">
    <Table>
        <TableHeader>
            <TableRow>
                <TableHead class="sticky left-0 bg-muted/80 backdrop-blur-xs">
                    <div class="grid place-items-center w-6">
                        <input
                            type="checkbox"
                            checked={allSelected}
                            onchange={toggleAllSelection}
                        />
                    </div>
                </TableHead>
                <TableHead>Title</TableHead>
                <TableHead>Description</TableHead>
                <TableHead>Feed</TableHead>
                <TableHead>Author</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Published</TableHead>
                <TableHead>Created</TableHead>
                <TableHead>Updated</TableHead>
                <TableHead>External</TableHead>
                <TableHead class="sticky right-0 bg-muted/80 backdrop-blur-xs">
                    <!-- Actions -->
                </TableHead>
            </TableRow>
        </TableHeader>
        <TableBody>
            {@const colspan = 11}

            {#if articles.isPending}
                <TableRow>
                    <TableCell {colspan}>
                        <Pending />
                    </TableCell>
                </TableRow>
            {:else if articles.isError}
                <TableRow>
                    <TableCell {colspan}>
                        <ErrorAlert error={articles.error} />
                    </TableCell>
                </TableRow>
            {:else if articles.isSuccess}
                {#if articles.data.data.length === 0}
                    <TableRow>
                        <TableCell {colspan}>
                            <Empty>
                                <EmptyHeader>
                                    <EmptyTitle>No articles</EmptyTitle>
                                    <EmptyDescription>
                                        There are no articles to display.
                                    </EmptyDescription>
                                </EmptyHeader>
                            </Empty>
                        </TableCell>
                    </TableRow>
                {/if}

                {#each articles.data.data as article (article.id)}
                    {@const feed = feedsMap[article.rss_feed_id]}
                    {@const feedUrl = feed?.parsedUrl()}

                    <TableRow>
                        <TableCell class="sticky left-0 bg-background/80 backdrop-blur-xs">
                            <div class="grid place-items-center">
                                <input
                                    type="checkbox"
                                    bind:group={selection}
                                    value={article.id}
                                    disabled={article.status === "pending"}
                                />
                            </div>
                        </TableCell>
                        <TableCell>
                            <Action anchor href="/articles/{article.id}" variant="link">
                                {#if article.favicon}
                                    <div class="size-4">
                                        <img src={article.favicon} alt="favicon" loading="lazy" />
                                    </div>
                                {/if}
                                <span class="max-w-80 truncate">
                                    {article.title}
                                </span>
                            </Action>
                        </TableCell>
                        <TableCell>
                            <p
                                class="line-clamp-2 w-96 text-xs text-wrap whitespace-normal text-muted-foreground"
                            >
                                {article.excerpt || "--"}
                            </p>
                        </TableCell>
                        <TableCell>
                            {#if feeds.isPending || !feedUrl}
                                Unknown
                            {:else}
                                {feedUrl?.hostname}
                            {/if}
                        </TableCell>
                        <TableCell>
                            {article.byline || "--"}
                        </TableCell>
                        <TableCell>
                            {#if article.status === "error"}
                                <Popover>
                                    {#snippet trigger(props)}
                                        <button {...props}>
                                            <ArticleStatus status="error" />
                                        </button>
                                    {/snippet}
                                    <PopoverContent class="w-72">
                                        <ErrorAlert
                                            title="Processing error"
                                            description={article.error_reason}
                                        />
                                    </PopoverContent>
                                </Popover>
                            {:else}
                                <ArticleStatus status={article.status} />
                            {/if}
                        </TableCell>
                        <TableCell>
                            {#if article.published_time}
                                <Badge variant="secondary">
                                    {article.formattedPublishedTime()}
                                </Badge>
                            {:else}
                                <span class="text-muted-foreground">--</span>
                            {/if}
                        </TableCell>
                        <TableCell>
                            <Badge variant="secondary">
                                {article.formattedCreatedAt()}
                            </Badge>
                        </TableCell>
                        <TableCell>
                            <Badge
                                variant={article.created_at.toISOString() ===
                                article.updated_at.toISOString()
                                    ? "ghost"
                                    : "secondary"}
                            >
                                {article.formattedUpdatedAt()}
                            </Badge>
                        </TableCell>
                        <TableCell>
                            <Action
                                anchor
                                href={article.url}
                                target="_blank"
                                variant="link"
                                class="text-muted-foreground"
                            >
                                <ExternalLink />
                                <span class="max-w-56 truncate">
                                    {article.url}
                                </span>
                            </Action>
                        </TableCell>
                        <TableCell class="sticky right-0 bg-background/80 backdrop-blur-xs">
                            <ArticleActionsMenu {article}>
                                {#snippet trigger(props)}
                                    <Action button size="icon-sm" variant="outline" {...props}>
                                        <EllipsisVertical />
                                        <span class="sr-only">Article actions</span>
                                    </Action>
                                {/snippet}
                            </ArticleActionsMenu>
                        </TableCell>
                    </TableRow>
                {/each}
            {/if}
        </TableBody>
    </Table>
</TableContainer>

{#if articles.isSuccess && articles.data.meta.total > 0}
    <StickyBar>
        <PaginationControls
            class="justify-start"
            url={page.url}
            pageIndex={getArticlesSearchParams.page_index}
            totalPages={articles.data.meta.total_pages}
            buildSearchParams={(pageIndex) => getUpdatedSearchParams({ pageIndex })}
        />
        <NativeSelect value={getArticlesSearchParams.limit} onchange={handlePageSizeChange}>
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
