<script>
    import ErrorAlert from "$lib/components/error-alert.svelte";
    import Pending from "$lib/components/pending.svelte";
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
    <Pending />
{:else if article.isError}
    <ErrorAlert error={article.error} />
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
