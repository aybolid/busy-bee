<script>
    import Badge from "$lib/components/ui/badge.svelte";
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
    <p>Loading...</p>
{:else if article.isError}
    <p>Error: {article.error.message}</p>
{:else if article.isSuccess}
    <div class="flex flex-wrap gap-2">
        {#if article.data.byline}
            <Badge>{article.data.byline}</Badge>
        {/if}
        {#if article.data.published_time}
            <Badge variant="secondary">
                {dayjs(article.data.published_time).format("MMM DD, YYYY, HH:mm")}
            </Badge>
        {/if}
    </div>

    <article class="prose max-w-full pt-8 prose-neutral dark:prose-invert">
        <h1>{article.data.title}</h1>
        {@html article.data.content}
    </article>
{/if}
