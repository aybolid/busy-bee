<script>
    import Badge from "$lib/components/ui/badge.svelte";
    import EmptyDescription from "$lib/components/ui/empty/empty-description.svelte";
    import EmptyHeader from "$lib/components/ui/empty/empty-header.svelte";
    import EmptyTitle from "$lib/components/ui/empty/empty-title.svelte";
    import Empty from "$lib/components/ui/empty/empty.svelte";
    import { getArticleQueryOptions } from "$lib/query/articles";
    import { createQuery } from "@tanstack/svelte-query";
    import dayjs from "dayjs";

    /** @type {import('./$types').PageProps} */
    const props = $props();

    const articleId = /** @type {import('$lib/api/articles').ArticleId} */ (
        $derived(props.params.articleId)
    );

    const article = createQuery(() =>
        getArticleQueryOptions(props.data.ky, { params: { id: articleId } }),
    );
</script>

{#if article.isLoading}
    <Empty class="animate-pulse">
        <EmptyHeader>
            <EmptyTitle>Loading article...</EmptyTitle>
            <EmptyDescription>This should not take long</EmptyDescription>
        </EmptyHeader>
    </Empty>
{:else if article.isError}
    <p class="text-destructive">Error: {article.error.message}</p>
{:else if article.isSuccess}
    <article class="mx-auto prose max-w-4xl pt-8 prose-neutral dark:prose-invert">
        <h1>{article.data.title}</h1>
        <div class="not-prose flex flex-wrap gap-2">
            {#if article.data.byline}
                <Badge>{article.data.byline}</Badge>
            {/if}
            {#if article.data.published_time}
                <Badge variant="secondary">
                    {dayjs(article.data.published_time).format("MMM DD, YYYY, HH:mm")}
                </Badge>
            {/if}
        </div>
        {@html article.data.content}
    </article>
{/if}
