<script>
    import Action from "$lib/components/ui/action.svelte";
    import Badge from "$lib/components/ui/badge.svelte";
    import PaginationAction from "$lib/components/ui/pagination/pagination-action.svelte";
    import PaginationContent from "$lib/components/ui/pagination/pagination-content.svelte";
    import PaginationEllipsis from "$lib/components/ui/pagination/pagination-ellipsis.svelte";
    import PaginationItem from "$lib/components/ui/pagination/pagination-item.svelte";
    import PaginationNext from "$lib/components/ui/pagination/pagination-next.svelte";
    import PaginationPrevious from "$lib/components/ui/pagination/pagination-previous.svelte";
    import Pagination from "$lib/components/ui/pagination/pagination.svelte";
    import TableBody from "$lib/components/ui/table/table-body.svelte";
    import TableCell from "$lib/components/ui/table/table-cell.svelte";
    import TableHead from "$lib/components/ui/table/table-head.svelte";
    import TableHeader from "$lib/components/ui/table/table-header.svelte";
    import TableRow from "$lib/components/ui/table/table-row.svelte";
    import Table from "$lib/components/ui/table/table.svelte";
    import { getArticlesQueryOptions } from "$lib/query/articles";
    import { createQuery } from "@tanstack/svelte-query";
    import dayjs from "dayjs";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    /** @type {import('$lib/api/articles').GetArticlesSearchParams} */
    const getArticlesSearchParams = $derived({
        page_index: props.data.searchParams.page_index,
        limit: props.data.searchParams.limit,
    });

    const articles = createQuery(function () {
        return getArticlesQueryOptions(props.data.ky, { searchParams: getArticlesSearchParams });
    });

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

<Table>
    <TableHeader>
        <TableRow>
            <TableHead>Title</TableHead>
            <TableHead>Author</TableHead>
            <TableHead>Published</TableHead>
            <TableHead>Created</TableHead>
            <TableHead></TableHead>
        </TableRow>
    </TableHeader>
    <TableBody>
        {#if articles.isLoading}
            <TableRow>
                <TableCell colspan={5} class="animate-pulse text-center">
                    Loading articles...
                </TableCell>
            </TableRow>
        {:else if articles.isError}
            <TableRow>
                <TableCell colspan={5} class="text-center text-destructive">
                    Error: {articles.error.message}
                </TableCell>
            </TableRow>
        {:else if articles.isSuccess}
            {#each articles.data.data as article (article.id)}
                <TableRow class="group">
                    <TableCell class="font-medium">{article.title}</TableCell>
                    <TableCell class="text-muted-foreground">
                        {article.byline ?? "--"}
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
                        <Action anchor href="/articles/{article.id}" size="xs">View</Action>
                    </TableCell>
                </TableRow>
            {/each}
        {/if}
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
