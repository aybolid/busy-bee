<script module>
    const NUMBER_FORMAT = new Intl.NumberFormat("en-US");
</script>

<script>
    import ArticleStatus from "$lib/components/article-status.svelte";
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import Action from "$lib/components/ui/action.svelte";
    import AlertCloseAction from "$lib/components/ui/alert-dialog/alert-close-action.svelte";
    import AlertContinueAction from "$lib/components/ui/alert-dialog/alert-continue-action.svelte";
    import AlertDialogContent from "$lib/components/ui/alert-dialog/alert-dialog-content.svelte";
    import AlertDialogDescription from "$lib/components/ui/alert-dialog/alert-dialog-description.svelte";
    import AlertDialogFooter from "$lib/components/ui/alert-dialog/alert-dialog-footer.svelte";
    import AlertDialogHeader from "$lib/components/ui/alert-dialog/alert-dialog-header.svelte";
    import AlertDialogTitle from "$lib/components/ui/alert-dialog/alert-dialog-title.svelte";
    import AlertDialog from "$lib/components/ui/alert-dialog/alert-dialog.svelte";
    import Badge from "$lib/components/ui/badge.svelte";
    import EllipsisVertical from "$lib/components/ui/icons/ellipsis-vertical.svelte";
    import ExternalLink from "$lib/components/ui/icons/external-link.svelte";
    import Refresh from "$lib/components/ui/icons/refresh.svelte";
    import Trash from "$lib/components/ui/icons/trash.svelte";
    import MenuActionItem from "$lib/components/ui/menu/menu-action-item.svelte";
    import MenuContent from "$lib/components/ui/menu/menu-content.svelte";
    import MenuGroup from "$lib/components/ui/menu/menu-group.svelte";
    import MenuLabel from "$lib/components/ui/menu/menu-label.svelte";
    import Menu from "$lib/components/ui/menu/menu.svelte";
    import PaginationAction from "$lib/components/ui/pagination/pagination-action.svelte";
    import PaginationContent from "$lib/components/ui/pagination/pagination-content.svelte";
    import PaginationEllipsis from "$lib/components/ui/pagination/pagination-ellipsis.svelte";
    import PaginationItem from "$lib/components/ui/pagination/pagination-item.svelte";
    import PaginationNext from "$lib/components/ui/pagination/pagination-next.svelte";
    import PaginationPrevious from "$lib/components/ui/pagination/pagination-previous.svelte";
    import Pagination from "$lib/components/ui/pagination/pagination.svelte";
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
        createDeleteArticleMutation,
        getArticlesQueryOptions,
        getArticleStatsQueryOptions,
    } from "$lib/query/articles";
    import { createQuery } from "@tanstack/svelte-query";
    import dayjs from "dayjs";

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

    function refresh() {
        void props.data.queryClient.invalidateQueries({
            predicate: (q) => q.queryKey[0] === articlesQueryOptions.queryKey[0],
        });
        void articleStats.refetch();
    }

    const deleteMutation = createDeleteArticleMutation();

    /**
     * @param {import('$lib/api/articles').ArticleId} id Article id to delete
     */
    function deleteArticle(id) {
        deleteMutation.mutate([props.data.ky, { params: { id } }], {
            onError: (err) => alert(err.message),
            onSuccess: () => {
                void props.data.queryClient.invalidateQueries({
                    predicate: (q) => q.queryKey[0] === articlesQueryOptions.queryKey[0],
                });
            },
        });
    }

    const isFirstPage = $derived(getArticlesSearchParams.page_index === 0);
    const isLastPage = $derived(
        getArticlesSearchParams.page_index >= (articles.data?.meta.total_pages ?? 0) - 1,
    );

    const visiblePages = $derived.by(() => {
        const total = articles.data?.meta.total_pages ?? 0;
        const current = getArticlesSearchParams.page_index;

        // If 7 or fewer pages, just show all of them
        if (total <= 7) {
            return Array.from({ length: total }, (_, i) => i);
        }

        // Near the beginning: 1, 2, 3, 4, 5, ..., Last
        if (current <= 3) {
            return [0, 1, 2, 3, 4, "ellipsis-right", total - 1];
        }

        // Near the end: 1, ..., Last-4, Last-3, Last-2, Last-1, Last
        if (current >= total - 4) {
            return [0, "ellipsis-left", total - 5, total - 4, total - 3, total - 2, total - 1];
        }

        // In the middle: 1, ..., Current-1, Current, Current+1, ..., Last
        return [0, "ellipsis-left", current - 1, current, current + 1, "ellipsis-right", total - 1];
    });
</script>

{#if articles.isPending}
    <Pending />
{:else if articles.isError}
    <ErrorAlert error={articles.error} />
{:else if articles.isSuccess}
    <div class="flex justify-end">
        <Action button variant="secondary" onclick={refresh}>
            {#if articles.isFetching}
                <Spinner />
            {:else}
                <Refresh />
            {/if}
            <span>Refresh</span>
        </Action>
    </div>

    <div class="py-8">
        {#if articleStats.isPending}
            <div class="grid h-9 grid-cols-4 gap-4">
                <Skeleton class="h-full" />
                <Skeleton />
                <Skeleton />
                <Skeleton />
            </div>
        {:else if articleStats.isError}
            <ErrorAlert error={articleStats.error} />
        {:else if articleStats.isSuccess}
            {@const total = articleStats.data.total}
            {#snippet statusCountLi(
                /** @type {import('$lib/api/articles').ArticleStatus} */ status,
                /** @type {number} */ count,
            )}
                {@const value = (count / total) * 100}
                <li>
                    <div class="flex items-center justify-between gap-2 pb-2">
                        <ArticleStatus {status} />
                        <div class="font-semibold tabular-nums">
                            <span>
                                {NUMBER_FORMAT.format(count)}
                            </span>
                            <span class="text-xs font-normal text-muted-foreground">/{total}</span>
                        </div>
                    </div>
                    <Progress>
                        <ProgressIndicator
                            {value}
                            class={[status === "error" && "bg-destructive"]}
                        />
                    </Progress>
                </li>
            {/snippet}

            <ul class="grid grid-cols-4 gap-4">
                {@render statusCountLi("new", articleStats.data.new)}
                {@render statusCountLi("pending", articleStats.data.pending)}
                {@render statusCountLi("processed", articleStats.data.processed)}
                {@render statusCountLi("error", articleStats.data.error)}
            </ul>
        {/if}
    </div>

    <Table>
        <TableHeader>
            <TableRow>
                <TableHead>Title</TableHead>
                <TableHead>Description</TableHead>
                <TableHead>Author</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Published</TableHead>
                <TableHead>Created</TableHead>
                <TableHead>Updated</TableHead>
                <TableHead class="sticky right-0 bg-background/80 backdrop-blur-xs">
                    <!-- Actions -->
                </TableHead>
            </TableRow>
        </TableHeader>
        <TableBody>
            {#each articles.data.data as article (article.id)}
                <TableRow
                    class={[
                        "group",
                        article.status === "error" && "bg-destructive/10 hover:bg-destructive/15",
                    ]}
                >
                    <TableCell class="max-w-80 truncate font-medium">
                        <a href="/articles/{article.id}" class="hover:underline">
                            {article.title}
                        </a>
                    </TableCell>
                    <TableCell class="text-xs whitespace-normal text-muted-foreground">
                        <p class="line-clamp-2 w-96 text-wrap">
                            {article.excerpt ?? "--"}
                        </p>
                    </TableCell>
                    <TableCell>
                        {article.byline ?? "--"}
                    </TableCell>
                    <TableCell>
                        <ArticleStatus status={article.status} />
                    </TableCell>
                    <TableCell>
                        {#if article.published_time}
                            <Badge variant="secondary">
                                {dayjs(article.published_time).format("MMM DD, YYYY, HH:mm")}
                            </Badge>
                        {:else}
                            <span class="text-muted-foreground">--</span>
                        {/if}
                    </TableCell>
                    <TableCell>
                        <Badge variant="secondary">
                            {dayjs(article.created_at).format("MMM DD, YYYY, HH:mm")}
                        </Badge>
                    </TableCell>
                    <TableCell>
                        <Badge
                            variant={article.created_at.toISOString() ===
                            article.updated_at.toISOString()
                                ? "ghost"
                                : "secondary"}
                        >
                            {dayjs(article.updated_at).format("MMM DD, YYYY, HH:mm")}
                        </Badge>
                    </TableCell>
                    <TableCell class="sticky right-0 bg-background/80 backdrop-blur-xs">
                        {@render articleMenu(article)}
                    </TableCell>
                </TableRow>
            {/each}
        </TableBody>
    </Table>

    <Pagination class="pt-8">
        <PaginationContent>
            <PaginationItem>
                <PaginationPrevious
                    anchor
                    data-sveltekit-noscroll
                    class={[isFirstPage && "pointer-events-none opacity-50"]}
                    aria-disabled={isFirstPage}
                    href="/?page_index={!isFirstPage ? getArticlesSearchParams.page_index - 1 : 0}"
                />
            </PaginationItem>

            {#each visiblePages as page (page)}
                {#if typeof page === "string" && page.startsWith("ellipsis")}
                    <PaginationItem>
                        <PaginationEllipsis />
                    </PaginationItem>
                {:else}
                    <PaginationItem>
                        <PaginationAction
                            anchor
                            data-sveltekit-noscroll
                            href="/?page_index={page}"
                            isActive={page === getArticlesSearchParams.page_index}
                        >
                            {Number(page) + 1}
                        </PaginationAction>
                    </PaginationItem>
                {/if}
            {/each}

            <PaginationItem>
                <PaginationNext
                    anchor
                    data-sveltekit-noscroll
                    class={[isLastPage && "pointer-events-none opacity-50"]}
                    aria-disabled={isLastPage}
                    href="/?page_index={!isLastPage
                        ? getArticlesSearchParams.page_index + 1
                        : getArticlesSearchParams.page_index}"
                />
            </PaginationItem>
        </PaginationContent>
    </Pagination>
{/if}

{#snippet articleMenu(/** @type {import('$lib/api/articles').Article} */ article)}
    <Menu>
        {#snippet trigger(props)}
            <Action button size="icon-sm" variant="outline" {...props}>
                <EllipsisVertical />
                <span class="sr-only">Article actions</span>
            </Action>
        {/snippet}
        <MenuContent>
            <MenuGroup>
                <MenuLabel>Article actions</MenuLabel>
                <MenuActionItem anchor href="/articles/{article.id}">View</MenuActionItem>
                {#if article.url && article.url.startsWith("http")}
                    <MenuActionItem anchor href={article.url} target="_blank">
                        <ExternalLink />
                        <span>View external</span>
                    </MenuActionItem>
                {/if}
                {#if article.status === "new" || article.status === "error"}
                    <MenuActionItem button>Create post</MenuActionItem>
                {/if}
                <AlertDialog>
                    {#snippet trigger(props)}
                        <MenuActionItem
                            button
                            variant="destructive"
                            disabled={deleteMutation.isPending}
                            {...props}
                        >
                            {#if deleteMutation.isPending}
                                <Spinner />
                            {:else}
                                <Trash />
                            {/if}
                            <span>Delete</span>
                        </MenuActionItem>
                    {/snippet}
                    <AlertDialogContent size="sm">
                        <AlertDialogHeader>
                            <AlertDialogTitle>Delete article?</AlertDialogTitle>
                            <AlertDialogDescription>
                                This action will delete the article and it cannot be undone later.
                            </AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                            <AlertCloseAction />
                            <AlertContinueAction
                                closeOnClick
                                onclick={() => deleteArticle(article.id)}
                                variant="destructive"
                            >
                                <Trash />
                                <span>Delete</span>
                            </AlertContinueAction>
                        </AlertDialogFooter>
                    </AlertDialogContent>
                </AlertDialog>
            </MenuGroup>
        </MenuContent>
    </Menu>
{/snippet}
