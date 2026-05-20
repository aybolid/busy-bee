<script>
    import ArticleStatus from "$lib/components/article-status.svelte";
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
    import Action from "$lib/components/ui/action.svelte";
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
    import Spinner from "$lib/components/ui/spinner.svelte";
    import TableBody from "$lib/components/ui/table/table-body.svelte";
    import TableCell from "$lib/components/ui/table/table-cell.svelte";
    import TableHead from "$lib/components/ui/table/table-head.svelte";
    import TableHeader from "$lib/components/ui/table/table-header.svelte";
    import TableRow from "$lib/components/ui/table/table-row.svelte";
    import Table from "$lib/components/ui/table/table.svelte";
    import { createDeleteArticleMutation, getArticlesQueryOptions } from "$lib/query/articles";
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

    function refreshArticles() {
        void props.data.queryClient.invalidateQueries({
            predicate: (q) => q.queryKey[0] === articlesQueryOptions.queryKey[0],
        });
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

{#if articles.isLoading}
    <Pending />
{:else if articles.isError}
    <ErrorAlert error={articles.error} />
{:else if articles.isSuccess}
    <div class="flex justify-end pb-8">
        <Action button variant="secondary" onclick={refreshArticles}>
            {#if articles.isFetching}
                <Spinner />
            {:else}
                <Refresh />
            {/if}
            <span>Refresh</span>
        </Action>
    </div>

    <Table>
        <TableHeader>
            <TableRow>
                <TableHead>Title</TableHead>
                <TableHead>Author</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Published</TableHead>
                <TableHead>Created</TableHead>
                <TableHead>Updated</TableHead>
                <TableHead></TableHead>
            </TableRow>
        </TableHeader>
        <TableBody>
            {#each articles.data.data as article (article.id)}
                <TableRow
                    class={[
                        "group",
                        article.status === "error" && "bg-destructive/10 hover:bg-destructive/15!",
                    ]}
                >
                    <TableCell class="max-w-80 truncate font-medium">
                        {article.title}
                    </TableCell>
                    <TableCell class="text-muted-foreground">
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
                    <TableCell>
                        <Menu>
                            {#snippet trigger(props)}
                                <Action button size="icon-xs" variant="outline" {...props}>
                                    <EllipsisVertical />
                                </Action>
                            {/snippet}
                            <MenuContent>
                                <MenuGroup>
                                    <MenuLabel>Article actions</MenuLabel>
                                    <MenuActionItem anchor href="/articles/{article.id}">
                                        View
                                    </MenuActionItem>
                                    {#if article.url && article.url.startsWith("http")}
                                        <MenuActionItem anchor href={article.url} target="_blank">
                                            <ExternalLink />
                                            <span>View external</span>
                                        </MenuActionItem>
                                    {/if}
                                    {#if article.status === "new" || article.status === "error"}
                                        <MenuActionItem button>Create post</MenuActionItem>
                                    {/if}
                                    <MenuActionItem
                                        button
                                        variant="destructive"
                                        onclick={() => deleteArticle(article.id)}
                                        disabled={deleteMutation.isPending}
                                    >
                                        {#if deleteMutation.isPending}
                                            <Spinner />
                                        {:else}
                                            <Trash />
                                        {/if}
                                        <span>Delete</span>
                                    </MenuActionItem>
                                </MenuGroup>
                            </MenuContent>
                        </Menu>
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
