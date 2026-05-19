<script>
    import Action from "$lib/components/ui/action.svelte";
    import Badge from "$lib/components/ui/badge.svelte";
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
